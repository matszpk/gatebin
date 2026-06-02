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
            self.byte = leb[(bits as usize) >> 3] & ((1 << (bits & 7)) - 1);
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

// other read writes

#[derive(Debug, thiserror::Error)]
pub enum ReadEncIntError {
    #[error("IO: {0}")]
    IO(#[from] io::Error),
    #[error("Positive overflow")]
    PosOverflow,
}

fn read_enc_usize<R: Read>(mut rd: R) -> Result<usize, ReadEncIntError> {
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

fn write_enc_usize<W: Write>(mut wr: W, value: usize) -> Result<(), io::Error> {
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

fn read_cut_enc_usize<R: Read>(
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

fn write_cut_enc_usize<W: Write>(
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

fn read_circuit_int<T, R: Read>(mut rd: R) -> Result<Circuit<T>, ReadCircuitError>
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

fn write_circuit_int<T, W: Write>(mut wr: W, circuit: &Circuit<T>) -> io::Result<()>
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
}
