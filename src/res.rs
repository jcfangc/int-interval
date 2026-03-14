#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OneTwo<T> {
    One(T),
    Two(T, T),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ZeroOneTwo<T> {
    Zero,
    One(T),
    Two(T, T),
}
