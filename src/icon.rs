use super::{Margin, Text};
use regex::Regex;
use std::fmt;
use yew::{classes, function_component, html, Callback, MouseEvent, Properties};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum IconType {
    Check,
    FileEdit,
    Bookmark,
    TriangleLeft,
    TriangleRight,
    TriangleDown,
    TriangleUp,
    Plus,
}

impl fmt::Display for IconType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let uppercase_re = Regex::new("(.)([A-Z])").unwrap();
        write!(
            f,
            "{}",
            uppercase_re
                .replace(&format!("{:?}", self), "$1-$2")
                .to_lowercase()
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub icon_type: IconType,
    #[prop_or_default]
    pub text_style: Vec<Text>,
    #[prop_or_default]
    pub margin: Vec<Margin>,
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Icon)]
pub fn icon(
    IconProps {
        icon_type,
        text_style,
        margin,
        href,
        onclick,
    }: &IconProps,
) -> Html {
    if let Some(href) = href {
        html! {
            <a href={href.clone()}
               onclick={onclick}
               class={classes!(text_style.clone(), margin.clone())}
               uk-icon={format!("icon: {}", icon_type)} />
        }
    } else {
        html! {
            <span class={classes!(text_style.clone(), margin.clone())}
                  uk-icon={format!("icon: {}", icon_type)} />
        }
    }
}
