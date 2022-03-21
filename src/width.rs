use yew::Classes;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Width {
    _1_1,
    _1_2,
    _1_3,
    _1_4,
    _1_5,
    _1_6,
    _2_3,
    _3_4,
    _4_5,
    _5_6,
    _Auto,
    _Expand,
    _Small,
    _Medium,
    _Large,
    _XLarge,
    _2XLarge,
}

impl From<Width> for Classes {
    fn from(width: Width) -> Self {
        format!("uk-width{:?}", width)
            .replace('_', "-")
            .to_lowercase()
            .into()
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ChildWidth {
    _1_1,
    _1_2,
    _1_3,
    _1_4,
    _1_5,
    _1_6,
    _Auto,
    _Expand,
}

impl From<ChildWidth> for Classes {
    fn from(width: ChildWidth) -> Self {
        format!("uk-child-width{:?}", width)
            .replace('_', "-")
            .to_lowercase()
            .into()
    }
}
