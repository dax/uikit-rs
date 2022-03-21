use regex::Regex;
use yew::Classes;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Padding {
    Default,
    Small,
    Large,
    Remove,
    RemoveTop,
    RemoveBottom,
    RemoveLeft,
    RemoveRight,
    RemoveVertical,
    RemoveHorizontal,
}

impl From<Padding> for Classes {
    fn from(padding: Padding) -> Self {
        let uppercase_re = Regex::new("(.)([A-Z])").unwrap();
        let class = match padding {
            Padding::Default => "uk-padding".to_string(),
            _ => uppercase_re
                .replace(&format!("uk-padding{:?}", padding), "$1-$2")
                .to_lowercase(),
        };
        class.into()
    }
}
