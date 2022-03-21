use yew::{classes, function_component, html, Children, Classes, Properties};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SectionStyle {
    Default,
    Muted,
    Primary,
    Secondary,
}

impl From<SectionStyle> for Classes {
    fn from(style: SectionStyle) -> Self {
        format!("uk-section-{:?}", style).to_lowercase().into()
    }
}

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub style: Option<SectionStyle>,
}

#[function_component(Section)]
pub fn section(SectionProps { children, style }: &SectionProps) -> Html {
    html! {
        <div class={classes!("uk-section", style)}>
          { for children.iter() }
        </div>
    }
}
