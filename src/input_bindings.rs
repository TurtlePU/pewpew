use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum AxisBinding {
    Horizontal,
    Vertical,
}

impl fmt::Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum ActionBinding {
    // TODO
}

impl fmt::Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct InputBindings;
impl BindingTypes for InputBindings {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}