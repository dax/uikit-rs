use yew::{classes, function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct IconNavProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(IconNav)]
pub fn icon_nav(IconNavProps { children }: &IconNavProps) -> Html {
    html! {
        <div class={classes!("uk-iconnav")}>
          { for children.iter() }
        </div>
    }
}
