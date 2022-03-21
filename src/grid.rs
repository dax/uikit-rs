use super::{ChildWidth, FlexVerticalAlignement, Margin, Text};
use yew::{classes, function_component, html, Children, Classes, Properties};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GridGapSize {
    Small,
    Medium,
    Large,
    Collapse,
}

impl From<GridGapSize> for Classes {
    fn from(gap_size: GridGapSize) -> Self {
        format!("uk-grid-{:?}", gap_size).to_lowercase().into()
    }
}

#[derive(Properties, PartialEq)]
pub struct GridProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub gap_size: Option<GridGapSize>,
    #[prop_or_default]
    pub margin: Vec<Margin>,
    #[prop_or_default]
    pub height_match: bool,
    #[prop_or_default]
    pub vertical_alignement: Option<FlexVerticalAlignement>,
    #[prop_or_default]
    pub child_width: Option<ChildWidth>,
    #[prop_or_default]
    pub text_style: Vec<Text>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Grid)]
pub fn grid(
    GridProps {
        children,
        gap_size,
        margin,
        height_match,
        vertical_alignement,
        child_width,
        text_style,
        class,
    }: &GridProps,
) -> Html {
    html! {
        <div class={classes!(
            class.clone(),
            gap_size,
            margin.clone(),
            vertical_alignement,
            child_width,
            text_style.clone(),
            height_match.then(|| "uk-grid-match")
        )} uk-grid="">
          { for children.iter() }
        </div>
    }
}
