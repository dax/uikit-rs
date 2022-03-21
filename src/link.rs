use yew::{function_component, html, Callback, Children, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Link)]
pub fn link(
    LinkProps {
        children,
        href,
        onclick,
    }: &LinkProps,
) -> Html {
    html! {
        <a href={href.clone()} onclick={onclick}>
        { for children.iter() }
        </a>
    }
}
