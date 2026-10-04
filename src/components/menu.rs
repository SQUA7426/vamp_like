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
            .add_plugins(gamemenu::GameMenuPlugin)
            .add_plugins(inventorymenu::InventoryMenuPlugin);
    }
}

mod gamemenu {
    use bevy::color::palettes::{
        basic::{BLACK, GRAY},
        css::{CORAL, DARK_BLUE, DARK_GREY, LIGHT_GREY},
    };
    use bevy::prelude::*;

    use crate::{
        components::menu::{
            GameState, MENU_BTN_HEIGHT, MENU_BTN_MARGIN, MENU_BTN_OUTLINE, MENU_BTN_RADIUS,
   MENU_BTN_WIDTH,
        },
        create_btn, create_btn_text, create_node,
    };

    #[derive(Component)]
    pub enum MenuBtn {
        Shop,
        Inventory,
        Home,
        Upgrade,
        Level,
    }
    #[derive(Component, Debug, Default, Clone, Copy, Eq, PartialEq)]
    pub enum MenuBtnActive {
        Active,
        #[default]
        Inactive
    }

    const NORMAL_TEXT: Color = Color::Srgba(BLACK);
    const NORMAL_BG: Color = Color::Srgba(LIGHT_GREY);
    const HOVER_TEXT: Color = Color::Srgba(DARK_BLUE);
    const HOVER_BG: Color = Color::Srgba(DARK_GREY);
    const PRESSED_TEXT: Color = Color::Srgba(CORAL);
    const PRESSED_BG: Color = Color::Srgba(GRAY);

    pub struct GameMenuPlugin;

    impl Plugin for GameMenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(/*OnEnter(GameState::Menu)*/ Startup, setup_gamemenu)
                .add_systems(Update, (button_system, button_action).chain());
        }
    }

    fn setup_gamemenu(mut cmds: Commands) {
        let gamemenu_node = create_node!(5.0, 5.0, 85.0, Some(5.0), 90.0, 10.0);

        cmds.spawn((
            // DespawnOnExit(GameState::Menu),
            gamemenu_node,
            // BackgroundColor(Color::srgba(0.0, 1.0, 0.0, 0.6)),
            Children::spawn(SpawnIter(
                [
                    ("SHOP", MenuBtn::Shop, MenuBtnActive::default()),
                    ("INV", MenuBtn::Inventory, MenuBtnActive::default()),
                    ("MENU", MenuBtn::Home, MenuBtnActive::Active),
                    ("UPGRADE", MenuBtn::Upgrade, MenuBtnActive::default()),
                    ("LEVEL", MenuBtn::Level, MenuBtnActive::default()),
                ]
                .into_iter()
                .map(|(t, comp, active)| (button(String::from(t)), comp, active)),
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
            BackgroundColor(NORMAL_BG),
            children![(create_btn_text!(t))],
        )
    }

    #[allow(clippy::type_complexity)]
    fn button_system(
        mut btn_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &mut MenuBtnActive,
                &Children,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        mut text_query: Query<&mut TextColor>,
    ) {
        for (interaction, mut bg_color, _border_color, mut active, children) in &mut btn_query {
            let mut text_color = text_query.get_mut(children[0]).unwrap();

            if *active == MenuBtnActive::Inactive {

                if *interaction == Interaction::Pressed {
                    *text_color = TextColor(PRESSED_TEXT);
                    *bg_color = BackgroundColor(PRESSED_BG);
                    *active = MenuBtnActive::Active;
                    // println!("Pressed: Active: {:?}", active.clone());
                }
                if *interaction == Interaction::Hovered {
                    *text_color = TextColor(HOVER_TEXT);
                    *bg_color = BackgroundColor(HOVER_BG);
                }
                if *interaction == Interaction::None {
                    *text_color = TextColor(NORMAL_TEXT);
                    *bg_color = BackgroundColor(NORMAL_BG);
                    *active = MenuBtnActive::default();
                    // println!("None: Active: {:?}", active.clone());
                }

            } else {

                if *interaction == Interaction::Pressed {
                    *active = MenuBtnActive::default();
                    // println!("Pressed: Active: {:?}", active.clone());
                }

                if *interaction == Interaction::None {
                    *text_color = TextColor(PRESSED_TEXT);
                    *bg_color = BackgroundColor(PRESSED_BG);
                }

            }
        }
    }

    #[allow(clippy::match_single_binding, clippy::type_complexity)]
    fn button_action(
        mut btn_query: Query<(&Interaction, &MenuBtn), (Changed<Interaction>, With<Button>)>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, btn) in &mut btn_query {
            if *interaction == Interaction::Pressed {
                match btn {
                    MenuBtn::Inventory => {
                        game_state.set(GameState::Inventory);
                    }
                    _ => {
                        game_state.set(GameState::default());
                    }
                }
            }

            if *interaction == Interaction::Hovered {
                match btn {
                    _ => {}
                }
            }

            if *interaction == Interaction::None {
                game_state.reset();
            }
        }
    }
}

mod inventorymenu {
    use bevy::prelude::*;
    
    use crate::{components::menu::GameState, create_node};

    #[derive(Debug)]
    pub struct InventoryMenuPlugin;

    impl Plugin for InventoryMenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Inventory), setup_inventory_menu);
        }
    }

    fn setup_inventory_menu(
        mut cmds: Commands,
    ) {

        let inventorymenu_node = create_node!(15.0, 15.0, 10.0, Some(20.0), 70.0, 70.0);
        cmds.spawn((
                DespawnOnExit(GameState::Inventory),
                inventorymenu_node,
                BackgroundColor(Color::srgba(0.0, 1.0, 0.0, 0.6)),
        ));
    }
}
