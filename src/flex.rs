use super::Width;
use yew::{classes, function_component, html, Callback, Children, Classes, MouseEvent, Properties};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FlexVerticalAlignement {
    Stretch,
    Top,
    Middle,
    Bottom,
}

impl From<FlexVerticalAlignement> for Classes {
    fn from(vertical_alignement: FlexVerticalAlignement) -> Self {
        format!("uk-flex-{:?}", vertical_alignement)
            .to_lowercase()
            .into()
    }
}

#[derive(Properties, PartialEq)]
pub struct FlexProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub vertical_alignement: Option<FlexVerticalAlignement>,
    #[prop_or_default]
    pub width: Option<Width>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Flex)]
pub fn container(
    FlexProps {
        children,
        vertical_alignement,
        width,
        onclick,
    }: &FlexProps,
) -> Html {
    html! {
        <div class={classes!("uk-flex", vertical_alignement, width)} onclick={onclick}>
        { for children.iter() }
        </div>
    }
}
