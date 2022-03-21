use yew::Classes;

pub mod card;
pub mod container;
pub mod divider;
pub mod filter;
pub mod flex;
pub mod grid;
pub mod icon;
pub mod iconnav;
pub mod link;
pub mod margin;
pub mod padding;
pub mod section;
pub mod text;
pub mod width;

pub use card::*;
pub use container::*;
pub use divider::*;
pub use filter::*;
pub use flex::*;
pub use grid::*;
pub use icon::*;
pub use iconnav::*;
pub use link::*;
pub use margin::*;
pub use padding::*;
pub use section::*;
pub use text::*;
pub use width::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum UIKitComponent {
    SubNav,
}

impl From<UIKitComponent> for Classes {
    fn from(component: UIKitComponent) -> Self {
        format!("uk-{:?}", component).to_lowercase().into()
    }
}
