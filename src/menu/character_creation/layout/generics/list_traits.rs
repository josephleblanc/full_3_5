use bevy::prelude::Component;

pub trait AsVec
where
    Self: Sized,
{
    fn vec() -> Vec<Self>;
}

pub trait AsButtonList
where
    Self: Copy + Clone + Sized,
{
    fn button_list() -> Vec<Self>;
}

pub trait HasDescr {
    fn description(&self) -> &String;
}

pub trait HasKey<T> {
    fn key(&self) -> T;
}

pub trait HasItemVec<T>
where
    T: bevy::prelude::Component + Clone + Copy,
{
    fn vec(&self) -> Vec<(&T, &String, &String)>;
}

pub trait HasArray<const N: usize>
where
    Self: Sized + Copy + Clone + Component,
{
    fn has_array() -> [Self; N];
}
