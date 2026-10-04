use bevy::prelude::*;

#[derive(Default, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub enum Size {
    Gigantic,
    Large,
    #[default]
    Normal,
    Little,
    Small,
    Tiny,
}
