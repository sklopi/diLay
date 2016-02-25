use std::cell::Cell;

pub struct Parameter{
    pub name        : String,
    pub label       : String,
    pub value       : Cell<f32>,
    pub automatable : bool
}

