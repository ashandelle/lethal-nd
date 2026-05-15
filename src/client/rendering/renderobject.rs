// use bincode::{Decode, Encode};
use bincode::{
    BorrowDecode, Decode, Encode, de, enc, error::{AllowedEnumVariants, DecodeError, EncodeError}
};
use mathnd::vecn::VecN;

use crate::Vecf64;

#[derive(Encode, Decode, Debug, Clone, Copy)]
pub struct RenderObject<const N: usize> {
    pub object: Object<N>,
    pub color: (f64, f64, f64),
}

#[derive(Debug, Clone, Copy)]
pub enum Object<const N: usize> {
    // Sphere,
    AABB {
        min: Vecf64<N>,
        max: Vecf64<N>,
    },
}

impl<const N: usize> Encode for Object<N> {
    fn encode<E: enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), EncodeError> {
        match self {
            // MyEnum::VariantA => {
            //     // Encode tag as u8
            //     0u8.encode(encoder, config)?;
            // }
            // MyEnum::VariantB(val) => {
            //     // Encode tag as u8, then the data
            //     1u8.encode(encoder, config)?;
            //     val.encode(encoder, config)?;
            // }
            Object::AABB { min, max } => {
                0u8.encode(encoder)?; // Enum variant tag
                min.e.encode(encoder)?;
                max.e.encode(encoder)?;
            },
        }
        Ok(())
    }
}

impl<Context, const N: usize> Decode<Context> for Object<N> {
    fn decode<D: de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let tag = u8::decode(decoder)?;
        match tag {
            // 0 => Ok(MyEnum::VariantA),
            // 1 => {
            //     // Read the data
            //     let val = u32::decode(decoder, config)?;
            //     Ok(MyEnum::VariantB(val))
            // }
            0 => {
                let min: [f64; N] = <[f64; N]>::decode(decoder)?;
                let max: [f64; N] = <[f64; N]>::decode(decoder)?;
                Ok(Object::AABB { min: VecN::new(min), max: VecN::new(max) })
            }
            _ => Err(DecodeError::UnexpectedVariant {
                type_name: "Object",
                allowed: &AllowedEnumVariants::Range { min: 0, max: 0 },
                found: tag as u32,
            }),
        }
    }
}

impl<'de, Context, const N: usize> BorrowDecode<'de, Context> for Object<N> {
    fn borrow_decode<D: de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let tag = u8::decode(decoder)?;
        match tag {
            // 0 => Ok(MyEnum::VariantA),
            // 1 => {
            //     // Read the data
            //     let val = u32::decode(decoder, config)?;
            //     Ok(MyEnum::VariantB(val))
            // }
            _ => Err(DecodeError::UnexpectedVariant {
                type_name: "Object",
                allowed: &AllowedEnumVariants::Range { min: 0, max: 0 },
                found: tag as u32,
            }),
        }
    }
}