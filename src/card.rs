use yew::{classes, function_component, html, Children, Classes, Properties};

use super::{Margin, Padding, Text, Width};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CardSize {
    Small,
}

impl From<CardSize> for Classes {
    fn from(card_size: CardSize) -> Self {
        format!("uk-card-{:?}", card_size).to_lowercase().into()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CardStyle {
    Default,
    Primary,
    Secondary,
}

impl From<CardStyle> for Classes {
    fn from(card_style: CardStyle) -> Self {
        format!("uk-card-{:?}", card_style).to_lowercase().into()
    }
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children, // TODO: enum typed
    #[prop_or_default]
    pub size: Option<CardSize>,
    #[prop_or_default]
    pub style: Option<CardStyle>,
    #[prop_or_default]
    pub hover: bool,
    #[prop_or_default]
    pub width: Option<Width>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Card)]
pub fn card(
    CardProps {
        children,
        size,
        style,
        hover,
        width,
        class,
    }: &CardProps,
) -> Html {
    html! {
        <div class={classes!("uk-card", style, size, hover.then(|| "uk-card-hover"), width, class.clone())}>
          { for children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub padding: Vec<Padding>,
    #[prop_or_default]
    pub margin: Vec<Margin>,
}

#[function_component(CardBody)]
pub fn card_body(
    CardBodyProps {
        children,
        padding,
        margin,
    }: &CardBodyProps,
) -> Html {
    html! {
        <div class={classes!("uk-card-body", padding.clone(), margin.clone())}>
        { for children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardTitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub text_style: Vec<Text>,
}

#[function_component(CardTitle)]
pub fn card_title(
    CardTitleProps {
        children,
        text_style,
    }: &CardTitleProps,
) -> Html {
    html! {
        <h3 class={classes!("uk-card-title", text_style.clone())}>
        { for children.iter() }
        </h3>
    }
}
