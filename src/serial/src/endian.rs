/* This file is part of DarkFi (https://dark.fi)
 *
 * Copyright (C) 2020-2025 Dyne.org foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

macro_rules! define_slice_to_be {
    ($name: ident, $type: ty) => {
        #[inline]
        #[allow(dead_code)]
        pub fn $name(slice: &[u8]) -> $type {
            assert_eq!(slice.len(), ::core::mem::size_of::<$type>());
            let mut res = 0;
            for i in 0..::core::mem::size_of::<$type>() {
                res |= (slice[i] as $type) << (::core::mem::size_of::<$type>() - i - 1) * 8;
            }
            res
        }
    };
}

macro_rules! define_slice_to_le {
    ($name: ident, $type: ty) => {
        #[inline]
        pub fn $name(slice: &[u8]) -> $type {
            assert_eq!(slice.len(), ::core::mem::size_of::<$type>());
            let mut res = 0;
            for i in 0..::core::mem::size_of::<$type>() {
                res |= (slice[i] as $type) << i * 8;
            }
            res
        }
    };
}

macro_rules! define_be_to_array {
    ($name: ident, $type: ty, $byte_len: expr) => {
        #[inline]
        #[allow(dead_code)]
        pub fn $name(val: $type) -> [u8; $byte_len] {
            assert_eq!(::core::mem::size_of::<$type>(), $byte_len);
            let mut res = [0; $byte_len];
            for i in 0..$byte_len {
                res[i] = ((val >> ($byte_len - i - 1) * 8) & 0xff) as u8;
            }
            res
        }
    };
}

macro_rules! define_le_to_array {
    ($name: ident, $type: ty, $byte_len: expr) => {
        #[inline]
        pub fn $name(val: $type) -> [u8; $byte_len] {
            assert_eq!(::core::mem::size_of::<$type>(), $byte_len);
            let mut res = [0; $byte_len];
            for i in 0..$byte_len {
                res[i] = ((val >> i * 8) & 0xff) as u8;
            }
            res
        }
    };
}

define_slice_to_be!(slice_to_u32_be, u32);
define_be_to_array!(u32_to_array_be, u32, 4);

define_slice_to_le!(slice_to_u16_le, u16);
define_slice_to_le!(slice_to_u32_le, u32);
define_slice_to_le!(slice_to_u64_le, u64);
define_slice_to_le!(slice_to_u128_le, u128);
//define_slice_to_le!(slice_to_usize_le, usize);
//define_slice_to_le!(slice_to_isize_le, isize);

define_le_to_array!(u16_to_array_le, u16, 2);
define_le_to_array!(u32_to_array_le, u32, 4);
define_le_to_array!(u64_to_array_le, u64, 8);
define_le_to_array!(u128_to_array_le, u128, 16);
//define_le_to_array!(usize_to_array_le, usize, usize::BITS as usize / 8);
//define_le_to_array!(isize_to_array_le, isize, isize::BITS as usize / 8);

#[inline]
pub fn i16_to_array_le(val: i16) -> [u8; 2] {
    u16_to_array_le(val as u16)
}

#[inline]
pub fn i32_to_array_le(val: i32) -> [u8; 4] {
    u32_to_array_le(val as u32)
}

#[inline]
pub fn i64_to_array_le(val: i64) -> [u8; 8] {
    u64_to_array_le(val as u64)
}

#[inline]
pub fn i128_to_array_le(val: i128) -> [u8; 16] {
    u128_to_array_le(val as u128)
}

#[inline]
pub fn slice_to_i16_le(slice: &[u8]) -> i16 {
    slice_to_u16_le(slice) as i16
}

#[inline]
pub fn slice_to_i32_le(slice: &[u8]) -> i32 {
    slice_to_u32_le(slice) as i32
}

#[inline]
pub fn slice_to_i64_le(slice: &[u8]) -> i64 {
    slice_to_u64_le(slice) as i64
}

#[inline]
pub fn slice_to_i128_le(slice: &[u8]) -> i128 {
    slice_to_u128_le(slice) as i128
}

#[inline]
pub fn f64_to_array_le(val: f64) -> [u8; 8] {
    assert_eq!(::core::mem::size_of::<f64>(), 8);
    val.to_le_bytes()
}
#[inline]
pub fn slice_to_f64_le(slice: &[u8; 8]) -> f64 {
    assert_eq!(slice.len(), ::core::mem::size_of::<f64>());
    f64::from_le_bytes(*slice)
}
#[inline]
pub fn f32_to_array_le(val: f32) -> [u8; 4] {
    assert_eq!(::core::mem::size_of::<f32>(), 4);
    val.to_le_bytes()
}
#[inline]
pub fn slice_to_f32_le(slice: &[u8; 4]) -> f32 {
    assert_eq!(slice.len(), ::core::mem::size_of::<f32>());
    f32::from_le_bytes(*slice)
}

macro_rules! define_chunk_slice_to_int {
    ($name: ident, $type: ty, $converter: ident) => {
        #[inline]
        #[allow(dead_code)]
        pub fn $name(inp: &[u8], outp: &mut [$type]) {
            //assert_eq!(inp.len(), outp.len() * ::core::mem::size_of::<$type>());
            assert_eq!(inp.len(), std::mem::size_of_val(outp));
            for (outp_val, data_bytes) in
                outp.iter_mut().zip(inp.chunks(::core::mem::size_of::<$type>()))
            {
                *outp_val = $converter(data_bytes);
            }
        }
    };
}

define_chunk_slice_to_int!(bytes_to_u64_slice_le, u64, slice_to_u64_le);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endianness_test() {
        assert_eq!(slice_to_u32_be(&[0xde, 0xad, 0xbe, 0xef]), 0xdeadbeef);
        assert_eq!(u32_to_array_be(0xdeadbeef), [0xde, 0xad, 0xbe, 0xef]);

        assert_eq!(slice_to_u16_le(&[0xad, 0xde]), 0xdead);
        assert_eq!(slice_to_u32_le(&[0xef, 0xbe, 0xad, 0xde]), 0xdeadbeef);
        assert_eq!(
            slice_to_u64_le(&[0xef, 0xbe, 0xad, 0xde, 0xfe, 0xca, 0xad, 0x1b]),
            0x1badcafedeadbeef
        );
        assert_eq!(u16_to_array_le(0xdead), [0xad, 0xde]);
        assert_eq!(u32_to_array_le(0xdeadbeef), [0xef, 0xbe, 0xad, 0xde]);
        assert_eq!(
            u64_to_array_le(0x1badcafedeadbeef),
            [0xef, 0xbe, 0xad, 0xde, 0xfe, 0xca, 0xad, 0x1b]
        );
    }

    #[test]
    fn endian_chunk_test() {
        let inp = [
            0xef, 0xbe, 0xad, 0xde, 0xfe, 0xca, 0xad, 0x1b, 0xfe, 0xca, 0xad, 0x1b, 0xce, 0xfa,
            0x01, 0x02,
        ];
        let mut out = [0; 2];
        bytes_to_u64_slice_le(&inp, &mut out);
        assert_eq!(out, [0x1badcafedeadbeef, 0x0201face1badcafe]);
    }
}
