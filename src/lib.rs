pub mod components {
    pub mod cam;
    pub mod enemy;
    pub mod level;
    pub mod menu;
    pub mod player;
    pub mod size;
}

pub mod traits {
    pub mod character;
}

pub mod util {
    pub mod macros;
    pub mod type_of;
}

#[allow(unused_imports)]
use components::{cam::*, enemy::*, level::*, menu::*, player::*, size::*};

#[allow(unused_imports)]
use traits::character::*;

#[allow(unused_imports)]
use util::{macros::*, type_of::*};
