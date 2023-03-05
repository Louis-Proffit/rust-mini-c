use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub struct DisplayableSet<'a, T>(pub &'a HashSet<T>);

impl<T: Display> Display for DisplayableSet<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for item in self.0 {
            write!(f, "{item},")?;
        }
        Ok(())
    }
}