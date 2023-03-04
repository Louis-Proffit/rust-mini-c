use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Mutex;
use crate::rtl::structure::Fresh;

static COUNTER: Mutex<u32> = Mutex::new(1);

pub type Register = Rc<_Register>;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct _Register {
    index: u32,
}

impl Fresh for Register {
    type Item = Register;

    fn fresh() -> Self::Item {
        let mut counter = COUNTER.lock().unwrap();
        let local_counter = counter.clone();

        *counter += 1;

        Rc::new(_Register { index: local_counter })
    }
}

impl Display for _Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.index)
    }
}
