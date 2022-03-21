use regex::Regex;
use yew::Classes;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Margin {
    Default,
    Top,
    Bottom,
    Left,
    Right,
    Small,
    SmallTop,
    SmallBottom,
    SmallLeft,
    SmallRight,
    Medium,
    MediumTop,
    MediumBottom,
    MediumLeft,
    MediumRight,
    Large,
    LargeTop,
    LargeBottom,
    LargeLeft,
    LargeRight,
    Xlarge,
    XlargeTop,
    XlargeBottom,
    XlargeLeft,
    XlargeRight,
    Remove,
    RemoveTop,
    RemoveBottom,
    RemoveLeft,
    RemoveRight,
    RemoveVertical,
    RemoveAdjacent,
    RemoveFirstChild,
    RemoveLastChild,
    RemoveLeft_s,
    RemoveRight_s,
    RemoveLeft_m,
    RemoveRight_m,
    RemoveLeft_l,
    RemoveRight_l,
    RemoveLeft_xl,
    RemoveRight_xl,
}

impl From<Margin> for Classes {
    fn from(margin: Margin) -> Self {
        let uppercase_re = Regex::new("(.)([A-Z])").unwrap();
        let class = match margin {
            Margin::Default => "uk-margin".to_string(),
            _ => uppercase_re
                .replace(&format!("uk-margin{:?}", margin), "$1-$2")
                .replace('_', "@")
                .to_lowercase(),
        };
        class.into()
    }
}
