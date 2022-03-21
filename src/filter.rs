use super::width::Width;
use super::UIKitComponent;
use yew::{classes, function_component, html, Children, Classes, Html, Properties};

#[derive(Debug, PartialEq)]
pub struct FilterData {
    pub class: String,
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct FilterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub target: String,
    #[prop_or_default]
    pub filter_width: Option<Width>,
    #[prop_or_default]
    pub filter_component: Option<UIKitComponent>,
    #[prop_or_default]
    pub filter_class: Classes,
    #[prop_or_default]
    pub filters: Vec<FilterData>,
}

#[function_component(Filter)]
pub fn filter(
    FilterProps {
        children,
        target,
        filter_width,
        filter_component,
        filter_class,
        filters,
    }: &FilterProps,
) -> Html {
    let filter = format!("target: {}", target);

    html! {
        <div uk-filter={filter}>
            <ul class={classes!(filter_component, filter_class.clone(), filter_width)}>
                {
                    filters.iter().map(|FilterData { class, label }|
                      html! {
                          <li uk-filter-control={class.clone()}><a href="#">{label}</a></li>
                      }
                    ).collect::<Html>()
                }
            </ul>
            { for children.iter() }
        </div>
    }
}
