pub trait FromRef<T> {
    fn from_ref(val: &T) -> Self;
}
