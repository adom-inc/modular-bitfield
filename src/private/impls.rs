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
    (int $prim:ty as $unsigned_prim:ty) => {
        impl Specifier for $prim {
            const BITS: usize = ::core::mem::size_of::<$prim>() * 8;
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
    };
    (float $prim:ty as $unsigned_prim:ty) => {
        impl Specifier for $prim {
            const BITS: usize = ::core::mem::size_of::<$prim>() * 8;
            type Bytes = $unsigned_prim;
            type InOut = $prim;

            #[inline]
            fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
                Ok(input.to_bits())
            }

            #[inline]
            fn from_bytes(bytes: Self::Bytes) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>> {
                Ok(<$prim>::from_bits(bytes))
            }
        }
    };
    ( $($kind:ident $individual1:tt as $individual2:tt), *$(,)?) => {
        $(
            impl_specifier_for_primitive!($kind $individual1 as $individual2);
        )*
    };
}

impl_specifier_for_primitive!(
    int u8 as u8,
    int u16 as u16,
    int u32 as u32,
    int u64 as u64,
    int u128 as u128,
    int i8 as u8,
    int i16 as u16,
    int i32 as u32,
    int i64 as u64,
    int i128 as u128,
    float f32 as u32,
    float f64 as u64,
);
