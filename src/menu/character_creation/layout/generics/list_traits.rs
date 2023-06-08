use bevy::prelude::Component;

pub trait AsVec
where
    Self: Sized,
{
    fn vec() -> Vec<Self>;
}

/// This trait means the item it has been implemented for has a default set of buttens, which will
/// be returned as a vec from the method below.
pub trait AsButtonList
where
    Self: Copy + Clone + Sized,
{
    /// Returns a vector of the default buttons for the implementing item.
    fn button_list() -> Vec<Self>;
}

/// The implementing item has a description, which should be returned as a string by the method
/// below. This is mostly used to get the description from Custom Assets, e.g. RaceAsset
pub trait HasDescr {
    /// Returns a string of the description contained in the implementing item.
    fn description(&self) -> &String;
}

/// The implementing item has a key, usually an enum, which differentiates it from other items.
/// This is mostly used to differentiate which instance of a custom asset to use, e.g. a
/// PlayableRace for a RaceAsset.
/// Examples of targets for T are: PlayableRace, PlayableClass
pub trait HasKey<T> {
    fn key(&self) -> T;
}

/// Used when the item has a vector of things we would like to extract, e.g. the traits assigned to
/// a race in DefaultTraitAsset.
pub trait HasItemVec<T>
where
    T: bevy::prelude::Component + Clone + Copy,
{
    fn vec(&self) -> Vec<(&T, &String, &String)>;
}

/// Used to list the member of an enum as an array by calling the array. This is useful in getting
/// the listed members of an enum when using the enum in generics.
pub trait HasArray<const N: usize>
where
    Self: Sized + Copy + Clone + Component,
{
    fn has_array() -> [Self; N];
}
