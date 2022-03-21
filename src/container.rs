use yew::{classes, function_component, html, Children, Classes, Properties};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ContainerSize {
    XSmall,
    Small,
    Large,
    XLarge,
    Expand,
}

impl From<ContainerSize> for Classes {
    fn from(size: ContainerSize) -> Self {
        format!("uk-container-{:?}", size).to_lowercase().into()
    }
}

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub size: Option<ContainerSize>,
}

#[function_component(Container)]
pub fn container(ContainerProps { children, size }: &ContainerProps) -> Html {
    html! {
        <div class={classes!("uk-container", size)}>
          { for children.iter() }
        </div>
    }
}
