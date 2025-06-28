pub trait FromExt<T>: Sized {
    fn from_ext(value: T) -> Self;
}

impl<T> FromExt<T> for T {
    fn from_ext(value: T) -> Self {
        value
    }
}

pub trait IntoExt<T>: Sized {
    fn into_ext(self) -> T;
}

impl<T, U> IntoExt<T> for U
where T: FromExt<U>,
{
    fn into_ext(self) -> T {
        T::from_ext(self)
    }
}
