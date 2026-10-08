

#[derive(Default, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub enum Size {
    Gigantic,
    Large,
    #[default]
    Normal,
    Little,
    Small,
    Tiny,
}
