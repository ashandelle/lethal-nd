// mod bivecn;
mod matn;
mod vecn;
mod traits;

// pub use bivecn::BiVecN;
pub use matn::MatN;
pub use vecn::VecN;
pub use traits::*;

pub type Vec3<T> = VecN<T,3>;
pub type Vec4<T> = VecN<T,4>;
pub type Vec5<T> = VecN<T,5>;

pub type Vec3f32 = VecN<f32,3>;
pub type Vec4f32 = VecN<f32,4>;
pub type Vec5f32 = VecN<f32,5>;

pub type Vec3f64 = VecN<f64,3>;
pub type Vec4f64 = VecN<f64,4>;
pub type Vec5f64 = VecN<f64,5>;

pub type Vec3i32 = VecN<i32,3>;
pub type Vec4i32 = VecN<i32,4>;
pub type Vec5i32 = VecN<i32,5>;