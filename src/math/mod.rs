// mod bivecn;
mod matn;
mod vecn;
mod traits;

// pub use bivecn::BiVecN;
pub use matn::MatN;
pub use vecn::VecN;
pub use traits::*;

pub type Vecf32<const N: usize> = VecN<f32,N>;
pub type Vecf64<const N: usize> = VecN<f64,N>;

pub type Matf32<const N: usize> = MatN<f32,N>;
pub type Matf64<const N: usize> = MatN<f64,N>;