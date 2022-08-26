use std::{fmt::Display, io::Write, str::FromStr};

use cookie_factory::{GenResult, WriteContext};
use nom::IResult;
use num_traits::PrimInt;

use crate::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VInt<T: PrimInt + From<u8>>(pub T);
impl<'a, T: PrimInt + From<u8>> Packet<'a> for VInt<T> {
    fn serialize<W: Write>(&self, mut w: WriteContext<W>) -> GenResult<W> {
        let mut val = self.0;
        loop {
            let mut bottom_byte = (val & (<T as From<u8>>::from(0xFFu8))).to_u8().unwrap();
            bottom_byte &= 0x7F;
            val = val.unsigned_shr(7);
            if val.is_zero() {
                w.write_all(&[bottom_byte])?;
                return Ok(w);
            }
            bottom_byte |= 0x80;
            w.write_all(&[bottom_byte])?;
        }
    }

    fn deserialize(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let max_len = {
            let max = T::max_value();
            max.count_ones().div_ceil(7)
        } as usize;
        let mut result: T = 0u8.into();
        for (pos, &val) in input.iter().take(max_len).enumerate() {
            let trimmed_byte: T = (val & 0x7F).into();
            result = result | (trimmed_byte.unsigned_shl(pos as u32 * 7));
            if val & 0x80 != 0x80 {
                return IResult::Ok((&input[pos + 1..], Self(result)));
            }
        }
        IResult::Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TooLarge,
        )))
    }
}
impl<T: PrimInt + From<u8>> VInt<T> {
    pub fn deserialize_self(input: &[u8]) -> IResult<&[u8], T> {
        Self::deserialize(input).map(|(i, this)| (i, this.0))
    }
    pub fn deserialize_prim<U: num_traits::NumCast>(input: &[u8]) -> IResult<&[u8], U> {
        Self::deserialize(input).map(|(i, this)| (i, U::from(this.0).unwrap()))
    }
}
impl<T: PrimInt + From<u8>> num_traits::Saturating for VInt<T> {
    fn saturating_add(self, v: Self) -> Self {
        Self(T::saturating_add(self.0, v.0))
    }

    fn saturating_sub(self, v: Self) -> Self {
        Self(T::saturating_sub(self.0, v.0))
    }
}
impl<T: PrimInt + From<u8>> num_traits::Num for VInt<T> {
    type FromStrRadixErr = <T as num_traits::Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Self)
    }
}
impl<T: PrimInt + From<u8>> core::ops::Add for VInt<T> {
    type Output = VInt<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(T::add(self.0, rhs.0))
    }
}
impl<T: PrimInt + From<u8>> core::ops::Sub for VInt<T> {
    type Output = VInt<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(T::sub(self.0, rhs.0))
    }
}
impl<T: PrimInt + From<u8>> core::ops::Mul for VInt<T> {
    type Output = VInt<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(T::mul(self.0, rhs.0))
    }
}
impl<T: PrimInt + From<u8>> core::ops::Div for VInt<T> {
    type Output = VInt<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Self(T::div(self.0, rhs.0))
    }
}
impl<T: PrimInt + From<u8>> core::ops::Rem for VInt<T> {
    type Output = VInt<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(T::rem(self.0, rhs.0))
    }
}
impl<T: PrimInt + From<u8>> num_traits::Zero for VInt<T> {
    fn zero() -> Self {
        Self(T::zero())
    }

    fn is_zero(&self) -> bool {
        T::is_zero(&self.0)
    }
}
impl<T: PrimInt + From<u8>> num_traits::One for VInt<T> {
    fn one() -> Self {
        Self(T::one())
    }
}
impl<T: PrimInt + From<u8>> num_traits::CheckedAdd for VInt<T> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        T::checked_add(&self.0, &v.0).map(Self)
    }
}
impl<T: PrimInt + From<u8>> num_traits::CheckedSub for VInt<T> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        T::checked_sub(&self.0, &v.0).map(Self)
    }
}
impl<T: PrimInt + From<u8>> num_traits::CheckedMul for VInt<T> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        T::checked_mul(&self.0, &v.0).map(Self)
    }
}
impl<T: PrimInt + From<u8>> num_traits::CheckedDiv for VInt<T> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        T::checked_div(&self.0, &v.0).map(Self)
    }
}
impl<T: PrimInt + From<u8>> core::ops::Shr<usize> for VInt<T> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(T::shr(self.0, rhs))
    }
}
impl<T: PrimInt + From<u8>> core::ops::Shl<usize> for VInt<T> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(T::shl(self.0, rhs))
    }
}
impl<T: PrimInt + From<u8>> core::ops::BitAnd for VInt<T> {
    type Output = VInt<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(T::bitand(self.0, rhs.0))
    }
}
impl<T: PrimInt + From<u8>> core::ops::BitOr for VInt<T> {
    type Output = VInt<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(T::bitor(self.0, rhs.0))
    }
}
impl<T: PrimInt + From<u8>> core::ops::BitXor for VInt<T> {
    type Output = VInt<T>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(T::bitxor(self.0, rhs.0))
    }
}
impl<T: PrimInt + From<u8>> core::ops::Not for VInt<T> {
    type Output = VInt<T>;

    fn not(self) -> Self::Output {
        Self(T::not(self.0))
    }
}
impl<T: PrimInt + From<u8>> num_traits::Bounded for VInt<T> {
    fn min_value() -> Self {
        Self(T::min_value())
    }

    fn max_value() -> Self {
        Self(T::max_value())
    }
}
impl<T: PrimInt + From<u8>> num_traits::ToPrimitive for VInt<T> {
    fn to_i64(&self) -> Option<i64> {
        T::to_i64(&self.0)
    }

    fn to_u64(&self) -> Option<u64> {
        T::to_u64(&self.0)
    }
}
impl<T: PrimInt + From<u8>> num_traits::NumCast for VInt<T> {
    fn from<U: num_traits::ToPrimitive>(n: U) -> Option<Self> {
        <T as num_traits::NumCast>::from(n).map(Self)
    }
}

impl<T: PrimInt + From<u8>> PrimInt for VInt<T> {
    fn count_ones(self) -> u32 {
        T::count_ones(self.0)
    }

    fn count_zeros(self) -> u32 {
        T::count_zeros(self.0)
    }

    fn leading_zeros(self) -> u32 {
        T::leading_zeros(self.0)
    }

    fn trailing_zeros(self) -> u32 {
        T::trailing_zeros(self.0)
    }

    fn rotate_left(self, n: u32) -> Self {
        Self(T::rotate_left(self.0, n))
    }

    fn rotate_right(self, n: u32) -> Self {
        Self(T::rotate_right(self.0, n))
    }

    fn signed_shl(self, n: u32) -> Self {
        Self(T::signed_shl(self.0, n))
    }

    fn signed_shr(self, n: u32) -> Self {
        Self(T::signed_shr(self.0, n))
    }

    fn unsigned_shl(self, n: u32) -> Self {
        Self(T::unsigned_shl(self.0, n))
    }

    fn unsigned_shr(self, n: u32) -> Self {
        Self(T::unsigned_shr(self.0, n))
    }

    fn swap_bytes(self) -> Self {
        Self(T::swap_bytes(self.0))
    }

    fn from_be(x: Self) -> Self {
        Self(T::from_be(x.0))
    }

    fn from_le(x: Self) -> Self {
        Self(T::from_le(x.0))
    }

    fn to_be(self) -> Self {
        Self(T::to_be(self.0))
    }

    fn to_le(self) -> Self {
        Self(T::to_le(self.0))
    }

    fn pow(self, exp: u32) -> Self {
        Self(T::pow(self.0, exp))
    }
}

impl<T: PrimInt + From<u8> + FromStr> FromStr for VInt<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        T::from_str(s).map(Self)
    }
}

impl<T: PrimInt + From<u8> + Display> Display for VInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
