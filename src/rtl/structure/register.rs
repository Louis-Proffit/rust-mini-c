use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Mutex;
use crate::rtl::structure::Fresh;

static COUNTER: Mutex<u32> = Mutex::new(1);

pub type PseudoRegister = Rc<_PseudoRegister>;

#[derive(Eq, PartialEq, Hash, Debug,Ord, PartialOrd)]
pub struct _PseudoRegister {
    index: u32,
}

impl Fresh for PseudoRegister {
    type Item = PseudoRegister;

    fn fresh() -> Self::Item {
        let mut counter = COUNTER.lock().unwrap();
        let local_counter = counter.clone();

        *counter += 1;

        Rc::new(_PseudoRegister { index: local_counter })
    }
}

impl Display for _PseudoRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.index)
    }
}
