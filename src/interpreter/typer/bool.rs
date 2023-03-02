use crate::interpreter::typer::Value;

pub trait Bool {
    fn bool(&self) -> bool;
}

impl Bool for Value {
    fn bool(&self) -> bool {
        *self != 0
    }
}

pub trait ToCBool {
    fn to_c_bool(&self) -> Value;
}

impl ToCBool for bool {
    fn to_c_bool(&self) -> Value {
        if *self { 1 } else { 0 }
    }
}
