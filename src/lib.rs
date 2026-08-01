use gate_calc_log_bits::calc_log_bits;
use gatesim::{Circuit, Gate, GateFunc};

// LE128 - encoding:
//
// Unsigned integer encoding (LEB128):
// * 7-bit integer: last 7 bit is 0.
// * 14-bit integer: 7 bit is 1, 15 bit is 0.
// * 21-bit integer: 7 bit is 1, 15 bit is 1, 23 bit is 0.
// * 28-bit integer: 7 bit is 1, 15 bit is 1, 23 bit is 1, 31 bit is 0.
// * 35-bit integer: 7 bit is 1, 15 bit is 1, 23 bit is 1, 31 bit is 1, 39 bit is 0.
// * n*7-bit integer: 7 bit is 1, 15 bit is 1, ....., n*8-1 bit is 0.
//
// File format:
// Header: 'GATB'.
// Info bit: 0.
// Circuit input length - LEB128 encoding.
// Circuit length - LEB128 encoding.
// Circuit output length - LEB128 encoding.
// Bitstream of GateFunc - entry is 2-bit. Number of entries = gate number.
//   Entry values: 0 - AND, 1 - NOR, 2 - NIMP, 3 - XOR.
//   Length aligned to byte.
// Bitstream of output negations. Entry is bit. Length aligned to byte.
// Gate wires entries. Entry holds two wire indices for two inputs of gates.
//   Wire index encoding. LEB128 cut to required bits of wire index.
//   If gate index is 2454 then requires 7+1+5 bits because last byte cut to 5 bits.
//   If integer stored in maximal number of bytes then last byte doesn't have end bit.
//   Wire index stored as cur_wire_index - 1 - wire_index, where cur_wire_index is
//   wire index of current gate.
//   Length aligned to byte.
// Output wires entries. Wire index stored as LEB128 in same format like in Gate wires entries.
//   Wire index stored as total_wire_num - 1 - wire_index, where total_wire_num is
//   total number of wires including gates.

// TODO: Write fast version of format:
// faster reading and writing by using constant bit length for gate arguments.

use std::fmt::Debug;
use std::io::{self, Read, Write};

#[derive(Debug)]
pub struct BinReader<R: Read> {
    reader: R,
    bits: u8,
    byte: u8,
}

#[inline]
fn bitmask_u8(bits: u8) -> u8 {
    if bits < 8 { (1u8 << bits) - 1 } else { 0xff }
}

// binary read and write

impl<R: Read> BinReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            bits: 0,
            byte: 0,
        }
    }

    pub fn read(&mut self) -> io::Result<bool> {
        if self.bits == 0 {
            let mut b = [0u8];
            self.reader.read_exact(&mut b)?;
            self.byte = b[0];
        }
        let v = ((self.byte >> self.bits) & 1) != 0;
        self.bits += 1;
        if self.bits == 8 {
            self.bits = 0;
        }
        Ok(v)
    }

    pub fn read_bits(&mut self, bits: u8) -> io::Result<u128> {
        if bits == 0 {
            return Ok(0);
        }
        assert!(bits <= 128);
        if self.bits + bits <= 8 {
            if self.bits == 0 {
                let mut b = [0u8];
                self.reader.read_exact(&mut b)?;
                self.byte = b[0];
            }
            let v = (u128::from(self.byte) >> self.bits) & u128::from(bitmask_u8(bits));
            self.bits += bits;
            if self.bits == 8 {
                self.bits = 0;
            }
            Ok(v)
        } else if self.bits == 0 {
            let mut b = [0u8; 16];
            self.reader
                .read_exact(&mut b[0..(((bits as usize) + 7) >> 3)])?;
            self.bits = bits & 7;
            if self.bits != 0 {
                self.byte = b[(bits as usize) >> 3];
            }
            let v = u128::from_le_bytes(b);
            Ok(if bits < 128 {
                v & ((1u128 << bits) - 1)
            } else {
                v
            })
        } else {
            let shift = 8 - self.bits;
            let new_bits = bits - shift;
            let mut v = u128::from(self.byte) >> self.bits;
            let mut b = [0u8; 16];
            self.reader
                .read_exact(&mut b[0..(((new_bits as usize) + 7) >> 3)])?;
            self.bits = new_bits & 7;
            if self.bits != 0 {
                self.byte = b[(new_bits as usize) >> 3];
            }
            v |= u128::from_le_bytes(b) << shift;
            Ok(if bits < 128 {
                v & ((1u128 << bits) - 1)
            } else {
                v
            })
        }
    }

    pub fn unread_bits(&self) -> u8 {
        if self.bits != 0 {
            self.byte >> self.bits
        } else {
            0
        }
    }

    pub fn inner(self) -> io::Result<R> {
        Ok(self.reader)
    }
}

#[derive(Debug)]
pub struct BinWriter<W: Write> {
    writer: Option<W>,
    bits: u8,
    byte: u8,
}

pub const BIN_WRITER_BUF_LEN: usize = 1024;

impl<W: Write> BinWriter<W> {
    #[inline]
    pub fn new(writer: W) -> Self {
        Self::new_with_buffer_len(writer, BIN_WRITER_BUF_LEN)
    }

    pub fn new_with_buffer_len(writer: W, buflen: usize) -> Self {
        assert_ne!(buflen, 0);
        Self {
            writer: Some(writer),
            bits: 0,
            byte: 0,
        }
    }

    pub fn write(&mut self, value: bool) -> io::Result<()> {
        self.byte |= u8::from(value) << self.bits;
        if self.bits == 7 {
            self.writer.as_mut().unwrap().write(&[self.byte])?;
            self.byte = 0;
            self.bits = 0;
        } else {
            self.bits += 1;
        }
        Ok(())
    }

    pub fn write_bits(&mut self, bits: u8, value: u128) -> io::Result<()> {
        if bits == 0 {
            return Ok(());
        }
        assert!(bits <= 128);
        let writer = self.writer.as_mut().unwrap();
        if self.bits == 0 {
            let leb = value.to_le_bytes();
            writer.write_all(&leb[0..(bits as usize) >> 3])?;
            self.byte = if bits < 128 {
                leb[(bits as usize) >> 3] & ((1 << (bits & 7)) - 1)
            } else {
                0
            };
            self.bits = bits & 7;
        } else {
            let shift = 8 - self.bits;
            let value0 = u8::try_from(value & ((1u128 << shift) - 1)).unwrap();
            self.byte |= value0 << self.bits;
            if bits > shift {
                let bits = bits - shift;
                let value1 = value >> shift;
                writer.write_all(&[self.byte])?;
                self.bits = 0;
                self.byte = 0;
                self.write_bits(bits, value1)?;
            } else {
                self.bits += bits;
                self.byte &= u8::try_from((1u16 << (self.bits)) - 1).unwrap();
                if self.bits == 8 {
                    writer.write_all(&[self.byte])?;
                    self.bits = 0;
                    self.byte = 0;
                }
            }
        }
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if self.bits != 0 {
            if let Some(w) = self.writer.as_mut() {
                w.write_all(&[self.byte])?;
                w.flush()?;
            }
            self.bits = 0;
            self.byte = 0;
        }
        Ok(())
    }

    pub fn inner(mut self) -> io::Result<W> {
        self.flush()?;
        Ok(self.writer.take().unwrap())
    }
}

impl<W: Write> Drop for BinWriter<W> {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

// other read writes

#[derive(Debug, thiserror::Error)]
pub enum ReadEncIntError {
    #[error("IO: {0}")]
    IO(#[from] io::Error),
    #[error("Positive overflow")]
    PosOverflow,
}

pub fn read_enc_usize<R: Read>(mut rd: R) -> Result<usize, ReadEncIntError> {
    const UBITS: u32 = usize::BITS;
    let mut d = 0;
    let mut bitpos = 0;
    let mut b = [0u8];
    loop {
        rd.read_exact(&mut b)?;
        let bv = b[0] & 0x7f;
        let bend = (b[0] & 0x80) == 0;
        if (bitpos >= UBITS && bv != 0)
            || (bitpos + 7 > UBITS && ((bv & !((1u8 << (UBITS - bitpos)) - 1)) != 0))
        {
            return Err(ReadEncIntError::PosOverflow);
        }
        if bitpos < UBITS {
            d |= (bv as usize) << bitpos;
        }
        if bend {
            break;
        }
        bitpos += 7;
        if bitpos >= UBITS {
            bitpos = UBITS; // stop at this position
        }
    }
    Ok(d)
}

pub fn write_enc_usize<W: Write>(mut wr: W, value: usize) -> Result<(), io::Error> {
    let mut d = value;
    let mut b = [0u8; 10];
    // start writing
    let mut len = 0;
    let bcont = d >= 128;
    b[len] = ((d & 127) as u8) | (u8::from(bcont) << 7);
    len += 1;
    d >>= 7;
    while d != 0 {
        let bcont = d >= 128;
        b[len] = ((d & 127) as u8) | (u8::from(bcont) << 7);
        len += 1;
        d >>= 7;
    }
    wr.write_all(&b[0..len])
}

pub fn read_enc_u64<R: Read>(mut rd: R) -> Result<u64, ReadEncIntError> {
    const UBITS: u32 = u64::BITS;
    let mut d = 0;
    let mut bitpos = 0;
    let mut b = [0u8];
    loop {
        rd.read_exact(&mut b)?;
        let bv = b[0] & 0x7f;
        let bend = (b[0] & 0x80) == 0;
        if (bitpos >= UBITS && bv != 0)
            || (bitpos + 7 > UBITS && ((bv & !((1u8 << (UBITS - bitpos)) - 1)) != 0))
        {
            return Err(ReadEncIntError::PosOverflow);
        }
        if bitpos < UBITS {
            d |= (bv as u64) << bitpos;
        }
        if bend {
            break;
        }
        bitpos += 7;
        if bitpos >= UBITS {
            bitpos = UBITS; // stop at this position
        }
    }
    Ok(d)
}

pub fn write_enc_u64<W: Write>(mut wr: W, value: u64) -> Result<(), io::Error> {
    let mut d = value;
    let mut b = [0u8; 10];
    // start writing
    let mut len = 0;
    let bcont = d >= 128;
    b[len] = ((d & 127) as u8) | (u8::from(bcont) << 7);
    len += 1;
    d >>= 7;
    while d != 0 {
        let bcont = d >= 128;
        b[len] = ((d & 127) as u8) | (u8::from(bcont) << 7);
        len += 1;
        d >>= 7;
    }
    wr.write_all(&b[0..len])
}

// cut read enc usize

pub fn read_cut_enc_usize<R: Read>(
    rd: &mut BinReader<R>,
    max_bits: u8,
) -> Result<usize, ReadEncIntError> {
    let mut d = 0;
    let mut bitpos = 0;
    while bitpos < max_bits {
        let this_bits = if bitpos + 7 < max_bits {
            8
        } else {
            max_bits - bitpos
        };
        let b = u8::try_from(rd.read_bits(this_bits)?).unwrap();
        let bv = b & 0x7f;
        let bend = (b & 0x80) == 0;
        if bitpos < max_bits {
            d |= (bv as usize) << bitpos;
        }
        if bend {
            break;
        }
        bitpos += std::cmp::min(7, max_bits - bitpos);
    }
    Ok(d)
}

pub fn write_cut_enc_usize<W: Write>(
    wr: &mut BinWriter<W>,
    value: usize,
    max_bits: u8,
) -> Result<(), io::Error> {
    if max_bits == 0 {
        return Ok(());
    }
    let mut d = value;
    let mut b = 0u128;
    let mut len = 0;
    let mut bits = 0u8;
    let bcont = d >= 128;
    if max_bits >= 8 {
        b |= (u128::try_from(d & 127).unwrap() | (u128::from(bcont) << 7)) << (len << 3);
        len += 1;
        bits += 8;
        d >>= 7;
        while d != 0 && (len * 7 + 7) < max_bits {
            let bcont = d >= 128;
            b |= (u128::try_from(d & 127).unwrap() | (u128::from(bcont) << 7)) << (len << 3);
            len += 1;
            bits += 8;
            d >>= 7;
        }
    }
    if d != 0 || max_bits < 8 {
        b |= u128::try_from(d & 127).unwrap() << (len << 3);
        bits += max_bits - (len * 7);
    }
    wr.write_bits(bits, b)
}

//

#[derive(Debug, thiserror::Error)]
pub enum ReadCircuitError {
    #[error("IO {0}")]
    IO(#[from] io::Error),
    #[error("EncInt {0}")]
    EncInt(#[from] ReadEncIntError),
    #[error("Bad header")]
    BadHeader,
    #[error("Verify")]
    Verify,
}

pub fn read_circuit_int<T, R: Read>(mut rd: R) -> Result<Circuit<T>, ReadCircuitError>
where
    T: Default + Clone + Copy + Ord,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    let input_len_u = read_enc_usize(&mut rd)?;
    let circuit_len_u = read_enc_usize(&mut rd)?;
    let output_len_u = read_enc_usize(&mut rd)?;
    // read gates func
    let mut binrd = BinReader::new(rd);
    let mut gates = vec![Gate::new_and(T::default(), T::default()); circuit_len_u];
    for gchunk in gates.chunks_mut(64) {
        let max_bits = u8::try_from(gchunk.len() << 1).unwrap();
        let gfunc_bits = binrd.read_bits(max_bits)?;
        for (i, gi) in gchunk.iter_mut().enumerate() {
            gi.func = match (gfunc_bits >> (i << 1)) & 3 {
                0 => GateFunc::And,
                1 => GateFunc::Nor,
                2 => GateFunc::Nimpl,
                3 => GateFunc::Xor,
                _ => {
                    panic!("Unexpected!");
                }
            };
        }
    }
    let rd = binrd.inner()?;
    // read output negations
    let mut binrd = BinReader::new(rd);
    let mut outputs = vec![(T::default(), false); output_len_u];
    for ochunk in outputs.chunks_mut(128) {
        let max_bits = u8::try_from(ochunk.len()).unwrap();
        let neg_bits = binrd.read_bits(max_bits)?;
        for (i, (_, on)) in ochunk.iter_mut().enumerate() {
            *on = ((neg_bits >> i) & 1) != 0;
        }
    }
    let rd = binrd.inner()?;
    // read gate wires
    let mut binrd = BinReader::new(rd);
    for (gi, gate) in gates.iter_mut().enumerate() {
        let wi = gi + input_len_u;
        let max_bits = u8::try_from(calc_log_bits(wi)).unwrap();
        let gi0 = wi - read_cut_enc_usize(&mut binrd, max_bits)? - 1;
        let gi1 = wi - read_cut_enc_usize(&mut binrd, max_bits)? - 1;
        gate.i0 = T::try_from(gi0).unwrap();
        gate.i1 = T::try_from(gi1).unwrap();
    }
    let rd = binrd.inner()?;
    // read output wires
    let mut binrd = BinReader::new(rd);
    let total_wire_num = input_len_u + circuit_len_u;
    let max_bits = u8::try_from(calc_log_bits(total_wire_num)).unwrap();
    for (o, _n) in outputs.iter_mut() {
        let o_u = total_wire_num - read_cut_enc_usize(&mut binrd, max_bits)? - 1;
        *o = T::try_from(o_u).unwrap();
    }
    binrd.inner()?;
    let input_len = T::try_from(input_len_u).unwrap();
    Circuit::new(input_len, gates, outputs).ok_or(ReadCircuitError::Verify)
}

pub fn read_circuit<T, R: Read>(mut rd: R) -> Result<Circuit<T>, ReadCircuitError>
where
    T: Default + Clone + Copy + Ord,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    let mut header = [0u8; 5];
    rd.read_exact(&mut header)?;
    if header != [0x47, 0x41, 0x54, 0x42, 0x00] {
        return Err(ReadCircuitError::BadHeader);
    }
    read_circuit_int(rd)
}

//

pub fn write_circuit_int<T, W: Write>(mut wr: W, circuit: &Circuit<T>) -> io::Result<()>
where
    T: Clone + Copy + Ord,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    let input_len_u = usize::try_from(circuit.input_len()).unwrap();
    let circuit_len_u = circuit.len();
    let output_len_u = circuit.outputs().len();
    write_enc_usize(&mut wr, input_len_u)?;
    write_enc_usize(&mut wr, circuit_len_u)?;
    write_enc_usize(&mut wr, output_len_u)?;
    // write gate funcs
    let mut binwr = BinWriter::new(wr);
    for gchunk in circuit.gates().chunks(64) {
        let gvalue = gchunk.into_iter().enumerate().fold(0u128, |a, (gi, g)| {
            a | ((match g.func {
                GateFunc::And => 0u128,
                GateFunc::Nor => 1u128,
                GateFunc::Nimpl => 2u128,
                GateFunc::Xor => 3u128,
            }) << (gi << 1))
        });
        binwr.write_bits(u8::try_from(gchunk.len() << 1).unwrap(), gvalue)?;
    }
    let wr = binwr.inner()?;
    // write output negations
    let mut binwr = BinWriter::new(wr);
    for ochunk in circuit.outputs().chunks(128) {
        let nvalue = ochunk
            .into_iter()
            .enumerate()
            .fold(0u128, |a, (oi, (_, n))| a | (u128::from(*n) << oi));
        binwr.write_bits(u8::try_from(ochunk.len()).unwrap(), nvalue)?;
    }
    let wr = binwr.inner()?;
    // write gate wires (arguments)
    let mut binwr = BinWriter::new(wr);
    for (gi, gate) in circuit.gates().into_iter().enumerate() {
        let wi = gi + input_len_u;
        let max_bits = u8::try_from(calc_log_bits(wi)).unwrap();
        let gi0 = usize::try_from(gate.i0).unwrap();
        let gi1 = usize::try_from(gate.i1).unwrap();
        write_cut_enc_usize(&mut binwr, wi - gi0 - 1, max_bits)?;
        write_cut_enc_usize(&mut binwr, wi - gi1 - 1, max_bits)?;
    }
    let wr = binwr.inner()?;
    // write outputs wire indices
    let mut binwr = BinWriter::new(wr);
    let total_wire_num = input_len_u + circuit_len_u;
    let max_bits = u8::try_from(calc_log_bits(total_wire_num)).unwrap();
    for (o, _) in circuit.outputs() {
        let o_u = usize::try_from(*o).unwrap();
        write_cut_enc_usize(&mut binwr, total_wire_num - o_u - 1, max_bits)?;
    }
    binwr.inner()?;
    Ok(())
}

pub fn write_circuit<T, W: Write>(mut wr: W, circuit: &Circuit<T>) -> io::Result<()>
where
    T: Clone + Copy + Ord,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    wr.write(&[0x47, 0x41, 0x54, 0x42, 0x00])?;
    write_circuit_int(wr, circuit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_bin_writer() {
        let mut writer = BinWriter::new(vec![]);
        for v in [false, false, true, false, false, true, true, false] {
            writer.write(v).unwrap();
        }
        assert_eq!(vec![0b01100100], writer.inner().unwrap());
        let mut writer = BinWriter::new(vec![]);
        for v in [false, false, true, false, false, true, true] {
            writer.write(v).unwrap();
        }
        assert_eq!(7, writer.bits);
        assert_eq!(vec![0b01100100], writer.inner().unwrap());
        let mut writer = BinWriter::new(vec![]);
        for v in [
            false, false, true, false, false, true, true, false, false, false, true, false, true,
            true, false, true,
        ] {
            writer.write(v).unwrap();
        }
        assert_eq!(vec![0b01100100, 0b10110100], writer.inner().unwrap());
    }

    #[test]
    fn test_bin_writer_write_bits() {
        let mut writer = BinWriter::new(vec![]);
        writer.write_bits(6, 0x72).unwrap();
        assert_eq!(6, writer.bits);
        assert_eq!(vec![0x32], writer.inner().unwrap());
        let mut writer = BinWriter::new(vec![]);
        writer.write_bits(24, 0xba5e72).unwrap();
        assert_eq!(0, writer.bits);
        assert_eq!(vec![0x72, 0x5e, 0xba], writer.inner().unwrap());
        let mut writer = BinWriter::new(vec![]);
        writer.write_bits(23, 0xba5e72).unwrap();
        assert_eq!(7, writer.bits);
        assert_eq!(vec![0x72, 0x5e, 0x3a], writer.inner().unwrap());
        let mut writer = BinWriter::new(vec![]);
        writer.write_bits(0, 0xba5e72).unwrap();
        assert_eq!(0, writer.bits);
        assert_eq!(Vec::<u8>::new(), writer.inner().unwrap());
        // unaligned
        let mut writer = BinWriter::new(vec![]);
        writer.write_bits(5, 0x72).unwrap();
        writer.write_bits(24, 0xe76da5).unwrap();
        assert_eq!(5, writer.bits);
        assert_eq!(vec![0xb2, 0xb4, 0xed, 0x1c], writer.inner().unwrap());
        let mut writer = BinWriter::new(vec![]);
        writer.write_bits(5, 0x72).unwrap();
        writer.write_bits(19, 0xe76da5).unwrap();
        assert_eq!(0, writer.bits);
        assert_eq!(vec![0xb2, 0xb4, 0xed], writer.inner().unwrap());
        let mut writer = BinWriter::new(vec![]);
        writer.write_bits(5, 0x72).unwrap();
        writer.write_bits(18, 0xe76da5).unwrap();
        assert_eq!(7, writer.bits);
        assert_eq!(vec![0xb2, 0xb4, 0x6d], writer.inner().unwrap());
        let mut writer = BinWriter::new(vec![]);
        writer.write_bits(5, 0x72).unwrap();
        writer.write_bits(3, 0x5).unwrap();
        assert_eq!(0, writer.bits);
        assert_eq!(vec![0xb2], writer.inner().unwrap());
        let mut writer = BinWriter::new(vec![]);
        writer.write_bits(5, 0x72).unwrap();
        writer.write_bits(2, 0x5).unwrap();
        assert_eq!(7, writer.bits);
        assert_eq!(vec![0x32], writer.inner().unwrap());
        let mut writer = BinWriter::new(vec![]);
        writer.write_bits(5, 0x72).unwrap();
        writer.write_bits(0, 0xba5e72).unwrap();
        assert_eq!(5, writer.bits);
        assert_eq!(vec![0x12], writer.inner().unwrap());
    }

    #[test]
    fn test_bin_reader() {
        let data = &[0xda, 0xb5, 0x3c, 0x94][..];
        let mut reader = BinReader::new(data);
        for b in 0..32usize {
            assert_eq!(reader.bits, u8::try_from(b & 7).unwrap());
            assert_eq!(((data[b >> 3] >> (b & 7)) & 1) != 0, reader.read().unwrap());
        }
    }

    #[test]
    fn test_bin_reader_read_bits() {
        let data = &[0xda, 0xb5, 0x3c, 0x94, 0xe9, 0x2a][..];
        let mut reader = BinReader::new(data);
        assert_eq!(0, reader.read_bits(0).unwrap());
        assert_eq!(0, reader.bits);
        assert_eq!(0xda & 7, reader.read_bits(3).unwrap());
        assert_eq!(0, reader.read_bits(0).unwrap());
        assert_eq!(
            ((0xb5 << 5) | (0xda >> 3)) & 0x7f,
            reader.read_bits(7).unwrap()
        );
        assert_eq!(2, reader.bits);
        assert_eq!((0xb5 >> 2) & 7, reader.read_bits(3).unwrap());
        assert_eq!(5, reader.bits);
        assert_eq!((0xb5 >> 5) & 7, reader.read_bits(3).unwrap());
        assert_eq!(0, reader.bits);
        assert_eq!(0x3c & 15, reader.read_bits(4).unwrap());
        assert_eq!(4, reader.bits);
        let mut reader = BinReader::new(data);
        assert_eq!(0xda, reader.read_bits(8).unwrap());
        //
        let data = &[0xda, 0xb5, 0x3c, 0x94, 0xe9, 0x2a][..];
        let mut reader = BinReader::new(data);
        assert_eq!(0xb5da & 0x3fff, reader.read_bits(14).unwrap());
        assert_eq!(6, reader.bits);
        // unaligned
        assert_eq!(
            ((0xe9943cb5) >> 6) & 0x1fffff,
            reader.read_bits(21).unwrap()
        );
        assert_eq!(3, reader.bits);
        //
        let data = &[0xda, 0xb5, 0x3f, 0x94, 0xe9, 0x2a][..];
        let mut reader = BinReader::new(data);
        assert_eq!(0x3fb5da & 0x3ffff, reader.read_bits(18).unwrap());
        assert_eq!(2, reader.bits);
        // unaligned
        assert_eq!(((0xe9943f) >> 2) & 0x1fffff, reader.read_bits(21).unwrap());
        assert_eq!(7, reader.bits);
        assert_eq!(((0x2ae9) >> 7) & 15, reader.read_bits(4).unwrap());
        assert_eq!(3, reader.bits);
        //
        let data = &[0xda, 0xb5, 0x3f, 0x94, 0xe9, 0x2a][..];
        let mut reader = BinReader::new(data);
        assert_eq!(0x3fb5da, reader.read_bits(24).unwrap());
        assert_eq!(0, reader.bits);
        assert_eq!(0x2ae994, reader.read_bits(24).unwrap());
        assert_eq!(0, reader.bits);
        //
        let data = &[0xda, 0xb5, 0x3f, 0x94, 0xe9, 0x2a][..];
        let mut reader = BinReader::new(data);
        assert_eq!(0xda & 0x3, reader.read_bits(2).unwrap());
        // unaligned
        assert_eq!(((0x3fb5da) >> 2) & 0x3fffff, reader.read_bits(22).unwrap());
        assert_eq!(0, reader.bits);
        //
        let data = &[0xda, 0xb5, 0x3f, 0x94, 0xe9, 0x2a][..];
        let mut reader = BinReader::new(data);
        assert_eq!(0xda & 0x3, reader.read_bits(2).unwrap());
        // unaligned
        assert_eq!(((0x3fb5da) >> 2) & 0x7ffff, reader.read_bits(19).unwrap());
        assert_eq!(5, reader.bits);
        //
        let data = &[0xda, 0xb5, 0x3f, 0x94, 0xe9, 0x2a][..];
        let mut reader = BinReader::new(data);
        assert_eq!(0xda & 0x3, reader.read_bits(2).unwrap());
        // unaligned
        assert_eq!(((0x3fb5da) >> 2) & 0xffff, reader.read_bits(16).unwrap());
        assert_eq!(2, reader.bits);
        assert_eq!(((0x3f) >> 2) & 0x1f, reader.read_bits(5).unwrap());
        assert_eq!(7, reader.bits);
    }

    //

    #[test]
    fn test_read_enc_usize() {
        assert_eq!(Ok(0), read_enc_usize(&[0u8][..]).map_err(|e| e.to_string()));
        assert_eq!(
            Ok(83),
            read_enc_usize(&[83u8][..]).map_err(|e| e.to_string())
        );
        assert_eq!(
            Ok(83),
            read_enc_usize(&[83u8, 12][..]).map_err(|e| e.to_string())
        );
        assert_eq!(
            Ok(0x1e | (38 << 7)),
            read_enc_usize(&[0x9eu8, 38][..]).map_err(|e| e.to_string())
        );
        assert_eq!(
            Ok(0x1e | (0x63 << 7) | (0x2d << 14) | (43 << 21)),
            read_enc_usize(&[0x9eu8, 0xe3, 0xad, 43][..]).map_err(|e| e.to_string())
        );
        assert_eq!(
            Ok(0x1e | (0x63 << 7) | (0x2d << 14) | (43 << 21)),
            read_enc_usize(
                &[
                    0x9eu8, 0xe3, 0xad, 0xab, 128, 128, 128, 128, 128, 128, 128, 128, 0
                ][..]
            )
            .map_err(|e| e.to_string())
        );
        assert_eq!(
            Err("Positive overflow".to_string()),
            read_enc_usize(
                &[
                    0x9eu8, 0xe3, 0xad, 0xab, 128, 128, 128, 128, 128, 128, 128, 129, 0
                ][..]
            )
            .map_err(|e| e.to_string())
        );
        assert_eq!(
            Err("Positive overflow".to_string()),
            read_enc_usize(&[0x91u8, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x02][..])
                .map_err(|e| e.to_string())
        );
        assert_eq!(
            Err("Positive overflow".to_string()),
            read_enc_usize(&[0x91u8, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x53][..])
                .map_err(|e| e.to_string())
        );
    }

    fn write_enc_usize_helper(value: usize) -> Vec<u8> {
        let mut wr = vec![];
        write_enc_usize(&mut wr, value).unwrap();
        wr
    }

    #[test]
    fn test_write_enc_usize() {
        assert_eq!(vec![0u8], write_enc_usize_helper(0));
        assert_eq!(vec![34u8], write_enc_usize_helper(34));
        assert_eq!(vec![196, 2], write_enc_usize_helper(324));
        assert_eq!(
            vec![43 | 128, 101 | 128, 69],
            write_enc_usize_helper(43 | (101 << 7) | (69 << 14))
        );
        assert_eq!(
            vec![77 | 128, 128, 56 | 128, 128, 114 | 128, 9],
            write_enc_usize_helper(
                77 | (0 << 7) | (56 << 14) | (0 << 21) | (114 << 28) | (9 << 35)
            )
        );
        assert_eq!(
            vec![0x9eu8, 0xe3, 0xad, 43],
            write_enc_usize_helper(0x1e | (0x63 << 7) | (0x2d << 14) | (43 << 21))
        );
    }

    // returns value and last bits of position
    fn read_cut_enc_usize_helper(data: &[u8], max_bits: u8) -> (usize, u8) {
        let mut rd = BinReader::new(data);
        let value = read_cut_enc_usize(&mut rd, max_bits).unwrap();
        (value, rd.bits)
    }

    #[test]
    fn test_read_cut_enc_usize() {
        assert_eq!((0, 0), read_cut_enc_usize_helper(&[][..], 0));
        assert_eq!((3, 3), read_cut_enc_usize_helper(&[3][..], 3));
        assert_eq!((5, 3), read_cut_enc_usize_helper(&[5][..], 3));
        assert_eq!((39, 7), read_cut_enc_usize_helper(&[39][..], 7));
        assert_eq!((106, 7), read_cut_enc_usize_helper(&[106][..], 7));
        assert_eq!((106, 0), read_cut_enc_usize_helper(&[106][..], 8));
        assert_eq!(
            (128 + 57, 1),
            read_cut_enc_usize_helper(&[128 | 57, 1][..], 8)
        );
        assert_eq!((57, 0), read_cut_enc_usize_helper(&[57, 0][..], 8));
        assert_eq!((57, 1), read_cut_enc_usize_helper(&[128 | 57, 0][..], 8));
        assert_eq!(
            (115 + 640, 3),
            read_cut_enc_usize_helper(&[128 | 115, 5][..], 10)
        );
        assert_eq!(
            (115 + 640, 3),
            read_cut_enc_usize_helper(&[128 | 115, 5][..], 10)
        );
        assert_eq!(
            (37 + (71 << 7), 7),
            read_cut_enc_usize_helper(&[128 | 37, 71][..], 14)
        );
        assert_eq!((45, 0), read_cut_enc_usize_helper(&[45][..], 16));
        assert_eq!((37, 0), read_cut_enc_usize_helper(&[37][..], 16));
        assert_eq!(
            (37 + (71 << 7), 0),
            read_cut_enc_usize_helper(&[128 | 37, 71][..], 16)
        );
        assert_eq!(
            (25 + (81 << 7) + (2 << 14), 2),
            read_cut_enc_usize_helper(&[128 | 25, 81 | 128, 2][..], 16)
        );
        assert_eq!(
            (25 + (81 << 7) + (2 << 14), 7),
            read_cut_enc_usize_helper(&[128 | 25, 81 | 128, 2][..], 21)
        );
    }

    // returns data and last bits in last byte.
    fn write_cut_enc_usize_helper(value: usize, max_bits: u8) -> (Vec<u8>, u8) {
        let data = vec![];
        let mut wr = BinWriter::new(data);
        write_cut_enc_usize(&mut wr, value, max_bits).unwrap();
        let bits = wr.bits;
        let data = wr.inner().unwrap();
        (data, bits)
    }

    #[test]
    fn test_write_cut_enc_usize() {
        assert_eq!((vec![], 0), write_cut_enc_usize_helper(0, 0));
        assert_eq!((vec![0], 3), write_cut_enc_usize_helper(0, 3));
        assert_eq!((vec![3], 3), write_cut_enc_usize_helper(3, 3));
        assert_eq!((vec![5], 3), write_cut_enc_usize_helper(5, 3));
        assert_eq!((vec![27], 5), write_cut_enc_usize_helper(27, 5));
        assert_eq!((vec![0], 7), write_cut_enc_usize_helper(0, 7));
        assert_eq!((vec![41], 7), write_cut_enc_usize_helper(41, 7));
        assert_eq!((vec![103], 7), write_cut_enc_usize_helper(103, 7));
        assert_eq!((vec![0], 0), write_cut_enc_usize_helper(0, 8));
        assert_eq!((vec![54], 0), write_cut_enc_usize_helper(54, 8));
        assert_eq!(
            (vec![128 | 57, 1], 1),
            write_cut_enc_usize_helper(128 + 57, 8)
        );
        assert_eq!((vec![0], 0), write_cut_enc_usize_helper(0, 10));
        assert_eq!(
            (vec![115 | 128, 5], 3),
            write_cut_enc_usize_helper(115 + 640, 10)
        );
        assert_eq!((vec![0], 0), write_cut_enc_usize_helper(0, 14));
        assert_eq!((vec![81], 0), write_cut_enc_usize_helper(81, 14));
        assert_eq!(
            (vec![81 | 128, 78], 7),
            write_cut_enc_usize_helper(81 + (78 << 7), 14)
        );
        assert_eq!((vec![0], 0), write_cut_enc_usize_helper(0, 19));
        assert_eq!((vec![81], 0), write_cut_enc_usize_helper(81, 19));
        assert_eq!(
            (vec![81 | 128, 78], 0),
            write_cut_enc_usize_helper(81 + (78 << 7), 19)
        );
        assert_eq!(
            (vec![81 | 128, 78 | 128, 13], 5),
            write_cut_enc_usize_helper(81 + (78 << 7) + (13 << 14), 19)
        );
    }

    fn test_read_write_circuit_helper(circuit: Circuit<u32>, msg: &str) {
        let mut data = vec![];
        write_circuit(&mut data, &circuit).unwrap();
        // println!("Written {}: {} {:?}", msg, data.len(), data);
        let rd_circuit = read_circuit(data.as_slice()).unwrap();
        assert_eq!(circuit, rd_circuit, "{}", msg);
    }

    #[test]
    fn test_read_write_circuit() {
        test_read_write_circuit_helper(Circuit::new(0, [], []).unwrap(), "empty");
        test_read_write_circuit_helper(
            Circuit::new(2, [Gate::new_and(0, 1)], [(2, false)]).unwrap(),
            "gate1",
        );
        test_read_write_circuit_helper(
            Circuit::new(2, [Gate::new_nimpl(1, 0)], [(2, true)]).unwrap(),
            "gate2",
        );
        test_read_write_circuit_helper(
            Circuit::new(
                3,
                [
                    Gate::new_xor(0, 1),
                    Gate::new_xor(2, 3),
                    Gate::new_nimpl(2, 3),
                    Gate::new_and(0, 1),
                    Gate::new_nor(5, 6),
                ],
                [(4, false), (7, true)],
            )
            .unwrap(),
            "simple1",
        );
        test_read_write_circuit_helper(
            Circuit::new(
                6,
                [
                    Gate::new_and(0, 1),
                    Gate::new_and(2, 3),
                    Gate::new_nor(7, 4),
                    Gate::new_xor(6, 2),
                    Gate::new_and(8, 6),
                    Gate::new_and(10, 7),
                    Gate::new_nor(5, 1),
                    Gate::new_nimpl(9, 12),
                    Gate::new_nimpl(11, 5),
                    Gate::new_and(0, 1),
                    Gate::new_nimpl(2, 3),
                    Gate::new_nimpl(16, 4),
                    Gate::new_nor(15, 2),
                    Gate::new_and(17, 15),
                    Gate::new_xor(19, 16),
                    Gate::new_nor(5, 1),
                    Gate::new_nimpl(18, 21),
                    Gate::new_nimpl(20, 5),
                    Gate::new_xor(10, 18),
                    Gate::new_xor(9, 19),
                    Gate::new_xor(13, 23),
                    Gate::new_xor(14, 22),
                    Gate::new_nor(24, 25),
                    Gate::new_and(26, 28),
                    Gate::new_and(27, 29),
                ],
                [(30, false)],
            )
            .unwrap(),
            "complex1",
        );
        test_read_write_circuit_helper(
            Circuit::from_str(
                r##"{
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
        and(0,6):0
        and(0,7)
        and(1,6)
        xor(13,14):1
        and(0,8)
        and(1,7)
        and(2,6)
        xor(17,18)
        xor(16,19)
        and(13,14)
        xor(20,21):2
        and(0,9)
        and(1,8)
        xor(23,24)
        and(2,7)
        and(3,6)
        xor(26,27)
        xor(25,28)
        and(17,18)
        xor(29,30)
        and(20,21)
        and(16,19)
        nor(32,33)
        xor(31,34):3n
        and(0,10)
        and(1,9)
        and(2,8)
        xor(37,38)
        and(3,7)
        and(4,6)
        xor(40,41)
        xor(39,42)
        xor(36,43)
        and(26,27)
        xor(44,45)
        and(25,28)
        and(23,24)
        nor(47,48)
        xor(46,49)
        nimpl(31,34)
        and(29,30)
        nor(51,52)
        xor(50,53):4
        and(0,11)
        and(1,10)
        and(2,9)
        xor(56,57)
        and(3,8)
        xor(58,59)
        and(4,7)
        and(5,6)
        xor(61,62)
        xor(60,63)
        and(40,41)
        xor(64,65)
        xor(55,66)
        and(39,42)
        and(37,38)
        nor(68,69)
        xor(67,70)
        and(44,45)
        and(36,43)
        nor(72,73)
        xor(71,74)
        nor(50,53)
        nimpl(46,49)
        nor(76,77)
        xor(75,78):5n
        and(1,11)
        and(2,10)
        xor(80,81)
        and(3,9)
        xor(82,83)
        and(4,8)
        and(5,7)
        xor(85,86)
        and(58,59)
        and(56,57)
        nor(88,89)
        xor(87,90)
        and(61,62)
        xor(91,92)
        xor(84,93)
        and(64,65)
        and(60,63)
        nor(95,96)
        xor(94,97)
        nimpl(67,70)
        and(55,66)
        nor(99,100)
        xor(98,101)
        nimpl(75,78)
        nor(71,74)
        nor(103,104)
        xor(102,105):6
        and(2,11)
        and(3,10)
        and(4,9)
        xor(108,109)
        and(5,8)
        xor(110,111)
        and(82,83)
        and(80,81)
        nor(113,114)
        xor(112,115)
        and(85,86)
        xor(116,117)
        xor(107,118)
        nimpl(92,91)
        nimpl(87,90)
        nor(120,121)
        xor(119,122)
        nor(94,97)
        nimpl(84,93)
        nor(124,125)
        xor(123,126)
        nor(102,105)
        nimpl(98,101)
        nor(128,129)
        xor(127,130):7
        and(3,11)
        and(4,10)
        and(5,9)
        xor(133,134)
        and(110,111)
        and(108,109)
        nor(136,137)
        xor(135,138)
        xor(132,139)
        nimpl(117,116)
        nimpl(112,115)
        nor(141,142)
        xor(140,143)
        nor(119,122)
        nimpl(107,118)
        nor(145,146)
        xor(144,147)
        nor(127,130)
        nimpl(123,126)
        nor(149,150)
        xor(148,151):8
        and(4,11)
        and(5,10)
        xor(153,154)
        nimpl(135,138)
        and(133,134)
        nor(156,157)
        xor(155,158)
        nor(140,143)
        nimpl(132,139)
        nor(160,161)
        xor(159,162)
        nor(148,151)
        nimpl(144,147)
        nor(164,165)
        xor(163,166):9n
        and(5,11)
        nimpl(155,158)
        and(153,154)
        nor(169,170)
        xor(168,171)
        nimpl(163,166)
        nor(159,162)
        nor(173,174)
        xor(172,175):10
        nor(172,175)
        nimpl(168,171)
        nor(177,178):11n
    }(12)
"##,
            )
            .unwrap(),
            "complex2",
        );
        test_read_write_circuit_helper(
            Circuit::from_str(
                r##"{
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
        12
        13
        14
        15
        and(0,8):0
        and(0,9)
        and(1,8)
        xor(17,18):1
        and(0,10)
        and(1,9)
        and(2,8)
        xor(21,22)
        xor(20,23)
        and(17,18)
        xor(24,25):2
        and(0,11)
        and(1,10)
        xor(27,28)
        and(2,9)
        and(3,8)
        xor(30,31)
        xor(29,32)
        and(21,22)
        xor(33,34)
        and(24,25)
        and(20,23)
        nor(36,37)
        xor(35,38):3n
        and(0,12)
        and(1,11)
        and(2,10)
        xor(41,42)
        and(3,9)
        and(4,8)
        xor(44,45)
        xor(43,46)
        xor(40,47)
        and(30,31)
        xor(48,49)
        and(29,32)
        and(27,28)
        nor(51,52)
        xor(50,53)
        nimpl(35,38)
        and(33,34)
        nor(55,56)
        xor(54,57):4
        and(0,13)
        and(1,12)
        and(2,11)
        xor(60,61)
        and(3,10)
        xor(62,63)
        and(4,9)
        and(5,8)
        xor(65,66)
        xor(64,67)
        and(44,45)
        xor(68,69)
        xor(59,70)
        and(43,46)
        and(41,42)
        nor(72,73)
        xor(71,74)
        and(48,49)
        and(40,47)
        nor(76,77)
        xor(75,78)
        nor(54,57)
        nimpl(50,53)
        nor(80,81)
        xor(79,82):5n
        and(0,14)
        and(1,13)
        xor(84,85)
        and(2,12)
        xor(86,87)
        and(3,11)
        and(4,10)
        xor(89,90)
        and(5,9)
        and(6,8)
        xor(92,93)
        xor(91,94)
        and(62,63)
        and(60,61)
        nor(96,97)
        xor(95,98)
        and(65,66)
        xor(99,100)
        xor(88,101)
        and(68,69)
        and(64,67)
        nor(103,104)
        xor(102,105)
        nimpl(71,74)
        and(59,70)
        nor(107,108)
        xor(106,109)
        nimpl(79,82)
        nor(75,78)
        nor(111,112)
        xor(110,113):6
        and(0,15)
        and(1,14)
        xor(115,116)
        and(2,13)
        xor(117,118)
        and(3,12)
        and(4,11)
        xor(120,121)
        and(5,10)
        xor(122,123)
        and(6,9)
        and(7,8)
        xor(125,126)
        xor(124,127)
        and(92,93)
        xor(128,129)
        and(86,87)
        and(84,85)
        nor(131,132)
        xor(130,133)
        and(91,94)
        and(89,90)
        nor(135,136)
        xor(134,137)
        xor(119,138)
        nimpl(100,99)
        nimpl(95,98)
        nor(140,141)
        xor(139,142)
        nor(102,105)
        nimpl(88,101)
        nor(144,145)
        xor(143,146)
        nor(110,113)
        nimpl(106,109)
        nor(148,149)
        xor(147,150):7n
        and(1,15)
        and(2,14)
        xor(152,153)
        and(3,13)
        and(4,12)
        xor(155,156)
        and(5,11)
        xor(157,158)
        xor(154,159)
        and(6,10)
        and(7,9)
        xor(161,162)
        and(122,123)
        and(120,121)
        nor(164,165)
        xor(163,166)
        and(125,126)
        xor(167,168)
        and(117,118)
        and(115,116)
        nor(170,171)
        xor(169,172)
        and(128,129)
        and(124,127)
        nor(174,175)
        xor(173,176)
        xor(160,177)
        nor(134,137)
        nimpl(130,133)
        nor(179,180)
        xor(178,181)
        nimpl(139,142)
        and(119,138)
        nor(183,184)
        xor(182,185)
        nimpl(147,150)
        nor(143,146)
        nor(187,188)
        xor(186,189):8
        and(2,15)
        and(3,14)
        xor(191,192)
        and(4,13)
        xor(193,194)
        and(5,12)
        and(6,11)
        xor(196,197)
        and(7,10)
        xor(198,199)
        and(157,158)
        and(155,156)
        nor(201,202)
        xor(200,203)
        and(161,162)
        xor(204,205)
        and(154,159)
        and(152,153)
        nor(207,208)
        xor(206,209)
        nimpl(168,167)
        nimpl(163,166)
        nor(211,212)
        xor(210,213)
        xor(195,214)
        nimpl(173,176)
        nor(169,172)
        nor(216,217)
        xor(215,218)
        nor(178,181)
        nimpl(160,177)
        nor(220,221)
        xor(219,222)
        nor(186,189)
        nimpl(182,185)
        nor(224,225)
        xor(223,226):9
        and(3,15)
        and(4,14)
        xor(228,229)
        and(5,13)
        xor(230,231)
        and(6,12)
        and(7,11)
        xor(233,234)
        and(198,199)
        and(196,197)
        nor(236,237)
        xor(235,238)
        and(193,194)
        and(191,192)
        nor(240,241)
        xor(239,242)
        nimpl(205,204)
        nimpl(200,203)
        nor(244,245)
        xor(243,246)
        xor(232,247)
        nimpl(210,213)
        nor(206,209)
        nor(249,250)
        xor(248,251)
        nor(215,218)
        nimpl(195,214)
        nor(253,254)
        xor(252,255)
        nor(223,226)
        nimpl(219,222)
        nor(257,258)
        xor(256,259):10
        and(4,15)
        and(5,14)
        and(6,13)
        xor(262,263)
        and(7,12)
        xor(264,265)
        and(230,231)
        and(228,229)
        nor(267,268)
        xor(266,269)
        nimpl(235,238)
        and(233,234)
        nor(271,272)
        xor(270,273)
        xor(261,274)
        nimpl(243,246)
        nor(239,242)
        nor(276,277)
        xor(275,278)
        nor(248,251)
        nimpl(232,247)
        nor(280,281)
        xor(279,282)
        nor(256,259)
        nimpl(252,255)
        nor(284,285)
        xor(283,286):11n
        and(5,15)
        and(6,14)
        and(7,13)
        xor(289,290)
        and(264,265)
        and(262,263)
        nor(292,293)
        xor(291,294)
        xor(288,295)
        nor(270,273)
        nimpl(266,269)
        nor(297,298)
        xor(296,299)
        nimpl(275,278)
        and(261,274)
        nor(301,302)
        xor(300,303)
        nimpl(283,286)
        nor(279,282)
        nor(305,306)
        xor(304,307):12
        and(6,15)
        and(7,14)
        xor(309,310)
        nimpl(291,294)
        and(289,290)
        nor(312,313)
        xor(311,314)
        nor(296,299)
        nimpl(288,295)
        nor(316,317)
        xor(315,318)
        nor(304,307)
        nimpl(300,303)
        nor(320,321)
        xor(319,322):13n
        and(7,15)
        nimpl(311,314)
        and(309,310)
        nor(325,326)
        xor(324,327)
        nimpl(319,322)
        nor(315,318)
        nor(329,330)
        xor(328,331):14
        nor(328,331)
        nimpl(324,327)
        nor(333,334):15n
    }(16)
"##,
            )
            .unwrap(),
            "complex3",
        );
        test_read_write_circuit_helper(
            Circuit::from_str(include_str!("sha256.circuit")).unwrap(),
            "sha256",
        );
    }
}
