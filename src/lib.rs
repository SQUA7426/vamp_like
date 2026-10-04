pub mod components {
    pub mod cam;
    pub mod enemy;
    pub mod level;
    pub mod menu;
    pub mod player;
    pub mod size;
}

pub mod util {
    pub mod macros;
}

#[allow(unused_imports)]
use components::{cam::*, enemy::*, level::*, menu::*, player::*, size::*};

#[allow(unused_imports)]
use util::macros::*;
