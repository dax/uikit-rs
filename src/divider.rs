use super::margin::Margin;
use yew::{classes, function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct DividerProps {
    #[prop_or_default]
    pub margin: Vec<Margin>,
}

#[function_component(Divider)]
pub fn divider(DividerProps { margin }: &DividerProps) -> Html {
    html! {
        <hr class={classes!(margin.clone())} />
    }
}
