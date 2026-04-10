pub trait Sqrt{
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt {
    ( $($ty:ty),* ) => {
        $(
            impl Sqrt for $ty {
                fn sqrt(self) -> Self {
                    self.sqrt()
                }
            }
        )*
    };
}

impl_sqrt!(f32, f64);

pub trait Abs{
    fn abs(self) -> Self;
}

macro_rules! impl_abs {
    ( $($ty:ty),* ) => {
        $(
            impl Abs for $ty {
                fn abs(self) -> Self {
                    self.abs()
                }
            }
        )*
    };
}

impl_abs!(
    isize,
    i8, i16, i32, i64,
    f32, f64
);

pub trait CosSin{
    fn cos(self) -> Self;
    fn sin(self) -> Self;
}

macro_rules! impl_cossin {
    ( $($ty:ty),* ) => {
        $(
            impl CosSin for $ty {
                fn cos(self) -> Self {
                    self.cos()
                }
                fn sin(self) -> Self {
                    self.sin()
                }
            }
        )*
    };
}

impl_cossin!(
    f32, f64
);

pub trait Zero{
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ( $( ($ty:ty, $zero:expr) ),* ) => {
        $(
            impl Zero for $ty {
                fn zero() -> Self {
                    $zero
                }
            }
        )*
    };
}

impl_zero!(
    (isize, 0isize),
    (usize, 0usize),
    (i8, 0i8),
    (i16, 0i16),
    (i32, 0i32),
    (i64, 0i64),
    (u8, 0u8),
    (u16, 0u16),
    (u32, 0u32),
    (u64, 0u64),
    (f32, 0f32),
    (f64, 0f64)
);

pub trait One{
    fn one() -> Self;
}

macro_rules! impl_one {
    ( $( ($ty:ty, $one:expr) ),* ) => {
        $(
            impl One for $ty {
                fn one() -> Self {
                    $one
                }
            }
        )*
    };
}

impl_one!(
    (isize, 1isize),
    (usize, 1usize),
    (i8, 1i8),
    (i16, 1i16),
    (i32, 1i32),
    (i64, 1i64),
    (u8, 1u8),
    (u16, 1u16),
    (u32, 1u32),
    (u64, 1u64),
    (f32, 1f32),
    (f64, 1f64)
);

pub trait Two{
    fn two() -> Self;
}

macro_rules! impl_two {
    ( $( ($ty:ty, $two:expr) ),* ) => {
        $(
            impl Two for $ty {
                fn two() -> Self {
                    $two
                }
            }
        )*
    };
}

impl_two!(
    (isize, 2isize),
    (usize, 2usize),
    (i8, 2i8),
    (i16, 2i16),
    (i32, 2i32),
    (i64, 2i64),
    (u8, 2u8),
    (u16, 2u16),
    (u32, 2u32),
    (u64, 2u64),
    (f32, 2f32),
    (f64, 2f64)
);

pub trait FromUsize{
    fn fromusize(n: usize) -> Self;
}

macro_rules! impl_fromusize {
    ( $($ty:ty),* ) => {
        $(
            impl FromUsize for $ty {
                fn fromusize(n: usize) -> Self {
                    n as $ty
                }
            }
        )*
    };
}

impl_fromusize!(
    i8, i16, i32, i64,
    f32, f64
);