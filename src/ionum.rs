use std::fmt::Debug;
use std::io::{self, Read, Write};
use std::ops::{BitAnd, BitOr, Shl, Shr};

pub fn read_numbers_2<R: Read, T>(mut rd: R, num_bits: u8, out_numbers: &mut [T]) -> io::Result<()>
where
    T: Clone
        + Copy
        + From<u8>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + Shl<u32, Output = T>
        + Shr<u32, Output = T>,
{
    match num_bits {
        1 => {
            let mut b = [0u8; 1];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 1 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                nums[0] = br0 & T::from(1u8);
                if len > 1 {
                    nums[1] = (br0 >> 1u32) & T::from(1u8);
                    if len > 2 {
                        nums[2] = (br0 >> 2u32) & T::from(1u8);
                        if len > 3 {
                            nums[3] = (br0 >> 3u32) & T::from(1u8);
                            if len > 4 {
                                nums[4] = (br0 >> 4u32) & T::from(1u8);
                                if len > 5 {
                                    nums[5] = (br0 >> 5u32) & T::from(1u8);
                                    if len > 6 {
                                        nums[6] = (br0 >> 6u32) & T::from(1u8);
                                        if len > 7 {
                                            nums[7] = br0 >> 7u32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        2 => {
            let mut b = [0u8; 2];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 2 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                nums[0] = br0 & T::from(3u8);
                if len > 1 {
                    nums[1] = (br0 >> 2u32) & T::from(3u8);
                    if len > 2 {
                        nums[2] = (br0 >> 4u32) & T::from(3u8);
                        if len > 3 {
                            nums[3] = br0 >> 6u32;
                            if len > 4 {
                                let br1 = T::from(b[1]);
                                nums[4] = br1 & T::from(3u8);
                                if len > 5 {
                                    nums[5] = (br1 >> 2u32) & T::from(3u8);
                                    if len > 6 {
                                        nums[6] = (br1 >> 4u32) & T::from(3u8);
                                        if len > 7 {
                                            nums[7] = br1 >> 6u32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        3 => {
            let mut b = [0u8; 3];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 3 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                nums[0] = br0 & T::from(7u8);
                if len > 1 {
                    nums[1] = (br0 >> 3u32) & T::from(7u8);
                    if len > 2 {
                        let br1 = T::from(b[1]);
                        nums[2] = (br0 >> 6u32) | ((br1 & T::from(1u8)) << 2u32);
                        if len > 3 {
                            nums[3] = (br1 >> 1u32) & T::from(7u8);
                            if len > 4 {
                                nums[4] = (br1 >> 4u32) & T::from(7u8);
                                if len > 5 {
                                    let br2 = T::from(b[2]);
                                    nums[5] = (br1 >> 7u32) | ((br2 & T::from(3u8)) << 1u32);
                                    if len > 6 {
                                        nums[6] = (br2 >> 2u32) & T::from(7u8);
                                        if len > 7 {
                                            nums[7] = br2 >> 5u32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        4 => {
            let mut b = [0u8; 4];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 4 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                nums[0] = br0 & T::from(15u8);
                if len > 1 {
                    nums[1] = br0 >> 4u32;
                    if len > 2 {
                        let br1 = T::from(b[1]);
                        nums[2] = br1 & T::from(15u8);
                        if len > 3 {
                            nums[3] = br1 >> 4u32;
                            if len > 4 {
                                let br2 = T::from(b[2]);
                                nums[4] = br2 & T::from(15u8);
                                if len > 5 {
                                    nums[5] = br2 >> 4u32;
                                    if len > 6 {
                                        let br3 = T::from(b[3]);
                                        nums[6] = br3 & T::from(15u8);
                                        if len > 7 {
                                            nums[7] = br3 >> 4u32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        5 => {
            let mut b = [0u8; 5];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 5 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                nums[0] = br0 & T::from(31u8);
                if len > 1 {
                    let br1 = T::from(b[1]);
                    nums[1] = (br0 >> 5u32) | ((br1 & T::from(3u8)) << 3u32);
                    if len > 2 {
                        nums[2] = (br1 >> 2u32) & T::from(31u8);
                        if len > 3 {
                            let br2 = T::from(b[2]);
                            nums[3] = (br1 >> 7u32) | ((br2 & T::from(15u8)) << 1u32);
                            if len > 4 {
                                let br3 = T::from(b[3]);
                                nums[4] = (br2 >> 4u32) | ((br3 & T::from(1u8)) << 4u32);
                                if len > 5 {
                                    nums[5] = (br3 >> 1u32) & T::from(31u8);
                                    if len > 6 {
                                        let br4 = T::from(b[4]);
                                        nums[6] = (br3 >> 6u32) | ((br4 & T::from(7u8)) << 2u32);
                                        if len > 7 {
                                            nums[7] = br4 >> 3u32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        6 => {
            let mut b = [0u8; 6];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 6 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                nums[0] = br0 & T::from(63u8);
                if len > 1 {
                    let br1 = T::from(b[1]);
                    nums[1] = (br0 >> 6u32) | ((br1 & T::from(15u8)) << 2u32);
                    if len > 2 {
                        let br2 = T::from(b[2]);
                        nums[2] = (br1 >> 4u32) | ((br2 & T::from(3u8)) << 4u32);
                        if len > 3 {
                            nums[3] = br2 >> 2u32;
                            if len > 4 {
                                let br3 = T::from(b[3]);
                                nums[4] = br3 & T::from(63u8);
                                if len > 5 {
                                    let br4 = T::from(b[4]);
                                    nums[5] = (br3 >> 6u32) | ((br4 & T::from(15u8)) << 2u32);
                                    if len > 6 {
                                        let br5 = T::from(b[5]);
                                        nums[6] = (br4 >> 4u32) | ((br5 & T::from(3u8)) << 4u32);
                                        if len > 7 {
                                            nums[7] = br5 >> 2u32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        7 => {
            let mut b = [0u8; 7];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 7 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                nums[0] = br0 & T::from(127u8);
                if len > 1 {
                    let br1 = T::from(b[1]);
                    nums[1] = (br0 >> 7u32) | ((br1 & T::from(63u8)) << 1u32);
                    if len > 2 {
                        let br2 = T::from(b[2]);
                        nums[2] = (br1 >> 6u32) | ((br2 & T::from(31u8)) << 2u32);
                        if len > 3 {
                            let br3 = T::from(b[3]);
                            nums[3] = (br2 >> 5u32) | ((br3 & T::from(15u8)) << 3u32);
                            if len > 4 {
                                let br4 = T::from(b[4]);
                                nums[4] = (br3 >> 4u32) | ((br4 & T::from(7u8)) << 4u32);
                                if len > 5 {
                                    let br5 = T::from(b[5]);
                                    nums[5] = (br4 >> 3u32) | ((br5 & T::from(3u8)) << 5u32);
                                    if len > 6 {
                                        let br6 = T::from(b[6]);
                                        nums[6] = (br5 >> 2u32) | ((br6 & T::from(1u8)) << 6u32);
                                        if len > 7 {
                                            nums[7] = br6 >> 1u32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        8 => {
            let mut b = [0u8; 8];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 8 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                nums[0] = br0;
                if len > 1 {
                    let br1 = T::from(b[1]);
                    nums[1] = br1;
                    if len > 2 {
                        let br2 = T::from(b[2]);
                        nums[2] = br2;
                        if len > 3 {
                            let br3 = T::from(b[3]);
                            nums[3] = br3;
                            if len > 4 {
                                let br4 = T::from(b[4]);
                                nums[4] = br4;
                                if len > 5 {
                                    let br5 = T::from(b[5]);
                                    nums[5] = br5;
                                    if len > 6 {
                                        let br6 = T::from(b[6]);
                                        nums[6] = br6;
                                        if len > 7 {
                                            let br7 = T::from(b[7]);
                                            nums[7] = br7;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        9 => {
            let mut b = [0u8; 9];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 9 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                nums[0] = br0 | ((br1 & T::from(1u8)) << 8u32);
                if len > 1 {
                    let br2 = T::from(b[2]);
                    nums[1] = (br1 >> 1u32) | ((br2 & T::from(3u8)) << 7u32);
                    if len > 2 {
                        let br3 = T::from(b[3]);
                        nums[2] = (br2 >> 2u32) | ((br3 & T::from(7u8)) << 6u32);
                        if len > 3 {
                            let br4 = T::from(b[4]);
                            nums[3] = (br3 >> 3u32) | ((br4 & T::from(15u8)) << 5u32);
                            if len > 4 {
                                let br5 = T::from(b[5]);
                                nums[4] = (br4 >> 4u32) | ((br5 & T::from(31u8)) << 4u32);
                                if len > 5 {
                                    let br6 = T::from(b[6]);
                                    nums[5] = (br5 >> 5u32) | ((br6 & T::from(63u8)) << 3u32);
                                    if len > 6 {
                                        let br7 = T::from(b[7]);
                                        nums[6] = (br6 >> 6u32) | ((br7 & T::from(127u8)) << 2u32);
                                        if len > 7 {
                                            let br8 = T::from(b[8]);
                                            nums[7] = (br7 >> 7u32) | (br8 << 1u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        10 => {
            let mut b = [0u8; 10];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 10 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                nums[0] = br0 | ((br1 & T::from(3u8)) << 8u32);
                if len > 1 {
                    let br2 = T::from(b[2]);
                    nums[1] = (br1 >> 2u32) | ((br2 & T::from(15u8)) << 6u32);
                    if len > 2 {
                        let br3 = T::from(b[3]);
                        nums[2] = (br2 >> 4u32) | ((br3 & T::from(63u8)) << 4u32);
                        if len > 3 {
                            let br4 = T::from(b[4]);
                            nums[3] = (br3 >> 6u32) | (br4 << 2u32);
                            if len > 4 {
                                let br5 = T::from(b[5]);
                                let br6 = T::from(b[6]);
                                nums[4] = br5 | ((br6 & T::from(3u8)) << 8u32);
                                if len > 5 {
                                    let br7 = T::from(b[7]);
                                    nums[5] = (br6 >> 2u32) | ((br7 & T::from(15u8)) << 6u32);
                                    if len > 6 {
                                        let br8 = T::from(b[8]);
                                        nums[6] = (br7 >> 4u32) | ((br8 & T::from(63u8)) << 4u32);
                                        if len > 7 {
                                            let br9 = T::from(b[9]);
                                            nums[7] = (br8 >> 6u32) | (br9 << 2u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        11 => {
            let mut b = [0u8; 11];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 11 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                nums[0] = br0 | ((br1 & T::from(7u8)) << 8u32);
                if len > 1 {
                    let br2 = T::from(b[2]);
                    nums[1] = (br1 >> 3u32) | ((br2 & T::from(63u8)) << 5u32);
                    if len > 2 {
                        let br3 = T::from(b[3]);
                        let br4 = T::from(b[4]);
                        nums[2] = (br2 >> 6u32) | (br3 << 2u32) | ((br4 & T::from(1u8)) << 10u32);
                        if len > 3 {
                            let br5 = T::from(b[5]);
                            nums[3] = (br4 >> 1u32) | ((br5 & T::from(15u8)) << 7u32);
                            if len > 4 {
                                let br6 = T::from(b[6]);
                                nums[4] = (br5 >> 4u32) | ((br6 & T::from(127u8)) << 4u32);
                                if len > 5 {
                                    let br7 = T::from(b[7]);
                                    let br8 = T::from(b[8]);
                                    nums[5] = (br6 >> 7u32)
                                        | (br7 << 1u32)
                                        | ((br8 & T::from(3u8)) << 9u32);
                                    if len > 6 {
                                        let br9 = T::from(b[9]);
                                        nums[6] = (br8 >> 2u32) | ((br9 & T::from(31u8)) << 6u32);
                                        if len > 7 {
                                            let br10 = T::from(b[10]);
                                            nums[7] = (br9 >> 5u32) | (br10 << 3u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        12 => {
            let mut b = [0u8; 12];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 12 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                nums[0] = br0 | ((br1 & T::from(15u8)) << 8u32);
                if len > 1 {
                    let br2 = T::from(b[2]);
                    nums[1] = (br1 >> 4u32) | (br2 << 4u32);
                    if len > 2 {
                        let br3 = T::from(b[3]);
                        let br4 = T::from(b[4]);
                        nums[2] = br3 | ((br4 & T::from(15u8)) << 8u32);
                        if len > 3 {
                            let br5 = T::from(b[5]);
                            nums[3] = (br4 >> 4u32) | (br5 << 4u32);
                            if len > 4 {
                                let br6 = T::from(b[6]);
                                let br7 = T::from(b[7]);
                                nums[4] = br6 | ((br7 & T::from(15u8)) << 8u32);
                                if len > 5 {
                                    let br8 = T::from(b[8]);
                                    nums[5] = (br7 >> 4u32) | (br8 << 4u32);
                                    if len > 6 {
                                        let br9 = T::from(b[9]);
                                        let br10 = T::from(b[10]);
                                        nums[6] = br9 | ((br10 & T::from(15u8)) << 8u32);
                                        if len > 7 {
                                            let br11 = T::from(b[11]);
                                            nums[7] = (br10 >> 4u32) | (br11 << 4u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        13 => {
            let mut b = [0u8; 13];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 13 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                nums[0] = br0 | ((br1 & T::from(31u8)) << 8u32);
                if len > 1 {
                    let br2 = T::from(b[2]);
                    let br3 = T::from(b[3]);
                    nums[1] = (br1 >> 5u32) | (br2 << 3u32) | ((br3 & T::from(3u8)) << 11u32);
                    if len > 2 {
                        let br4 = T::from(b[4]);
                        nums[2] = (br3 >> 2u32) | ((br4 & T::from(127u8)) << 6u32);
                        if len > 3 {
                            let br5 = T::from(b[5]);
                            let br6 = T::from(b[6]);
                            nums[3] =
                                (br4 >> 7u32) | (br5 << 1u32) | ((br6 & T::from(15u8)) << 9u32);
                            if len > 4 {
                                let br7 = T::from(b[7]);
                                let br8 = T::from(b[8]);
                                nums[4] =
                                    (br6 >> 4u32) | (br7 << 4u32) | ((br8 & T::from(1u8)) << 12u32);
                                if len > 5 {
                                    let br9 = T::from(b[9]);
                                    nums[5] = (br8 >> 1u32) | ((br9 & T::from(63u8)) << 7u32);
                                    if len > 6 {
                                        let br10 = T::from(b[10]);
                                        let br11 = T::from(b[11]);
                                        nums[6] = (br9 >> 6u32)
                                            | (br10 << 2u32)
                                            | ((br11 & T::from(7u8)) << 10u32);
                                        if len > 7 {
                                            let br12 = T::from(b[12]);
                                            nums[7] = (br11 >> 3u32) | (br12 << 5u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        14 => {
            let mut b = [0u8; 14];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 14 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                nums[0] = br0 | ((br1 & T::from(63u8)) << 8u32);
                if len > 1 {
                    let br2 = T::from(b[2]);
                    let br3 = T::from(b[3]);
                    nums[1] = (br1 >> 6u32) | (br2 << 2u32) | ((br3 & T::from(15u8)) << 10u32);
                    if len > 2 {
                        let br4 = T::from(b[4]);
                        let br5 = T::from(b[5]);
                        nums[2] = (br3 >> 4u32) | (br4 << 4u32) | ((br5 & T::from(3u8)) << 12u32);
                        if len > 3 {
                            let br6 = T::from(b[6]);
                            nums[3] = (br5 >> 2u32) | (br6 << 6u32);
                            if len > 4 {
                                let br7 = T::from(b[7]);
                                let br8 = T::from(b[8]);
                                nums[4] = br7 | ((br8 & T::from(63u8)) << 8u32);
                                if len > 5 {
                                    let br9 = T::from(b[9]);
                                    let br10 = T::from(b[10]);
                                    nums[5] = (br8 >> 6u32)
                                        | (br9 << 2u32)
                                        | ((br10 & T::from(15u8)) << 10u32);
                                    if len > 6 {
                                        let br11 = T::from(b[11]);
                                        let br12 = T::from(b[12]);
                                        nums[6] = (br10 >> 4u32)
                                            | (br11 << 4u32)
                                            | ((br12 & T::from(3u8)) << 12u32);
                                        if len > 7 {
                                            let br13 = T::from(b[13]);
                                            nums[7] = (br12 >> 2u32) | (br13 << 6u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        15 => {
            let mut b = [0u8; 15];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 15 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                nums[0] = br0 | ((br1 & T::from(127u8)) << 8u32);
                if len > 1 {
                    let br2 = T::from(b[2]);
                    let br3 = T::from(b[3]);
                    nums[1] = (br1 >> 7u32) | (br2 << 1u32) | ((br3 & T::from(63u8)) << 9u32);
                    if len > 2 {
                        let br4 = T::from(b[4]);
                        let br5 = T::from(b[5]);
                        nums[2] = (br3 >> 6u32) | (br4 << 2u32) | ((br5 & T::from(31u8)) << 10u32);
                        if len > 3 {
                            let br6 = T::from(b[6]);
                            let br7 = T::from(b[7]);
                            nums[3] =
                                (br5 >> 5u32) | (br6 << 3u32) | ((br7 & T::from(15u8)) << 11u32);
                            if len > 4 {
                                let br8 = T::from(b[8]);
                                let br9 = T::from(b[9]);
                                nums[4] =
                                    (br7 >> 4u32) | (br8 << 4u32) | ((br9 & T::from(7u8)) << 12u32);
                                if len > 5 {
                                    let br10 = T::from(b[10]);
                                    let br11 = T::from(b[11]);
                                    nums[5] = (br9 >> 3u32)
                                        | (br10 << 5u32)
                                        | ((br11 & T::from(3u8)) << 13u32);
                                    if len > 6 {
                                        let br12 = T::from(b[12]);
                                        let br13 = T::from(b[13]);
                                        nums[6] = (br11 >> 2u32)
                                            | (br12 << 6u32)
                                            | ((br13 & T::from(1u8)) << 14u32);
                                        if len > 7 {
                                            let br14 = T::from(b[14]);
                                            nums[7] = (br13 >> 1u32) | (br14 << 7u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        16 => {
            let mut b = [0u8; 16];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 16 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                nums[0] = br0 | (br1 << 8u32);
                if len > 1 {
                    let br2 = T::from(b[2]);
                    let br3 = T::from(b[3]);
                    nums[1] = br2 | (br3 << 8u32);
                    if len > 2 {
                        let br4 = T::from(b[4]);
                        let br5 = T::from(b[5]);
                        nums[2] = br4 | (br5 << 8u32);
                        if len > 3 {
                            let br6 = T::from(b[6]);
                            let br7 = T::from(b[7]);
                            nums[3] = br6 | (br7 << 8u32);
                            if len > 4 {
                                let br8 = T::from(b[8]);
                                let br9 = T::from(b[9]);
                                nums[4] = br8 | (br9 << 8u32);
                                if len > 5 {
                                    let br10 = T::from(b[10]);
                                    let br11 = T::from(b[11]);
                                    nums[5] = br10 | (br11 << 8u32);
                                    if len > 6 {
                                        let br12 = T::from(b[12]);
                                        let br13 = T::from(b[13]);
                                        nums[6] = br12 | (br13 << 8u32);
                                        if len > 7 {
                                            let br14 = T::from(b[14]);
                                            let br15 = T::from(b[15]);
                                            nums[7] = br14 | (br15 << 8u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        17 => {
            let mut b = [0u8; 17];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 17 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                nums[0] = br0 | (br1 << 8u32) | ((br2 & T::from(1u8)) << 16u32);
                if len > 1 {
                    let br3 = T::from(b[3]);
                    let br4 = T::from(b[4]);
                    nums[1] = (br2 >> 1u32) | (br3 << 7u32) | ((br4 & T::from(3u8)) << 15u32);
                    if len > 2 {
                        let br5 = T::from(b[5]);
                        let br6 = T::from(b[6]);
                        nums[2] = (br4 >> 2u32) | (br5 << 6u32) | ((br6 & T::from(7u8)) << 14u32);
                        if len > 3 {
                            let br7 = T::from(b[7]);
                            let br8 = T::from(b[8]);
                            nums[3] =
                                (br6 >> 3u32) | (br7 << 5u32) | ((br8 & T::from(15u8)) << 13u32);
                            if len > 4 {
                                let br9 = T::from(b[9]);
                                let br10 = T::from(b[10]);
                                nums[4] = (br8 >> 4u32)
                                    | (br9 << 4u32)
                                    | ((br10 & T::from(31u8)) << 12u32);
                                if len > 5 {
                                    let br11 = T::from(b[11]);
                                    let br12 = T::from(b[12]);
                                    nums[5] = (br10 >> 5u32)
                                        | (br11 << 3u32)
                                        | ((br12 & T::from(63u8)) << 11u32);
                                    if len > 6 {
                                        let br13 = T::from(b[13]);
                                        let br14 = T::from(b[14]);
                                        nums[6] = (br12 >> 6u32)
                                            | (br13 << 2u32)
                                            | ((br14 & T::from(127u8)) << 10u32);
                                        if len > 7 {
                                            let br15 = T::from(b[15]);
                                            let br16 = T::from(b[16]);
                                            nums[7] =
                                                (br14 >> 7u32) | (br15 << 1u32) | (br16 << 9u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        18 => {
            let mut b = [0u8; 18];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 18 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                nums[0] = br0 | (br1 << 8u32) | ((br2 & T::from(3u8)) << 16u32);
                if len > 1 {
                    let br3 = T::from(b[3]);
                    let br4 = T::from(b[4]);
                    nums[1] = (br2 >> 2u32) | (br3 << 6u32) | ((br4 & T::from(15u8)) << 14u32);
                    if len > 2 {
                        let br5 = T::from(b[5]);
                        let br6 = T::from(b[6]);
                        nums[2] = (br4 >> 4u32) | (br5 << 4u32) | ((br6 & T::from(63u8)) << 12u32);
                        if len > 3 {
                            let br7 = T::from(b[7]);
                            let br8 = T::from(b[8]);
                            nums[3] = (br6 >> 6u32) | (br7 << 2u32) | (br8 << 10u32);
                            if len > 4 {
                                let br9 = T::from(b[9]);
                                let br10 = T::from(b[10]);
                                let br11 = T::from(b[11]);
                                nums[4] = br9 | (br10 << 8u32) | ((br11 & T::from(3u8)) << 16u32);
                                if len > 5 {
                                    let br12 = T::from(b[12]);
                                    let br13 = T::from(b[13]);
                                    nums[5] = (br11 >> 2u32)
                                        | (br12 << 6u32)
                                        | ((br13 & T::from(15u8)) << 14u32);
                                    if len > 6 {
                                        let br14 = T::from(b[14]);
                                        let br15 = T::from(b[15]);
                                        nums[6] = (br13 >> 4u32)
                                            | (br14 << 4u32)
                                            | ((br15 & T::from(63u8)) << 12u32);
                                        if len > 7 {
                                            let br16 = T::from(b[16]);
                                            let br17 = T::from(b[17]);
                                            nums[7] =
                                                (br15 >> 6u32) | (br16 << 2u32) | (br17 << 10u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        19 => {
            let mut b = [0u8; 19];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 19 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                nums[0] = br0 | (br1 << 8u32) | ((br2 & T::from(7u8)) << 16u32);
                if len > 1 {
                    let br3 = T::from(b[3]);
                    let br4 = T::from(b[4]);
                    nums[1] = (br2 >> 3u32) | (br3 << 5u32) | ((br4 & T::from(63u8)) << 13u32);
                    if len > 2 {
                        let br5 = T::from(b[5]);
                        let br6 = T::from(b[6]);
                        let br7 = T::from(b[7]);
                        nums[2] = (br4 >> 6u32)
                            | (br5 << 2u32)
                            | (br6 << 10u32)
                            | ((br7 & T::from(1u8)) << 18u32);
                        if len > 3 {
                            let br8 = T::from(b[8]);
                            let br9 = T::from(b[9]);
                            nums[3] =
                                (br7 >> 1u32) | (br8 << 7u32) | ((br9 & T::from(15u8)) << 15u32);
                            if len > 4 {
                                let br10 = T::from(b[10]);
                                let br11 = T::from(b[11]);
                                nums[4] = (br9 >> 4u32)
                                    | (br10 << 4u32)
                                    | ((br11 & T::from(127u8)) << 12u32);
                                if len > 5 {
                                    let br12 = T::from(b[12]);
                                    let br13 = T::from(b[13]);
                                    let br14 = T::from(b[14]);
                                    nums[5] = (br11 >> 7u32)
                                        | (br12 << 1u32)
                                        | (br13 << 9u32)
                                        | ((br14 & T::from(3u8)) << 17u32);
                                    if len > 6 {
                                        let br15 = T::from(b[15]);
                                        let br16 = T::from(b[16]);
                                        nums[6] = (br14 >> 2u32)
                                            | (br15 << 6u32)
                                            | ((br16 & T::from(31u8)) << 14u32);
                                        if len > 7 {
                                            let br17 = T::from(b[17]);
                                            let br18 = T::from(b[18]);
                                            nums[7] =
                                                (br16 >> 5u32) | (br17 << 3u32) | (br18 << 11u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        20 => {
            let mut b = [0u8; 20];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 20 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                nums[0] = br0 | (br1 << 8u32) | ((br2 & T::from(15u8)) << 16u32);
                if len > 1 {
                    let br3 = T::from(b[3]);
                    let br4 = T::from(b[4]);
                    nums[1] = (br2 >> 4u32) | (br3 << 4u32) | (br4 << 12u32);
                    if len > 2 {
                        let br5 = T::from(b[5]);
                        let br6 = T::from(b[6]);
                        let br7 = T::from(b[7]);
                        nums[2] = br5 | (br6 << 8u32) | ((br7 & T::from(15u8)) << 16u32);
                        if len > 3 {
                            let br8 = T::from(b[8]);
                            let br9 = T::from(b[9]);
                            nums[3] = (br7 >> 4u32) | (br8 << 4u32) | (br9 << 12u32);
                            if len > 4 {
                                let br10 = T::from(b[10]);
                                let br11 = T::from(b[11]);
                                let br12 = T::from(b[12]);
                                nums[4] = br10 | (br11 << 8u32) | ((br12 & T::from(15u8)) << 16u32);
                                if len > 5 {
                                    let br13 = T::from(b[13]);
                                    let br14 = T::from(b[14]);
                                    nums[5] = (br12 >> 4u32) | (br13 << 4u32) | (br14 << 12u32);
                                    if len > 6 {
                                        let br15 = T::from(b[15]);
                                        let br16 = T::from(b[16]);
                                        let br17 = T::from(b[17]);
                                        nums[6] = br15
                                            | (br16 << 8u32)
                                            | ((br17 & T::from(15u8)) << 16u32);
                                        if len > 7 {
                                            let br18 = T::from(b[18]);
                                            let br19 = T::from(b[19]);
                                            nums[7] =
                                                (br17 >> 4u32) | (br18 << 4u32) | (br19 << 12u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        21 => {
            let mut b = [0u8; 21];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 21 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                nums[0] = br0 | (br1 << 8u32) | ((br2 & T::from(31u8)) << 16u32);
                if len > 1 {
                    let br3 = T::from(b[3]);
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    nums[1] = (br2 >> 5u32)
                        | (br3 << 3u32)
                        | (br4 << 11u32)
                        | ((br5 & T::from(3u8)) << 19u32);
                    if len > 2 {
                        let br6 = T::from(b[6]);
                        let br7 = T::from(b[7]);
                        nums[2] = (br5 >> 2u32) | (br6 << 6u32) | ((br7 & T::from(127u8)) << 14u32);
                        if len > 3 {
                            let br8 = T::from(b[8]);
                            let br9 = T::from(b[9]);
                            let br10 = T::from(b[10]);
                            nums[3] = (br7 >> 7u32)
                                | (br8 << 1u32)
                                | (br9 << 9u32)
                                | ((br10 & T::from(15u8)) << 17u32);
                            if len > 4 {
                                let br11 = T::from(b[11]);
                                let br12 = T::from(b[12]);
                                let br13 = T::from(b[13]);
                                nums[4] = (br10 >> 4u32)
                                    | (br11 << 4u32)
                                    | (br12 << 12u32)
                                    | ((br13 & T::from(1u8)) << 20u32);
                                if len > 5 {
                                    let br14 = T::from(b[14]);
                                    let br15 = T::from(b[15]);
                                    nums[5] = (br13 >> 1u32)
                                        | (br14 << 7u32)
                                        | ((br15 & T::from(63u8)) << 15u32);
                                    if len > 6 {
                                        let br16 = T::from(b[16]);
                                        let br17 = T::from(b[17]);
                                        let br18 = T::from(b[18]);
                                        nums[6] = (br15 >> 6u32)
                                            | (br16 << 2u32)
                                            | (br17 << 10u32)
                                            | ((br18 & T::from(7u8)) << 18u32);
                                        if len > 7 {
                                            let br19 = T::from(b[19]);
                                            let br20 = T::from(b[20]);
                                            nums[7] =
                                                (br18 >> 3u32) | (br19 << 5u32) | (br20 << 13u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        22 => {
            let mut b = [0u8; 22];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 22 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                nums[0] = br0 | (br1 << 8u32) | ((br2 & T::from(63u8)) << 16u32);
                if len > 1 {
                    let br3 = T::from(b[3]);
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    nums[1] = (br2 >> 6u32)
                        | (br3 << 2u32)
                        | (br4 << 10u32)
                        | ((br5 & T::from(15u8)) << 18u32);
                    if len > 2 {
                        let br6 = T::from(b[6]);
                        let br7 = T::from(b[7]);
                        let br8 = T::from(b[8]);
                        nums[2] = (br5 >> 4u32)
                            | (br6 << 4u32)
                            | (br7 << 12u32)
                            | ((br8 & T::from(3u8)) << 20u32);
                        if len > 3 {
                            let br9 = T::from(b[9]);
                            let br10 = T::from(b[10]);
                            nums[3] = (br8 >> 2u32) | (br9 << 6u32) | (br10 << 14u32);
                            if len > 4 {
                                let br11 = T::from(b[11]);
                                let br12 = T::from(b[12]);
                                let br13 = T::from(b[13]);
                                nums[4] = br11 | (br12 << 8u32) | ((br13 & T::from(63u8)) << 16u32);
                                if len > 5 {
                                    let br14 = T::from(b[14]);
                                    let br15 = T::from(b[15]);
                                    let br16 = T::from(b[16]);
                                    nums[5] = (br13 >> 6u32)
                                        | (br14 << 2u32)
                                        | (br15 << 10u32)
                                        | ((br16 & T::from(15u8)) << 18u32);
                                    if len > 6 {
                                        let br17 = T::from(b[17]);
                                        let br18 = T::from(b[18]);
                                        let br19 = T::from(b[19]);
                                        nums[6] = (br16 >> 4u32)
                                            | (br17 << 4u32)
                                            | (br18 << 12u32)
                                            | ((br19 & T::from(3u8)) << 20u32);
                                        if len > 7 {
                                            let br20 = T::from(b[20]);
                                            let br21 = T::from(b[21]);
                                            nums[7] =
                                                (br19 >> 2u32) | (br20 << 6u32) | (br21 << 14u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        23 => {
            let mut b = [0u8; 23];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 23 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                nums[0] = br0 | (br1 << 8u32) | ((br2 & T::from(127u8)) << 16u32);
                if len > 1 {
                    let br3 = T::from(b[3]);
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    nums[1] = (br2 >> 7u32)
                        | (br3 << 1u32)
                        | (br4 << 9u32)
                        | ((br5 & T::from(63u8)) << 17u32);
                    if len > 2 {
                        let br6 = T::from(b[6]);
                        let br7 = T::from(b[7]);
                        let br8 = T::from(b[8]);
                        nums[2] = (br5 >> 6u32)
                            | (br6 << 2u32)
                            | (br7 << 10u32)
                            | ((br8 & T::from(31u8)) << 18u32);
                        if len > 3 {
                            let br9 = T::from(b[9]);
                            let br10 = T::from(b[10]);
                            let br11 = T::from(b[11]);
                            nums[3] = (br8 >> 5u32)
                                | (br9 << 3u32)
                                | (br10 << 11u32)
                                | ((br11 & T::from(15u8)) << 19u32);
                            if len > 4 {
                                let br12 = T::from(b[12]);
                                let br13 = T::from(b[13]);
                                let br14 = T::from(b[14]);
                                nums[4] = (br11 >> 4u32)
                                    | (br12 << 4u32)
                                    | (br13 << 12u32)
                                    | ((br14 & T::from(7u8)) << 20u32);
                                if len > 5 {
                                    let br15 = T::from(b[15]);
                                    let br16 = T::from(b[16]);
                                    let br17 = T::from(b[17]);
                                    nums[5] = (br14 >> 3u32)
                                        | (br15 << 5u32)
                                        | (br16 << 13u32)
                                        | ((br17 & T::from(3u8)) << 21u32);
                                    if len > 6 {
                                        let br18 = T::from(b[18]);
                                        let br19 = T::from(b[19]);
                                        let br20 = T::from(b[20]);
                                        nums[6] = (br17 >> 2u32)
                                            | (br18 << 6u32)
                                            | (br19 << 14u32)
                                            | ((br20 & T::from(1u8)) << 22u32);
                                        if len > 7 {
                                            let br21 = T::from(b[21]);
                                            let br22 = T::from(b[22]);
                                            nums[7] =
                                                (br20 >> 1u32) | (br21 << 7u32) | (br22 << 15u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        24 => {
            let mut b = [0u8; 24];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 24 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                nums[0] = br0 | (br1 << 8u32) | (br2 << 16u32);
                if len > 1 {
                    let br3 = T::from(b[3]);
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    nums[1] = br3 | (br4 << 8u32) | (br5 << 16u32);
                    if len > 2 {
                        let br6 = T::from(b[6]);
                        let br7 = T::from(b[7]);
                        let br8 = T::from(b[8]);
                        nums[2] = br6 | (br7 << 8u32) | (br8 << 16u32);
                        if len > 3 {
                            let br9 = T::from(b[9]);
                            let br10 = T::from(b[10]);
                            let br11 = T::from(b[11]);
                            nums[3] = br9 | (br10 << 8u32) | (br11 << 16u32);
                            if len > 4 {
                                let br12 = T::from(b[12]);
                                let br13 = T::from(b[13]);
                                let br14 = T::from(b[14]);
                                nums[4] = br12 | (br13 << 8u32) | (br14 << 16u32);
                                if len > 5 {
                                    let br15 = T::from(b[15]);
                                    let br16 = T::from(b[16]);
                                    let br17 = T::from(b[17]);
                                    nums[5] = br15 | (br16 << 8u32) | (br17 << 16u32);
                                    if len > 6 {
                                        let br18 = T::from(b[18]);
                                        let br19 = T::from(b[19]);
                                        let br20 = T::from(b[20]);
                                        nums[6] = br18 | (br19 << 8u32) | (br20 << 16u32);
                                        if len > 7 {
                                            let br21 = T::from(b[21]);
                                            let br22 = T::from(b[22]);
                                            let br23 = T::from(b[23]);
                                            nums[7] = br21 | (br22 << 8u32) | (br23 << 16u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        25 => {
            let mut b = [0u8; 25];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 25 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                nums[0] = br0 | (br1 << 8u32) | (br2 << 16u32) | ((br3 & T::from(1u8)) << 24u32);
                if len > 1 {
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    nums[1] = (br3 >> 1u32)
                        | (br4 << 7u32)
                        | (br5 << 15u32)
                        | ((br6 & T::from(3u8)) << 23u32);
                    if len > 2 {
                        let br7 = T::from(b[7]);
                        let br8 = T::from(b[8]);
                        let br9 = T::from(b[9]);
                        nums[2] = (br6 >> 2u32)
                            | (br7 << 6u32)
                            | (br8 << 14u32)
                            | ((br9 & T::from(7u8)) << 22u32);
                        if len > 3 {
                            let br10 = T::from(b[10]);
                            let br11 = T::from(b[11]);
                            let br12 = T::from(b[12]);
                            nums[3] = (br9 >> 3u32)
                                | (br10 << 5u32)
                                | (br11 << 13u32)
                                | ((br12 & T::from(15u8)) << 21u32);
                            if len > 4 {
                                let br13 = T::from(b[13]);
                                let br14 = T::from(b[14]);
                                let br15 = T::from(b[15]);
                                nums[4] = (br12 >> 4u32)
                                    | (br13 << 4u32)
                                    | (br14 << 12u32)
                                    | ((br15 & T::from(31u8)) << 20u32);
                                if len > 5 {
                                    let br16 = T::from(b[16]);
                                    let br17 = T::from(b[17]);
                                    let br18 = T::from(b[18]);
                                    nums[5] = (br15 >> 5u32)
                                        | (br16 << 3u32)
                                        | (br17 << 11u32)
                                        | ((br18 & T::from(63u8)) << 19u32);
                                    if len > 6 {
                                        let br19 = T::from(b[19]);
                                        let br20 = T::from(b[20]);
                                        let br21 = T::from(b[21]);
                                        nums[6] = (br18 >> 6u32)
                                            | (br19 << 2u32)
                                            | (br20 << 10u32)
                                            | ((br21 & T::from(127u8)) << 18u32);
                                        if len > 7 {
                                            let br22 = T::from(b[22]);
                                            let br23 = T::from(b[23]);
                                            let br24 = T::from(b[24]);
                                            nums[7] = (br21 >> 7u32)
                                                | (br22 << 1u32)
                                                | (br23 << 9u32)
                                                | (br24 << 17u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        26 => {
            let mut b = [0u8; 26];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 26 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                nums[0] = br0 | (br1 << 8u32) | (br2 << 16u32) | ((br3 & T::from(3u8)) << 24u32);
                if len > 1 {
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    nums[1] = (br3 >> 2u32)
                        | (br4 << 6u32)
                        | (br5 << 14u32)
                        | ((br6 & T::from(15u8)) << 22u32);
                    if len > 2 {
                        let br7 = T::from(b[7]);
                        let br8 = T::from(b[8]);
                        let br9 = T::from(b[9]);
                        nums[2] = (br6 >> 4u32)
                            | (br7 << 4u32)
                            | (br8 << 12u32)
                            | ((br9 & T::from(63u8)) << 20u32);
                        if len > 3 {
                            let br10 = T::from(b[10]);
                            let br11 = T::from(b[11]);
                            let br12 = T::from(b[12]);
                            nums[3] =
                                (br9 >> 6u32) | (br10 << 2u32) | (br11 << 10u32) | (br12 << 18u32);
                            if len > 4 {
                                let br13 = T::from(b[13]);
                                let br14 = T::from(b[14]);
                                let br15 = T::from(b[15]);
                                let br16 = T::from(b[16]);
                                nums[4] = br13
                                    | (br14 << 8u32)
                                    | (br15 << 16u32)
                                    | ((br16 & T::from(3u8)) << 24u32);
                                if len > 5 {
                                    let br17 = T::from(b[17]);
                                    let br18 = T::from(b[18]);
                                    let br19 = T::from(b[19]);
                                    nums[5] = (br16 >> 2u32)
                                        | (br17 << 6u32)
                                        | (br18 << 14u32)
                                        | ((br19 & T::from(15u8)) << 22u32);
                                    if len > 6 {
                                        let br20 = T::from(b[20]);
                                        let br21 = T::from(b[21]);
                                        let br22 = T::from(b[22]);
                                        nums[6] = (br19 >> 4u32)
                                            | (br20 << 4u32)
                                            | (br21 << 12u32)
                                            | ((br22 & T::from(63u8)) << 20u32);
                                        if len > 7 {
                                            let br23 = T::from(b[23]);
                                            let br24 = T::from(b[24]);
                                            let br25 = T::from(b[25]);
                                            nums[7] = (br22 >> 6u32)
                                                | (br23 << 2u32)
                                                | (br24 << 10u32)
                                                | (br25 << 18u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        27 => {
            let mut b = [0u8; 27];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 27 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                nums[0] = br0 | (br1 << 8u32) | (br2 << 16u32) | ((br3 & T::from(7u8)) << 24u32);
                if len > 1 {
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    nums[1] = (br3 >> 3u32)
                        | (br4 << 5u32)
                        | (br5 << 13u32)
                        | ((br6 & T::from(63u8)) << 21u32);
                    if len > 2 {
                        let br7 = T::from(b[7]);
                        let br8 = T::from(b[8]);
                        let br9 = T::from(b[9]);
                        let br10 = T::from(b[10]);
                        nums[2] = (br6 >> 6u32)
                            | (br7 << 2u32)
                            | (br8 << 10u32)
                            | (br9 << 18u32)
                            | ((br10 & T::from(1u8)) << 26u32);
                        if len > 3 {
                            let br11 = T::from(b[11]);
                            let br12 = T::from(b[12]);
                            let br13 = T::from(b[13]);
                            nums[3] = (br10 >> 1u32)
                                | (br11 << 7u32)
                                | (br12 << 15u32)
                                | ((br13 & T::from(15u8)) << 23u32);
                            if len > 4 {
                                let br14 = T::from(b[14]);
                                let br15 = T::from(b[15]);
                                let br16 = T::from(b[16]);
                                nums[4] = (br13 >> 4u32)
                                    | (br14 << 4u32)
                                    | (br15 << 12u32)
                                    | ((br16 & T::from(127u8)) << 20u32);
                                if len > 5 {
                                    let br17 = T::from(b[17]);
                                    let br18 = T::from(b[18]);
                                    let br19 = T::from(b[19]);
                                    let br20 = T::from(b[20]);
                                    nums[5] = (br16 >> 7u32)
                                        | (br17 << 1u32)
                                        | (br18 << 9u32)
                                        | (br19 << 17u32)
                                        | ((br20 & T::from(3u8)) << 25u32);
                                    if len > 6 {
                                        let br21 = T::from(b[21]);
                                        let br22 = T::from(b[22]);
                                        let br23 = T::from(b[23]);
                                        nums[6] = (br20 >> 2u32)
                                            | (br21 << 6u32)
                                            | (br22 << 14u32)
                                            | ((br23 & T::from(31u8)) << 22u32);
                                        if len > 7 {
                                            let br24 = T::from(b[24]);
                                            let br25 = T::from(b[25]);
                                            let br26 = T::from(b[26]);
                                            nums[7] = (br23 >> 5u32)
                                                | (br24 << 3u32)
                                                | (br25 << 11u32)
                                                | (br26 << 19u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        28 => {
            let mut b = [0u8; 28];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 28 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                nums[0] = br0 | (br1 << 8u32) | (br2 << 16u32) | ((br3 & T::from(15u8)) << 24u32);
                if len > 1 {
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    nums[1] = (br3 >> 4u32) | (br4 << 4u32) | (br5 << 12u32) | (br6 << 20u32);
                    if len > 2 {
                        let br7 = T::from(b[7]);
                        let br8 = T::from(b[8]);
                        let br9 = T::from(b[9]);
                        let br10 = T::from(b[10]);
                        nums[2] = br7
                            | (br8 << 8u32)
                            | (br9 << 16u32)
                            | ((br10 & T::from(15u8)) << 24u32);
                        if len > 3 {
                            let br11 = T::from(b[11]);
                            let br12 = T::from(b[12]);
                            let br13 = T::from(b[13]);
                            nums[3] =
                                (br10 >> 4u32) | (br11 << 4u32) | (br12 << 12u32) | (br13 << 20u32);
                            if len > 4 {
                                let br14 = T::from(b[14]);
                                let br15 = T::from(b[15]);
                                let br16 = T::from(b[16]);
                                let br17 = T::from(b[17]);
                                nums[4] = br14
                                    | (br15 << 8u32)
                                    | (br16 << 16u32)
                                    | ((br17 & T::from(15u8)) << 24u32);
                                if len > 5 {
                                    let br18 = T::from(b[18]);
                                    let br19 = T::from(b[19]);
                                    let br20 = T::from(b[20]);
                                    nums[5] = (br17 >> 4u32)
                                        | (br18 << 4u32)
                                        | (br19 << 12u32)
                                        | (br20 << 20u32);
                                    if len > 6 {
                                        let br21 = T::from(b[21]);
                                        let br22 = T::from(b[22]);
                                        let br23 = T::from(b[23]);
                                        let br24 = T::from(b[24]);
                                        nums[6] = br21
                                            | (br22 << 8u32)
                                            | (br23 << 16u32)
                                            | ((br24 & T::from(15u8)) << 24u32);
                                        if len > 7 {
                                            let br25 = T::from(b[25]);
                                            let br26 = T::from(b[26]);
                                            let br27 = T::from(b[27]);
                                            nums[7] = (br24 >> 4u32)
                                                | (br25 << 4u32)
                                                | (br26 << 12u32)
                                                | (br27 << 20u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        29 => {
            let mut b = [0u8; 29];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 29 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                nums[0] = br0 | (br1 << 8u32) | (br2 << 16u32) | ((br3 & T::from(31u8)) << 24u32);
                if len > 1 {
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    nums[1] = (br3 >> 5u32)
                        | (br4 << 3u32)
                        | (br5 << 11u32)
                        | (br6 << 19u32)
                        | ((br7 & T::from(3u8)) << 27u32);
                    if len > 2 {
                        let br8 = T::from(b[8]);
                        let br9 = T::from(b[9]);
                        let br10 = T::from(b[10]);
                        nums[2] = (br7 >> 2u32)
                            | (br8 << 6u32)
                            | (br9 << 14u32)
                            | ((br10 & T::from(127u8)) << 22u32);
                        if len > 3 {
                            let br11 = T::from(b[11]);
                            let br12 = T::from(b[12]);
                            let br13 = T::from(b[13]);
                            let br14 = T::from(b[14]);
                            nums[3] = (br10 >> 7u32)
                                | (br11 << 1u32)
                                | (br12 << 9u32)
                                | (br13 << 17u32)
                                | ((br14 & T::from(15u8)) << 25u32);
                            if len > 4 {
                                let br15 = T::from(b[15]);
                                let br16 = T::from(b[16]);
                                let br17 = T::from(b[17]);
                                let br18 = T::from(b[18]);
                                nums[4] = (br14 >> 4u32)
                                    | (br15 << 4u32)
                                    | (br16 << 12u32)
                                    | (br17 << 20u32)
                                    | ((br18 & T::from(1u8)) << 28u32);
                                if len > 5 {
                                    let br19 = T::from(b[19]);
                                    let br20 = T::from(b[20]);
                                    let br21 = T::from(b[21]);
                                    nums[5] = (br18 >> 1u32)
                                        | (br19 << 7u32)
                                        | (br20 << 15u32)
                                        | ((br21 & T::from(63u8)) << 23u32);
                                    if len > 6 {
                                        let br22 = T::from(b[22]);
                                        let br23 = T::from(b[23]);
                                        let br24 = T::from(b[24]);
                                        let br25 = T::from(b[25]);
                                        nums[6] = (br21 >> 6u32)
                                            | (br22 << 2u32)
                                            | (br23 << 10u32)
                                            | (br24 << 18u32)
                                            | ((br25 & T::from(7u8)) << 26u32);
                                        if len > 7 {
                                            let br26 = T::from(b[26]);
                                            let br27 = T::from(b[27]);
                                            let br28 = T::from(b[28]);
                                            nums[7] = (br25 >> 3u32)
                                                | (br26 << 5u32)
                                                | (br27 << 13u32)
                                                | (br28 << 21u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        30 => {
            let mut b = [0u8; 30];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 30 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                nums[0] = br0 | (br1 << 8u32) | (br2 << 16u32) | ((br3 & T::from(63u8)) << 24u32);
                if len > 1 {
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    nums[1] = (br3 >> 6u32)
                        | (br4 << 2u32)
                        | (br5 << 10u32)
                        | (br6 << 18u32)
                        | ((br7 & T::from(15u8)) << 26u32);
                    if len > 2 {
                        let br8 = T::from(b[8]);
                        let br9 = T::from(b[9]);
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        nums[2] = (br7 >> 4u32)
                            | (br8 << 4u32)
                            | (br9 << 12u32)
                            | (br10 << 20u32)
                            | ((br11 & T::from(3u8)) << 28u32);
                        if len > 3 {
                            let br12 = T::from(b[12]);
                            let br13 = T::from(b[13]);
                            let br14 = T::from(b[14]);
                            nums[3] =
                                (br11 >> 2u32) | (br12 << 6u32) | (br13 << 14u32) | (br14 << 22u32);
                            if len > 4 {
                                let br15 = T::from(b[15]);
                                let br16 = T::from(b[16]);
                                let br17 = T::from(b[17]);
                                let br18 = T::from(b[18]);
                                nums[4] = br15
                                    | (br16 << 8u32)
                                    | (br17 << 16u32)
                                    | ((br18 & T::from(63u8)) << 24u32);
                                if len > 5 {
                                    let br19 = T::from(b[19]);
                                    let br20 = T::from(b[20]);
                                    let br21 = T::from(b[21]);
                                    let br22 = T::from(b[22]);
                                    nums[5] = (br18 >> 6u32)
                                        | (br19 << 2u32)
                                        | (br20 << 10u32)
                                        | (br21 << 18u32)
                                        | ((br22 & T::from(15u8)) << 26u32);
                                    if len > 6 {
                                        let br23 = T::from(b[23]);
                                        let br24 = T::from(b[24]);
                                        let br25 = T::from(b[25]);
                                        let br26 = T::from(b[26]);
                                        nums[6] = (br22 >> 4u32)
                                            | (br23 << 4u32)
                                            | (br24 << 12u32)
                                            | (br25 << 20u32)
                                            | ((br26 & T::from(3u8)) << 28u32);
                                        if len > 7 {
                                            let br27 = T::from(b[27]);
                                            let br28 = T::from(b[28]);
                                            let br29 = T::from(b[29]);
                                            nums[7] = (br26 >> 2u32)
                                                | (br27 << 6u32)
                                                | (br28 << 14u32)
                                                | (br29 << 22u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        31 => {
            let mut b = [0u8; 31];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 31 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                nums[0] = br0 | (br1 << 8u32) | (br2 << 16u32) | ((br3 & T::from(127u8)) << 24u32);
                if len > 1 {
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    nums[1] = (br3 >> 7u32)
                        | (br4 << 1u32)
                        | (br5 << 9u32)
                        | (br6 << 17u32)
                        | ((br7 & T::from(63u8)) << 25u32);
                    if len > 2 {
                        let br8 = T::from(b[8]);
                        let br9 = T::from(b[9]);
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        nums[2] = (br7 >> 6u32)
                            | (br8 << 2u32)
                            | (br9 << 10u32)
                            | (br10 << 18u32)
                            | ((br11 & T::from(31u8)) << 26u32);
                        if len > 3 {
                            let br12 = T::from(b[12]);
                            let br13 = T::from(b[13]);
                            let br14 = T::from(b[14]);
                            let br15 = T::from(b[15]);
                            nums[3] = (br11 >> 5u32)
                                | (br12 << 3u32)
                                | (br13 << 11u32)
                                | (br14 << 19u32)
                                | ((br15 & T::from(15u8)) << 27u32);
                            if len > 4 {
                                let br16 = T::from(b[16]);
                                let br17 = T::from(b[17]);
                                let br18 = T::from(b[18]);
                                let br19 = T::from(b[19]);
                                nums[4] = (br15 >> 4u32)
                                    | (br16 << 4u32)
                                    | (br17 << 12u32)
                                    | (br18 << 20u32)
                                    | ((br19 & T::from(7u8)) << 28u32);
                                if len > 5 {
                                    let br20 = T::from(b[20]);
                                    let br21 = T::from(b[21]);
                                    let br22 = T::from(b[22]);
                                    let br23 = T::from(b[23]);
                                    nums[5] = (br19 >> 3u32)
                                        | (br20 << 5u32)
                                        | (br21 << 13u32)
                                        | (br22 << 21u32)
                                        | ((br23 & T::from(3u8)) << 29u32);
                                    if len > 6 {
                                        let br24 = T::from(b[24]);
                                        let br25 = T::from(b[25]);
                                        let br26 = T::from(b[26]);
                                        let br27 = T::from(b[27]);
                                        nums[6] = (br23 >> 2u32)
                                            | (br24 << 6u32)
                                            | (br25 << 14u32)
                                            | (br26 << 22u32)
                                            | ((br27 & T::from(1u8)) << 30u32);
                                        if len > 7 {
                                            let br28 = T::from(b[28]);
                                            let br29 = T::from(b[29]);
                                            let br30 = T::from(b[30]);
                                            nums[7] = (br27 >> 1u32)
                                                | (br28 << 7u32)
                                                | (br29 << 15u32)
                                                | (br30 << 23u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        32 => {
            let mut b = [0u8; 32];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 32 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                nums[0] = br0 | (br1 << 8u32) | (br2 << 16u32) | (br3 << 24u32);
                if len > 1 {
                    let br4 = T::from(b[4]);
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    nums[1] = br4 | (br5 << 8u32) | (br6 << 16u32) | (br7 << 24u32);
                    if len > 2 {
                        let br8 = T::from(b[8]);
                        let br9 = T::from(b[9]);
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        nums[2] = br8 | (br9 << 8u32) | (br10 << 16u32) | (br11 << 24u32);
                        if len > 3 {
                            let br12 = T::from(b[12]);
                            let br13 = T::from(b[13]);
                            let br14 = T::from(b[14]);
                            let br15 = T::from(b[15]);
                            nums[3] = br12 | (br13 << 8u32) | (br14 << 16u32) | (br15 << 24u32);
                            if len > 4 {
                                let br16 = T::from(b[16]);
                                let br17 = T::from(b[17]);
                                let br18 = T::from(b[18]);
                                let br19 = T::from(b[19]);
                                nums[4] = br16 | (br17 << 8u32) | (br18 << 16u32) | (br19 << 24u32);
                                if len > 5 {
                                    let br20 = T::from(b[20]);
                                    let br21 = T::from(b[21]);
                                    let br22 = T::from(b[22]);
                                    let br23 = T::from(b[23]);
                                    nums[5] =
                                        br20 | (br21 << 8u32) | (br22 << 16u32) | (br23 << 24u32);
                                    if len > 6 {
                                        let br24 = T::from(b[24]);
                                        let br25 = T::from(b[25]);
                                        let br26 = T::from(b[26]);
                                        let br27 = T::from(b[27]);
                                        nums[6] = br24
                                            | (br25 << 8u32)
                                            | (br26 << 16u32)
                                            | (br27 << 24u32);
                                        if len > 7 {
                                            let br28 = T::from(b[28]);
                                            let br29 = T::from(b[29]);
                                            let br30 = T::from(b[30]);
                                            let br31 = T::from(b[31]);
                                            nums[7] = br28
                                                | (br29 << 8u32)
                                                | (br30 << 16u32)
                                                | (br31 << 24u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        33 => {
            let mut b = [0u8; 33];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 33 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | ((br4 & T::from(1u8)) << 32u32);
                if len > 1 {
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    nums[1] = (br4 >> 1u32)
                        | (br5 << 7u32)
                        | (br6 << 15u32)
                        | (br7 << 23u32)
                        | ((br8 & T::from(3u8)) << 31u32);
                    if len > 2 {
                        let br9 = T::from(b[9]);
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        nums[2] = (br8 >> 2u32)
                            | (br9 << 6u32)
                            | (br10 << 14u32)
                            | (br11 << 22u32)
                            | ((br12 & T::from(7u8)) << 30u32);
                        if len > 3 {
                            let br13 = T::from(b[13]);
                            let br14 = T::from(b[14]);
                            let br15 = T::from(b[15]);
                            let br16 = T::from(b[16]);
                            nums[3] = (br12 >> 3u32)
                                | (br13 << 5u32)
                                | (br14 << 13u32)
                                | (br15 << 21u32)
                                | ((br16 & T::from(15u8)) << 29u32);
                            if len > 4 {
                                let br17 = T::from(b[17]);
                                let br18 = T::from(b[18]);
                                let br19 = T::from(b[19]);
                                let br20 = T::from(b[20]);
                                nums[4] = (br16 >> 4u32)
                                    | (br17 << 4u32)
                                    | (br18 << 12u32)
                                    | (br19 << 20u32)
                                    | ((br20 & T::from(31u8)) << 28u32);
                                if len > 5 {
                                    let br21 = T::from(b[21]);
                                    let br22 = T::from(b[22]);
                                    let br23 = T::from(b[23]);
                                    let br24 = T::from(b[24]);
                                    nums[5] = (br20 >> 5u32)
                                        | (br21 << 3u32)
                                        | (br22 << 11u32)
                                        | (br23 << 19u32)
                                        | ((br24 & T::from(63u8)) << 27u32);
                                    if len > 6 {
                                        let br25 = T::from(b[25]);
                                        let br26 = T::from(b[26]);
                                        let br27 = T::from(b[27]);
                                        let br28 = T::from(b[28]);
                                        nums[6] = (br24 >> 6u32)
                                            | (br25 << 2u32)
                                            | (br26 << 10u32)
                                            | (br27 << 18u32)
                                            | ((br28 & T::from(127u8)) << 26u32);
                                        if len > 7 {
                                            let br29 = T::from(b[29]);
                                            let br30 = T::from(b[30]);
                                            let br31 = T::from(b[31]);
                                            let br32 = T::from(b[32]);
                                            nums[7] = (br28 >> 7u32)
                                                | (br29 << 1u32)
                                                | (br30 << 9u32)
                                                | (br31 << 17u32)
                                                | (br32 << 25u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        34 => {
            let mut b = [0u8; 34];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 34 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | ((br4 & T::from(3u8)) << 32u32);
                if len > 1 {
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    nums[1] = (br4 >> 2u32)
                        | (br5 << 6u32)
                        | (br6 << 14u32)
                        | (br7 << 22u32)
                        | ((br8 & T::from(15u8)) << 30u32);
                    if len > 2 {
                        let br9 = T::from(b[9]);
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        nums[2] = (br8 >> 4u32)
                            | (br9 << 4u32)
                            | (br10 << 12u32)
                            | (br11 << 20u32)
                            | ((br12 & T::from(63u8)) << 28u32);
                        if len > 3 {
                            let br13 = T::from(b[13]);
                            let br14 = T::from(b[14]);
                            let br15 = T::from(b[15]);
                            let br16 = T::from(b[16]);
                            nums[3] = (br12 >> 6u32)
                                | (br13 << 2u32)
                                | (br14 << 10u32)
                                | (br15 << 18u32)
                                | (br16 << 26u32);
                            if len > 4 {
                                let br17 = T::from(b[17]);
                                let br18 = T::from(b[18]);
                                let br19 = T::from(b[19]);
                                let br20 = T::from(b[20]);
                                let br21 = T::from(b[21]);
                                nums[4] = br17
                                    | (br18 << 8u32)
                                    | (br19 << 16u32)
                                    | (br20 << 24u32)
                                    | ((br21 & T::from(3u8)) << 32u32);
                                if len > 5 {
                                    let br22 = T::from(b[22]);
                                    let br23 = T::from(b[23]);
                                    let br24 = T::from(b[24]);
                                    let br25 = T::from(b[25]);
                                    nums[5] = (br21 >> 2u32)
                                        | (br22 << 6u32)
                                        | (br23 << 14u32)
                                        | (br24 << 22u32)
                                        | ((br25 & T::from(15u8)) << 30u32);
                                    if len > 6 {
                                        let br26 = T::from(b[26]);
                                        let br27 = T::from(b[27]);
                                        let br28 = T::from(b[28]);
                                        let br29 = T::from(b[29]);
                                        nums[6] = (br25 >> 4u32)
                                            | (br26 << 4u32)
                                            | (br27 << 12u32)
                                            | (br28 << 20u32)
                                            | ((br29 & T::from(63u8)) << 28u32);
                                        if len > 7 {
                                            let br30 = T::from(b[30]);
                                            let br31 = T::from(b[31]);
                                            let br32 = T::from(b[32]);
                                            let br33 = T::from(b[33]);
                                            nums[7] = (br29 >> 6u32)
                                                | (br30 << 2u32)
                                                | (br31 << 10u32)
                                                | (br32 << 18u32)
                                                | (br33 << 26u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        35 => {
            let mut b = [0u8; 35];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 35 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | ((br4 & T::from(7u8)) << 32u32);
                if len > 1 {
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    nums[1] = (br4 >> 3u32)
                        | (br5 << 5u32)
                        | (br6 << 13u32)
                        | (br7 << 21u32)
                        | ((br8 & T::from(63u8)) << 29u32);
                    if len > 2 {
                        let br9 = T::from(b[9]);
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        nums[2] = (br8 >> 6u32)
                            | (br9 << 2u32)
                            | (br10 << 10u32)
                            | (br11 << 18u32)
                            | (br12 << 26u32)
                            | ((br13 & T::from(1u8)) << 34u32);
                        if len > 3 {
                            let br14 = T::from(b[14]);
                            let br15 = T::from(b[15]);
                            let br16 = T::from(b[16]);
                            let br17 = T::from(b[17]);
                            nums[3] = (br13 >> 1u32)
                                | (br14 << 7u32)
                                | (br15 << 15u32)
                                | (br16 << 23u32)
                                | ((br17 & T::from(15u8)) << 31u32);
                            if len > 4 {
                                let br18 = T::from(b[18]);
                                let br19 = T::from(b[19]);
                                let br20 = T::from(b[20]);
                                let br21 = T::from(b[21]);
                                nums[4] = (br17 >> 4u32)
                                    | (br18 << 4u32)
                                    | (br19 << 12u32)
                                    | (br20 << 20u32)
                                    | ((br21 & T::from(127u8)) << 28u32);
                                if len > 5 {
                                    let br22 = T::from(b[22]);
                                    let br23 = T::from(b[23]);
                                    let br24 = T::from(b[24]);
                                    let br25 = T::from(b[25]);
                                    let br26 = T::from(b[26]);
                                    nums[5] = (br21 >> 7u32)
                                        | (br22 << 1u32)
                                        | (br23 << 9u32)
                                        | (br24 << 17u32)
                                        | (br25 << 25u32)
                                        | ((br26 & T::from(3u8)) << 33u32);
                                    if len > 6 {
                                        let br27 = T::from(b[27]);
                                        let br28 = T::from(b[28]);
                                        let br29 = T::from(b[29]);
                                        let br30 = T::from(b[30]);
                                        nums[6] = (br26 >> 2u32)
                                            | (br27 << 6u32)
                                            | (br28 << 14u32)
                                            | (br29 << 22u32)
                                            | ((br30 & T::from(31u8)) << 30u32);
                                        if len > 7 {
                                            let br31 = T::from(b[31]);
                                            let br32 = T::from(b[32]);
                                            let br33 = T::from(b[33]);
                                            let br34 = T::from(b[34]);
                                            nums[7] = (br30 >> 5u32)
                                                | (br31 << 3u32)
                                                | (br32 << 11u32)
                                                | (br33 << 19u32)
                                                | (br34 << 27u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        36 => {
            let mut b = [0u8; 36];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 36 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | ((br4 & T::from(15u8)) << 32u32);
                if len > 1 {
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    nums[1] = (br4 >> 4u32)
                        | (br5 << 4u32)
                        | (br6 << 12u32)
                        | (br7 << 20u32)
                        | (br8 << 28u32);
                    if len > 2 {
                        let br9 = T::from(b[9]);
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        nums[2] = br9
                            | (br10 << 8u32)
                            | (br11 << 16u32)
                            | (br12 << 24u32)
                            | ((br13 & T::from(15u8)) << 32u32);
                        if len > 3 {
                            let br14 = T::from(b[14]);
                            let br15 = T::from(b[15]);
                            let br16 = T::from(b[16]);
                            let br17 = T::from(b[17]);
                            nums[3] = (br13 >> 4u32)
                                | (br14 << 4u32)
                                | (br15 << 12u32)
                                | (br16 << 20u32)
                                | (br17 << 28u32);
                            if len > 4 {
                                let br18 = T::from(b[18]);
                                let br19 = T::from(b[19]);
                                let br20 = T::from(b[20]);
                                let br21 = T::from(b[21]);
                                let br22 = T::from(b[22]);
                                nums[4] = br18
                                    | (br19 << 8u32)
                                    | (br20 << 16u32)
                                    | (br21 << 24u32)
                                    | ((br22 & T::from(15u8)) << 32u32);
                                if len > 5 {
                                    let br23 = T::from(b[23]);
                                    let br24 = T::from(b[24]);
                                    let br25 = T::from(b[25]);
                                    let br26 = T::from(b[26]);
                                    nums[5] = (br22 >> 4u32)
                                        | (br23 << 4u32)
                                        | (br24 << 12u32)
                                        | (br25 << 20u32)
                                        | (br26 << 28u32);
                                    if len > 6 {
                                        let br27 = T::from(b[27]);
                                        let br28 = T::from(b[28]);
                                        let br29 = T::from(b[29]);
                                        let br30 = T::from(b[30]);
                                        let br31 = T::from(b[31]);
                                        nums[6] = br27
                                            | (br28 << 8u32)
                                            | (br29 << 16u32)
                                            | (br30 << 24u32)
                                            | ((br31 & T::from(15u8)) << 32u32);
                                        if len > 7 {
                                            let br32 = T::from(b[32]);
                                            let br33 = T::from(b[33]);
                                            let br34 = T::from(b[34]);
                                            let br35 = T::from(b[35]);
                                            nums[7] = (br31 >> 4u32)
                                                | (br32 << 4u32)
                                                | (br33 << 12u32)
                                                | (br34 << 20u32)
                                                | (br35 << 28u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        37 => {
            let mut b = [0u8; 37];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 37 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | ((br4 & T::from(31u8)) << 32u32);
                if len > 1 {
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    nums[1] = (br4 >> 5u32)
                        | (br5 << 3u32)
                        | (br6 << 11u32)
                        | (br7 << 19u32)
                        | (br8 << 27u32)
                        | ((br9 & T::from(3u8)) << 35u32);
                    if len > 2 {
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        nums[2] = (br9 >> 2u32)
                            | (br10 << 6u32)
                            | (br11 << 14u32)
                            | (br12 << 22u32)
                            | ((br13 & T::from(127u8)) << 30u32);
                        if len > 3 {
                            let br14 = T::from(b[14]);
                            let br15 = T::from(b[15]);
                            let br16 = T::from(b[16]);
                            let br17 = T::from(b[17]);
                            let br18 = T::from(b[18]);
                            nums[3] = (br13 >> 7u32)
                                | (br14 << 1u32)
                                | (br15 << 9u32)
                                | (br16 << 17u32)
                                | (br17 << 25u32)
                                | ((br18 & T::from(15u8)) << 33u32);
                            if len > 4 {
                                let br19 = T::from(b[19]);
                                let br20 = T::from(b[20]);
                                let br21 = T::from(b[21]);
                                let br22 = T::from(b[22]);
                                let br23 = T::from(b[23]);
                                nums[4] = (br18 >> 4u32)
                                    | (br19 << 4u32)
                                    | (br20 << 12u32)
                                    | (br21 << 20u32)
                                    | (br22 << 28u32)
                                    | ((br23 & T::from(1u8)) << 36u32);
                                if len > 5 {
                                    let br24 = T::from(b[24]);
                                    let br25 = T::from(b[25]);
                                    let br26 = T::from(b[26]);
                                    let br27 = T::from(b[27]);
                                    nums[5] = (br23 >> 1u32)
                                        | (br24 << 7u32)
                                        | (br25 << 15u32)
                                        | (br26 << 23u32)
                                        | ((br27 & T::from(63u8)) << 31u32);
                                    if len > 6 {
                                        let br28 = T::from(b[28]);
                                        let br29 = T::from(b[29]);
                                        let br30 = T::from(b[30]);
                                        let br31 = T::from(b[31]);
                                        let br32 = T::from(b[32]);
                                        nums[6] = (br27 >> 6u32)
                                            | (br28 << 2u32)
                                            | (br29 << 10u32)
                                            | (br30 << 18u32)
                                            | (br31 << 26u32)
                                            | ((br32 & T::from(7u8)) << 34u32);
                                        if len > 7 {
                                            let br33 = T::from(b[33]);
                                            let br34 = T::from(b[34]);
                                            let br35 = T::from(b[35]);
                                            let br36 = T::from(b[36]);
                                            nums[7] = (br32 >> 3u32)
                                                | (br33 << 5u32)
                                                | (br34 << 13u32)
                                                | (br35 << 21u32)
                                                | (br36 << 29u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        38 => {
            let mut b = [0u8; 38];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 38 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | ((br4 & T::from(63u8)) << 32u32);
                if len > 1 {
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    nums[1] = (br4 >> 6u32)
                        | (br5 << 2u32)
                        | (br6 << 10u32)
                        | (br7 << 18u32)
                        | (br8 << 26u32)
                        | ((br9 & T::from(15u8)) << 34u32);
                    if len > 2 {
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        nums[2] = (br9 >> 4u32)
                            | (br10 << 4u32)
                            | (br11 << 12u32)
                            | (br12 << 20u32)
                            | (br13 << 28u32)
                            | ((br14 & T::from(3u8)) << 36u32);
                        if len > 3 {
                            let br15 = T::from(b[15]);
                            let br16 = T::from(b[16]);
                            let br17 = T::from(b[17]);
                            let br18 = T::from(b[18]);
                            nums[3] = (br14 >> 2u32)
                                | (br15 << 6u32)
                                | (br16 << 14u32)
                                | (br17 << 22u32)
                                | (br18 << 30u32);
                            if len > 4 {
                                let br19 = T::from(b[19]);
                                let br20 = T::from(b[20]);
                                let br21 = T::from(b[21]);
                                let br22 = T::from(b[22]);
                                let br23 = T::from(b[23]);
                                nums[4] = br19
                                    | (br20 << 8u32)
                                    | (br21 << 16u32)
                                    | (br22 << 24u32)
                                    | ((br23 & T::from(63u8)) << 32u32);
                                if len > 5 {
                                    let br24 = T::from(b[24]);
                                    let br25 = T::from(b[25]);
                                    let br26 = T::from(b[26]);
                                    let br27 = T::from(b[27]);
                                    let br28 = T::from(b[28]);
                                    nums[5] = (br23 >> 6u32)
                                        | (br24 << 2u32)
                                        | (br25 << 10u32)
                                        | (br26 << 18u32)
                                        | (br27 << 26u32)
                                        | ((br28 & T::from(15u8)) << 34u32);
                                    if len > 6 {
                                        let br29 = T::from(b[29]);
                                        let br30 = T::from(b[30]);
                                        let br31 = T::from(b[31]);
                                        let br32 = T::from(b[32]);
                                        let br33 = T::from(b[33]);
                                        nums[6] = (br28 >> 4u32)
                                            | (br29 << 4u32)
                                            | (br30 << 12u32)
                                            | (br31 << 20u32)
                                            | (br32 << 28u32)
                                            | ((br33 & T::from(3u8)) << 36u32);
                                        if len > 7 {
                                            let br34 = T::from(b[34]);
                                            let br35 = T::from(b[35]);
                                            let br36 = T::from(b[36]);
                                            let br37 = T::from(b[37]);
                                            nums[7] = (br33 >> 2u32)
                                                | (br34 << 6u32)
                                                | (br35 << 14u32)
                                                | (br36 << 22u32)
                                                | (br37 << 30u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        39 => {
            let mut b = [0u8; 39];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 39 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | ((br4 & T::from(127u8)) << 32u32);
                if len > 1 {
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    nums[1] = (br4 >> 7u32)
                        | (br5 << 1u32)
                        | (br6 << 9u32)
                        | (br7 << 17u32)
                        | (br8 << 25u32)
                        | ((br9 & T::from(63u8)) << 33u32);
                    if len > 2 {
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        nums[2] = (br9 >> 6u32)
                            | (br10 << 2u32)
                            | (br11 << 10u32)
                            | (br12 << 18u32)
                            | (br13 << 26u32)
                            | ((br14 & T::from(31u8)) << 34u32);
                        if len > 3 {
                            let br15 = T::from(b[15]);
                            let br16 = T::from(b[16]);
                            let br17 = T::from(b[17]);
                            let br18 = T::from(b[18]);
                            let br19 = T::from(b[19]);
                            nums[3] = (br14 >> 5u32)
                                | (br15 << 3u32)
                                | (br16 << 11u32)
                                | (br17 << 19u32)
                                | (br18 << 27u32)
                                | ((br19 & T::from(15u8)) << 35u32);
                            if len > 4 {
                                let br20 = T::from(b[20]);
                                let br21 = T::from(b[21]);
                                let br22 = T::from(b[22]);
                                let br23 = T::from(b[23]);
                                let br24 = T::from(b[24]);
                                nums[4] = (br19 >> 4u32)
                                    | (br20 << 4u32)
                                    | (br21 << 12u32)
                                    | (br22 << 20u32)
                                    | (br23 << 28u32)
                                    | ((br24 & T::from(7u8)) << 36u32);
                                if len > 5 {
                                    let br25 = T::from(b[25]);
                                    let br26 = T::from(b[26]);
                                    let br27 = T::from(b[27]);
                                    let br28 = T::from(b[28]);
                                    let br29 = T::from(b[29]);
                                    nums[5] = (br24 >> 3u32)
                                        | (br25 << 5u32)
                                        | (br26 << 13u32)
                                        | (br27 << 21u32)
                                        | (br28 << 29u32)
                                        | ((br29 & T::from(3u8)) << 37u32);
                                    if len > 6 {
                                        let br30 = T::from(b[30]);
                                        let br31 = T::from(b[31]);
                                        let br32 = T::from(b[32]);
                                        let br33 = T::from(b[33]);
                                        let br34 = T::from(b[34]);
                                        nums[6] = (br29 >> 2u32)
                                            | (br30 << 6u32)
                                            | (br31 << 14u32)
                                            | (br32 << 22u32)
                                            | (br33 << 30u32)
                                            | ((br34 & T::from(1u8)) << 38u32);
                                        if len > 7 {
                                            let br35 = T::from(b[35]);
                                            let br36 = T::from(b[36]);
                                            let br37 = T::from(b[37]);
                                            let br38 = T::from(b[38]);
                                            nums[7] = (br34 >> 1u32)
                                                | (br35 << 7u32)
                                                | (br36 << 15u32)
                                                | (br37 << 23u32)
                                                | (br38 << 31u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        40 => {
            let mut b = [0u8; 40];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 40 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                nums[0] = br0 | (br1 << 8u32) | (br2 << 16u32) | (br3 << 24u32) | (br4 << 32u32);
                if len > 1 {
                    let br5 = T::from(b[5]);
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    nums[1] =
                        br5 | (br6 << 8u32) | (br7 << 16u32) | (br8 << 24u32) | (br9 << 32u32);
                    if len > 2 {
                        let br10 = T::from(b[10]);
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        nums[2] = br10
                            | (br11 << 8u32)
                            | (br12 << 16u32)
                            | (br13 << 24u32)
                            | (br14 << 32u32);
                        if len > 3 {
                            let br15 = T::from(b[15]);
                            let br16 = T::from(b[16]);
                            let br17 = T::from(b[17]);
                            let br18 = T::from(b[18]);
                            let br19 = T::from(b[19]);
                            nums[3] = br15
                                | (br16 << 8u32)
                                | (br17 << 16u32)
                                | (br18 << 24u32)
                                | (br19 << 32u32);
                            if len > 4 {
                                let br20 = T::from(b[20]);
                                let br21 = T::from(b[21]);
                                let br22 = T::from(b[22]);
                                let br23 = T::from(b[23]);
                                let br24 = T::from(b[24]);
                                nums[4] = br20
                                    | (br21 << 8u32)
                                    | (br22 << 16u32)
                                    | (br23 << 24u32)
                                    | (br24 << 32u32);
                                if len > 5 {
                                    let br25 = T::from(b[25]);
                                    let br26 = T::from(b[26]);
                                    let br27 = T::from(b[27]);
                                    let br28 = T::from(b[28]);
                                    let br29 = T::from(b[29]);
                                    nums[5] = br25
                                        | (br26 << 8u32)
                                        | (br27 << 16u32)
                                        | (br28 << 24u32)
                                        | (br29 << 32u32);
                                    if len > 6 {
                                        let br30 = T::from(b[30]);
                                        let br31 = T::from(b[31]);
                                        let br32 = T::from(b[32]);
                                        let br33 = T::from(b[33]);
                                        let br34 = T::from(b[34]);
                                        nums[6] = br30
                                            | (br31 << 8u32)
                                            | (br32 << 16u32)
                                            | (br33 << 24u32)
                                            | (br34 << 32u32);
                                        if len > 7 {
                                            let br35 = T::from(b[35]);
                                            let br36 = T::from(b[36]);
                                            let br37 = T::from(b[37]);
                                            let br38 = T::from(b[38]);
                                            let br39 = T::from(b[39]);
                                            nums[7] = br35
                                                | (br36 << 8u32)
                                                | (br37 << 16u32)
                                                | (br38 << 24u32)
                                                | (br39 << 32u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        41 => {
            let mut b = [0u8; 41];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 41 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | ((br5 & T::from(1u8)) << 40u32);
                if len > 1 {
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    nums[1] = (br5 >> 1u32)
                        | (br6 << 7u32)
                        | (br7 << 15u32)
                        | (br8 << 23u32)
                        | (br9 << 31u32)
                        | ((br10 & T::from(3u8)) << 39u32);
                    if len > 2 {
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        nums[2] = (br10 >> 2u32)
                            | (br11 << 6u32)
                            | (br12 << 14u32)
                            | (br13 << 22u32)
                            | (br14 << 30u32)
                            | ((br15 & T::from(7u8)) << 38u32);
                        if len > 3 {
                            let br16 = T::from(b[16]);
                            let br17 = T::from(b[17]);
                            let br18 = T::from(b[18]);
                            let br19 = T::from(b[19]);
                            let br20 = T::from(b[20]);
                            nums[3] = (br15 >> 3u32)
                                | (br16 << 5u32)
                                | (br17 << 13u32)
                                | (br18 << 21u32)
                                | (br19 << 29u32)
                                | ((br20 & T::from(15u8)) << 37u32);
                            if len > 4 {
                                let br21 = T::from(b[21]);
                                let br22 = T::from(b[22]);
                                let br23 = T::from(b[23]);
                                let br24 = T::from(b[24]);
                                let br25 = T::from(b[25]);
                                nums[4] = (br20 >> 4u32)
                                    | (br21 << 4u32)
                                    | (br22 << 12u32)
                                    | (br23 << 20u32)
                                    | (br24 << 28u32)
                                    | ((br25 & T::from(31u8)) << 36u32);
                                if len > 5 {
                                    let br26 = T::from(b[26]);
                                    let br27 = T::from(b[27]);
                                    let br28 = T::from(b[28]);
                                    let br29 = T::from(b[29]);
                                    let br30 = T::from(b[30]);
                                    nums[5] = (br25 >> 5u32)
                                        | (br26 << 3u32)
                                        | (br27 << 11u32)
                                        | (br28 << 19u32)
                                        | (br29 << 27u32)
                                        | ((br30 & T::from(63u8)) << 35u32);
                                    if len > 6 {
                                        let br31 = T::from(b[31]);
                                        let br32 = T::from(b[32]);
                                        let br33 = T::from(b[33]);
                                        let br34 = T::from(b[34]);
                                        let br35 = T::from(b[35]);
                                        nums[6] = (br30 >> 6u32)
                                            | (br31 << 2u32)
                                            | (br32 << 10u32)
                                            | (br33 << 18u32)
                                            | (br34 << 26u32)
                                            | ((br35 & T::from(127u8)) << 34u32);
                                        if len > 7 {
                                            let br36 = T::from(b[36]);
                                            let br37 = T::from(b[37]);
                                            let br38 = T::from(b[38]);
                                            let br39 = T::from(b[39]);
                                            let br40 = T::from(b[40]);
                                            nums[7] = (br35 >> 7u32)
                                                | (br36 << 1u32)
                                                | (br37 << 9u32)
                                                | (br38 << 17u32)
                                                | (br39 << 25u32)
                                                | (br40 << 33u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        42 => {
            let mut b = [0u8; 42];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 42 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | ((br5 & T::from(3u8)) << 40u32);
                if len > 1 {
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    nums[1] = (br5 >> 2u32)
                        | (br6 << 6u32)
                        | (br7 << 14u32)
                        | (br8 << 22u32)
                        | (br9 << 30u32)
                        | ((br10 & T::from(15u8)) << 38u32);
                    if len > 2 {
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        nums[2] = (br10 >> 4u32)
                            | (br11 << 4u32)
                            | (br12 << 12u32)
                            | (br13 << 20u32)
                            | (br14 << 28u32)
                            | ((br15 & T::from(63u8)) << 36u32);
                        if len > 3 {
                            let br16 = T::from(b[16]);
                            let br17 = T::from(b[17]);
                            let br18 = T::from(b[18]);
                            let br19 = T::from(b[19]);
                            let br20 = T::from(b[20]);
                            nums[3] = (br15 >> 6u32)
                                | (br16 << 2u32)
                                | (br17 << 10u32)
                                | (br18 << 18u32)
                                | (br19 << 26u32)
                                | (br20 << 34u32);
                            if len > 4 {
                                let br21 = T::from(b[21]);
                                let br22 = T::from(b[22]);
                                let br23 = T::from(b[23]);
                                let br24 = T::from(b[24]);
                                let br25 = T::from(b[25]);
                                let br26 = T::from(b[26]);
                                nums[4] = br21
                                    | (br22 << 8u32)
                                    | (br23 << 16u32)
                                    | (br24 << 24u32)
                                    | (br25 << 32u32)
                                    | ((br26 & T::from(3u8)) << 40u32);
                                if len > 5 {
                                    let br27 = T::from(b[27]);
                                    let br28 = T::from(b[28]);
                                    let br29 = T::from(b[29]);
                                    let br30 = T::from(b[30]);
                                    let br31 = T::from(b[31]);
                                    nums[5] = (br26 >> 2u32)
                                        | (br27 << 6u32)
                                        | (br28 << 14u32)
                                        | (br29 << 22u32)
                                        | (br30 << 30u32)
                                        | ((br31 & T::from(15u8)) << 38u32);
                                    if len > 6 {
                                        let br32 = T::from(b[32]);
                                        let br33 = T::from(b[33]);
                                        let br34 = T::from(b[34]);
                                        let br35 = T::from(b[35]);
                                        let br36 = T::from(b[36]);
                                        nums[6] = (br31 >> 4u32)
                                            | (br32 << 4u32)
                                            | (br33 << 12u32)
                                            | (br34 << 20u32)
                                            | (br35 << 28u32)
                                            | ((br36 & T::from(63u8)) << 36u32);
                                        if len > 7 {
                                            let br37 = T::from(b[37]);
                                            let br38 = T::from(b[38]);
                                            let br39 = T::from(b[39]);
                                            let br40 = T::from(b[40]);
                                            let br41 = T::from(b[41]);
                                            nums[7] = (br36 >> 6u32)
                                                | (br37 << 2u32)
                                                | (br38 << 10u32)
                                                | (br39 << 18u32)
                                                | (br40 << 26u32)
                                                | (br41 << 34u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        43 => {
            let mut b = [0u8; 43];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 43 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | ((br5 & T::from(7u8)) << 40u32);
                if len > 1 {
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    nums[1] = (br5 >> 3u32)
                        | (br6 << 5u32)
                        | (br7 << 13u32)
                        | (br8 << 21u32)
                        | (br9 << 29u32)
                        | ((br10 & T::from(63u8)) << 37u32);
                    if len > 2 {
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        nums[2] = (br10 >> 6u32)
                            | (br11 << 2u32)
                            | (br12 << 10u32)
                            | (br13 << 18u32)
                            | (br14 << 26u32)
                            | (br15 << 34u32)
                            | ((br16 & T::from(1u8)) << 42u32);
                        if len > 3 {
                            let br17 = T::from(b[17]);
                            let br18 = T::from(b[18]);
                            let br19 = T::from(b[19]);
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            nums[3] = (br16 >> 1u32)
                                | (br17 << 7u32)
                                | (br18 << 15u32)
                                | (br19 << 23u32)
                                | (br20 << 31u32)
                                | ((br21 & T::from(15u8)) << 39u32);
                            if len > 4 {
                                let br22 = T::from(b[22]);
                                let br23 = T::from(b[23]);
                                let br24 = T::from(b[24]);
                                let br25 = T::from(b[25]);
                                let br26 = T::from(b[26]);
                                nums[4] = (br21 >> 4u32)
                                    | (br22 << 4u32)
                                    | (br23 << 12u32)
                                    | (br24 << 20u32)
                                    | (br25 << 28u32)
                                    | ((br26 & T::from(127u8)) << 36u32);
                                if len > 5 {
                                    let br27 = T::from(b[27]);
                                    let br28 = T::from(b[28]);
                                    let br29 = T::from(b[29]);
                                    let br30 = T::from(b[30]);
                                    let br31 = T::from(b[31]);
                                    let br32 = T::from(b[32]);
                                    nums[5] = (br26 >> 7u32)
                                        | (br27 << 1u32)
                                        | (br28 << 9u32)
                                        | (br29 << 17u32)
                                        | (br30 << 25u32)
                                        | (br31 << 33u32)
                                        | ((br32 & T::from(3u8)) << 41u32);
                                    if len > 6 {
                                        let br33 = T::from(b[33]);
                                        let br34 = T::from(b[34]);
                                        let br35 = T::from(b[35]);
                                        let br36 = T::from(b[36]);
                                        let br37 = T::from(b[37]);
                                        nums[6] = (br32 >> 2u32)
                                            | (br33 << 6u32)
                                            | (br34 << 14u32)
                                            | (br35 << 22u32)
                                            | (br36 << 30u32)
                                            | ((br37 & T::from(31u8)) << 38u32);
                                        if len > 7 {
                                            let br38 = T::from(b[38]);
                                            let br39 = T::from(b[39]);
                                            let br40 = T::from(b[40]);
                                            let br41 = T::from(b[41]);
                                            let br42 = T::from(b[42]);
                                            nums[7] = (br37 >> 5u32)
                                                | (br38 << 3u32)
                                                | (br39 << 11u32)
                                                | (br40 << 19u32)
                                                | (br41 << 27u32)
                                                | (br42 << 35u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        44 => {
            let mut b = [0u8; 44];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 44 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | ((br5 & T::from(15u8)) << 40u32);
                if len > 1 {
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    nums[1] = (br5 >> 4u32)
                        | (br6 << 4u32)
                        | (br7 << 12u32)
                        | (br8 << 20u32)
                        | (br9 << 28u32)
                        | (br10 << 36u32);
                    if len > 2 {
                        let br11 = T::from(b[11]);
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        nums[2] = br11
                            | (br12 << 8u32)
                            | (br13 << 16u32)
                            | (br14 << 24u32)
                            | (br15 << 32u32)
                            | ((br16 & T::from(15u8)) << 40u32);
                        if len > 3 {
                            let br17 = T::from(b[17]);
                            let br18 = T::from(b[18]);
                            let br19 = T::from(b[19]);
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            nums[3] = (br16 >> 4u32)
                                | (br17 << 4u32)
                                | (br18 << 12u32)
                                | (br19 << 20u32)
                                | (br20 << 28u32)
                                | (br21 << 36u32);
                            if len > 4 {
                                let br22 = T::from(b[22]);
                                let br23 = T::from(b[23]);
                                let br24 = T::from(b[24]);
                                let br25 = T::from(b[25]);
                                let br26 = T::from(b[26]);
                                let br27 = T::from(b[27]);
                                nums[4] = br22
                                    | (br23 << 8u32)
                                    | (br24 << 16u32)
                                    | (br25 << 24u32)
                                    | (br26 << 32u32)
                                    | ((br27 & T::from(15u8)) << 40u32);
                                if len > 5 {
                                    let br28 = T::from(b[28]);
                                    let br29 = T::from(b[29]);
                                    let br30 = T::from(b[30]);
                                    let br31 = T::from(b[31]);
                                    let br32 = T::from(b[32]);
                                    nums[5] = (br27 >> 4u32)
                                        | (br28 << 4u32)
                                        | (br29 << 12u32)
                                        | (br30 << 20u32)
                                        | (br31 << 28u32)
                                        | (br32 << 36u32);
                                    if len > 6 {
                                        let br33 = T::from(b[33]);
                                        let br34 = T::from(b[34]);
                                        let br35 = T::from(b[35]);
                                        let br36 = T::from(b[36]);
                                        let br37 = T::from(b[37]);
                                        let br38 = T::from(b[38]);
                                        nums[6] = br33
                                            | (br34 << 8u32)
                                            | (br35 << 16u32)
                                            | (br36 << 24u32)
                                            | (br37 << 32u32)
                                            | ((br38 & T::from(15u8)) << 40u32);
                                        if len > 7 {
                                            let br39 = T::from(b[39]);
                                            let br40 = T::from(b[40]);
                                            let br41 = T::from(b[41]);
                                            let br42 = T::from(b[42]);
                                            let br43 = T::from(b[43]);
                                            nums[7] = (br38 >> 4u32)
                                                | (br39 << 4u32)
                                                | (br40 << 12u32)
                                                | (br41 << 20u32)
                                                | (br42 << 28u32)
                                                | (br43 << 36u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        45 => {
            let mut b = [0u8; 45];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 45 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | ((br5 & T::from(31u8)) << 40u32);
                if len > 1 {
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    nums[1] = (br5 >> 5u32)
                        | (br6 << 3u32)
                        | (br7 << 11u32)
                        | (br8 << 19u32)
                        | (br9 << 27u32)
                        | (br10 << 35u32)
                        | ((br11 & T::from(3u8)) << 43u32);
                    if len > 2 {
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        nums[2] = (br11 >> 2u32)
                            | (br12 << 6u32)
                            | (br13 << 14u32)
                            | (br14 << 22u32)
                            | (br15 << 30u32)
                            | ((br16 & T::from(127u8)) << 38u32);
                        if len > 3 {
                            let br17 = T::from(b[17]);
                            let br18 = T::from(b[18]);
                            let br19 = T::from(b[19]);
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            nums[3] = (br16 >> 7u32)
                                | (br17 << 1u32)
                                | (br18 << 9u32)
                                | (br19 << 17u32)
                                | (br20 << 25u32)
                                | (br21 << 33u32)
                                | ((br22 & T::from(15u8)) << 41u32);
                            if len > 4 {
                                let br23 = T::from(b[23]);
                                let br24 = T::from(b[24]);
                                let br25 = T::from(b[25]);
                                let br26 = T::from(b[26]);
                                let br27 = T::from(b[27]);
                                let br28 = T::from(b[28]);
                                nums[4] = (br22 >> 4u32)
                                    | (br23 << 4u32)
                                    | (br24 << 12u32)
                                    | (br25 << 20u32)
                                    | (br26 << 28u32)
                                    | (br27 << 36u32)
                                    | ((br28 & T::from(1u8)) << 44u32);
                                if len > 5 {
                                    let br29 = T::from(b[29]);
                                    let br30 = T::from(b[30]);
                                    let br31 = T::from(b[31]);
                                    let br32 = T::from(b[32]);
                                    let br33 = T::from(b[33]);
                                    nums[5] = (br28 >> 1u32)
                                        | (br29 << 7u32)
                                        | (br30 << 15u32)
                                        | (br31 << 23u32)
                                        | (br32 << 31u32)
                                        | ((br33 & T::from(63u8)) << 39u32);
                                    if len > 6 {
                                        let br34 = T::from(b[34]);
                                        let br35 = T::from(b[35]);
                                        let br36 = T::from(b[36]);
                                        let br37 = T::from(b[37]);
                                        let br38 = T::from(b[38]);
                                        let br39 = T::from(b[39]);
                                        nums[6] = (br33 >> 6u32)
                                            | (br34 << 2u32)
                                            | (br35 << 10u32)
                                            | (br36 << 18u32)
                                            | (br37 << 26u32)
                                            | (br38 << 34u32)
                                            | ((br39 & T::from(7u8)) << 42u32);
                                        if len > 7 {
                                            let br40 = T::from(b[40]);
                                            let br41 = T::from(b[41]);
                                            let br42 = T::from(b[42]);
                                            let br43 = T::from(b[43]);
                                            let br44 = T::from(b[44]);
                                            nums[7] = (br39 >> 3u32)
                                                | (br40 << 5u32)
                                                | (br41 << 13u32)
                                                | (br42 << 21u32)
                                                | (br43 << 29u32)
                                                | (br44 << 37u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        46 => {
            let mut b = [0u8; 46];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 46 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | ((br5 & T::from(63u8)) << 40u32);
                if len > 1 {
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    nums[1] = (br5 >> 6u32)
                        | (br6 << 2u32)
                        | (br7 << 10u32)
                        | (br8 << 18u32)
                        | (br9 << 26u32)
                        | (br10 << 34u32)
                        | ((br11 & T::from(15u8)) << 42u32);
                    if len > 2 {
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        nums[2] = (br11 >> 4u32)
                            | (br12 << 4u32)
                            | (br13 << 12u32)
                            | (br14 << 20u32)
                            | (br15 << 28u32)
                            | (br16 << 36u32)
                            | ((br17 & T::from(3u8)) << 44u32);
                        if len > 3 {
                            let br18 = T::from(b[18]);
                            let br19 = T::from(b[19]);
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            nums[3] = (br17 >> 2u32)
                                | (br18 << 6u32)
                                | (br19 << 14u32)
                                | (br20 << 22u32)
                                | (br21 << 30u32)
                                | (br22 << 38u32);
                            if len > 4 {
                                let br23 = T::from(b[23]);
                                let br24 = T::from(b[24]);
                                let br25 = T::from(b[25]);
                                let br26 = T::from(b[26]);
                                let br27 = T::from(b[27]);
                                let br28 = T::from(b[28]);
                                nums[4] = br23
                                    | (br24 << 8u32)
                                    | (br25 << 16u32)
                                    | (br26 << 24u32)
                                    | (br27 << 32u32)
                                    | ((br28 & T::from(63u8)) << 40u32);
                                if len > 5 {
                                    let br29 = T::from(b[29]);
                                    let br30 = T::from(b[30]);
                                    let br31 = T::from(b[31]);
                                    let br32 = T::from(b[32]);
                                    let br33 = T::from(b[33]);
                                    let br34 = T::from(b[34]);
                                    nums[5] = (br28 >> 6u32)
                                        | (br29 << 2u32)
                                        | (br30 << 10u32)
                                        | (br31 << 18u32)
                                        | (br32 << 26u32)
                                        | (br33 << 34u32)
                                        | ((br34 & T::from(15u8)) << 42u32);
                                    if len > 6 {
                                        let br35 = T::from(b[35]);
                                        let br36 = T::from(b[36]);
                                        let br37 = T::from(b[37]);
                                        let br38 = T::from(b[38]);
                                        let br39 = T::from(b[39]);
                                        let br40 = T::from(b[40]);
                                        nums[6] = (br34 >> 4u32)
                                            | (br35 << 4u32)
                                            | (br36 << 12u32)
                                            | (br37 << 20u32)
                                            | (br38 << 28u32)
                                            | (br39 << 36u32)
                                            | ((br40 & T::from(3u8)) << 44u32);
                                        if len > 7 {
                                            let br41 = T::from(b[41]);
                                            let br42 = T::from(b[42]);
                                            let br43 = T::from(b[43]);
                                            let br44 = T::from(b[44]);
                                            let br45 = T::from(b[45]);
                                            nums[7] = (br40 >> 2u32)
                                                | (br41 << 6u32)
                                                | (br42 << 14u32)
                                                | (br43 << 22u32)
                                                | (br44 << 30u32)
                                                | (br45 << 38u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        47 => {
            let mut b = [0u8; 47];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 47 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | ((br5 & T::from(127u8)) << 40u32);
                if len > 1 {
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    nums[1] = (br5 >> 7u32)
                        | (br6 << 1u32)
                        | (br7 << 9u32)
                        | (br8 << 17u32)
                        | (br9 << 25u32)
                        | (br10 << 33u32)
                        | ((br11 & T::from(63u8)) << 41u32);
                    if len > 2 {
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        nums[2] = (br11 >> 6u32)
                            | (br12 << 2u32)
                            | (br13 << 10u32)
                            | (br14 << 18u32)
                            | (br15 << 26u32)
                            | (br16 << 34u32)
                            | ((br17 & T::from(31u8)) << 42u32);
                        if len > 3 {
                            let br18 = T::from(b[18]);
                            let br19 = T::from(b[19]);
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            nums[3] = (br17 >> 5u32)
                                | (br18 << 3u32)
                                | (br19 << 11u32)
                                | (br20 << 19u32)
                                | (br21 << 27u32)
                                | (br22 << 35u32)
                                | ((br23 & T::from(15u8)) << 43u32);
                            if len > 4 {
                                let br24 = T::from(b[24]);
                                let br25 = T::from(b[25]);
                                let br26 = T::from(b[26]);
                                let br27 = T::from(b[27]);
                                let br28 = T::from(b[28]);
                                let br29 = T::from(b[29]);
                                nums[4] = (br23 >> 4u32)
                                    | (br24 << 4u32)
                                    | (br25 << 12u32)
                                    | (br26 << 20u32)
                                    | (br27 << 28u32)
                                    | (br28 << 36u32)
                                    | ((br29 & T::from(7u8)) << 44u32);
                                if len > 5 {
                                    let br30 = T::from(b[30]);
                                    let br31 = T::from(b[31]);
                                    let br32 = T::from(b[32]);
                                    let br33 = T::from(b[33]);
                                    let br34 = T::from(b[34]);
                                    let br35 = T::from(b[35]);
                                    nums[5] = (br29 >> 3u32)
                                        | (br30 << 5u32)
                                        | (br31 << 13u32)
                                        | (br32 << 21u32)
                                        | (br33 << 29u32)
                                        | (br34 << 37u32)
                                        | ((br35 & T::from(3u8)) << 45u32);
                                    if len > 6 {
                                        let br36 = T::from(b[36]);
                                        let br37 = T::from(b[37]);
                                        let br38 = T::from(b[38]);
                                        let br39 = T::from(b[39]);
                                        let br40 = T::from(b[40]);
                                        let br41 = T::from(b[41]);
                                        nums[6] = (br35 >> 2u32)
                                            | (br36 << 6u32)
                                            | (br37 << 14u32)
                                            | (br38 << 22u32)
                                            | (br39 << 30u32)
                                            | (br40 << 38u32)
                                            | ((br41 & T::from(1u8)) << 46u32);
                                        if len > 7 {
                                            let br42 = T::from(b[42]);
                                            let br43 = T::from(b[43]);
                                            let br44 = T::from(b[44]);
                                            let br45 = T::from(b[45]);
                                            let br46 = T::from(b[46]);
                                            nums[7] = (br41 >> 1u32)
                                                | (br42 << 7u32)
                                                | (br43 << 15u32)
                                                | (br44 << 23u32)
                                                | (br45 << 31u32)
                                                | (br46 << 39u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        48 => {
            let mut b = [0u8; 48];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 48 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32);
                if len > 1 {
                    let br6 = T::from(b[6]);
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    nums[1] = br6
                        | (br7 << 8u32)
                        | (br8 << 16u32)
                        | (br9 << 24u32)
                        | (br10 << 32u32)
                        | (br11 << 40u32);
                    if len > 2 {
                        let br12 = T::from(b[12]);
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        nums[2] = br12
                            | (br13 << 8u32)
                            | (br14 << 16u32)
                            | (br15 << 24u32)
                            | (br16 << 32u32)
                            | (br17 << 40u32);
                        if len > 3 {
                            let br18 = T::from(b[18]);
                            let br19 = T::from(b[19]);
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            nums[3] = br18
                                | (br19 << 8u32)
                                | (br20 << 16u32)
                                | (br21 << 24u32)
                                | (br22 << 32u32)
                                | (br23 << 40u32);
                            if len > 4 {
                                let br24 = T::from(b[24]);
                                let br25 = T::from(b[25]);
                                let br26 = T::from(b[26]);
                                let br27 = T::from(b[27]);
                                let br28 = T::from(b[28]);
                                let br29 = T::from(b[29]);
                                nums[4] = br24
                                    | (br25 << 8u32)
                                    | (br26 << 16u32)
                                    | (br27 << 24u32)
                                    | (br28 << 32u32)
                                    | (br29 << 40u32);
                                if len > 5 {
                                    let br30 = T::from(b[30]);
                                    let br31 = T::from(b[31]);
                                    let br32 = T::from(b[32]);
                                    let br33 = T::from(b[33]);
                                    let br34 = T::from(b[34]);
                                    let br35 = T::from(b[35]);
                                    nums[5] = br30
                                        | (br31 << 8u32)
                                        | (br32 << 16u32)
                                        | (br33 << 24u32)
                                        | (br34 << 32u32)
                                        | (br35 << 40u32);
                                    if len > 6 {
                                        let br36 = T::from(b[36]);
                                        let br37 = T::from(b[37]);
                                        let br38 = T::from(b[38]);
                                        let br39 = T::from(b[39]);
                                        let br40 = T::from(b[40]);
                                        let br41 = T::from(b[41]);
                                        nums[6] = br36
                                            | (br37 << 8u32)
                                            | (br38 << 16u32)
                                            | (br39 << 24u32)
                                            | (br40 << 32u32)
                                            | (br41 << 40u32);
                                        if len > 7 {
                                            let br42 = T::from(b[42]);
                                            let br43 = T::from(b[43]);
                                            let br44 = T::from(b[44]);
                                            let br45 = T::from(b[45]);
                                            let br46 = T::from(b[46]);
                                            let br47 = T::from(b[47]);
                                            nums[7] = br42
                                                | (br43 << 8u32)
                                                | (br44 << 16u32)
                                                | (br45 << 24u32)
                                                | (br46 << 32u32)
                                                | (br47 << 40u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        49 => {
            let mut b = [0u8; 49];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 49 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | ((br6 & T::from(1u8)) << 48u32);
                if len > 1 {
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    nums[1] = (br6 >> 1u32)
                        | (br7 << 7u32)
                        | (br8 << 15u32)
                        | (br9 << 23u32)
                        | (br10 << 31u32)
                        | (br11 << 39u32)
                        | ((br12 & T::from(3u8)) << 47u32);
                    if len > 2 {
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        nums[2] = (br12 >> 2u32)
                            | (br13 << 6u32)
                            | (br14 << 14u32)
                            | (br15 << 22u32)
                            | (br16 << 30u32)
                            | (br17 << 38u32)
                            | ((br18 & T::from(7u8)) << 46u32);
                        if len > 3 {
                            let br19 = T::from(b[19]);
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            nums[3] = (br18 >> 3u32)
                                | (br19 << 5u32)
                                | (br20 << 13u32)
                                | (br21 << 21u32)
                                | (br22 << 29u32)
                                | (br23 << 37u32)
                                | ((br24 & T::from(15u8)) << 45u32);
                            if len > 4 {
                                let br25 = T::from(b[25]);
                                let br26 = T::from(b[26]);
                                let br27 = T::from(b[27]);
                                let br28 = T::from(b[28]);
                                let br29 = T::from(b[29]);
                                let br30 = T::from(b[30]);
                                nums[4] = (br24 >> 4u32)
                                    | (br25 << 4u32)
                                    | (br26 << 12u32)
                                    | (br27 << 20u32)
                                    | (br28 << 28u32)
                                    | (br29 << 36u32)
                                    | ((br30 & T::from(31u8)) << 44u32);
                                if len > 5 {
                                    let br31 = T::from(b[31]);
                                    let br32 = T::from(b[32]);
                                    let br33 = T::from(b[33]);
                                    let br34 = T::from(b[34]);
                                    let br35 = T::from(b[35]);
                                    let br36 = T::from(b[36]);
                                    nums[5] = (br30 >> 5u32)
                                        | (br31 << 3u32)
                                        | (br32 << 11u32)
                                        | (br33 << 19u32)
                                        | (br34 << 27u32)
                                        | (br35 << 35u32)
                                        | ((br36 & T::from(63u8)) << 43u32);
                                    if len > 6 {
                                        let br37 = T::from(b[37]);
                                        let br38 = T::from(b[38]);
                                        let br39 = T::from(b[39]);
                                        let br40 = T::from(b[40]);
                                        let br41 = T::from(b[41]);
                                        let br42 = T::from(b[42]);
                                        nums[6] = (br36 >> 6u32)
                                            | (br37 << 2u32)
                                            | (br38 << 10u32)
                                            | (br39 << 18u32)
                                            | (br40 << 26u32)
                                            | (br41 << 34u32)
                                            | ((br42 & T::from(127u8)) << 42u32);
                                        if len > 7 {
                                            let br43 = T::from(b[43]);
                                            let br44 = T::from(b[44]);
                                            let br45 = T::from(b[45]);
                                            let br46 = T::from(b[46]);
                                            let br47 = T::from(b[47]);
                                            let br48 = T::from(b[48]);
                                            nums[7] = (br42 >> 7u32)
                                                | (br43 << 1u32)
                                                | (br44 << 9u32)
                                                | (br45 << 17u32)
                                                | (br46 << 25u32)
                                                | (br47 << 33u32)
                                                | (br48 << 41u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        50 => {
            let mut b = [0u8; 50];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 50 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | ((br6 & T::from(3u8)) << 48u32);
                if len > 1 {
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    nums[1] = (br6 >> 2u32)
                        | (br7 << 6u32)
                        | (br8 << 14u32)
                        | (br9 << 22u32)
                        | (br10 << 30u32)
                        | (br11 << 38u32)
                        | ((br12 & T::from(15u8)) << 46u32);
                    if len > 2 {
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        nums[2] = (br12 >> 4u32)
                            | (br13 << 4u32)
                            | (br14 << 12u32)
                            | (br15 << 20u32)
                            | (br16 << 28u32)
                            | (br17 << 36u32)
                            | ((br18 & T::from(63u8)) << 44u32);
                        if len > 3 {
                            let br19 = T::from(b[19]);
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            nums[3] = (br18 >> 6u32)
                                | (br19 << 2u32)
                                | (br20 << 10u32)
                                | (br21 << 18u32)
                                | (br22 << 26u32)
                                | (br23 << 34u32)
                                | (br24 << 42u32);
                            if len > 4 {
                                let br25 = T::from(b[25]);
                                let br26 = T::from(b[26]);
                                let br27 = T::from(b[27]);
                                let br28 = T::from(b[28]);
                                let br29 = T::from(b[29]);
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                nums[4] = br25
                                    | (br26 << 8u32)
                                    | (br27 << 16u32)
                                    | (br28 << 24u32)
                                    | (br29 << 32u32)
                                    | (br30 << 40u32)
                                    | ((br31 & T::from(3u8)) << 48u32);
                                if len > 5 {
                                    let br32 = T::from(b[32]);
                                    let br33 = T::from(b[33]);
                                    let br34 = T::from(b[34]);
                                    let br35 = T::from(b[35]);
                                    let br36 = T::from(b[36]);
                                    let br37 = T::from(b[37]);
                                    nums[5] = (br31 >> 2u32)
                                        | (br32 << 6u32)
                                        | (br33 << 14u32)
                                        | (br34 << 22u32)
                                        | (br35 << 30u32)
                                        | (br36 << 38u32)
                                        | ((br37 & T::from(15u8)) << 46u32);
                                    if len > 6 {
                                        let br38 = T::from(b[38]);
                                        let br39 = T::from(b[39]);
                                        let br40 = T::from(b[40]);
                                        let br41 = T::from(b[41]);
                                        let br42 = T::from(b[42]);
                                        let br43 = T::from(b[43]);
                                        nums[6] = (br37 >> 4u32)
                                            | (br38 << 4u32)
                                            | (br39 << 12u32)
                                            | (br40 << 20u32)
                                            | (br41 << 28u32)
                                            | (br42 << 36u32)
                                            | ((br43 & T::from(63u8)) << 44u32);
                                        if len > 7 {
                                            let br44 = T::from(b[44]);
                                            let br45 = T::from(b[45]);
                                            let br46 = T::from(b[46]);
                                            let br47 = T::from(b[47]);
                                            let br48 = T::from(b[48]);
                                            let br49 = T::from(b[49]);
                                            nums[7] = (br43 >> 6u32)
                                                | (br44 << 2u32)
                                                | (br45 << 10u32)
                                                | (br46 << 18u32)
                                                | (br47 << 26u32)
                                                | (br48 << 34u32)
                                                | (br49 << 42u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        51 => {
            let mut b = [0u8; 51];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 51 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | ((br6 & T::from(7u8)) << 48u32);
                if len > 1 {
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    nums[1] = (br6 >> 3u32)
                        | (br7 << 5u32)
                        | (br8 << 13u32)
                        | (br9 << 21u32)
                        | (br10 << 29u32)
                        | (br11 << 37u32)
                        | ((br12 & T::from(63u8)) << 45u32);
                    if len > 2 {
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        let br19 = T::from(b[19]);
                        nums[2] = (br12 >> 6u32)
                            | (br13 << 2u32)
                            | (br14 << 10u32)
                            | (br15 << 18u32)
                            | (br16 << 26u32)
                            | (br17 << 34u32)
                            | (br18 << 42u32)
                            | ((br19 & T::from(1u8)) << 50u32);
                        if len > 3 {
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            let br25 = T::from(b[25]);
                            nums[3] = (br19 >> 1u32)
                                | (br20 << 7u32)
                                | (br21 << 15u32)
                                | (br22 << 23u32)
                                | (br23 << 31u32)
                                | (br24 << 39u32)
                                | ((br25 & T::from(15u8)) << 47u32);
                            if len > 4 {
                                let br26 = T::from(b[26]);
                                let br27 = T::from(b[27]);
                                let br28 = T::from(b[28]);
                                let br29 = T::from(b[29]);
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                nums[4] = (br25 >> 4u32)
                                    | (br26 << 4u32)
                                    | (br27 << 12u32)
                                    | (br28 << 20u32)
                                    | (br29 << 28u32)
                                    | (br30 << 36u32)
                                    | ((br31 & T::from(127u8)) << 44u32);
                                if len > 5 {
                                    let br32 = T::from(b[32]);
                                    let br33 = T::from(b[33]);
                                    let br34 = T::from(b[34]);
                                    let br35 = T::from(b[35]);
                                    let br36 = T::from(b[36]);
                                    let br37 = T::from(b[37]);
                                    let br38 = T::from(b[38]);
                                    nums[5] = (br31 >> 7u32)
                                        | (br32 << 1u32)
                                        | (br33 << 9u32)
                                        | (br34 << 17u32)
                                        | (br35 << 25u32)
                                        | (br36 << 33u32)
                                        | (br37 << 41u32)
                                        | ((br38 & T::from(3u8)) << 49u32);
                                    if len > 6 {
                                        let br39 = T::from(b[39]);
                                        let br40 = T::from(b[40]);
                                        let br41 = T::from(b[41]);
                                        let br42 = T::from(b[42]);
                                        let br43 = T::from(b[43]);
                                        let br44 = T::from(b[44]);
                                        nums[6] = (br38 >> 2u32)
                                            | (br39 << 6u32)
                                            | (br40 << 14u32)
                                            | (br41 << 22u32)
                                            | (br42 << 30u32)
                                            | (br43 << 38u32)
                                            | ((br44 & T::from(31u8)) << 46u32);
                                        if len > 7 {
                                            let br45 = T::from(b[45]);
                                            let br46 = T::from(b[46]);
                                            let br47 = T::from(b[47]);
                                            let br48 = T::from(b[48]);
                                            let br49 = T::from(b[49]);
                                            let br50 = T::from(b[50]);
                                            nums[7] = (br44 >> 5u32)
                                                | (br45 << 3u32)
                                                | (br46 << 11u32)
                                                | (br47 << 19u32)
                                                | (br48 << 27u32)
                                                | (br49 << 35u32)
                                                | (br50 << 43u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        52 => {
            let mut b = [0u8; 52];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 52 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | ((br6 & T::from(15u8)) << 48u32);
                if len > 1 {
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    nums[1] = (br6 >> 4u32)
                        | (br7 << 4u32)
                        | (br8 << 12u32)
                        | (br9 << 20u32)
                        | (br10 << 28u32)
                        | (br11 << 36u32)
                        | (br12 << 44u32);
                    if len > 2 {
                        let br13 = T::from(b[13]);
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        let br19 = T::from(b[19]);
                        nums[2] = br13
                            | (br14 << 8u32)
                            | (br15 << 16u32)
                            | (br16 << 24u32)
                            | (br17 << 32u32)
                            | (br18 << 40u32)
                            | ((br19 & T::from(15u8)) << 48u32);
                        if len > 3 {
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            let br25 = T::from(b[25]);
                            nums[3] = (br19 >> 4u32)
                                | (br20 << 4u32)
                                | (br21 << 12u32)
                                | (br22 << 20u32)
                                | (br23 << 28u32)
                                | (br24 << 36u32)
                                | (br25 << 44u32);
                            if len > 4 {
                                let br26 = T::from(b[26]);
                                let br27 = T::from(b[27]);
                                let br28 = T::from(b[28]);
                                let br29 = T::from(b[29]);
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                let br32 = T::from(b[32]);
                                nums[4] = br26
                                    | (br27 << 8u32)
                                    | (br28 << 16u32)
                                    | (br29 << 24u32)
                                    | (br30 << 32u32)
                                    | (br31 << 40u32)
                                    | ((br32 & T::from(15u8)) << 48u32);
                                if len > 5 {
                                    let br33 = T::from(b[33]);
                                    let br34 = T::from(b[34]);
                                    let br35 = T::from(b[35]);
                                    let br36 = T::from(b[36]);
                                    let br37 = T::from(b[37]);
                                    let br38 = T::from(b[38]);
                                    nums[5] = (br32 >> 4u32)
                                        | (br33 << 4u32)
                                        | (br34 << 12u32)
                                        | (br35 << 20u32)
                                        | (br36 << 28u32)
                                        | (br37 << 36u32)
                                        | (br38 << 44u32);
                                    if len > 6 {
                                        let br39 = T::from(b[39]);
                                        let br40 = T::from(b[40]);
                                        let br41 = T::from(b[41]);
                                        let br42 = T::from(b[42]);
                                        let br43 = T::from(b[43]);
                                        let br44 = T::from(b[44]);
                                        let br45 = T::from(b[45]);
                                        nums[6] = br39
                                            | (br40 << 8u32)
                                            | (br41 << 16u32)
                                            | (br42 << 24u32)
                                            | (br43 << 32u32)
                                            | (br44 << 40u32)
                                            | ((br45 & T::from(15u8)) << 48u32);
                                        if len > 7 {
                                            let br46 = T::from(b[46]);
                                            let br47 = T::from(b[47]);
                                            let br48 = T::from(b[48]);
                                            let br49 = T::from(b[49]);
                                            let br50 = T::from(b[50]);
                                            let br51 = T::from(b[51]);
                                            nums[7] = (br45 >> 4u32)
                                                | (br46 << 4u32)
                                                | (br47 << 12u32)
                                                | (br48 << 20u32)
                                                | (br49 << 28u32)
                                                | (br50 << 36u32)
                                                | (br51 << 44u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        53 => {
            let mut b = [0u8; 53];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 53 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | ((br6 & T::from(31u8)) << 48u32);
                if len > 1 {
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    let br13 = T::from(b[13]);
                    nums[1] = (br6 >> 5u32)
                        | (br7 << 3u32)
                        | (br8 << 11u32)
                        | (br9 << 19u32)
                        | (br10 << 27u32)
                        | (br11 << 35u32)
                        | (br12 << 43u32)
                        | ((br13 & T::from(3u8)) << 51u32);
                    if len > 2 {
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        let br19 = T::from(b[19]);
                        nums[2] = (br13 >> 2u32)
                            | (br14 << 6u32)
                            | (br15 << 14u32)
                            | (br16 << 22u32)
                            | (br17 << 30u32)
                            | (br18 << 38u32)
                            | ((br19 & T::from(127u8)) << 46u32);
                        if len > 3 {
                            let br20 = T::from(b[20]);
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            let br25 = T::from(b[25]);
                            let br26 = T::from(b[26]);
                            nums[3] = (br19 >> 7u32)
                                | (br20 << 1u32)
                                | (br21 << 9u32)
                                | (br22 << 17u32)
                                | (br23 << 25u32)
                                | (br24 << 33u32)
                                | (br25 << 41u32)
                                | ((br26 & T::from(15u8)) << 49u32);
                            if len > 4 {
                                let br27 = T::from(b[27]);
                                let br28 = T::from(b[28]);
                                let br29 = T::from(b[29]);
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                let br32 = T::from(b[32]);
                                let br33 = T::from(b[33]);
                                nums[4] = (br26 >> 4u32)
                                    | (br27 << 4u32)
                                    | (br28 << 12u32)
                                    | (br29 << 20u32)
                                    | (br30 << 28u32)
                                    | (br31 << 36u32)
                                    | (br32 << 44u32)
                                    | ((br33 & T::from(1u8)) << 52u32);
                                if len > 5 {
                                    let br34 = T::from(b[34]);
                                    let br35 = T::from(b[35]);
                                    let br36 = T::from(b[36]);
                                    let br37 = T::from(b[37]);
                                    let br38 = T::from(b[38]);
                                    let br39 = T::from(b[39]);
                                    nums[5] = (br33 >> 1u32)
                                        | (br34 << 7u32)
                                        | (br35 << 15u32)
                                        | (br36 << 23u32)
                                        | (br37 << 31u32)
                                        | (br38 << 39u32)
                                        | ((br39 & T::from(63u8)) << 47u32);
                                    if len > 6 {
                                        let br40 = T::from(b[40]);
                                        let br41 = T::from(b[41]);
                                        let br42 = T::from(b[42]);
                                        let br43 = T::from(b[43]);
                                        let br44 = T::from(b[44]);
                                        let br45 = T::from(b[45]);
                                        let br46 = T::from(b[46]);
                                        nums[6] = (br39 >> 6u32)
                                            | (br40 << 2u32)
                                            | (br41 << 10u32)
                                            | (br42 << 18u32)
                                            | (br43 << 26u32)
                                            | (br44 << 34u32)
                                            | (br45 << 42u32)
                                            | ((br46 & T::from(7u8)) << 50u32);
                                        if len > 7 {
                                            let br47 = T::from(b[47]);
                                            let br48 = T::from(b[48]);
                                            let br49 = T::from(b[49]);
                                            let br50 = T::from(b[50]);
                                            let br51 = T::from(b[51]);
                                            let br52 = T::from(b[52]);
                                            nums[7] = (br46 >> 3u32)
                                                | (br47 << 5u32)
                                                | (br48 << 13u32)
                                                | (br49 << 21u32)
                                                | (br50 << 29u32)
                                                | (br51 << 37u32)
                                                | (br52 << 45u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        54 => {
            let mut b = [0u8; 54];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 54 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | ((br6 & T::from(63u8)) << 48u32);
                if len > 1 {
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    let br13 = T::from(b[13]);
                    nums[1] = (br6 >> 6u32)
                        | (br7 << 2u32)
                        | (br8 << 10u32)
                        | (br9 << 18u32)
                        | (br10 << 26u32)
                        | (br11 << 34u32)
                        | (br12 << 42u32)
                        | ((br13 & T::from(15u8)) << 50u32);
                    if len > 2 {
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        let br19 = T::from(b[19]);
                        let br20 = T::from(b[20]);
                        nums[2] = (br13 >> 4u32)
                            | (br14 << 4u32)
                            | (br15 << 12u32)
                            | (br16 << 20u32)
                            | (br17 << 28u32)
                            | (br18 << 36u32)
                            | (br19 << 44u32)
                            | ((br20 & T::from(3u8)) << 52u32);
                        if len > 3 {
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            let br25 = T::from(b[25]);
                            let br26 = T::from(b[26]);
                            nums[3] = (br20 >> 2u32)
                                | (br21 << 6u32)
                                | (br22 << 14u32)
                                | (br23 << 22u32)
                                | (br24 << 30u32)
                                | (br25 << 38u32)
                                | (br26 << 46u32);
                            if len > 4 {
                                let br27 = T::from(b[27]);
                                let br28 = T::from(b[28]);
                                let br29 = T::from(b[29]);
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                let br32 = T::from(b[32]);
                                let br33 = T::from(b[33]);
                                nums[4] = br27
                                    | (br28 << 8u32)
                                    | (br29 << 16u32)
                                    | (br30 << 24u32)
                                    | (br31 << 32u32)
                                    | (br32 << 40u32)
                                    | ((br33 & T::from(63u8)) << 48u32);
                                if len > 5 {
                                    let br34 = T::from(b[34]);
                                    let br35 = T::from(b[35]);
                                    let br36 = T::from(b[36]);
                                    let br37 = T::from(b[37]);
                                    let br38 = T::from(b[38]);
                                    let br39 = T::from(b[39]);
                                    let br40 = T::from(b[40]);
                                    nums[5] = (br33 >> 6u32)
                                        | (br34 << 2u32)
                                        | (br35 << 10u32)
                                        | (br36 << 18u32)
                                        | (br37 << 26u32)
                                        | (br38 << 34u32)
                                        | (br39 << 42u32)
                                        | ((br40 & T::from(15u8)) << 50u32);
                                    if len > 6 {
                                        let br41 = T::from(b[41]);
                                        let br42 = T::from(b[42]);
                                        let br43 = T::from(b[43]);
                                        let br44 = T::from(b[44]);
                                        let br45 = T::from(b[45]);
                                        let br46 = T::from(b[46]);
                                        let br47 = T::from(b[47]);
                                        nums[6] = (br40 >> 4u32)
                                            | (br41 << 4u32)
                                            | (br42 << 12u32)
                                            | (br43 << 20u32)
                                            | (br44 << 28u32)
                                            | (br45 << 36u32)
                                            | (br46 << 44u32)
                                            | ((br47 & T::from(3u8)) << 52u32);
                                        if len > 7 {
                                            let br48 = T::from(b[48]);
                                            let br49 = T::from(b[49]);
                                            let br50 = T::from(b[50]);
                                            let br51 = T::from(b[51]);
                                            let br52 = T::from(b[52]);
                                            let br53 = T::from(b[53]);
                                            nums[7] = (br47 >> 2u32)
                                                | (br48 << 6u32)
                                                | (br49 << 14u32)
                                                | (br50 << 22u32)
                                                | (br51 << 30u32)
                                                | (br52 << 38u32)
                                                | (br53 << 46u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        55 => {
            let mut b = [0u8; 55];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 55 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | ((br6 & T::from(127u8)) << 48u32);
                if len > 1 {
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    let br13 = T::from(b[13]);
                    nums[1] = (br6 >> 7u32)
                        | (br7 << 1u32)
                        | (br8 << 9u32)
                        | (br9 << 17u32)
                        | (br10 << 25u32)
                        | (br11 << 33u32)
                        | (br12 << 41u32)
                        | ((br13 & T::from(63u8)) << 49u32);
                    if len > 2 {
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        let br19 = T::from(b[19]);
                        let br20 = T::from(b[20]);
                        nums[2] = (br13 >> 6u32)
                            | (br14 << 2u32)
                            | (br15 << 10u32)
                            | (br16 << 18u32)
                            | (br17 << 26u32)
                            | (br18 << 34u32)
                            | (br19 << 42u32)
                            | ((br20 & T::from(31u8)) << 50u32);
                        if len > 3 {
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            let br25 = T::from(b[25]);
                            let br26 = T::from(b[26]);
                            let br27 = T::from(b[27]);
                            nums[3] = (br20 >> 5u32)
                                | (br21 << 3u32)
                                | (br22 << 11u32)
                                | (br23 << 19u32)
                                | (br24 << 27u32)
                                | (br25 << 35u32)
                                | (br26 << 43u32)
                                | ((br27 & T::from(15u8)) << 51u32);
                            if len > 4 {
                                let br28 = T::from(b[28]);
                                let br29 = T::from(b[29]);
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                let br32 = T::from(b[32]);
                                let br33 = T::from(b[33]);
                                let br34 = T::from(b[34]);
                                nums[4] = (br27 >> 4u32)
                                    | (br28 << 4u32)
                                    | (br29 << 12u32)
                                    | (br30 << 20u32)
                                    | (br31 << 28u32)
                                    | (br32 << 36u32)
                                    | (br33 << 44u32)
                                    | ((br34 & T::from(7u8)) << 52u32);
                                if len > 5 {
                                    let br35 = T::from(b[35]);
                                    let br36 = T::from(b[36]);
                                    let br37 = T::from(b[37]);
                                    let br38 = T::from(b[38]);
                                    let br39 = T::from(b[39]);
                                    let br40 = T::from(b[40]);
                                    let br41 = T::from(b[41]);
                                    nums[5] = (br34 >> 3u32)
                                        | (br35 << 5u32)
                                        | (br36 << 13u32)
                                        | (br37 << 21u32)
                                        | (br38 << 29u32)
                                        | (br39 << 37u32)
                                        | (br40 << 45u32)
                                        | ((br41 & T::from(3u8)) << 53u32);
                                    if len > 6 {
                                        let br42 = T::from(b[42]);
                                        let br43 = T::from(b[43]);
                                        let br44 = T::from(b[44]);
                                        let br45 = T::from(b[45]);
                                        let br46 = T::from(b[46]);
                                        let br47 = T::from(b[47]);
                                        let br48 = T::from(b[48]);
                                        nums[6] = (br41 >> 2u32)
                                            | (br42 << 6u32)
                                            | (br43 << 14u32)
                                            | (br44 << 22u32)
                                            | (br45 << 30u32)
                                            | (br46 << 38u32)
                                            | (br47 << 46u32)
                                            | ((br48 & T::from(1u8)) << 54u32);
                                        if len > 7 {
                                            let br49 = T::from(b[49]);
                                            let br50 = T::from(b[50]);
                                            let br51 = T::from(b[51]);
                                            let br52 = T::from(b[52]);
                                            let br53 = T::from(b[53]);
                                            let br54 = T::from(b[54]);
                                            nums[7] = (br48 >> 1u32)
                                                | (br49 << 7u32)
                                                | (br50 << 15u32)
                                                | (br51 << 23u32)
                                                | (br52 << 31u32)
                                                | (br53 << 39u32)
                                                | (br54 << 47u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        56 => {
            let mut b = [0u8; 56];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 56 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | (br6 << 48u32);
                if len > 1 {
                    let br7 = T::from(b[7]);
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    let br13 = T::from(b[13]);
                    nums[1] = br7
                        | (br8 << 8u32)
                        | (br9 << 16u32)
                        | (br10 << 24u32)
                        | (br11 << 32u32)
                        | (br12 << 40u32)
                        | (br13 << 48u32);
                    if len > 2 {
                        let br14 = T::from(b[14]);
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        let br19 = T::from(b[19]);
                        let br20 = T::from(b[20]);
                        nums[2] = br14
                            | (br15 << 8u32)
                            | (br16 << 16u32)
                            | (br17 << 24u32)
                            | (br18 << 32u32)
                            | (br19 << 40u32)
                            | (br20 << 48u32);
                        if len > 3 {
                            let br21 = T::from(b[21]);
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            let br25 = T::from(b[25]);
                            let br26 = T::from(b[26]);
                            let br27 = T::from(b[27]);
                            nums[3] = br21
                                | (br22 << 8u32)
                                | (br23 << 16u32)
                                | (br24 << 24u32)
                                | (br25 << 32u32)
                                | (br26 << 40u32)
                                | (br27 << 48u32);
                            if len > 4 {
                                let br28 = T::from(b[28]);
                                let br29 = T::from(b[29]);
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                let br32 = T::from(b[32]);
                                let br33 = T::from(b[33]);
                                let br34 = T::from(b[34]);
                                nums[4] = br28
                                    | (br29 << 8u32)
                                    | (br30 << 16u32)
                                    | (br31 << 24u32)
                                    | (br32 << 32u32)
                                    | (br33 << 40u32)
                                    | (br34 << 48u32);
                                if len > 5 {
                                    let br35 = T::from(b[35]);
                                    let br36 = T::from(b[36]);
                                    let br37 = T::from(b[37]);
                                    let br38 = T::from(b[38]);
                                    let br39 = T::from(b[39]);
                                    let br40 = T::from(b[40]);
                                    let br41 = T::from(b[41]);
                                    nums[5] = br35
                                        | (br36 << 8u32)
                                        | (br37 << 16u32)
                                        | (br38 << 24u32)
                                        | (br39 << 32u32)
                                        | (br40 << 40u32)
                                        | (br41 << 48u32);
                                    if len > 6 {
                                        let br42 = T::from(b[42]);
                                        let br43 = T::from(b[43]);
                                        let br44 = T::from(b[44]);
                                        let br45 = T::from(b[45]);
                                        let br46 = T::from(b[46]);
                                        let br47 = T::from(b[47]);
                                        let br48 = T::from(b[48]);
                                        nums[6] = br42
                                            | (br43 << 8u32)
                                            | (br44 << 16u32)
                                            | (br45 << 24u32)
                                            | (br46 << 32u32)
                                            | (br47 << 40u32)
                                            | (br48 << 48u32);
                                        if len > 7 {
                                            let br49 = T::from(b[49]);
                                            let br50 = T::from(b[50]);
                                            let br51 = T::from(b[51]);
                                            let br52 = T::from(b[52]);
                                            let br53 = T::from(b[53]);
                                            let br54 = T::from(b[54]);
                                            let br55 = T::from(b[55]);
                                            nums[7] = br49
                                                | (br50 << 8u32)
                                                | (br51 << 16u32)
                                                | (br52 << 24u32)
                                                | (br53 << 32u32)
                                                | (br54 << 40u32)
                                                | (br55 << 48u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        57 => {
            let mut b = [0u8; 57];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 57 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                let br7 = T::from(b[7]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | (br6 << 48u32)
                    | ((br7 & T::from(1u8)) << 56u32);
                if len > 1 {
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    let br13 = T::from(b[13]);
                    let br14 = T::from(b[14]);
                    nums[1] = (br7 >> 1u32)
                        | (br8 << 7u32)
                        | (br9 << 15u32)
                        | (br10 << 23u32)
                        | (br11 << 31u32)
                        | (br12 << 39u32)
                        | (br13 << 47u32)
                        | ((br14 & T::from(3u8)) << 55u32);
                    if len > 2 {
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        let br19 = T::from(b[19]);
                        let br20 = T::from(b[20]);
                        let br21 = T::from(b[21]);
                        nums[2] = (br14 >> 2u32)
                            | (br15 << 6u32)
                            | (br16 << 14u32)
                            | (br17 << 22u32)
                            | (br18 << 30u32)
                            | (br19 << 38u32)
                            | (br20 << 46u32)
                            | ((br21 & T::from(7u8)) << 54u32);
                        if len > 3 {
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            let br25 = T::from(b[25]);
                            let br26 = T::from(b[26]);
                            let br27 = T::from(b[27]);
                            let br28 = T::from(b[28]);
                            nums[3] = (br21 >> 3u32)
                                | (br22 << 5u32)
                                | (br23 << 13u32)
                                | (br24 << 21u32)
                                | (br25 << 29u32)
                                | (br26 << 37u32)
                                | (br27 << 45u32)
                                | ((br28 & T::from(15u8)) << 53u32);
                            if len > 4 {
                                let br29 = T::from(b[29]);
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                let br32 = T::from(b[32]);
                                let br33 = T::from(b[33]);
                                let br34 = T::from(b[34]);
                                let br35 = T::from(b[35]);
                                nums[4] = (br28 >> 4u32)
                                    | (br29 << 4u32)
                                    | (br30 << 12u32)
                                    | (br31 << 20u32)
                                    | (br32 << 28u32)
                                    | (br33 << 36u32)
                                    | (br34 << 44u32)
                                    | ((br35 & T::from(31u8)) << 52u32);
                                if len > 5 {
                                    let br36 = T::from(b[36]);
                                    let br37 = T::from(b[37]);
                                    let br38 = T::from(b[38]);
                                    let br39 = T::from(b[39]);
                                    let br40 = T::from(b[40]);
                                    let br41 = T::from(b[41]);
                                    let br42 = T::from(b[42]);
                                    nums[5] = (br35 >> 5u32)
                                        | (br36 << 3u32)
                                        | (br37 << 11u32)
                                        | (br38 << 19u32)
                                        | (br39 << 27u32)
                                        | (br40 << 35u32)
                                        | (br41 << 43u32)
                                        | ((br42 & T::from(63u8)) << 51u32);
                                    if len > 6 {
                                        let br43 = T::from(b[43]);
                                        let br44 = T::from(b[44]);
                                        let br45 = T::from(b[45]);
                                        let br46 = T::from(b[46]);
                                        let br47 = T::from(b[47]);
                                        let br48 = T::from(b[48]);
                                        let br49 = T::from(b[49]);
                                        nums[6] = (br42 >> 6u32)
                                            | (br43 << 2u32)
                                            | (br44 << 10u32)
                                            | (br45 << 18u32)
                                            | (br46 << 26u32)
                                            | (br47 << 34u32)
                                            | (br48 << 42u32)
                                            | ((br49 & T::from(127u8)) << 50u32);
                                        if len > 7 {
                                            let br50 = T::from(b[50]);
                                            let br51 = T::from(b[51]);
                                            let br52 = T::from(b[52]);
                                            let br53 = T::from(b[53]);
                                            let br54 = T::from(b[54]);
                                            let br55 = T::from(b[55]);
                                            let br56 = T::from(b[56]);
                                            nums[7] = (br49 >> 7u32)
                                                | (br50 << 1u32)
                                                | (br51 << 9u32)
                                                | (br52 << 17u32)
                                                | (br53 << 25u32)
                                                | (br54 << 33u32)
                                                | (br55 << 41u32)
                                                | (br56 << 49u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        58 => {
            let mut b = [0u8; 58];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 58 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                let br7 = T::from(b[7]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | (br6 << 48u32)
                    | ((br7 & T::from(3u8)) << 56u32);
                if len > 1 {
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    let br13 = T::from(b[13]);
                    let br14 = T::from(b[14]);
                    nums[1] = (br7 >> 2u32)
                        | (br8 << 6u32)
                        | (br9 << 14u32)
                        | (br10 << 22u32)
                        | (br11 << 30u32)
                        | (br12 << 38u32)
                        | (br13 << 46u32)
                        | ((br14 & T::from(15u8)) << 54u32);
                    if len > 2 {
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        let br19 = T::from(b[19]);
                        let br20 = T::from(b[20]);
                        let br21 = T::from(b[21]);
                        nums[2] = (br14 >> 4u32)
                            | (br15 << 4u32)
                            | (br16 << 12u32)
                            | (br17 << 20u32)
                            | (br18 << 28u32)
                            | (br19 << 36u32)
                            | (br20 << 44u32)
                            | ((br21 & T::from(63u8)) << 52u32);
                        if len > 3 {
                            let br22 = T::from(b[22]);
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            let br25 = T::from(b[25]);
                            let br26 = T::from(b[26]);
                            let br27 = T::from(b[27]);
                            let br28 = T::from(b[28]);
                            nums[3] = (br21 >> 6u32)
                                | (br22 << 2u32)
                                | (br23 << 10u32)
                                | (br24 << 18u32)
                                | (br25 << 26u32)
                                | (br26 << 34u32)
                                | (br27 << 42u32)
                                | (br28 << 50u32);
                            if len > 4 {
                                let br29 = T::from(b[29]);
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                let br32 = T::from(b[32]);
                                let br33 = T::from(b[33]);
                                let br34 = T::from(b[34]);
                                let br35 = T::from(b[35]);
                                let br36 = T::from(b[36]);
                                nums[4] = br29
                                    | (br30 << 8u32)
                                    | (br31 << 16u32)
                                    | (br32 << 24u32)
                                    | (br33 << 32u32)
                                    | (br34 << 40u32)
                                    | (br35 << 48u32)
                                    | ((br36 & T::from(3u8)) << 56u32);
                                if len > 5 {
                                    let br37 = T::from(b[37]);
                                    let br38 = T::from(b[38]);
                                    let br39 = T::from(b[39]);
                                    let br40 = T::from(b[40]);
                                    let br41 = T::from(b[41]);
                                    let br42 = T::from(b[42]);
                                    let br43 = T::from(b[43]);
                                    nums[5] = (br36 >> 2u32)
                                        | (br37 << 6u32)
                                        | (br38 << 14u32)
                                        | (br39 << 22u32)
                                        | (br40 << 30u32)
                                        | (br41 << 38u32)
                                        | (br42 << 46u32)
                                        | ((br43 & T::from(15u8)) << 54u32);
                                    if len > 6 {
                                        let br44 = T::from(b[44]);
                                        let br45 = T::from(b[45]);
                                        let br46 = T::from(b[46]);
                                        let br47 = T::from(b[47]);
                                        let br48 = T::from(b[48]);
                                        let br49 = T::from(b[49]);
                                        let br50 = T::from(b[50]);
                                        nums[6] = (br43 >> 4u32)
                                            | (br44 << 4u32)
                                            | (br45 << 12u32)
                                            | (br46 << 20u32)
                                            | (br47 << 28u32)
                                            | (br48 << 36u32)
                                            | (br49 << 44u32)
                                            | ((br50 & T::from(63u8)) << 52u32);
                                        if len > 7 {
                                            let br51 = T::from(b[51]);
                                            let br52 = T::from(b[52]);
                                            let br53 = T::from(b[53]);
                                            let br54 = T::from(b[54]);
                                            let br55 = T::from(b[55]);
                                            let br56 = T::from(b[56]);
                                            let br57 = T::from(b[57]);
                                            nums[7] = (br50 >> 6u32)
                                                | (br51 << 2u32)
                                                | (br52 << 10u32)
                                                | (br53 << 18u32)
                                                | (br54 << 26u32)
                                                | (br55 << 34u32)
                                                | (br56 << 42u32)
                                                | (br57 << 50u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        59 => {
            let mut b = [0u8; 59];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 59 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                let br7 = T::from(b[7]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | (br6 << 48u32)
                    | ((br7 & T::from(7u8)) << 56u32);
                if len > 1 {
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    let br13 = T::from(b[13]);
                    let br14 = T::from(b[14]);
                    nums[1] = (br7 >> 3u32)
                        | (br8 << 5u32)
                        | (br9 << 13u32)
                        | (br10 << 21u32)
                        | (br11 << 29u32)
                        | (br12 << 37u32)
                        | (br13 << 45u32)
                        | ((br14 & T::from(63u8)) << 53u32);
                    if len > 2 {
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        let br19 = T::from(b[19]);
                        let br20 = T::from(b[20]);
                        let br21 = T::from(b[21]);
                        let br22 = T::from(b[22]);
                        nums[2] = (br14 >> 6u32)
                            | (br15 << 2u32)
                            | (br16 << 10u32)
                            | (br17 << 18u32)
                            | (br18 << 26u32)
                            | (br19 << 34u32)
                            | (br20 << 42u32)
                            | (br21 << 50u32)
                            | ((br22 & T::from(1u8)) << 58u32);
                        if len > 3 {
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            let br25 = T::from(b[25]);
                            let br26 = T::from(b[26]);
                            let br27 = T::from(b[27]);
                            let br28 = T::from(b[28]);
                            let br29 = T::from(b[29]);
                            nums[3] = (br22 >> 1u32)
                                | (br23 << 7u32)
                                | (br24 << 15u32)
                                | (br25 << 23u32)
                                | (br26 << 31u32)
                                | (br27 << 39u32)
                                | (br28 << 47u32)
                                | ((br29 & T::from(15u8)) << 55u32);
                            if len > 4 {
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                let br32 = T::from(b[32]);
                                let br33 = T::from(b[33]);
                                let br34 = T::from(b[34]);
                                let br35 = T::from(b[35]);
                                let br36 = T::from(b[36]);
                                nums[4] = (br29 >> 4u32)
                                    | (br30 << 4u32)
                                    | (br31 << 12u32)
                                    | (br32 << 20u32)
                                    | (br33 << 28u32)
                                    | (br34 << 36u32)
                                    | (br35 << 44u32)
                                    | ((br36 & T::from(127u8)) << 52u32);
                                if len > 5 {
                                    let br37 = T::from(b[37]);
                                    let br38 = T::from(b[38]);
                                    let br39 = T::from(b[39]);
                                    let br40 = T::from(b[40]);
                                    let br41 = T::from(b[41]);
                                    let br42 = T::from(b[42]);
                                    let br43 = T::from(b[43]);
                                    let br44 = T::from(b[44]);
                                    nums[5] = (br36 >> 7u32)
                                        | (br37 << 1u32)
                                        | (br38 << 9u32)
                                        | (br39 << 17u32)
                                        | (br40 << 25u32)
                                        | (br41 << 33u32)
                                        | (br42 << 41u32)
                                        | (br43 << 49u32)
                                        | ((br44 & T::from(3u8)) << 57u32);
                                    if len > 6 {
                                        let br45 = T::from(b[45]);
                                        let br46 = T::from(b[46]);
                                        let br47 = T::from(b[47]);
                                        let br48 = T::from(b[48]);
                                        let br49 = T::from(b[49]);
                                        let br50 = T::from(b[50]);
                                        let br51 = T::from(b[51]);
                                        nums[6] = (br44 >> 2u32)
                                            | (br45 << 6u32)
                                            | (br46 << 14u32)
                                            | (br47 << 22u32)
                                            | (br48 << 30u32)
                                            | (br49 << 38u32)
                                            | (br50 << 46u32)
                                            | ((br51 & T::from(31u8)) << 54u32);
                                        if len > 7 {
                                            let br52 = T::from(b[52]);
                                            let br53 = T::from(b[53]);
                                            let br54 = T::from(b[54]);
                                            let br55 = T::from(b[55]);
                                            let br56 = T::from(b[56]);
                                            let br57 = T::from(b[57]);
                                            let br58 = T::from(b[58]);
                                            nums[7] = (br51 >> 5u32)
                                                | (br52 << 3u32)
                                                | (br53 << 11u32)
                                                | (br54 << 19u32)
                                                | (br55 << 27u32)
                                                | (br56 << 35u32)
                                                | (br57 << 43u32)
                                                | (br58 << 51u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        60 => {
            let mut b = [0u8; 60];
            for nums in out_numbers.chunks_mut(8) {
                let len = nums.len();
                rd.read_exact(&mut b[0..(len * 60 + 7) >> 3])?;
                let br0 = T::from(b[0]);
                let br1 = T::from(b[1]);
                let br2 = T::from(b[2]);
                let br3 = T::from(b[3]);
                let br4 = T::from(b[4]);
                let br5 = T::from(b[5]);
                let br6 = T::from(b[6]);
                let br7 = T::from(b[7]);
                nums[0] = br0
                    | (br1 << 8u32)
                    | (br2 << 16u32)
                    | (br3 << 24u32)
                    | (br4 << 32u32)
                    | (br5 << 40u32)
                    | (br6 << 48u32)
                    | ((br7 & T::from(15u8)) << 56u32);
                if len > 1 {
                    let br8 = T::from(b[8]);
                    let br9 = T::from(b[9]);
                    let br10 = T::from(b[10]);
                    let br11 = T::from(b[11]);
                    let br12 = T::from(b[12]);
                    let br13 = T::from(b[13]);
                    let br14 = T::from(b[14]);
                    nums[1] = (br7 >> 4u32)
                        | (br8 << 4u32)
                        | (br9 << 12u32)
                        | (br10 << 20u32)
                        | (br11 << 28u32)
                        | (br12 << 36u32)
                        | (br13 << 44u32)
                        | (br14 << 52u32);
                    if len > 2 {
                        let br15 = T::from(b[15]);
                        let br16 = T::from(b[16]);
                        let br17 = T::from(b[17]);
                        let br18 = T::from(b[18]);
                        let br19 = T::from(b[19]);
                        let br20 = T::from(b[20]);
                        let br21 = T::from(b[21]);
                        let br22 = T::from(b[22]);
                        nums[2] = br15
                            | (br16 << 8u32)
                            | (br17 << 16u32)
                            | (br18 << 24u32)
                            | (br19 << 32u32)
                            | (br20 << 40u32)
                            | (br21 << 48u32)
                            | ((br22 & T::from(15u8)) << 56u32);
                        if len > 3 {
                            let br23 = T::from(b[23]);
                            let br24 = T::from(b[24]);
                            let br25 = T::from(b[25]);
                            let br26 = T::from(b[26]);
                            let br27 = T::from(b[27]);
                            let br28 = T::from(b[28]);
                            let br29 = T::from(b[29]);
                            nums[3] = (br22 >> 4u32)
                                | (br23 << 4u32)
                                | (br24 << 12u32)
                                | (br25 << 20u32)
                                | (br26 << 28u32)
                                | (br27 << 36u32)
                                | (br28 << 44u32)
                                | (br29 << 52u32);
                            if len > 4 {
                                let br30 = T::from(b[30]);
                                let br31 = T::from(b[31]);
                                let br32 = T::from(b[32]);
                                let br33 = T::from(b[33]);
                                let br34 = T::from(b[34]);
                                let br35 = T::from(b[35]);
                                let br36 = T::from(b[36]);
                                let br37 = T::from(b[37]);
                                nums[4] = br30
                                    | (br31 << 8u32)
                                    | (br32 << 16u32)
                                    | (br33 << 24u32)
                                    | (br34 << 32u32)
                                    | (br35 << 40u32)
                                    | (br36 << 48u32)
                                    | ((br37 & T::from(15u8)) << 56u32);
                                if len > 5 {
                                    let br38 = T::from(b[38]);
                                    let br39 = T::from(b[39]);
                                    let br40 = T::from(b[40]);
                                    let br41 = T::from(b[41]);
                                    let br42 = T::from(b[42]);
                                    let br43 = T::from(b[43]);
                                    let br44 = T::from(b[44]);
                                    nums[5] = (br37 >> 4u32)
                                        | (br38 << 4u32)
                                        | (br39 << 12u32)
                                        | (br40 << 20u32)
                                        | (br41 << 28u32)
                                        | (br42 << 36u32)
                                        | (br43 << 44u32)
                                        | (br44 << 52u32);
                                    if len > 6 {
                                        let br45 = T::from(b[45]);
                                        let br46 = T::from(b[46]);
                                        let br47 = T::from(b[47]);
                                        let br48 = T::from(b[48]);
                                        let br49 = T::from(b[49]);
                                        let br50 = T::from(b[50]);
                                        let br51 = T::from(b[51]);
                                        let br52 = T::from(b[52]);
                                        nums[6] = br45
                                            | (br46 << 8u32)
                                            | (br47 << 16u32)
                                            | (br48 << 24u32)
                                            | (br49 << 32u32)
                                            | (br50 << 40u32)
                                            | (br51 << 48u32)
                                            | ((br52 & T::from(15u8)) << 56u32);
                                        if len > 7 {
                                            let br53 = T::from(b[53]);
                                            let br54 = T::from(b[54]);
                                            let br55 = T::from(b[55]);
                                            let br56 = T::from(b[56]);
                                            let br57 = T::from(b[57]);
                                            let br58 = T::from(b[58]);
                                            let br59 = T::from(b[59]);
                                            nums[7] = (br52 >> 4u32)
                                                | (br53 << 4u32)
                                                | (br54 << 12u32)
                                                | (br55 << 20u32)
                                                | (br56 << 28u32)
                                                | (br57 << 36u32)
                                                | (br58 << 44u32)
                                                | (br59 << 52u32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        _ => {
            panic!("Unsupported");
        }
    }
    Ok(())
}

pub fn write_numbers_2<W: Write, T>(mut wr: W, num_bits: u8, numbers: &[T]) -> io::Result<()>
where
    T: Clone
        + Copy
        + From<u8>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + Shl<u32, Output = T>
        + Shr<u32, Output = T>,
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: Debug,
{
    match num_bits {
        1 => {
            let mut b = [0u8; 1];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(1u8)).unwrap();
                if len > 1 {
                    b[0] |= u8::try_from(nums[1] & T::from(1u8)).unwrap() << 1u32;
                }
                if len > 2 {
                    b[0] |= u8::try_from(nums[2] & T::from(1u8)).unwrap() << 2u32;
                }
                if len > 3 {
                    b[0] |= u8::try_from(nums[3] & T::from(1u8)).unwrap() << 3u32;
                }
                if len > 4 {
                    b[0] |= u8::try_from(nums[4] & T::from(1u8)).unwrap() << 4u32;
                }
                if len > 5 {
                    b[0] |= u8::try_from(nums[5] & T::from(1u8)).unwrap() << 5u32;
                }
                if len > 6 {
                    b[0] |= u8::try_from(nums[6] & T::from(1u8)).unwrap() << 6u32;
                }
                if len > 7 {
                    b[0] |= u8::try_from(nums[7] & T::from(1u8)).unwrap() << 7u32;
                }
                wr.write_all(&b[0..(len * 1 + 7) >> 3])?;
            }
        }

        2 => {
            let mut b = [0u8; 2];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(3u8)).unwrap();
                if len > 1 {
                    b[0] |= u8::try_from(nums[1] & T::from(3u8)).unwrap() << 2u32;
                }
                if len > 2 {
                    b[0] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 4u32;
                }
                if len > 3 {
                    b[0] |= u8::try_from(nums[3] & T::from(3u8)).unwrap() << 6u32;
                }
                if len > 4 {
                    b[1] = u8::try_from(nums[4] & T::from(3u8)).unwrap();
                }
                if len > 5 {
                    b[1] |= u8::try_from(nums[5] & T::from(3u8)).unwrap() << 2u32;
                }
                if len > 6 {
                    b[1] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 4u32;
                }
                if len > 7 {
                    b[1] |= u8::try_from(nums[7] & T::from(3u8)).unwrap() << 6u32;
                }
                wr.write_all(&b[0..(len * 2 + 7) >> 3])?;
            }
        }

        3 => {
            let mut b = [0u8; 3];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(7u8)).unwrap();
                if len > 1 {
                    b[0] |= u8::try_from(nums[1] & T::from(7u8)).unwrap() << 3u32;
                }
                if len > 2 {
                    b[0] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[1] = u8::try_from((nums[2] >> 2) & T::from(1u8)).unwrap();
                }
                if len > 3 {
                    b[1] |= u8::try_from(nums[3] & T::from(7u8)).unwrap() << 1u32;
                }
                if len > 4 {
                    b[1] |= u8::try_from(nums[4] & T::from(7u8)).unwrap() << 4u32;
                }
                if len > 5 {
                    b[1] |= u8::try_from(nums[5] & T::from(1u8)).unwrap() << 7u32;
                    b[2] = u8::try_from((nums[5] >> 1) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[2] |= u8::try_from(nums[6] & T::from(7u8)).unwrap() << 2u32;
                }
                if len > 7 {
                    b[2] |= u8::try_from(nums[7] & T::from(7u8)).unwrap() << 5u32;
                }
                wr.write_all(&b[0..(len * 3 + 7) >> 3])?;
            }
        }

        4 => {
            let mut b = [0u8; 4];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(15u8)).unwrap();
                if len > 1 {
                    b[0] |= u8::try_from(nums[1] & T::from(15u8)).unwrap() << 4u32;
                }
                if len > 2 {
                    b[1] = u8::try_from(nums[2] & T::from(15u8)).unwrap();
                }
                if len > 3 {
                    b[1] |= u8::try_from(nums[3] & T::from(15u8)).unwrap() << 4u32;
                }
                if len > 4 {
                    b[2] = u8::try_from(nums[4] & T::from(15u8)).unwrap();
                }
                if len > 5 {
                    b[2] |= u8::try_from(nums[5] & T::from(15u8)).unwrap() << 4u32;
                }
                if len > 6 {
                    b[3] = u8::try_from(nums[6] & T::from(15u8)).unwrap();
                }
                if len > 7 {
                    b[3] |= u8::try_from(nums[7] & T::from(15u8)).unwrap() << 4u32;
                }
                wr.write_all(&b[0..(len * 4 + 7) >> 3])?;
            }
        }

        5 => {
            let mut b = [0u8; 5];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(31u8)).unwrap();
                if len > 1 {
                    b[0] |= u8::try_from(nums[1] & T::from(7u8)).unwrap() << 5u32;
                    b[1] = u8::try_from((nums[1] >> 3) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[1] |= u8::try_from(nums[2] & T::from(31u8)).unwrap() << 2u32;
                }
                if len > 3 {
                    b[1] |= u8::try_from(nums[3] & T::from(1u8)).unwrap() << 7u32;
                    b[2] = u8::try_from((nums[3] >> 1) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[2] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[3] = u8::try_from((nums[4] >> 4) & T::from(1u8)).unwrap();
                }
                if len > 5 {
                    b[3] |= u8::try_from(nums[5] & T::from(31u8)).unwrap() << 1u32;
                }
                if len > 6 {
                    b[3] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[4] = u8::try_from((nums[6] >> 2) & T::from(7u8)).unwrap();
                }
                if len > 7 {
                    b[4] |= u8::try_from(nums[7] & T::from(31u8)).unwrap() << 3u32;
                }
                wr.write_all(&b[0..(len * 5 + 7) >> 3])?;
            }
        }

        6 => {
            let mut b = [0u8; 6];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(63u8)).unwrap();
                if len > 1 {
                    b[0] |= u8::try_from(nums[1] & T::from(3u8)).unwrap() << 6u32;
                    b[1] = u8::try_from((nums[1] >> 2) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[1] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[2] = u8::try_from((nums[2] >> 4) & T::from(3u8)).unwrap();
                }
                if len > 3 {
                    b[2] |= u8::try_from(nums[3] & T::from(63u8)).unwrap() << 2u32;
                }
                if len > 4 {
                    b[3] = u8::try_from(nums[4] & T::from(63u8)).unwrap();
                }
                if len > 5 {
                    b[3] |= u8::try_from(nums[5] & T::from(3u8)).unwrap() << 6u32;
                    b[4] = u8::try_from((nums[5] >> 2) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[4] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[5] = u8::try_from((nums[6] >> 4) & T::from(3u8)).unwrap();
                }
                if len > 7 {
                    b[5] |= u8::try_from(nums[7] & T::from(63u8)).unwrap() << 2u32;
                }
                wr.write_all(&b[0..(len * 6 + 7) >> 3])?;
            }
        }

        7 => {
            let mut b = [0u8; 7];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(127u8)).unwrap();
                if len > 1 {
                    b[0] |= u8::try_from(nums[1] & T::from(1u8)).unwrap() << 7u32;
                    b[1] = u8::try_from((nums[1] >> 1) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[1] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[2] = u8::try_from((nums[2] >> 2) & T::from(31u8)).unwrap();
                }
                if len > 3 {
                    b[2] |= u8::try_from(nums[3] & T::from(7u8)).unwrap() << 5u32;
                    b[3] = u8::try_from((nums[3] >> 3) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[3] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[4] = u8::try_from((nums[4] >> 4) & T::from(7u8)).unwrap();
                }
                if len > 5 {
                    b[4] |= u8::try_from(nums[5] & T::from(31u8)).unwrap() << 3u32;
                    b[5] = u8::try_from((nums[5] >> 5) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[5] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[6] = u8::try_from((nums[6] >> 6) & T::from(1u8)).unwrap();
                }
                if len > 7 {
                    b[6] |= u8::try_from(nums[7] & T::from(127u8)).unwrap() << 1u32;
                }
                wr.write_all(&b[0..(len * 7 + 7) >> 3])?;
            }
        }

        8 => {
            let mut b = [0u8; 8];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                if len > 1 {
                    b[1] = u8::try_from(nums[1] & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[2] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                }
                if len > 3 {
                    b[3] = u8::try_from(nums[3] & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[4] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                }
                if len > 5 {
                    b[5] = u8::try_from(nums[5] & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[6] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                }
                if len > 7 {
                    b[7] = u8::try_from(nums[7] & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 8 + 7) >> 3])?;
            }
        }

        9 => {
            let mut b = [0u8; 9];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(1u8)).unwrap();
                if len > 1 {
                    b[1] |= u8::try_from(nums[1] & T::from(127u8)).unwrap() << 1u32;
                    b[2] = u8::try_from((nums[1] >> 7) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[2] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[3] = u8::try_from((nums[2] >> 6) & T::from(7u8)).unwrap();
                }
                if len > 3 {
                    b[3] |= u8::try_from(nums[3] & T::from(31u8)).unwrap() << 3u32;
                    b[4] = u8::try_from((nums[3] >> 5) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[4] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[5] = u8::try_from((nums[4] >> 4) & T::from(31u8)).unwrap();
                }
                if len > 5 {
                    b[5] |= u8::try_from(nums[5] & T::from(7u8)).unwrap() << 5u32;
                    b[6] = u8::try_from((nums[5] >> 3) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[6] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[7] = u8::try_from((nums[6] >> 2) & T::from(127u8)).unwrap();
                }
                if len > 7 {
                    b[7] |= u8::try_from(nums[7] & T::from(1u8)).unwrap() << 7u32;
                    b[8] = u8::try_from((nums[7] >> 1) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 9 + 7) >> 3])?;
            }
        }

        10 => {
            let mut b = [0u8; 10];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(3u8)).unwrap();
                if len > 1 {
                    b[1] |= u8::try_from(nums[1] & T::from(63u8)).unwrap() << 2u32;
                    b[2] = u8::try_from((nums[1] >> 6) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[2] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[3] = u8::try_from((nums[2] >> 4) & T::from(63u8)).unwrap();
                }
                if len > 3 {
                    b[3] |= u8::try_from(nums[3] & T::from(3u8)).unwrap() << 6u32;
                    b[4] = u8::try_from((nums[3] >> 2) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[5] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[4] >> 8) & T::from(3u8)).unwrap();
                }
                if len > 5 {
                    b[6] |= u8::try_from(nums[5] & T::from(63u8)).unwrap() << 2u32;
                    b[7] = u8::try_from((nums[5] >> 6) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[7] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[8] = u8::try_from((nums[6] >> 4) & T::from(63u8)).unwrap();
                }
                if len > 7 {
                    b[8] |= u8::try_from(nums[7] & T::from(3u8)).unwrap() << 6u32;
                    b[9] = u8::try_from((nums[7] >> 2) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 10 + 7) >> 3])?;
            }
        }

        11 => {
            let mut b = [0u8; 11];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(7u8)).unwrap();
                if len > 1 {
                    b[1] |= u8::try_from(nums[1] & T::from(31u8)).unwrap() << 3u32;
                    b[2] = u8::try_from((nums[1] >> 5) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[2] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[3] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[4] = u8::try_from((nums[2] >> 10) & T::from(1u8)).unwrap();
                }
                if len > 3 {
                    b[4] |= u8::try_from(nums[3] & T::from(127u8)).unwrap() << 1u32;
                    b[5] = u8::try_from((nums[3] >> 7) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[5] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[6] = u8::try_from((nums[4] >> 4) & T::from(127u8)).unwrap();
                }
                if len > 5 {
                    b[6] |= u8::try_from(nums[5] & T::from(1u8)).unwrap() << 7u32;
                    b[7] = u8::try_from((nums[5] >> 1) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[5] >> 9) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[8] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[9] = u8::try_from((nums[6] >> 6) & T::from(31u8)).unwrap();
                }
                if len > 7 {
                    b[9] |= u8::try_from(nums[7] & T::from(7u8)).unwrap() << 5u32;
                    b[10] = u8::try_from((nums[7] >> 3) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 11 + 7) >> 3])?;
            }
        }

        12 => {
            let mut b = [0u8; 12];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(15u8)).unwrap();
                if len > 1 {
                    b[1] |= u8::try_from(nums[1] & T::from(15u8)).unwrap() << 4u32;
                    b[2] = u8::try_from((nums[1] >> 4) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[3] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[4] = u8::try_from((nums[2] >> 8) & T::from(15u8)).unwrap();
                }
                if len > 3 {
                    b[4] |= u8::try_from(nums[3] & T::from(15u8)).unwrap() << 4u32;
                    b[5] = u8::try_from((nums[3] >> 4) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[6] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[4] >> 8) & T::from(15u8)).unwrap();
                }
                if len > 5 {
                    b[7] |= u8::try_from(nums[5] & T::from(15u8)).unwrap() << 4u32;
                    b[8] = u8::try_from((nums[5] >> 4) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[9] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[6] >> 8) & T::from(15u8)).unwrap();
                }
                if len > 7 {
                    b[10] |= u8::try_from(nums[7] & T::from(15u8)).unwrap() << 4u32;
                    b[11] = u8::try_from((nums[7] >> 4) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 12 + 7) >> 3])?;
            }
        }

        13 => {
            let mut b = [0u8; 13];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(31u8)).unwrap();
                if len > 1 {
                    b[1] |= u8::try_from(nums[1] & T::from(7u8)).unwrap() << 5u32;
                    b[2] = u8::try_from((nums[1] >> 3) & T::from(255u8)).unwrap();
                    b[3] = u8::try_from((nums[1] >> 11) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[3] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[4] = u8::try_from((nums[2] >> 6) & T::from(127u8)).unwrap();
                }
                if len > 3 {
                    b[4] |= u8::try_from(nums[3] & T::from(1u8)).unwrap() << 7u32;
                    b[5] = u8::try_from((nums[3] >> 1) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[3] >> 9) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[6] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[7] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[4] >> 12) & T::from(1u8)).unwrap();
                }
                if len > 5 {
                    b[8] |= u8::try_from(nums[5] & T::from(127u8)).unwrap() << 1u32;
                    b[9] = u8::try_from((nums[5] >> 7) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[9] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[10] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[6] >> 10) & T::from(7u8)).unwrap();
                }
                if len > 7 {
                    b[11] |= u8::try_from(nums[7] & T::from(31u8)).unwrap() << 3u32;
                    b[12] = u8::try_from((nums[7] >> 5) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 13 + 7) >> 3])?;
            }
        }

        14 => {
            let mut b = [0u8; 14];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(63u8)).unwrap();
                if len > 1 {
                    b[1] |= u8::try_from(nums[1] & T::from(3u8)).unwrap() << 6u32;
                    b[2] = u8::try_from((nums[1] >> 2) & T::from(255u8)).unwrap();
                    b[3] = u8::try_from((nums[1] >> 10) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[3] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[4] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[2] >> 12) & T::from(3u8)).unwrap();
                }
                if len > 3 {
                    b[5] |= u8::try_from(nums[3] & T::from(63u8)).unwrap() << 2u32;
                    b[6] = u8::try_from((nums[3] >> 6) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[7] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[4] >> 8) & T::from(63u8)).unwrap();
                }
                if len > 5 {
                    b[8] |= u8::try_from(nums[5] & T::from(3u8)).unwrap() << 6u32;
                    b[9] = u8::try_from((nums[5] >> 2) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[5] >> 10) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[10] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[11] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[6] >> 12) & T::from(3u8)).unwrap();
                }
                if len > 7 {
                    b[12] |= u8::try_from(nums[7] & T::from(63u8)).unwrap() << 2u32;
                    b[13] = u8::try_from((nums[7] >> 6) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 14 + 7) >> 3])?;
            }
        }

        15 => {
            let mut b = [0u8; 15];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(127u8)).unwrap();
                if len > 1 {
                    b[1] |= u8::try_from(nums[1] & T::from(1u8)).unwrap() << 7u32;
                    b[2] = u8::try_from((nums[1] >> 1) & T::from(255u8)).unwrap();
                    b[3] = u8::try_from((nums[1] >> 9) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[3] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[4] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[2] >> 10) & T::from(31u8)).unwrap();
                }
                if len > 3 {
                    b[5] |= u8::try_from(nums[3] & T::from(7u8)).unwrap() << 5u32;
                    b[6] = u8::try_from((nums[3] >> 3) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[3] >> 11) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[7] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[8] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[4] >> 12) & T::from(7u8)).unwrap();
                }
                if len > 5 {
                    b[9] |= u8::try_from(nums[5] & T::from(31u8)).unwrap() << 3u32;
                    b[10] = u8::try_from((nums[5] >> 5) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[5] >> 13) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[11] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[12] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[6] >> 14) & T::from(1u8)).unwrap();
                }
                if len > 7 {
                    b[13] |= u8::try_from(nums[7] & T::from(127u8)).unwrap() << 1u32;
                    b[14] = u8::try_from((nums[7] >> 7) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 15 + 7) >> 3])?;
            }
        }

        16 => {
            let mut b = [0u8; 16];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                if len > 1 {
                    b[2] = u8::try_from(nums[1] & T::from(255u8)).unwrap();
                    b[3] = u8::try_from((nums[1] >> 8) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[4] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                }
                if len > 3 {
                    b[6] = u8::try_from(nums[3] & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[3] >> 8) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[8] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                }
                if len > 5 {
                    b[10] = u8::try_from(nums[5] & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[5] >> 8) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[12] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                }
                if len > 7 {
                    b[14] = u8::try_from(nums[7] & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[7] >> 8) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 16 + 7) >> 3])?;
            }
        }

        17 => {
            let mut b = [0u8; 17];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(1u8)).unwrap();
                if len > 1 {
                    b[2] |= u8::try_from(nums[1] & T::from(127u8)).unwrap() << 1u32;
                    b[3] = u8::try_from((nums[1] >> 7) & T::from(255u8)).unwrap();
                    b[4] = u8::try_from((nums[1] >> 15) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[4] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[5] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[2] >> 14) & T::from(7u8)).unwrap();
                }
                if len > 3 {
                    b[6] |= u8::try_from(nums[3] & T::from(31u8)).unwrap() << 3u32;
                    b[7] = u8::try_from((nums[3] >> 5) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[3] >> 13) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[8] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[9] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[4] >> 12) & T::from(31u8)).unwrap();
                }
                if len > 5 {
                    b[10] |= u8::try_from(nums[5] & T::from(7u8)).unwrap() << 5u32;
                    b[11] = u8::try_from((nums[5] >> 3) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[5] >> 11) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[12] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[13] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[6] >> 10) & T::from(127u8)).unwrap();
                }
                if len > 7 {
                    b[14] |= u8::try_from(nums[7] & T::from(1u8)).unwrap() << 7u32;
                    b[15] = u8::try_from((nums[7] >> 1) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[7] >> 9) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 17 + 7) >> 3])?;
            }
        }

        18 => {
            let mut b = [0u8; 18];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(3u8)).unwrap();
                if len > 1 {
                    b[2] |= u8::try_from(nums[1] & T::from(63u8)).unwrap() << 2u32;
                    b[3] = u8::try_from((nums[1] >> 6) & T::from(255u8)).unwrap();
                    b[4] = u8::try_from((nums[1] >> 14) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[4] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[5] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[2] >> 12) & T::from(63u8)).unwrap();
                }
                if len > 3 {
                    b[6] |= u8::try_from(nums[3] & T::from(3u8)).unwrap() << 6u32;
                    b[7] = u8::try_from((nums[3] >> 2) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[3] >> 10) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[9] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[4] >> 16) & T::from(3u8)).unwrap();
                }
                if len > 5 {
                    b[11] |= u8::try_from(nums[5] & T::from(63u8)).unwrap() << 2u32;
                    b[12] = u8::try_from((nums[5] >> 6) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[5] >> 14) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[13] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[14] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[6] >> 12) & T::from(63u8)).unwrap();
                }
                if len > 7 {
                    b[15] |= u8::try_from(nums[7] & T::from(3u8)).unwrap() << 6u32;
                    b[16] = u8::try_from((nums[7] >> 2) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[7] >> 10) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 18 + 7) >> 3])?;
            }
        }

        19 => {
            let mut b = [0u8; 19];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(7u8)).unwrap();
                if len > 1 {
                    b[2] |= u8::try_from(nums[1] & T::from(31u8)).unwrap() << 3u32;
                    b[3] = u8::try_from((nums[1] >> 5) & T::from(255u8)).unwrap();
                    b[4] = u8::try_from((nums[1] >> 13) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[4] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[5] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[2] >> 18) & T::from(1u8)).unwrap();
                }
                if len > 3 {
                    b[7] |= u8::try_from(nums[3] & T::from(127u8)).unwrap() << 1u32;
                    b[8] = u8::try_from((nums[3] >> 7) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[3] >> 15) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[9] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[10] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[4] >> 12) & T::from(127u8)).unwrap();
                }
                if len > 5 {
                    b[11] |= u8::try_from(nums[5] & T::from(1u8)).unwrap() << 7u32;
                    b[12] = u8::try_from((nums[5] >> 1) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[5] >> 9) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[5] >> 17) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[14] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[15] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[6] >> 14) & T::from(31u8)).unwrap();
                }
                if len > 7 {
                    b[16] |= u8::try_from(nums[7] & T::from(7u8)).unwrap() << 5u32;
                    b[17] = u8::try_from((nums[7] >> 3) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[7] >> 11) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 19 + 7) >> 3])?;
            }
        }

        20 => {
            let mut b = [0u8; 20];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(15u8)).unwrap();
                if len > 1 {
                    b[2] |= u8::try_from(nums[1] & T::from(15u8)).unwrap() << 4u32;
                    b[3] = u8::try_from((nums[1] >> 4) & T::from(255u8)).unwrap();
                    b[4] = u8::try_from((nums[1] >> 12) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[5] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[2] >> 16) & T::from(15u8)).unwrap();
                }
                if len > 3 {
                    b[7] |= u8::try_from(nums[3] & T::from(15u8)).unwrap() << 4u32;
                    b[8] = u8::try_from((nums[3] >> 4) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[3] >> 12) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[10] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[4] >> 16) & T::from(15u8)).unwrap();
                }
                if len > 5 {
                    b[12] |= u8::try_from(nums[5] & T::from(15u8)).unwrap() << 4u32;
                    b[13] = u8::try_from((nums[5] >> 4) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[5] >> 12) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[15] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[6] >> 16) & T::from(15u8)).unwrap();
                }
                if len > 7 {
                    b[17] |= u8::try_from(nums[7] & T::from(15u8)).unwrap() << 4u32;
                    b[18] = u8::try_from((nums[7] >> 4) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[7] >> 12) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 20 + 7) >> 3])?;
            }
        }

        21 => {
            let mut b = [0u8; 21];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(31u8)).unwrap();
                if len > 1 {
                    b[2] |= u8::try_from(nums[1] & T::from(7u8)).unwrap() << 5u32;
                    b[3] = u8::try_from((nums[1] >> 3) & T::from(255u8)).unwrap();
                    b[4] = u8::try_from((nums[1] >> 11) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 19) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[5] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[6] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[2] >> 14) & T::from(127u8)).unwrap();
                }
                if len > 3 {
                    b[7] |= u8::try_from(nums[3] & T::from(1u8)).unwrap() << 7u32;
                    b[8] = u8::try_from((nums[3] >> 1) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[3] >> 9) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[3] >> 17) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[10] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[11] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[4] >> 20) & T::from(1u8)).unwrap();
                }
                if len > 5 {
                    b[13] |= u8::try_from(nums[5] & T::from(127u8)).unwrap() << 1u32;
                    b[14] = u8::try_from((nums[5] >> 7) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[5] >> 15) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[15] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[16] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[6] >> 10) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[6] >> 18) & T::from(7u8)).unwrap();
                }
                if len > 7 {
                    b[18] |= u8::try_from(nums[7] & T::from(31u8)).unwrap() << 3u32;
                    b[19] = u8::try_from((nums[7] >> 5) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[7] >> 13) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 21 + 7) >> 3])?;
            }
        }

        22 => {
            let mut b = [0u8; 22];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(63u8)).unwrap();
                if len > 1 {
                    b[2] |= u8::try_from(nums[1] & T::from(3u8)).unwrap() << 6u32;
                    b[3] = u8::try_from((nums[1] >> 2) & T::from(255u8)).unwrap();
                    b[4] = u8::try_from((nums[1] >> 10) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 18) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[5] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[6] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[2] >> 12) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[2] >> 20) & T::from(3u8)).unwrap();
                }
                if len > 3 {
                    b[8] |= u8::try_from(nums[3] & T::from(63u8)).unwrap() << 2u32;
                    b[9] = u8::try_from((nums[3] >> 6) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[3] >> 14) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[11] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[4] >> 16) & T::from(63u8)).unwrap();
                }
                if len > 5 {
                    b[13] |= u8::try_from(nums[5] & T::from(3u8)).unwrap() << 6u32;
                    b[14] = u8::try_from((nums[5] >> 2) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[5] >> 10) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[5] >> 18) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[16] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[17] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[6] >> 12) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[6] >> 20) & T::from(3u8)).unwrap();
                }
                if len > 7 {
                    b[19] |= u8::try_from(nums[7] & T::from(63u8)).unwrap() << 2u32;
                    b[20] = u8::try_from((nums[7] >> 6) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[7] >> 14) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 22 + 7) >> 3])?;
            }
        }

        23 => {
            let mut b = [0u8; 23];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(127u8)).unwrap();
                if len > 1 {
                    b[2] |= u8::try_from(nums[1] & T::from(1u8)).unwrap() << 7u32;
                    b[3] = u8::try_from((nums[1] >> 1) & T::from(255u8)).unwrap();
                    b[4] = u8::try_from((nums[1] >> 9) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 17) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[5] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[6] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[2] >> 18) & T::from(31u8)).unwrap();
                }
                if len > 3 {
                    b[8] |= u8::try_from(nums[3] & T::from(7u8)).unwrap() << 5u32;
                    b[9] = u8::try_from((nums[3] >> 3) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[3] >> 11) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[3] >> 19) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[11] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[12] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[4] >> 20) & T::from(7u8)).unwrap();
                }
                if len > 5 {
                    b[14] |= u8::try_from(nums[5] & T::from(31u8)).unwrap() << 3u32;
                    b[15] = u8::try_from((nums[5] >> 5) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[5] >> 13) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[5] >> 21) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[17] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[18] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[6] >> 14) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[6] >> 22) & T::from(1u8)).unwrap();
                }
                if len > 7 {
                    b[20] |= u8::try_from(nums[7] & T::from(127u8)).unwrap() << 1u32;
                    b[21] = u8::try_from((nums[7] >> 7) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[7] >> 15) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 23 + 7) >> 3])?;
            }
        }

        24 => {
            let mut b = [0u8; 24];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                if len > 1 {
                    b[3] = u8::try_from(nums[1] & T::from(255u8)).unwrap();
                    b[4] = u8::try_from((nums[1] >> 8) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 16) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[6] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[2] >> 16) & T::from(255u8)).unwrap();
                }
                if len > 3 {
                    b[9] = u8::try_from(nums[3] & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[3] >> 8) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[3] >> 16) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[12] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                }
                if len > 5 {
                    b[15] = u8::try_from(nums[5] & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[5] >> 8) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[5] >> 16) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[18] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[6] >> 16) & T::from(255u8)).unwrap();
                }
                if len > 7 {
                    b[21] = u8::try_from(nums[7] & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[7] >> 8) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[7] >> 16) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 24 + 7) >> 3])?;
            }
        }

        25 => {
            let mut b = [0u8; 25];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(1u8)).unwrap();
                if len > 1 {
                    b[3] |= u8::try_from(nums[1] & T::from(127u8)).unwrap() << 1u32;
                    b[4] = u8::try_from((nums[1] >> 7) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 15) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 23) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[6] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[7] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[2] >> 14) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[2] >> 22) & T::from(7u8)).unwrap();
                }
                if len > 3 {
                    b[9] |= u8::try_from(nums[3] & T::from(31u8)).unwrap() << 3u32;
                    b[10] = u8::try_from((nums[3] >> 5) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[3] >> 13) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[3] >> 21) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[12] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[13] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[4] >> 20) & T::from(31u8)).unwrap();
                }
                if len > 5 {
                    b[15] |= u8::try_from(nums[5] & T::from(7u8)).unwrap() << 5u32;
                    b[16] = u8::try_from((nums[5] >> 3) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[5] >> 11) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[5] >> 19) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[18] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[19] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[6] >> 10) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[6] >> 18) & T::from(127u8)).unwrap();
                }
                if len > 7 {
                    b[21] |= u8::try_from(nums[7] & T::from(1u8)).unwrap() << 7u32;
                    b[22] = u8::try_from((nums[7] >> 1) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[7] >> 9) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[7] >> 17) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 25 + 7) >> 3])?;
            }
        }

        26 => {
            let mut b = [0u8; 26];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(3u8)).unwrap();
                if len > 1 {
                    b[3] |= u8::try_from(nums[1] & T::from(63u8)).unwrap() << 2u32;
                    b[4] = u8::try_from((nums[1] >> 6) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 14) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 22) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[6] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[7] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[2] >> 12) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[2] >> 20) & T::from(63u8)).unwrap();
                }
                if len > 3 {
                    b[9] |= u8::try_from(nums[3] & T::from(3u8)).unwrap() << 6u32;
                    b[10] = u8::try_from((nums[3] >> 2) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[3] >> 10) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[3] >> 18) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[13] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[4] >> 24) & T::from(3u8)).unwrap();
                }
                if len > 5 {
                    b[16] |= u8::try_from(nums[5] & T::from(63u8)).unwrap() << 2u32;
                    b[17] = u8::try_from((nums[5] >> 6) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[5] >> 14) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[5] >> 22) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[19] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[20] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[6] >> 12) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[6] >> 20) & T::from(63u8)).unwrap();
                }
                if len > 7 {
                    b[22] |= u8::try_from(nums[7] & T::from(3u8)).unwrap() << 6u32;
                    b[23] = u8::try_from((nums[7] >> 2) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[7] >> 10) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[7] >> 18) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 26 + 7) >> 3])?;
            }
        }

        27 => {
            let mut b = [0u8; 27];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(7u8)).unwrap();
                if len > 1 {
                    b[3] |= u8::try_from(nums[1] & T::from(31u8)).unwrap() << 3u32;
                    b[4] = u8::try_from((nums[1] >> 5) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 13) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 21) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[6] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[7] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[2] >> 18) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[2] >> 26) & T::from(1u8)).unwrap();
                }
                if len > 3 {
                    b[10] |= u8::try_from(nums[3] & T::from(127u8)).unwrap() << 1u32;
                    b[11] = u8::try_from((nums[3] >> 7) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[3] >> 15) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[3] >> 23) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[13] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[14] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[4] >> 20) & T::from(127u8)).unwrap();
                }
                if len > 5 {
                    b[16] |= u8::try_from(nums[5] & T::from(1u8)).unwrap() << 7u32;
                    b[17] = u8::try_from((nums[5] >> 1) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[5] >> 9) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[5] >> 17) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[5] >> 25) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[20] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[21] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[6] >> 14) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[6] >> 22) & T::from(31u8)).unwrap();
                }
                if len > 7 {
                    b[23] |= u8::try_from(nums[7] & T::from(7u8)).unwrap() << 5u32;
                    b[24] = u8::try_from((nums[7] >> 3) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[7] >> 11) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[7] >> 19) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 27 + 7) >> 3])?;
            }
        }

        28 => {
            let mut b = [0u8; 28];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(15u8)).unwrap();
                if len > 1 {
                    b[3] |= u8::try_from(nums[1] & T::from(15u8)).unwrap() << 4u32;
                    b[4] = u8::try_from((nums[1] >> 4) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 12) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 20) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[7] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[2] >> 16) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[2] >> 24) & T::from(15u8)).unwrap();
                }
                if len > 3 {
                    b[10] |= u8::try_from(nums[3] & T::from(15u8)).unwrap() << 4u32;
                    b[11] = u8::try_from((nums[3] >> 4) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[3] >> 12) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[3] >> 20) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[14] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[4] >> 24) & T::from(15u8)).unwrap();
                }
                if len > 5 {
                    b[17] |= u8::try_from(nums[5] & T::from(15u8)).unwrap() << 4u32;
                    b[18] = u8::try_from((nums[5] >> 4) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[5] >> 12) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[5] >> 20) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[21] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[6] >> 16) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[6] >> 24) & T::from(15u8)).unwrap();
                }
                if len > 7 {
                    b[24] |= u8::try_from(nums[7] & T::from(15u8)).unwrap() << 4u32;
                    b[25] = u8::try_from((nums[7] >> 4) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[7] >> 12) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[7] >> 20) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 28 + 7) >> 3])?;
            }
        }

        29 => {
            let mut b = [0u8; 29];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(31u8)).unwrap();
                if len > 1 {
                    b[3] |= u8::try_from(nums[1] & T::from(7u8)).unwrap() << 5u32;
                    b[4] = u8::try_from((nums[1] >> 3) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 11) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 19) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 27) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[7] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[8] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[2] >> 14) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[2] >> 22) & T::from(127u8)).unwrap();
                }
                if len > 3 {
                    b[10] |= u8::try_from(nums[3] & T::from(1u8)).unwrap() << 7u32;
                    b[11] = u8::try_from((nums[3] >> 1) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[3] >> 9) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[3] >> 17) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[3] >> 25) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[14] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[15] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[4] >> 28) & T::from(1u8)).unwrap();
                }
                if len > 5 {
                    b[18] |= u8::try_from(nums[5] & T::from(127u8)).unwrap() << 1u32;
                    b[19] = u8::try_from((nums[5] >> 7) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[5] >> 15) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[5] >> 23) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[21] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[22] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[6] >> 10) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[6] >> 18) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[6] >> 26) & T::from(7u8)).unwrap();
                }
                if len > 7 {
                    b[25] |= u8::try_from(nums[7] & T::from(31u8)).unwrap() << 3u32;
                    b[26] = u8::try_from((nums[7] >> 5) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[7] >> 13) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[7] >> 21) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 29 + 7) >> 3])?;
            }
        }

        30 => {
            let mut b = [0u8; 30];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(63u8)).unwrap();
                if len > 1 {
                    b[3] |= u8::try_from(nums[1] & T::from(3u8)).unwrap() << 6u32;
                    b[4] = u8::try_from((nums[1] >> 2) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 10) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 18) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 26) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[7] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[8] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[2] >> 12) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[2] >> 20) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 28) & T::from(3u8)).unwrap();
                }
                if len > 3 {
                    b[11] |= u8::try_from(nums[3] & T::from(63u8)).unwrap() << 2u32;
                    b[12] = u8::try_from((nums[3] >> 6) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[3] >> 14) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[3] >> 22) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[15] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[4] >> 24) & T::from(63u8)).unwrap();
                }
                if len > 5 {
                    b[18] |= u8::try_from(nums[5] & T::from(3u8)).unwrap() << 6u32;
                    b[19] = u8::try_from((nums[5] >> 2) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[5] >> 10) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[5] >> 18) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[5] >> 26) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[22] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[23] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[6] >> 12) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[6] >> 20) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[6] >> 28) & T::from(3u8)).unwrap();
                }
                if len > 7 {
                    b[26] |= u8::try_from(nums[7] & T::from(63u8)).unwrap() << 2u32;
                    b[27] = u8::try_from((nums[7] >> 6) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[7] >> 14) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[7] >> 22) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 30 + 7) >> 3])?;
            }
        }

        31 => {
            let mut b = [0u8; 31];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(127u8)).unwrap();
                if len > 1 {
                    b[3] |= u8::try_from(nums[1] & T::from(1u8)).unwrap() << 7u32;
                    b[4] = u8::try_from((nums[1] >> 1) & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 9) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 17) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 25) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[7] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[8] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[2] >> 18) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 26) & T::from(31u8)).unwrap();
                }
                if len > 3 {
                    b[11] |= u8::try_from(nums[3] & T::from(7u8)).unwrap() << 5u32;
                    b[12] = u8::try_from((nums[3] >> 3) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[3] >> 11) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[3] >> 19) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[3] >> 27) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[15] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[16] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[4] >> 28) & T::from(7u8)).unwrap();
                }
                if len > 5 {
                    b[19] |= u8::try_from(nums[5] & T::from(31u8)).unwrap() << 3u32;
                    b[20] = u8::try_from((nums[5] >> 5) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[5] >> 13) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[5] >> 21) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[5] >> 29) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[23] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[24] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[6] >> 14) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[6] >> 22) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[6] >> 30) & T::from(1u8)).unwrap();
                }
                if len > 7 {
                    b[27] |= u8::try_from(nums[7] & T::from(127u8)).unwrap() << 1u32;
                    b[28] = u8::try_from((nums[7] >> 7) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[7] >> 15) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[7] >> 23) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 31 + 7) >> 3])?;
            }
        }

        32 => {
            let mut b = [0u8; 32];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                if len > 1 {
                    b[4] = u8::try_from(nums[1] & T::from(255u8)).unwrap();
                    b[5] = u8::try_from((nums[1] >> 8) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 16) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 24) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[8] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[2] >> 16) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 24) & T::from(255u8)).unwrap();
                }
                if len > 3 {
                    b[12] = u8::try_from(nums[3] & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[3] >> 8) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[3] >> 16) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[3] >> 24) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[16] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                }
                if len > 5 {
                    b[20] = u8::try_from(nums[5] & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[5] >> 8) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[5] >> 16) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[5] >> 24) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[24] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[6] >> 16) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[6] >> 24) & T::from(255u8)).unwrap();
                }
                if len > 7 {
                    b[28] = u8::try_from(nums[7] & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[7] >> 8) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[7] >> 16) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[7] >> 24) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 32 + 7) >> 3])?;
            }
        }

        33 => {
            let mut b = [0u8; 33];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(1u8)).unwrap();
                if len > 1 {
                    b[4] |= u8::try_from(nums[1] & T::from(127u8)).unwrap() << 1u32;
                    b[5] = u8::try_from((nums[1] >> 7) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 15) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 23) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 31) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[8] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[9] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[2] >> 14) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 22) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 30) & T::from(7u8)).unwrap();
                }
                if len > 3 {
                    b[12] |= u8::try_from(nums[3] & T::from(31u8)).unwrap() << 3u32;
                    b[13] = u8::try_from((nums[3] >> 5) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[3] >> 13) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[3] >> 21) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[3] >> 29) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[16] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[17] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[4] >> 28) & T::from(31u8)).unwrap();
                }
                if len > 5 {
                    b[20] |= u8::try_from(nums[5] & T::from(7u8)).unwrap() << 5u32;
                    b[21] = u8::try_from((nums[5] >> 3) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[5] >> 11) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[5] >> 19) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[5] >> 27) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[24] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[25] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[6] >> 10) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[6] >> 18) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[6] >> 26) & T::from(127u8)).unwrap();
                }
                if len > 7 {
                    b[28] |= u8::try_from(nums[7] & T::from(1u8)).unwrap() << 7u32;
                    b[29] = u8::try_from((nums[7] >> 1) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[7] >> 9) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[7] >> 17) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[7] >> 25) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 33 + 7) >> 3])?;
            }
        }

        34 => {
            let mut b = [0u8; 34];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(3u8)).unwrap();
                if len > 1 {
                    b[4] |= u8::try_from(nums[1] & T::from(63u8)).unwrap() << 2u32;
                    b[5] = u8::try_from((nums[1] >> 6) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 14) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 22) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 30) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[8] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[9] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[2] >> 12) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 20) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 28) & T::from(63u8)).unwrap();
                }
                if len > 3 {
                    b[12] |= u8::try_from(nums[3] & T::from(3u8)).unwrap() << 6u32;
                    b[13] = u8::try_from((nums[3] >> 2) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[3] >> 10) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[3] >> 18) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[3] >> 26) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[17] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[4] >> 32) & T::from(3u8)).unwrap();
                }
                if len > 5 {
                    b[21] |= u8::try_from(nums[5] & T::from(63u8)).unwrap() << 2u32;
                    b[22] = u8::try_from((nums[5] >> 6) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[5] >> 14) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[5] >> 22) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[5] >> 30) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[25] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[26] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[6] >> 12) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[6] >> 20) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[6] >> 28) & T::from(63u8)).unwrap();
                }
                if len > 7 {
                    b[29] |= u8::try_from(nums[7] & T::from(3u8)).unwrap() << 6u32;
                    b[30] = u8::try_from((nums[7] >> 2) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[7] >> 10) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[7] >> 18) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[7] >> 26) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 34 + 7) >> 3])?;
            }
        }

        35 => {
            let mut b = [0u8; 35];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(7u8)).unwrap();
                if len > 1 {
                    b[4] |= u8::try_from(nums[1] & T::from(31u8)).unwrap() << 3u32;
                    b[5] = u8::try_from((nums[1] >> 5) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 13) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 21) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 29) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[8] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[9] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 18) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 26) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 34) & T::from(1u8)).unwrap();
                }
                if len > 3 {
                    b[13] |= u8::try_from(nums[3] & T::from(127u8)).unwrap() << 1u32;
                    b[14] = u8::try_from((nums[3] >> 7) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[3] >> 15) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[3] >> 23) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[3] >> 31) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[17] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[18] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[4] >> 28) & T::from(127u8)).unwrap();
                }
                if len > 5 {
                    b[21] |= u8::try_from(nums[5] & T::from(1u8)).unwrap() << 7u32;
                    b[22] = u8::try_from((nums[5] >> 1) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[5] >> 9) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[5] >> 17) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[5] >> 25) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[5] >> 33) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[26] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[27] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[6] >> 14) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[6] >> 22) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[6] >> 30) & T::from(31u8)).unwrap();
                }
                if len > 7 {
                    b[30] |= u8::try_from(nums[7] & T::from(7u8)).unwrap() << 5u32;
                    b[31] = u8::try_from((nums[7] >> 3) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[7] >> 11) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[7] >> 19) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[7] >> 27) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 35 + 7) >> 3])?;
            }
        }

        36 => {
            let mut b = [0u8; 36];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(15u8)).unwrap();
                if len > 1 {
                    b[4] |= u8::try_from(nums[1] & T::from(15u8)).unwrap() << 4u32;
                    b[5] = u8::try_from((nums[1] >> 4) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 12) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 20) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 28) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[9] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 16) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 24) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 32) & T::from(15u8)).unwrap();
                }
                if len > 3 {
                    b[13] |= u8::try_from(nums[3] & T::from(15u8)).unwrap() << 4u32;
                    b[14] = u8::try_from((nums[3] >> 4) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[3] >> 12) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[3] >> 20) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[3] >> 28) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[18] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[4] >> 32) & T::from(15u8)).unwrap();
                }
                if len > 5 {
                    b[22] |= u8::try_from(nums[5] & T::from(15u8)).unwrap() << 4u32;
                    b[23] = u8::try_from((nums[5] >> 4) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[5] >> 12) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[5] >> 20) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[5] >> 28) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[27] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[6] >> 16) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[6] >> 24) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[6] >> 32) & T::from(15u8)).unwrap();
                }
                if len > 7 {
                    b[31] |= u8::try_from(nums[7] & T::from(15u8)).unwrap() << 4u32;
                    b[32] = u8::try_from((nums[7] >> 4) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[7] >> 12) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[7] >> 20) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[7] >> 28) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 36 + 7) >> 3])?;
            }
        }

        37 => {
            let mut b = [0u8; 37];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(31u8)).unwrap();
                if len > 1 {
                    b[4] |= u8::try_from(nums[1] & T::from(7u8)).unwrap() << 5u32;
                    b[5] = u8::try_from((nums[1] >> 3) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 11) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 19) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 27) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 35) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[9] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[10] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 14) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 22) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 30) & T::from(127u8)).unwrap();
                }
                if len > 3 {
                    b[13] |= u8::try_from(nums[3] & T::from(1u8)).unwrap() << 7u32;
                    b[14] = u8::try_from((nums[3] >> 1) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[3] >> 9) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[3] >> 17) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[3] >> 25) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[3] >> 33) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[18] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[19] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[4] >> 36) & T::from(1u8)).unwrap();
                }
                if len > 5 {
                    b[23] |= u8::try_from(nums[5] & T::from(127u8)).unwrap() << 1u32;
                    b[24] = u8::try_from((nums[5] >> 7) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[5] >> 15) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[5] >> 23) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[5] >> 31) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[27] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[28] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[6] >> 10) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[6] >> 18) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[6] >> 26) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[6] >> 34) & T::from(7u8)).unwrap();
                }
                if len > 7 {
                    b[32] |= u8::try_from(nums[7] & T::from(31u8)).unwrap() << 3u32;
                    b[33] = u8::try_from((nums[7] >> 5) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[7] >> 13) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[7] >> 21) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[7] >> 29) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 37 + 7) >> 3])?;
            }
        }

        38 => {
            let mut b = [0u8; 38];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(63u8)).unwrap();
                if len > 1 {
                    b[4] |= u8::try_from(nums[1] & T::from(3u8)).unwrap() << 6u32;
                    b[5] = u8::try_from((nums[1] >> 2) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 10) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 18) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 26) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 34) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[9] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[10] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 12) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 20) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 28) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 36) & T::from(3u8)).unwrap();
                }
                if len > 3 {
                    b[14] |= u8::try_from(nums[3] & T::from(63u8)).unwrap() << 2u32;
                    b[15] = u8::try_from((nums[3] >> 6) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[3] >> 14) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[3] >> 22) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[3] >> 30) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[19] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[4] >> 32) & T::from(63u8)).unwrap();
                }
                if len > 5 {
                    b[23] |= u8::try_from(nums[5] & T::from(3u8)).unwrap() << 6u32;
                    b[24] = u8::try_from((nums[5] >> 2) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[5] >> 10) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[5] >> 18) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[5] >> 26) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[5] >> 34) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[28] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[29] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[6] >> 12) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[6] >> 20) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[6] >> 28) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[6] >> 36) & T::from(3u8)).unwrap();
                }
                if len > 7 {
                    b[33] |= u8::try_from(nums[7] & T::from(63u8)).unwrap() << 2u32;
                    b[34] = u8::try_from((nums[7] >> 6) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[7] >> 14) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[7] >> 22) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[7] >> 30) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 38 + 7) >> 3])?;
            }
        }

        39 => {
            let mut b = [0u8; 39];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(127u8)).unwrap();
                if len > 1 {
                    b[4] |= u8::try_from(nums[1] & T::from(1u8)).unwrap() << 7u32;
                    b[5] = u8::try_from((nums[1] >> 1) & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 9) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 17) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 25) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 33) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[9] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[10] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 18) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 26) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 34) & T::from(31u8)).unwrap();
                }
                if len > 3 {
                    b[14] |= u8::try_from(nums[3] & T::from(7u8)).unwrap() << 5u32;
                    b[15] = u8::try_from((nums[3] >> 3) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[3] >> 11) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[3] >> 19) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[3] >> 27) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[3] >> 35) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[19] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[20] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[4] >> 36) & T::from(7u8)).unwrap();
                }
                if len > 5 {
                    b[24] |= u8::try_from(nums[5] & T::from(31u8)).unwrap() << 3u32;
                    b[25] = u8::try_from((nums[5] >> 5) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[5] >> 13) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[5] >> 21) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[5] >> 29) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[5] >> 37) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[29] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[30] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[6] >> 14) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[6] >> 22) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[6] >> 30) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[6] >> 38) & T::from(1u8)).unwrap();
                }
                if len > 7 {
                    b[34] |= u8::try_from(nums[7] & T::from(127u8)).unwrap() << 1u32;
                    b[35] = u8::try_from((nums[7] >> 7) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[7] >> 15) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[7] >> 23) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[7] >> 31) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 39 + 7) >> 3])?;
            }
        }

        40 => {
            let mut b = [0u8; 40];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                if len > 1 {
                    b[5] = u8::try_from(nums[1] & T::from(255u8)).unwrap();
                    b[6] = u8::try_from((nums[1] >> 8) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 16) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 24) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 32) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[10] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 16) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 24) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 32) & T::from(255u8)).unwrap();
                }
                if len > 3 {
                    b[15] = u8::try_from(nums[3] & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[3] >> 8) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[3] >> 16) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[3] >> 24) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[3] >> 32) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[20] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                }
                if len > 5 {
                    b[25] = u8::try_from(nums[5] & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[5] >> 8) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[5] >> 16) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[5] >> 24) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[5] >> 32) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[30] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[6] >> 16) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[6] >> 24) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[6] >> 32) & T::from(255u8)).unwrap();
                }
                if len > 7 {
                    b[35] = u8::try_from(nums[7] & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[7] >> 8) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[7] >> 16) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[7] >> 24) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[7] >> 32) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 40 + 7) >> 3])?;
            }
        }

        41 => {
            let mut b = [0u8; 41];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(1u8)).unwrap();
                if len > 1 {
                    b[5] |= u8::try_from(nums[1] & T::from(127u8)).unwrap() << 1u32;
                    b[6] = u8::try_from((nums[1] >> 7) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 15) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 23) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 31) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 39) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[10] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[11] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 14) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 22) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 30) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 38) & T::from(7u8)).unwrap();
                }
                if len > 3 {
                    b[15] |= u8::try_from(nums[3] & T::from(31u8)).unwrap() << 3u32;
                    b[16] = u8::try_from((nums[3] >> 5) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[3] >> 13) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[3] >> 21) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[3] >> 29) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[3] >> 37) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[20] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[21] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[4] >> 36) & T::from(31u8)).unwrap();
                }
                if len > 5 {
                    b[25] |= u8::try_from(nums[5] & T::from(7u8)).unwrap() << 5u32;
                    b[26] = u8::try_from((nums[5] >> 3) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[5] >> 11) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[5] >> 19) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[5] >> 27) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[5] >> 35) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[30] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[31] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[6] >> 10) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[6] >> 18) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[6] >> 26) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[6] >> 34) & T::from(127u8)).unwrap();
                }
                if len > 7 {
                    b[35] |= u8::try_from(nums[7] & T::from(1u8)).unwrap() << 7u32;
                    b[36] = u8::try_from((nums[7] >> 1) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[7] >> 9) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[7] >> 17) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[7] >> 25) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[7] >> 33) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 41 + 7) >> 3])?;
            }
        }

        42 => {
            let mut b = [0u8; 42];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(3u8)).unwrap();
                if len > 1 {
                    b[5] |= u8::try_from(nums[1] & T::from(63u8)).unwrap() << 2u32;
                    b[6] = u8::try_from((nums[1] >> 6) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 14) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 22) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 30) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 38) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[10] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[11] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 12) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 20) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 28) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 36) & T::from(63u8)).unwrap();
                }
                if len > 3 {
                    b[15] |= u8::try_from(nums[3] & T::from(3u8)).unwrap() << 6u32;
                    b[16] = u8::try_from((nums[3] >> 2) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[3] >> 10) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[3] >> 18) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[3] >> 26) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[3] >> 34) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[21] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[4] >> 40) & T::from(3u8)).unwrap();
                }
                if len > 5 {
                    b[26] |= u8::try_from(nums[5] & T::from(63u8)).unwrap() << 2u32;
                    b[27] = u8::try_from((nums[5] >> 6) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[5] >> 14) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[5] >> 22) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[5] >> 30) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[5] >> 38) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[31] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[32] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[6] >> 12) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[6] >> 20) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[6] >> 28) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[6] >> 36) & T::from(63u8)).unwrap();
                }
                if len > 7 {
                    b[36] |= u8::try_from(nums[7] & T::from(3u8)).unwrap() << 6u32;
                    b[37] = u8::try_from((nums[7] >> 2) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[7] >> 10) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[7] >> 18) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[7] >> 26) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[7] >> 34) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 42 + 7) >> 3])?;
            }
        }

        43 => {
            let mut b = [0u8; 43];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(7u8)).unwrap();
                if len > 1 {
                    b[5] |= u8::try_from(nums[1] & T::from(31u8)).unwrap() << 3u32;
                    b[6] = u8::try_from((nums[1] >> 5) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 13) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 21) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 29) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 37) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[10] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[11] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 18) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 26) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 34) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 42) & T::from(1u8)).unwrap();
                }
                if len > 3 {
                    b[16] |= u8::try_from(nums[3] & T::from(127u8)).unwrap() << 1u32;
                    b[17] = u8::try_from((nums[3] >> 7) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[3] >> 15) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[3] >> 23) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[3] >> 31) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 39) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[21] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[22] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[4] >> 36) & T::from(127u8)).unwrap();
                }
                if len > 5 {
                    b[26] |= u8::try_from(nums[5] & T::from(1u8)).unwrap() << 7u32;
                    b[27] = u8::try_from((nums[5] >> 1) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[5] >> 9) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[5] >> 17) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[5] >> 25) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[5] >> 33) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[5] >> 41) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[32] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[33] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[6] >> 14) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[6] >> 22) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[6] >> 30) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[6] >> 38) & T::from(31u8)).unwrap();
                }
                if len > 7 {
                    b[37] |= u8::try_from(nums[7] & T::from(7u8)).unwrap() << 5u32;
                    b[38] = u8::try_from((nums[7] >> 3) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[7] >> 11) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[7] >> 19) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[7] >> 27) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[7] >> 35) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 43 + 7) >> 3])?;
            }
        }

        44 => {
            let mut b = [0u8; 44];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(15u8)).unwrap();
                if len > 1 {
                    b[5] |= u8::try_from(nums[1] & T::from(15u8)).unwrap() << 4u32;
                    b[6] = u8::try_from((nums[1] >> 4) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 12) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 20) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 28) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 36) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[11] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 16) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 24) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 32) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 40) & T::from(15u8)).unwrap();
                }
                if len > 3 {
                    b[16] |= u8::try_from(nums[3] & T::from(15u8)).unwrap() << 4u32;
                    b[17] = u8::try_from((nums[3] >> 4) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[3] >> 12) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[3] >> 20) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[3] >> 28) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 36) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[22] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[4] >> 40) & T::from(15u8)).unwrap();
                }
                if len > 5 {
                    b[27] |= u8::try_from(nums[5] & T::from(15u8)).unwrap() << 4u32;
                    b[28] = u8::try_from((nums[5] >> 4) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[5] >> 12) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[5] >> 20) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[5] >> 28) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[5] >> 36) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[33] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[6] >> 16) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[6] >> 24) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[6] >> 32) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[6] >> 40) & T::from(15u8)).unwrap();
                }
                if len > 7 {
                    b[38] |= u8::try_from(nums[7] & T::from(15u8)).unwrap() << 4u32;
                    b[39] = u8::try_from((nums[7] >> 4) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[7] >> 12) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[7] >> 20) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[7] >> 28) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[7] >> 36) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 44 + 7) >> 3])?;
            }
        }

        45 => {
            let mut b = [0u8; 45];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(31u8)).unwrap();
                if len > 1 {
                    b[5] |= u8::try_from(nums[1] & T::from(7u8)).unwrap() << 5u32;
                    b[6] = u8::try_from((nums[1] >> 3) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 11) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 19) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 27) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 35) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 43) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[11] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[12] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 14) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 22) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 30) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 38) & T::from(127u8)).unwrap();
                }
                if len > 3 {
                    b[16] |= u8::try_from(nums[3] & T::from(1u8)).unwrap() << 7u32;
                    b[17] = u8::try_from((nums[3] >> 1) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[3] >> 9) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[3] >> 17) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[3] >> 25) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 33) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 41) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[22] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[23] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[4] >> 36) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[4] >> 44) & T::from(1u8)).unwrap();
                }
                if len > 5 {
                    b[28] |= u8::try_from(nums[5] & T::from(127u8)).unwrap() << 1u32;
                    b[29] = u8::try_from((nums[5] >> 7) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[5] >> 15) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[5] >> 23) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[5] >> 31) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[5] >> 39) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[33] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[34] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[6] >> 10) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[6] >> 18) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[6] >> 26) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[6] >> 34) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[6] >> 42) & T::from(7u8)).unwrap();
                }
                if len > 7 {
                    b[39] |= u8::try_from(nums[7] & T::from(31u8)).unwrap() << 3u32;
                    b[40] = u8::try_from((nums[7] >> 5) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[7] >> 13) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[7] >> 21) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[7] >> 29) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[7] >> 37) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 45 + 7) >> 3])?;
            }
        }

        46 => {
            let mut b = [0u8; 46];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(63u8)).unwrap();
                if len > 1 {
                    b[5] |= u8::try_from(nums[1] & T::from(3u8)).unwrap() << 6u32;
                    b[6] = u8::try_from((nums[1] >> 2) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 10) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 18) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 26) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 34) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 42) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[11] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[12] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 12) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 20) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 28) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 36) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 44) & T::from(3u8)).unwrap();
                }
                if len > 3 {
                    b[17] |= u8::try_from(nums[3] & T::from(63u8)).unwrap() << 2u32;
                    b[18] = u8::try_from((nums[3] >> 6) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[3] >> 14) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[3] >> 22) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 30) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 38) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[23] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[4] >> 40) & T::from(63u8)).unwrap();
                }
                if len > 5 {
                    b[28] |= u8::try_from(nums[5] & T::from(3u8)).unwrap() << 6u32;
                    b[29] = u8::try_from((nums[5] >> 2) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[5] >> 10) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[5] >> 18) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[5] >> 26) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[5] >> 34) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[5] >> 42) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[34] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[35] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[6] >> 12) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[6] >> 20) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[6] >> 28) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[6] >> 36) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[6] >> 44) & T::from(3u8)).unwrap();
                }
                if len > 7 {
                    b[40] |= u8::try_from(nums[7] & T::from(63u8)).unwrap() << 2u32;
                    b[41] = u8::try_from((nums[7] >> 6) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[7] >> 14) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[7] >> 22) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[7] >> 30) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[7] >> 38) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 46 + 7) >> 3])?;
            }
        }

        47 => {
            let mut b = [0u8; 47];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(127u8)).unwrap();
                if len > 1 {
                    b[5] |= u8::try_from(nums[1] & T::from(1u8)).unwrap() << 7u32;
                    b[6] = u8::try_from((nums[1] >> 1) & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 9) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 17) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 25) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 33) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 41) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[11] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[12] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 18) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 26) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 34) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 42) & T::from(31u8)).unwrap();
                }
                if len > 3 {
                    b[17] |= u8::try_from(nums[3] & T::from(7u8)).unwrap() << 5u32;
                    b[18] = u8::try_from((nums[3] >> 3) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[3] >> 11) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[3] >> 19) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 27) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 35) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 43) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[23] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[24] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[4] >> 36) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[4] >> 44) & T::from(7u8)).unwrap();
                }
                if len > 5 {
                    b[29] |= u8::try_from(nums[5] & T::from(31u8)).unwrap() << 3u32;
                    b[30] = u8::try_from((nums[5] >> 5) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[5] >> 13) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[5] >> 21) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[5] >> 29) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[5] >> 37) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[5] >> 45) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[35] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[36] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[6] >> 14) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[6] >> 22) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[6] >> 30) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[6] >> 38) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[6] >> 46) & T::from(1u8)).unwrap();
                }
                if len > 7 {
                    b[41] |= u8::try_from(nums[7] & T::from(127u8)).unwrap() << 1u32;
                    b[42] = u8::try_from((nums[7] >> 7) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[7] >> 15) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[7] >> 23) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[7] >> 31) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[7] >> 39) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 47 + 7) >> 3])?;
            }
        }

        48 => {
            let mut b = [0u8; 48];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                if len > 1 {
                    b[6] = u8::try_from(nums[1] & T::from(255u8)).unwrap();
                    b[7] = u8::try_from((nums[1] >> 8) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 16) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 24) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 32) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 40) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[12] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 16) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 24) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 32) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 40) & T::from(255u8)).unwrap();
                }
                if len > 3 {
                    b[18] = u8::try_from(nums[3] & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[3] >> 8) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[3] >> 16) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 24) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 32) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 40) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[24] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[4] >> 40) & T::from(255u8)).unwrap();
                }
                if len > 5 {
                    b[30] = u8::try_from(nums[5] & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[5] >> 8) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[5] >> 16) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[5] >> 24) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[5] >> 32) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[5] >> 40) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[36] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[6] >> 16) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[6] >> 24) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[6] >> 32) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[6] >> 40) & T::from(255u8)).unwrap();
                }
                if len > 7 {
                    b[42] = u8::try_from(nums[7] & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[7] >> 8) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[7] >> 16) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[7] >> 24) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[7] >> 32) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[7] >> 40) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 48 + 7) >> 3])?;
            }
        }

        49 => {
            let mut b = [0u8; 49];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(1u8)).unwrap();
                if len > 1 {
                    b[6] |= u8::try_from(nums[1] & T::from(127u8)).unwrap() << 1u32;
                    b[7] = u8::try_from((nums[1] >> 7) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 15) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 23) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 31) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 39) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 47) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[12] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[13] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 14) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 22) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 30) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 38) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 46) & T::from(7u8)).unwrap();
                }
                if len > 3 {
                    b[18] |= u8::try_from(nums[3] & T::from(31u8)).unwrap() << 3u32;
                    b[19] = u8::try_from((nums[3] >> 5) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[3] >> 13) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 21) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 29) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 37) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 45) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[24] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[25] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[4] >> 36) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[4] >> 44) & T::from(31u8)).unwrap();
                }
                if len > 5 {
                    b[30] |= u8::try_from(nums[5] & T::from(7u8)).unwrap() << 5u32;
                    b[31] = u8::try_from((nums[5] >> 3) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[5] >> 11) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[5] >> 19) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[5] >> 27) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[5] >> 35) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[5] >> 43) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[36] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[37] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[6] >> 10) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[6] >> 18) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[6] >> 26) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[6] >> 34) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[6] >> 42) & T::from(127u8)).unwrap();
                }
                if len > 7 {
                    b[42] |= u8::try_from(nums[7] & T::from(1u8)).unwrap() << 7u32;
                    b[43] = u8::try_from((nums[7] >> 1) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[7] >> 9) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[7] >> 17) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[7] >> 25) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[7] >> 33) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[7] >> 41) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 49 + 7) >> 3])?;
            }
        }

        50 => {
            let mut b = [0u8; 50];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(3u8)).unwrap();
                if len > 1 {
                    b[6] |= u8::try_from(nums[1] & T::from(63u8)).unwrap() << 2u32;
                    b[7] = u8::try_from((nums[1] >> 6) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 14) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 22) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 30) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 38) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 46) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[12] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[13] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 12) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 20) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 28) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 36) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 44) & T::from(63u8)).unwrap();
                }
                if len > 3 {
                    b[18] |= u8::try_from(nums[3] & T::from(3u8)).unwrap() << 6u32;
                    b[19] = u8::try_from((nums[3] >> 2) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[3] >> 10) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 18) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 26) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 34) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 42) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[25] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[4] >> 40) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 48) & T::from(3u8)).unwrap();
                }
                if len > 5 {
                    b[31] |= u8::try_from(nums[5] & T::from(63u8)).unwrap() << 2u32;
                    b[32] = u8::try_from((nums[5] >> 6) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[5] >> 14) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[5] >> 22) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[5] >> 30) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[5] >> 38) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[5] >> 46) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[37] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[38] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[6] >> 12) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[6] >> 20) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[6] >> 28) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[6] >> 36) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[6] >> 44) & T::from(63u8)).unwrap();
                }
                if len > 7 {
                    b[43] |= u8::try_from(nums[7] & T::from(3u8)).unwrap() << 6u32;
                    b[44] = u8::try_from((nums[7] >> 2) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[7] >> 10) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[7] >> 18) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[7] >> 26) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[7] >> 34) & T::from(255u8)).unwrap();
                    b[49] = u8::try_from((nums[7] >> 42) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 50 + 7) >> 3])?;
            }
        }

        51 => {
            let mut b = [0u8; 51];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(7u8)).unwrap();
                if len > 1 {
                    b[6] |= u8::try_from(nums[1] & T::from(31u8)).unwrap() << 3u32;
                    b[7] = u8::try_from((nums[1] >> 5) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 13) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 21) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 29) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 37) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 45) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[12] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[13] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 18) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 26) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 34) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 42) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[2] >> 50) & T::from(1u8)).unwrap();
                }
                if len > 3 {
                    b[19] |= u8::try_from(nums[3] & T::from(127u8)).unwrap() << 1u32;
                    b[20] = u8::try_from((nums[3] >> 7) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 15) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 23) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 31) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 39) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[3] >> 47) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[25] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[26] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[4] >> 36) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 44) & T::from(127u8)).unwrap();
                }
                if len > 5 {
                    b[31] |= u8::try_from(nums[5] & T::from(1u8)).unwrap() << 7u32;
                    b[32] = u8::try_from((nums[5] >> 1) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[5] >> 9) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[5] >> 17) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[5] >> 25) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[5] >> 33) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[5] >> 41) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[5] >> 49) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[38] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[39] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[6] >> 14) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[6] >> 22) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[6] >> 30) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[6] >> 38) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[6] >> 46) & T::from(31u8)).unwrap();
                }
                if len > 7 {
                    b[44] |= u8::try_from(nums[7] & T::from(7u8)).unwrap() << 5u32;
                    b[45] = u8::try_from((nums[7] >> 3) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[7] >> 11) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[7] >> 19) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[7] >> 27) & T::from(255u8)).unwrap();
                    b[49] = u8::try_from((nums[7] >> 35) & T::from(255u8)).unwrap();
                    b[50] = u8::try_from((nums[7] >> 43) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 51 + 7) >> 3])?;
            }
        }

        52 => {
            let mut b = [0u8; 52];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(15u8)).unwrap();
                if len > 1 {
                    b[6] |= u8::try_from(nums[1] & T::from(15u8)).unwrap() << 4u32;
                    b[7] = u8::try_from((nums[1] >> 4) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 12) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 20) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 28) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 36) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 44) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[13] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 16) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 24) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 32) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 40) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[2] >> 48) & T::from(15u8)).unwrap();
                }
                if len > 3 {
                    b[19] |= u8::try_from(nums[3] & T::from(15u8)).unwrap() << 4u32;
                    b[20] = u8::try_from((nums[3] >> 4) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 12) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 20) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 28) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 36) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[3] >> 44) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[26] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 40) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[4] >> 48) & T::from(15u8)).unwrap();
                }
                if len > 5 {
                    b[32] |= u8::try_from(nums[5] & T::from(15u8)).unwrap() << 4u32;
                    b[33] = u8::try_from((nums[5] >> 4) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[5] >> 12) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[5] >> 20) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[5] >> 28) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[5] >> 36) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[5] >> 44) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[39] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[6] >> 16) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[6] >> 24) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[6] >> 32) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[6] >> 40) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[6] >> 48) & T::from(15u8)).unwrap();
                }
                if len > 7 {
                    b[45] |= u8::try_from(nums[7] & T::from(15u8)).unwrap() << 4u32;
                    b[46] = u8::try_from((nums[7] >> 4) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[7] >> 12) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[7] >> 20) & T::from(255u8)).unwrap();
                    b[49] = u8::try_from((nums[7] >> 28) & T::from(255u8)).unwrap();
                    b[50] = u8::try_from((nums[7] >> 36) & T::from(255u8)).unwrap();
                    b[51] = u8::try_from((nums[7] >> 44) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 52 + 7) >> 3])?;
            }
        }

        53 => {
            let mut b = [0u8; 53];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(31u8)).unwrap();
                if len > 1 {
                    b[6] |= u8::try_from(nums[1] & T::from(7u8)).unwrap() << 5u32;
                    b[7] = u8::try_from((nums[1] >> 3) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 11) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 19) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 27) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 35) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 43) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[1] >> 51) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[13] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[14] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 14) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 22) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 30) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 38) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[2] >> 46) & T::from(127u8)).unwrap();
                }
                if len > 3 {
                    b[19] |= u8::try_from(nums[3] & T::from(1u8)).unwrap() << 7u32;
                    b[20] = u8::try_from((nums[3] >> 1) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[3] >> 9) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 17) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 25) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 33) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[3] >> 41) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[3] >> 49) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[26] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[27] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 36) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[4] >> 44) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[4] >> 52) & T::from(1u8)).unwrap();
                }
                if len > 5 {
                    b[33] |= u8::try_from(nums[5] & T::from(127u8)).unwrap() << 1u32;
                    b[34] = u8::try_from((nums[5] >> 7) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[5] >> 15) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[5] >> 23) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[5] >> 31) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[5] >> 39) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[5] >> 47) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[39] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[40] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[6] >> 10) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[6] >> 18) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[6] >> 26) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[6] >> 34) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[6] >> 42) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[6] >> 50) & T::from(7u8)).unwrap();
                }
                if len > 7 {
                    b[46] |= u8::try_from(nums[7] & T::from(31u8)).unwrap() << 3u32;
                    b[47] = u8::try_from((nums[7] >> 5) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[7] >> 13) & T::from(255u8)).unwrap();
                    b[49] = u8::try_from((nums[7] >> 21) & T::from(255u8)).unwrap();
                    b[50] = u8::try_from((nums[7] >> 29) & T::from(255u8)).unwrap();
                    b[51] = u8::try_from((nums[7] >> 37) & T::from(255u8)).unwrap();
                    b[52] = u8::try_from((nums[7] >> 45) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 53 + 7) >> 3])?;
            }
        }

        54 => {
            let mut b = [0u8; 54];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(63u8)).unwrap();
                if len > 1 {
                    b[6] |= u8::try_from(nums[1] & T::from(3u8)).unwrap() << 6u32;
                    b[7] = u8::try_from((nums[1] >> 2) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 10) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 18) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 26) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 34) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 42) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[1] >> 50) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[13] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[14] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 12) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 20) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 28) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 36) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[2] >> 44) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[2] >> 52) & T::from(3u8)).unwrap();
                }
                if len > 3 {
                    b[20] |= u8::try_from(nums[3] & T::from(63u8)).unwrap() << 2u32;
                    b[21] = u8::try_from((nums[3] >> 6) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 14) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 22) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 30) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[3] >> 38) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[3] >> 46) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[27] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[4] >> 40) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[4] >> 48) & T::from(63u8)).unwrap();
                }
                if len > 5 {
                    b[33] |= u8::try_from(nums[5] & T::from(3u8)).unwrap() << 6u32;
                    b[34] = u8::try_from((nums[5] >> 2) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[5] >> 10) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[5] >> 18) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[5] >> 26) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[5] >> 34) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[5] >> 42) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[5] >> 50) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[40] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[41] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[6] >> 12) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[6] >> 20) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[6] >> 28) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[6] >> 36) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[6] >> 44) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[6] >> 52) & T::from(3u8)).unwrap();
                }
                if len > 7 {
                    b[47] |= u8::try_from(nums[7] & T::from(63u8)).unwrap() << 2u32;
                    b[48] = u8::try_from((nums[7] >> 6) & T::from(255u8)).unwrap();
                    b[49] = u8::try_from((nums[7] >> 14) & T::from(255u8)).unwrap();
                    b[50] = u8::try_from((nums[7] >> 22) & T::from(255u8)).unwrap();
                    b[51] = u8::try_from((nums[7] >> 30) & T::from(255u8)).unwrap();
                    b[52] = u8::try_from((nums[7] >> 38) & T::from(255u8)).unwrap();
                    b[53] = u8::try_from((nums[7] >> 46) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 54 + 7) >> 3])?;
            }
        }

        55 => {
            let mut b = [0u8; 55];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(127u8)).unwrap();
                if len > 1 {
                    b[6] |= u8::try_from(nums[1] & T::from(1u8)).unwrap() << 7u32;
                    b[7] = u8::try_from((nums[1] >> 1) & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 9) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 17) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 25) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 33) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 41) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[1] >> 49) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[13] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[14] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 18) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 26) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 34) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[2] >> 42) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[2] >> 50) & T::from(31u8)).unwrap();
                }
                if len > 3 {
                    b[20] |= u8::try_from(nums[3] & T::from(7u8)).unwrap() << 5u32;
                    b[21] = u8::try_from((nums[3] >> 3) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 11) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 19) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 27) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[3] >> 35) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[3] >> 43) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[3] >> 51) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[27] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[28] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[4] >> 36) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[4] >> 44) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[4] >> 52) & T::from(7u8)).unwrap();
                }
                if len > 5 {
                    b[34] |= u8::try_from(nums[5] & T::from(31u8)).unwrap() << 3u32;
                    b[35] = u8::try_from((nums[5] >> 5) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[5] >> 13) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[5] >> 21) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[5] >> 29) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[5] >> 37) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[5] >> 45) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[5] >> 53) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[41] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[42] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[6] >> 14) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[6] >> 22) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[6] >> 30) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[6] >> 38) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[6] >> 46) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[6] >> 54) & T::from(1u8)).unwrap();
                }
                if len > 7 {
                    b[48] |= u8::try_from(nums[7] & T::from(127u8)).unwrap() << 1u32;
                    b[49] = u8::try_from((nums[7] >> 7) & T::from(255u8)).unwrap();
                    b[50] = u8::try_from((nums[7] >> 15) & T::from(255u8)).unwrap();
                    b[51] = u8::try_from((nums[7] >> 23) & T::from(255u8)).unwrap();
                    b[52] = u8::try_from((nums[7] >> 31) & T::from(255u8)).unwrap();
                    b[53] = u8::try_from((nums[7] >> 39) & T::from(255u8)).unwrap();
                    b[54] = u8::try_from((nums[7] >> 47) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 55 + 7) >> 3])?;
            }
        }

        56 => {
            let mut b = [0u8; 56];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(255u8)).unwrap();
                if len > 1 {
                    b[7] = u8::try_from(nums[1] & T::from(255u8)).unwrap();
                    b[8] = u8::try_from((nums[1] >> 8) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 16) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 24) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 32) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 40) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[1] >> 48) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[14] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[15] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 16) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 24) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 32) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[2] >> 40) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[2] >> 48) & T::from(255u8)).unwrap();
                }
                if len > 3 {
                    b[21] = u8::try_from(nums[3] & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[3] >> 8) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 16) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 24) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[3] >> 32) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[3] >> 40) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[3] >> 48) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[28] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[4] >> 40) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[4] >> 48) & T::from(255u8)).unwrap();
                }
                if len > 5 {
                    b[35] = u8::try_from(nums[5] & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[5] >> 8) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[5] >> 16) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[5] >> 24) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[5] >> 32) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[5] >> 40) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[5] >> 48) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[42] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[6] >> 16) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[6] >> 24) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[6] >> 32) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[6] >> 40) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[6] >> 48) & T::from(255u8)).unwrap();
                }
                if len > 7 {
                    b[49] = u8::try_from(nums[7] & T::from(255u8)).unwrap();
                    b[50] = u8::try_from((nums[7] >> 8) & T::from(255u8)).unwrap();
                    b[51] = u8::try_from((nums[7] >> 16) & T::from(255u8)).unwrap();
                    b[52] = u8::try_from((nums[7] >> 24) & T::from(255u8)).unwrap();
                    b[53] = u8::try_from((nums[7] >> 32) & T::from(255u8)).unwrap();
                    b[54] = u8::try_from((nums[7] >> 40) & T::from(255u8)).unwrap();
                    b[55] = u8::try_from((nums[7] >> 48) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 56 + 7) >> 3])?;
            }
        }

        57 => {
            let mut b = [0u8; 57];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(255u8)).unwrap();
                b[7] = u8::try_from((nums[0] >> 56) & T::from(1u8)).unwrap();
                if len > 1 {
                    b[7] |= u8::try_from(nums[1] & T::from(127u8)).unwrap() << 1u32;
                    b[8] = u8::try_from((nums[1] >> 7) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 15) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 23) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 31) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 39) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[1] >> 47) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[1] >> 55) & T::from(3u8)).unwrap();
                }
                if len > 2 {
                    b[14] |= u8::try_from(nums[2] & T::from(63u8)).unwrap() << 2u32;
                    b[15] = u8::try_from((nums[2] >> 6) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 14) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 22) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 30) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[2] >> 38) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[2] >> 46) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[2] >> 54) & T::from(7u8)).unwrap();
                }
                if len > 3 {
                    b[21] |= u8::try_from(nums[3] & T::from(31u8)).unwrap() << 3u32;
                    b[22] = u8::try_from((nums[3] >> 5) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 13) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 21) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[3] >> 29) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[3] >> 37) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[3] >> 45) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[3] >> 53) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[28] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[29] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[4] >> 36) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[4] >> 44) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[4] >> 52) & T::from(31u8)).unwrap();
                }
                if len > 5 {
                    b[35] |= u8::try_from(nums[5] & T::from(7u8)).unwrap() << 5u32;
                    b[36] = u8::try_from((nums[5] >> 3) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[5] >> 11) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[5] >> 19) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[5] >> 27) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[5] >> 35) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[5] >> 43) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[5] >> 51) & T::from(63u8)).unwrap();
                }
                if len > 6 {
                    b[42] |= u8::try_from(nums[6] & T::from(3u8)).unwrap() << 6u32;
                    b[43] = u8::try_from((nums[6] >> 2) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[6] >> 10) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[6] >> 18) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[6] >> 26) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[6] >> 34) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[6] >> 42) & T::from(255u8)).unwrap();
                    b[49] = u8::try_from((nums[6] >> 50) & T::from(127u8)).unwrap();
                }
                if len > 7 {
                    b[49] |= u8::try_from(nums[7] & T::from(1u8)).unwrap() << 7u32;
                    b[50] = u8::try_from((nums[7] >> 1) & T::from(255u8)).unwrap();
                    b[51] = u8::try_from((nums[7] >> 9) & T::from(255u8)).unwrap();
                    b[52] = u8::try_from((nums[7] >> 17) & T::from(255u8)).unwrap();
                    b[53] = u8::try_from((nums[7] >> 25) & T::from(255u8)).unwrap();
                    b[54] = u8::try_from((nums[7] >> 33) & T::from(255u8)).unwrap();
                    b[55] = u8::try_from((nums[7] >> 41) & T::from(255u8)).unwrap();
                    b[56] = u8::try_from((nums[7] >> 49) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 57 + 7) >> 3])?;
            }
        }

        58 => {
            let mut b = [0u8; 58];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(255u8)).unwrap();
                b[7] = u8::try_from((nums[0] >> 56) & T::from(3u8)).unwrap();
                if len > 1 {
                    b[7] |= u8::try_from(nums[1] & T::from(63u8)).unwrap() << 2u32;
                    b[8] = u8::try_from((nums[1] >> 6) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 14) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 22) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 30) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 38) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[1] >> 46) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[1] >> 54) & T::from(15u8)).unwrap();
                }
                if len > 2 {
                    b[14] |= u8::try_from(nums[2] & T::from(15u8)).unwrap() << 4u32;
                    b[15] = u8::try_from((nums[2] >> 4) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 12) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 20) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 28) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[2] >> 36) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[2] >> 44) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[2] >> 52) & T::from(63u8)).unwrap();
                }
                if len > 3 {
                    b[21] |= u8::try_from(nums[3] & T::from(3u8)).unwrap() << 6u32;
                    b[22] = u8::try_from((nums[3] >> 2) & T::from(255u8)).unwrap();
                    b[23] = u8::try_from((nums[3] >> 10) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 18) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[3] >> 26) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[3] >> 34) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[3] >> 42) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[3] >> 50) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[29] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[30] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[4] >> 40) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[4] >> 48) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[4] >> 56) & T::from(3u8)).unwrap();
                }
                if len > 5 {
                    b[36] |= u8::try_from(nums[5] & T::from(63u8)).unwrap() << 2u32;
                    b[37] = u8::try_from((nums[5] >> 6) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[5] >> 14) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[5] >> 22) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[5] >> 30) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[5] >> 38) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[5] >> 46) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[5] >> 54) & T::from(15u8)).unwrap();
                }
                if len > 6 {
                    b[43] |= u8::try_from(nums[6] & T::from(15u8)).unwrap() << 4u32;
                    b[44] = u8::try_from((nums[6] >> 4) & T::from(255u8)).unwrap();
                    b[45] = u8::try_from((nums[6] >> 12) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[6] >> 20) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[6] >> 28) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[6] >> 36) & T::from(255u8)).unwrap();
                    b[49] = u8::try_from((nums[6] >> 44) & T::from(255u8)).unwrap();
                    b[50] = u8::try_from((nums[6] >> 52) & T::from(63u8)).unwrap();
                }
                if len > 7 {
                    b[50] |= u8::try_from(nums[7] & T::from(3u8)).unwrap() << 6u32;
                    b[51] = u8::try_from((nums[7] >> 2) & T::from(255u8)).unwrap();
                    b[52] = u8::try_from((nums[7] >> 10) & T::from(255u8)).unwrap();
                    b[53] = u8::try_from((nums[7] >> 18) & T::from(255u8)).unwrap();
                    b[54] = u8::try_from((nums[7] >> 26) & T::from(255u8)).unwrap();
                    b[55] = u8::try_from((nums[7] >> 34) & T::from(255u8)).unwrap();
                    b[56] = u8::try_from((nums[7] >> 42) & T::from(255u8)).unwrap();
                    b[57] = u8::try_from((nums[7] >> 50) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 58 + 7) >> 3])?;
            }
        }

        59 => {
            let mut b = [0u8; 59];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(255u8)).unwrap();
                b[7] = u8::try_from((nums[0] >> 56) & T::from(7u8)).unwrap();
                if len > 1 {
                    b[7] |= u8::try_from(nums[1] & T::from(31u8)).unwrap() << 3u32;
                    b[8] = u8::try_from((nums[1] >> 5) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 13) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 21) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 29) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 37) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[1] >> 45) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[1] >> 53) & T::from(63u8)).unwrap();
                }
                if len > 2 {
                    b[14] |= u8::try_from(nums[2] & T::from(3u8)).unwrap() << 6u32;
                    b[15] = u8::try_from((nums[2] >> 2) & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 10) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 18) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 26) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[2] >> 34) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[2] >> 42) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[2] >> 50) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[2] >> 58) & T::from(1u8)).unwrap();
                }
                if len > 3 {
                    b[22] |= u8::try_from(nums[3] & T::from(127u8)).unwrap() << 1u32;
                    b[23] = u8::try_from((nums[3] >> 7) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 15) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[3] >> 23) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[3] >> 31) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[3] >> 39) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[3] >> 47) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[3] >> 55) & T::from(15u8)).unwrap();
                }
                if len > 4 {
                    b[29] |= u8::try_from(nums[4] & T::from(15u8)).unwrap() << 4u32;
                    b[30] = u8::try_from((nums[4] >> 4) & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 12) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[4] >> 20) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[4] >> 28) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[4] >> 36) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[4] >> 44) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[4] >> 52) & T::from(127u8)).unwrap();
                }
                if len > 5 {
                    b[36] |= u8::try_from(nums[5] & T::from(1u8)).unwrap() << 7u32;
                    b[37] = u8::try_from((nums[5] >> 1) & T::from(255u8)).unwrap();
                    b[38] = u8::try_from((nums[5] >> 9) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[5] >> 17) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[5] >> 25) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[5] >> 33) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[5] >> 41) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[5] >> 49) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[5] >> 57) & T::from(3u8)).unwrap();
                }
                if len > 6 {
                    b[44] |= u8::try_from(nums[6] & T::from(63u8)).unwrap() << 2u32;
                    b[45] = u8::try_from((nums[6] >> 6) & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[6] >> 14) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[6] >> 22) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[6] >> 30) & T::from(255u8)).unwrap();
                    b[49] = u8::try_from((nums[6] >> 38) & T::from(255u8)).unwrap();
                    b[50] = u8::try_from((nums[6] >> 46) & T::from(255u8)).unwrap();
                    b[51] = u8::try_from((nums[6] >> 54) & T::from(31u8)).unwrap();
                }
                if len > 7 {
                    b[51] |= u8::try_from(nums[7] & T::from(7u8)).unwrap() << 5u32;
                    b[52] = u8::try_from((nums[7] >> 3) & T::from(255u8)).unwrap();
                    b[53] = u8::try_from((nums[7] >> 11) & T::from(255u8)).unwrap();
                    b[54] = u8::try_from((nums[7] >> 19) & T::from(255u8)).unwrap();
                    b[55] = u8::try_from((nums[7] >> 27) & T::from(255u8)).unwrap();
                    b[56] = u8::try_from((nums[7] >> 35) & T::from(255u8)).unwrap();
                    b[57] = u8::try_from((nums[7] >> 43) & T::from(255u8)).unwrap();
                    b[58] = u8::try_from((nums[7] >> 51) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 59 + 7) >> 3])?;
            }
        }

        60 => {
            let mut b = [0u8; 60];
            for nums in numbers.chunks(8) {
                let len = nums.len();
                b[0] = u8::try_from(nums[0] & T::from(255u8)).unwrap();
                b[1] = u8::try_from((nums[0] >> 8) & T::from(255u8)).unwrap();
                b[2] = u8::try_from((nums[0] >> 16) & T::from(255u8)).unwrap();
                b[3] = u8::try_from((nums[0] >> 24) & T::from(255u8)).unwrap();
                b[4] = u8::try_from((nums[0] >> 32) & T::from(255u8)).unwrap();
                b[5] = u8::try_from((nums[0] >> 40) & T::from(255u8)).unwrap();
                b[6] = u8::try_from((nums[0] >> 48) & T::from(255u8)).unwrap();
                b[7] = u8::try_from((nums[0] >> 56) & T::from(15u8)).unwrap();
                if len > 1 {
                    b[7] |= u8::try_from(nums[1] & T::from(15u8)).unwrap() << 4u32;
                    b[8] = u8::try_from((nums[1] >> 4) & T::from(255u8)).unwrap();
                    b[9] = u8::try_from((nums[1] >> 12) & T::from(255u8)).unwrap();
                    b[10] = u8::try_from((nums[1] >> 20) & T::from(255u8)).unwrap();
                    b[11] = u8::try_from((nums[1] >> 28) & T::from(255u8)).unwrap();
                    b[12] = u8::try_from((nums[1] >> 36) & T::from(255u8)).unwrap();
                    b[13] = u8::try_from((nums[1] >> 44) & T::from(255u8)).unwrap();
                    b[14] = u8::try_from((nums[1] >> 52) & T::from(255u8)).unwrap();
                }
                if len > 2 {
                    b[15] = u8::try_from(nums[2] & T::from(255u8)).unwrap();
                    b[16] = u8::try_from((nums[2] >> 8) & T::from(255u8)).unwrap();
                    b[17] = u8::try_from((nums[2] >> 16) & T::from(255u8)).unwrap();
                    b[18] = u8::try_from((nums[2] >> 24) & T::from(255u8)).unwrap();
                    b[19] = u8::try_from((nums[2] >> 32) & T::from(255u8)).unwrap();
                    b[20] = u8::try_from((nums[2] >> 40) & T::from(255u8)).unwrap();
                    b[21] = u8::try_from((nums[2] >> 48) & T::from(255u8)).unwrap();
                    b[22] = u8::try_from((nums[2] >> 56) & T::from(15u8)).unwrap();
                }
                if len > 3 {
                    b[22] |= u8::try_from(nums[3] & T::from(15u8)).unwrap() << 4u32;
                    b[23] = u8::try_from((nums[3] >> 4) & T::from(255u8)).unwrap();
                    b[24] = u8::try_from((nums[3] >> 12) & T::from(255u8)).unwrap();
                    b[25] = u8::try_from((nums[3] >> 20) & T::from(255u8)).unwrap();
                    b[26] = u8::try_from((nums[3] >> 28) & T::from(255u8)).unwrap();
                    b[27] = u8::try_from((nums[3] >> 36) & T::from(255u8)).unwrap();
                    b[28] = u8::try_from((nums[3] >> 44) & T::from(255u8)).unwrap();
                    b[29] = u8::try_from((nums[3] >> 52) & T::from(255u8)).unwrap();
                }
                if len > 4 {
                    b[30] = u8::try_from(nums[4] & T::from(255u8)).unwrap();
                    b[31] = u8::try_from((nums[4] >> 8) & T::from(255u8)).unwrap();
                    b[32] = u8::try_from((nums[4] >> 16) & T::from(255u8)).unwrap();
                    b[33] = u8::try_from((nums[4] >> 24) & T::from(255u8)).unwrap();
                    b[34] = u8::try_from((nums[4] >> 32) & T::from(255u8)).unwrap();
                    b[35] = u8::try_from((nums[4] >> 40) & T::from(255u8)).unwrap();
                    b[36] = u8::try_from((nums[4] >> 48) & T::from(255u8)).unwrap();
                    b[37] = u8::try_from((nums[4] >> 56) & T::from(15u8)).unwrap();
                }
                if len > 5 {
                    b[37] |= u8::try_from(nums[5] & T::from(15u8)).unwrap() << 4u32;
                    b[38] = u8::try_from((nums[5] >> 4) & T::from(255u8)).unwrap();
                    b[39] = u8::try_from((nums[5] >> 12) & T::from(255u8)).unwrap();
                    b[40] = u8::try_from((nums[5] >> 20) & T::from(255u8)).unwrap();
                    b[41] = u8::try_from((nums[5] >> 28) & T::from(255u8)).unwrap();
                    b[42] = u8::try_from((nums[5] >> 36) & T::from(255u8)).unwrap();
                    b[43] = u8::try_from((nums[5] >> 44) & T::from(255u8)).unwrap();
                    b[44] = u8::try_from((nums[5] >> 52) & T::from(255u8)).unwrap();
                }
                if len > 6 {
                    b[45] = u8::try_from(nums[6] & T::from(255u8)).unwrap();
                    b[46] = u8::try_from((nums[6] >> 8) & T::from(255u8)).unwrap();
                    b[47] = u8::try_from((nums[6] >> 16) & T::from(255u8)).unwrap();
                    b[48] = u8::try_from((nums[6] >> 24) & T::from(255u8)).unwrap();
                    b[49] = u8::try_from((nums[6] >> 32) & T::from(255u8)).unwrap();
                    b[50] = u8::try_from((nums[6] >> 40) & T::from(255u8)).unwrap();
                    b[51] = u8::try_from((nums[6] >> 48) & T::from(255u8)).unwrap();
                    b[52] = u8::try_from((nums[6] >> 56) & T::from(15u8)).unwrap();
                }
                if len > 7 {
                    b[52] |= u8::try_from(nums[7] & T::from(15u8)).unwrap() << 4u32;
                    b[53] = u8::try_from((nums[7] >> 4) & T::from(255u8)).unwrap();
                    b[54] = u8::try_from((nums[7] >> 12) & T::from(255u8)).unwrap();
                    b[55] = u8::try_from((nums[7] >> 20) & T::from(255u8)).unwrap();
                    b[56] = u8::try_from((nums[7] >> 28) & T::from(255u8)).unwrap();
                    b[57] = u8::try_from((nums[7] >> 36) & T::from(255u8)).unwrap();
                    b[58] = u8::try_from((nums[7] >> 44) & T::from(255u8)).unwrap();
                    b[59] = u8::try_from((nums[7] >> 52) & T::from(255u8)).unwrap();
                }
                wr.write_all(&b[0..(len * 60 + 7) >> 3])?;
            }
        }

        _ => {
            panic!("Unsupported");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinReader, ReadEncIntError};
    use rand::RngExt;

    pub fn read_numbers<R: Read>(
        rd: R,
        num_bits: u8,
        num_num: usize,
    ) -> Result<Vec<u64>, ReadEncIntError> {
        let mut binrd = BinReader::new(rd);
        let mut out_numbers = vec![0u64; num_num];
        for num in &mut out_numbers {
            *num = u64::try_from(binrd.read_bits(num_bits)?).unwrap();
        }
        binrd.inner()?;
        Ok(out_numbers)
    }

    #[test]
    fn test_read_numbers_2() {
        let mut rng = rand::rng();
        for i in 0..4 {
            for bits in 1..=60 {
                for len in 0..300 {
                    let byte_len = (len * usize::try_from(bits).unwrap() + 7) >> 3;
                    let input = (0..byte_len)
                        .map(|_| rng.random::<u8>())
                        .collect::<Vec<_>>();
                    let expected = read_numbers(input.as_slice(), bits, len).unwrap();
                    let mut result = vec![0; expected.len()];
                    read_numbers_2::<_, u64>(input.as_slice(), bits, &mut result).unwrap();
                    assert_eq!(expected, result, "{} {} {}", i, bits, len);
                }
            }
        }
    }

    #[test]
    fn test_write_numbers_2() {
        let mut rng = rand::rng();
        for i in 0..4 {
            for bits in 1..=60 {
                for len in 0..300 {
                    let bits_u = usize::try_from(bits).unwrap();
                    let byte_len = (len * bits_u + 7) >> 3;
                    let last_bits = (len * bits_u) & 7;
                    let input = {
                        let mut input = (0..byte_len)
                            .map(|_| rng.random::<u8>())
                            .collect::<Vec<_>>();
                        // fix last byte if not aligned
                        if last_bits != 0 {
                            *input.last_mut().unwrap() &= (1u8 << last_bits) - 1;
                        }
                        input
                    };
                    let numbers = read_numbers(input.as_slice(), bits, len).unwrap();
                    let mut output = vec![];
                    write_numbers_2::<_, u64>(&mut output, bits, &numbers).unwrap();
                    assert_eq!(input, output, "{} {} {}", i, bits, len);
                }
            }
        }
    }
}
