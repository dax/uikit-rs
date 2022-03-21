use regex::Regex;
use yew::Classes;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Text {
    Lead,
    Meta,
    Small,
    Default,
    Large,
    Light,
    Normal,
    Bold,
    Lighter,
    Bolder,
    Italic,
    Capitalize,
    Uppercase,
    LowerCase,
    DecorationNone,
    Muted,
    Emphasis,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Background,
    Left,
    Right,
    Center,
    Justify,
    Top,
    Middle,
    Bottom,
    Baseline,
    Truncate,
    Break,
    Nowrap,
    Left_s,
    Center_s,
    Right_s,
    Left_m,
    Center_m,
    Right_m,
    Left_l,
    Center_l,
    Right_l,
    Left_xl,
    Center_xl,
    Right_xl,
}

impl From<Text> for Classes {
    fn from(text: Text) -> Self {
        let uppercase_re = Regex::new("(.)([A-Z])").unwrap();
        uppercase_re
            .replace(&format!("uk-text{:?}", text), "$1-$2")
            .replace('_', "@")
            .to_lowercase()
            .into()
    }
}
