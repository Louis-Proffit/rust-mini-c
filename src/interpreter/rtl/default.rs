pub mod malloc {
    use std::collections::HashMap;
    use std::sync::Mutex;
    use crate::interpreter::rtl::{RtlInterpFun, RtlInterpreterResult, Value};
    use crate::interpreter::rtl::context::Context;
    use crate::interpreter::rtl::error::RtlInterpreterError;
    use crate::rtl::structure::Fresh;
    use crate::rtl::structure::label::Label;
    use crate::rtl::structure::register::Register;

    static MALLOC_VALUE_INDEX: Mutex<Value> = Mutex::new(1);

    pub struct Malloc {
        result: Register,
        args: Vec<Register>,
    }

    impl Malloc {
        pub fn new() -> Malloc {
            Malloc { result: Register::fresh(), args: vec![Register::fresh()] }
        }
    }

    impl RtlInterpFun for Malloc {
        fn interp_label(&self, _context: &Context, label: &Label) -> RtlInterpreterResult<()> {
            Err(RtlInterpreterError::NoInstructionForDefaultFunction(label.clone()))
        }

        fn fun_result(&self) -> &Register {
            &self.result
        }

        fn fun_arguments(&self) -> &Vec<Register> {
            &self.args
        }

        fn interp_fun(&self, context: &Context) -> RtlInterpreterResult<()> {
            let mut address_mutex = MALLOC_VALUE_INDEX.lock().unwrap();

            let address = *address_mutex;
            *address_mutex = address + 1;

            context
                .memory()
                .borrow_mut()
                .insert(address, HashMap::new());


            context.put(&self.result, address);

            Ok(())
        }
    }
}

pub mod putchar {
    use crate::interpreter::rtl::context::Context;
    use crate::interpreter::rtl::{RtlInterpFun, RtlInterpreterResult};
    use crate::interpreter::rtl::error::RtlInterpreterError;
    use crate::rtl::structure::Fresh;
    use crate::rtl::structure::label::Label;
    use crate::rtl::structure::register::Register;

    pub struct Putchar {
        result: Register,
        args: Vec<Register>,
    }

    impl Putchar {
        pub fn new() -> Putchar {
            Putchar { result: Register::fresh(), args: vec![Register::fresh()] }
        }
    }

    impl RtlInterpFun for Putchar {
        fn interp_label(&self, _context: &Context, label: &Label) -> RtlInterpreterResult<()> {
            Err(RtlInterpreterError::NoInstructionForDefaultFunction(label.clone()))
        }

        fn fun_result(&self) -> &Register {
            &self.result
        }

        fn fun_arguments(&self) -> &Vec<Register> {
            &self.args
        }

        fn interp_fun(&self, context: &Context) -> RtlInterpreterResult<()> {
            let arg = self.args
                .first()
                .ok_or(RtlInterpreterError::Other("Pas d'argument pour la fonction putchar"))?;

            let val = context.get(arg);
            context.stdout().putchar(val as u8);

            Ok(())
        }
    }
}
