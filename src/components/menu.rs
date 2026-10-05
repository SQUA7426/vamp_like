use bevy::color::palettes::basic::BLACK;
use bevy::prelude::*;

#[derive(States, Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Pause,
}

#[derive(States, Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum ResetState {
    ResetBtn,
    ResetBtnColor,
    #[default]
    NonReset,
}

#[derive(States, Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum MenuState {
    Inactive,
    StartMenu,
    #[default]
    Menu,
    Shop,
    Inventory,
    Home,
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
            .init_state::<MenuState>()
            .init_state::<ResetState>()
            .add_plugins(gamemenu::GameMenuPlugin)
            .add_plugins(inventorymenu::InventoryMenuPlugin)
            .add_plugins(home::HomePlugin);
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
            MENU_BTN_WIDTH, MenuState, ResetState,
        },
        create_btn, create_btn_text, create_node,
    };

    #[derive(Component, Eq, PartialEq)]
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
        Inactive,
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
            app.add_systems(OnEnter(GameState::Menu), setup_gamemenu)
                .add_systems(OnEnter(ResetState::ResetBtn), reset_btns)
                .add_systems(OnEnter(ResetState::ResetBtnColor), reset_btns_colors)
                .add_systems(Update, (button_system, button_action).chain());
        }
    }

    fn setup_gamemenu(
        mut cmds: Commands,
        mut reset_state: ResMut<NextState<ResetState>>,
        mut menu_state: ResMut<NextState<MenuState>>,
    ) {
        let gamemenu_node = create_node!(5.0, 5.0, 85.0, Some(5.0), 90.0, 10.0);

        cmds.spawn((
            DespawnOnExit(GameState::Menu),
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

        menu_state.set(MenuState::Home);
        reset_state.set(ResetState::ResetBtn);
    }

    fn reset_btns(
        mut btn_query: Query<(&MenuBtn, &mut MenuBtnActive), With<Button>>,
        mut reset_state: ResMut<NextState<ResetState>>,
    ) {
        for (_menu_btn, mut active) in &mut btn_query {
            *active = MenuBtnActive::Inactive;
        }

        reset_state.set(ResetState::NonReset);
    }

    fn reset_btns_colors(
        mut btn_query: Query<(&MenuBtn, &mut BackgroundColor, &Children), With<Button>>,
        mut text_query: Query<&mut TextColor>,
        mut reset_state: ResMut<NextState<ResetState>>,
    ) {
        for (_menu_btn, mut bg_color, children) in &mut btn_query {
            let mut text_color = text_query.get_mut(children[0]).unwrap();
            *text_color = TextColor(NORMAL_TEXT);
            *bg_color = BackgroundColor(NORMAL_BG);
        }
        reset_state.set(ResetState::NonReset);
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
                &MenuBtn,
                &mut BackgroundColor,
                &mut BorderColor,
                &mut MenuBtnActive,
                &Children,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        mut text_query: Query<&mut TextColor>,
        mut reset_state: ResMut<NextState<ResetState>>,
    ) {
        for (interaction, _menu_btn, mut bg_color, _border_color, mut active, children) in
            &mut btn_query
        {
            let mut text_color = text_query.get_mut(children[0]).unwrap();

            if *active == MenuBtnActive::Inactive {
                if *interaction == Interaction::Pressed {
                    reset_state.set(ResetState::ResetBtnColor);
                    *active = MenuBtnActive::Active;

                    *text_color = TextColor(PRESSED_TEXT);
                    *bg_color = BackgroundColor(PRESSED_BG);
                }
                if *interaction == Interaction::Hovered {
                    *text_color = TextColor(HOVER_TEXT);
                    *bg_color = BackgroundColor(HOVER_BG);
                }
                if *interaction == Interaction::None {
                    *text_color = TextColor(NORMAL_TEXT);
                    *bg_color = BackgroundColor(NORMAL_BG);
                }
            } else {
                if *interaction == Interaction::Pressed {
                    *active = MenuBtnActive::default();
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
        mut btn_query: Query<
            (&Interaction, &MenuBtn, &mut MenuBtnActive),
            (Changed<Interaction>, With<Button>),
        >,
        mut reset_state: ResMut<NextState<ResetState>>,
        mut menu_state: ResMut<NextState<MenuState>>,
    ) {
        for (interaction, btn, active) in &mut btn_query {
            if *interaction == Interaction::Pressed {
                [
                    (MenuBtn::Shop, MenuState::Shop),
                    (MenuBtn::Inventory, MenuState::Inventory),
                    (MenuBtn::Home, MenuState::Home),
                    (MenuBtn::Upgrade, MenuState::Upgrade),
                    (MenuBtn::Level, MenuState::Level),
                ]
                .into_iter()
                .for_each(|(b, a)| {
                    if *btn == b && *active == MenuBtnActive::Active {
                        reset_state.set(ResetState::ResetBtn);
                        reset_state.set(ResetState::ResetBtnColor);
                        menu_state.set(a);
                    }
                });
            }

            if *interaction == Interaction::Hovered {
                match btn {
                    _ => {}
                }
            }

            if *interaction == Interaction::None {
                menu_state.reset();
            }
        }
    }
}

mod home {
    use bevy::color::palettes::{
        basic::{BLACK, GRAY},
        css::{CORAL, DARK_BLUE, DARK_GREY, LIGHT_GREY},
    };
    use bevy::prelude::*;

    use crate::{
        components::menu::{GameState, MenuState},
        create_btn, create_btn_text, create_node,
    };
    const NORMAL_TEXT: Color = Color::Srgba(BLACK);
    const NORMAL_BG: Color = Color::Srgba(LIGHT_GREY);
    const HOVER_TEXT: Color = Color::Srgba(DARK_BLUE);
    const HOVER_BG: Color = Color::Srgba(DARK_GREY);
    const PRESSED_TEXT: Color = Color::Srgba(CORAL);
    const PRESSED_BG: Color = Color::Srgba(GRAY);

    #[derive(Debug)]
    pub struct HomePlugin;

    impl Plugin for HomePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(MenuState::Home), setup_home)
                .add_systems(Update, (playing_system, playing_action).chain());
        }
    }

    #[derive(Component, Debug)]
    pub enum HomeBtn {
        Profile,
        Option,
        Play,
    }

    fn button(t: String) -> impl Bundle {
        const H: f32 = 100.0;
        const Z: f32 = 0.0;
        const TWENTY: f32 = 20.0;
        (
            Button,
            create_btn!(H, H, Z, TWENTY),
            // BorderColor::all(MENU_BTN_OUTLINE),
            BackgroundColor(NORMAL_BG),
            children![(create_btn_text!(t))],
        )
    }

    fn setup_home(mut cmds: Commands) {
        let home_node = create_node!(5.0, 5.0, 5.0, Some(35.0), 90.0, 60.0);

        cmds.spawn((
            DespawnOnExit(MenuState::Home),
            home_node,
            // BackgroundColor(Color::srgba(0.0, 1.0, 0.0, 0.6)),
            Children::spawn(SpawnIter(
                [
                    (
                        HomeBtn::Profile,
                        "PROF",
                        0.0,
                        90.0,
                        0.0,
                        Some(90.0),
                        10.0,
                        10.0,
                    ),
                    (
                        HomeBtn::Option,
                        "OPT",
                        90.0,
                        00.0,
                        0.0,
                        Some(90.0),
                        10.0,
                        10.0,
                    ),
                    (
                        HomeBtn::Play,
                        "PLAY",
                        40.0,
                        40.0,
                        80.0,
                        Some(0.0),
                        20.0,
                        20.0,
                    ),
                ]
                .into_iter()
                .map(|(btn, text, l, r, t, b, w, h)| {
                    (
                        create_node!(l, r, t, b, w, h),
                        children![(button(String::from(text)), btn)],
                    )
                }),
            )),
        ));
    }

    #[allow(clippy::type_complexity)]
    fn playing_system(
        mut btn_query: Query<
            (&Interaction, &mut BackgroundColor, &HomeBtn, &Children),
            (Changed<Interaction>, With<Button>),
        >,
        mut text_query: Query<&mut TextColor>,
    ) {
        for (interaction, mut bg_color, _btn, children) in &mut btn_query {
            let mut text_color = text_query.get_mut(children[0]).unwrap();

            if *interaction == Interaction::Pressed {
                *text_color = TextColor(PRESSED_TEXT);
                *bg_color = BackgroundColor(PRESSED_BG);
            }
            if *interaction == Interaction::Hovered {
                *text_color = TextColor(HOVER_TEXT);
                *bg_color = BackgroundColor(HOVER_BG);
            }
            if *interaction == Interaction::None {
                *text_color = TextColor(NORMAL_TEXT);
                *bg_color = BackgroundColor(NORMAL_BG);
            }
        }
    }

    #[allow(clippy::match_single_binding, clippy::type_complexity)]
    fn playing_action(
        mut btn_query: Query<(&Interaction, &HomeBtn), (Changed<Interaction>, With<Button>)>,
        mut game_state: ResMut<NextState<GameState>>,
        mut menu_state: ResMut<NextState<MenuState>>,
    ) {
        for (interaction, btn) in &mut btn_query {
            if *interaction == Interaction::Pressed
                && let HomeBtn::Play = btn
            {
                game_state.set(GameState::Playing);
                menu_state.set(MenuState::Inactive);
            }

            if *interaction == Interaction::Hovered {
                match btn {
                    _ => {}
                }
            }

            if *interaction == Interaction::None {
                menu_state.reset();
            }
        }
    }
}

mod inventorymenu {
    use bevy::{
        color::palettes::css::{DARK_GREY, LIGHT_GREY},
        prelude::*,
    };

    use crate::{components::menu::MenuState, create_node};

    #[derive(Debug)]
    pub struct InventoryMenuPlugin;

    impl Plugin for InventoryMenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(MenuState::Inventory), setup_inventory_menu);
        }
    }

    fn setup_inventory_menu(mut cmds: Commands) {
        let inventorymenu_node = create_node!(15.0, 15.0, 10.0, Some(20.0), 70.0, 70.0);
        cmds.spawn((
            DespawnOnExit(MenuState::Inventory),
            inventorymenu_node,
            // BackgroundColor(Color::srgba(0.0, 1.0, 0.0, 0.6)),
            BackgroundColor(Color::from(LIGHT_GREY)),
            Children::spawn(SpawnIter(
                [
                    (25.0, 68.0, 5.0, Some(88.0), 7.0, 7.0),
                    (15.0, 78.0, 15.0, Some(78.0), 7.0, 7.0),
                    (15.0, 78.0, 25.0, Some(68.0), 7.0, 7.0),
                    (25.0, 68.0, 35.0, Some(58.0), 7.0, 7.0),
                    (68.0, 25.0, 5.0, Some(88.0), 7.0, 7.0),
                    (78.0, 15.0, 15.0, Some(78.0), 7.0, 7.0),
                    (78.0, 15.0, 25.0, Some(68.0), 7.0, 7.0),
                    (68.0, 25.0, 35.0, Some(58.0), 7.0, 7.0),
                ]
                .into_iter()
                .map(|(l, r, t, b, w, h)| {
                    (
                        create_node!(l, r, t, b, w, h),
                        Button,
                        BackgroundColor(Color::from(DARK_GREY)),
                    )
                }),
            )),
        ));
    }
}
