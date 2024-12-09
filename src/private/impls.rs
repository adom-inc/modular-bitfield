use crate::{
    error::{InvalidBitPattern, OutOfBounds},
    Specifier,
};

impl Specifier for bool {
    const BITS: usize = 1;
    type Bytes = u8;
    type InOut = bool;

    #[inline]
    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
        Ok(input as u8)
    }

    #[inline]
    fn from_bytes(
        bytes: Self::Bytes,
    ) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>> {
        match bytes {
            0 => Ok(false),
            1 => Ok(true),
            invalid_bytes => Err(InvalidBitPattern { invalid_bytes }),
        }
    }
}

macro_rules! impl_specifier_for_primitive {
    ( $( ($prim:ty as $unsigned_prim:ty: $bits:literal) ),* $(,)? ) => {
        $(
            impl Specifier for $prim {
                const BITS: usize = $bits;
                type Bytes = $unsigned_prim;
                type InOut = $prim;

                #[inline]
                fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
                    Ok(input as $unsigned_prim)
                }

                #[inline]
                fn from_bytes(bytes: Self::Bytes) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>> {
                    Ok(bytes as $prim)
                }
            }
        )*
    };
}

impl_specifier_for_primitive!(
    (u8 as u8: 8),
    (u16 as u16: 16),
    (u32 as u32: 32),
    (u64 as u64: 64),
    (u128 as u128: 128),
);

impl_specifier_for_primitive!(
    (i8 as u8: 8),
    (i16 as u16: 16),
    (i32 as u32: 32),
    (i64 as u64: 64),
    (i128 as u128: 128),
);
