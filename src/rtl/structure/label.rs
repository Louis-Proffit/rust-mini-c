use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Mutex;
use crate::rtl::structure::Fresh;

static COUNTER: Mutex<u32> = Mutex::new(0);

pub type Label = Rc<_Label>;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct _Label {
    index: u32,
}

impl Fresh for Label {
    type Item = Label;

    fn fresh() -> Self::Item {
        let mut counter = COUNTER.lock().unwrap();
        let local_counter = counter.clone();

        *counter += 1;

        Rc::new(_Label { index: local_counter })
    }
}

impl Display for _Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "L{}", self.index)
    }
}
