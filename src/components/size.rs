use bevy::prelude::*;

#[derive(Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Size {
    Gigantic,
    Large,
    #[default]
    Normal,
    Little,
    Small,
    Tiny,
}
