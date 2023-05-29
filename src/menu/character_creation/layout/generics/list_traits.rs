pub trait AsVec
where
    Self: Sized,
{
    fn vec() -> Vec<Self>;
}

pub trait HasDescr {
    fn description(&self) -> &String;
}

pub trait HasKey<T> {
    fn key(&self) -> T;
}
