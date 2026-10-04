#[macro_export]
macro_rules! create_node {
    ($l: expr, $r: expr, $t: expr, $b: expr, $w: expr, $h: expr) => {
        Node {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            left: percent($l),
            right: percent($r),
            top: percent($t),
            bottom: match $b {
                Some(bot) => percent(bot),
                None => Val::Auto,
            },
            width: percent($w),
            height: percent($h),
            ..default()
        }
    };
}

#[macro_export]
macro_rules! create_btn {
    ($h: ident, $w: ident, $m: ident, $radius: ident) => {
        Node {
            height: percent($h),
            width: percent($w),
            margin: UiRect::all(percent($m)),
            border_radius: BorderRadius::all(px($radius)),
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            justify_items: JustifyItems::Center,
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    };
}

#[macro_export]
macro_rules! create_btn_text {
    ($text: expr) => {
        (
            Text::new($text),
            TextFont {
                font_size: 22.,
                ..default()
            },
            TextColor(Color::srgba(50., 50., 50., 0.75)),
        )
    };
}
