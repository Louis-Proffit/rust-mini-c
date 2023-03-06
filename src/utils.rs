use std::collections::{HashSet};
use std::fmt::{Display, Formatter};


pub struct DisplayableVec<'a, T>(pub &'a Vec<T>);

pub struct DisplayableSet<'a, T>(pub &'a HashSet<T>);

impl<T: Display> Display for DisplayableSet<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for item in self.0 {
            write!(f, "{item},")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}


impl<T: Display> Display for DisplayableVec<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for item in self.0 {
            write!(f, "{item},")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}