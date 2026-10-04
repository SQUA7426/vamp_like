use bevy::color::palettes::basic::BLACK;
use bevy::prelude::*;

#[derive(States, Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum GameState {
    Shop,
    Inventory,
    StartMenu,
    #[default]
    Menu,
    Upgrade,
    Level,
}

const MENU_BTN_HEIGHT: f32 = 100.0;
const MENU_BTN_WIDTH: f32 = 20.0;
const MENU_BTN_MARGIN: f32 = 0.0;
const MENU_BTN_RADIUS: f32 = 20.0;
const MENU_BTN_OUTLINE: Color = Color::Srgba(BLACK);

#[derive(Debug)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(gamemenu::GameMenuPlugin);
    }
}

mod gamemenu {
    use bevy::prelude::*;
    use bevy::color::palettes::basic::GRAY;

    use crate::{
        components::menu::{
            GameState, MENU_BTN_HEIGHT, MENU_BTN_MARGIN, MENU_BTN_OUTLINE, MENU_BTN_RADIUS, MENU_BTN_WIDTH,
        }, create_btn, create_btn_text, create_node,
    };

    #[derive(Component)]
    pub enum MenuBtn {
        Shop,
        Inventory,
        Home,
        Upgrade,
        Level,
    }

    pub struct GameMenuPlugin;

    impl Plugin for GameMenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Menu), setup_gamemenu);
        }
    }

    fn setup_gamemenu(mut cmds: Commands) {
        let gamemenu_node = create_node!(5.0, 5.0, 85.0, Some(5.0), 90.0, 10.0);

        cmds.spawn((
            DespawnOnExit(GameState::Menu),
            gamemenu_node,
            // BackgroundColor(Color::srgba(0.0, 1.0, 0.0, 0.6)),
            Children::spawn(SpawnIter(
                [
                    ("SHOP", MenuBtn::Shop),
                    ("INV", MenuBtn::Inventory),
                    ("MENU", MenuBtn::Home),
                    ("UPGRADE", MenuBtn::Upgrade),
                    ("LEVEL", MenuBtn::Level),
                ]
                .into_iter()
                .map(|(t, comp)| (button(String::from(t)), comp)),
            )),
        ));
    }

    fn button(t: String) -> impl Bundle {
        (
            Button,
            create_btn!(
                MENU_BTN_HEIGHT,
                MENU_BTN_WIDTH,
                MENU_BTN_MARGIN,
                MENU_BTN_RADIUS
            ),
            BorderColor::all(MENU_BTN_OUTLINE),
            BackgroundColor(Color::Srgba(GRAY)),
            children![(create_btn_text!(t))],
        )
    }
}
