pub mod malloc {
    use std::collections::HashMap;
    use std::sync::Mutex;
    use crate::rtl::interpreter::context::Context;
    use crate::rtl::interpreter::{RtlInterpFun, RtlInterpreterResult, Value};
    use crate::rtl::structure::Fresh;
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

    impl<'a> RtlInterpFun<'a> for Malloc {
        fn fun_result(&self) -> &Register {
            &self.result
        }

        fn fun_arguments(&self) -> &Vec<Register> {
            &self.args
        }

        fn interp_fun(&self, context: &Context<'a>) -> RtlInterpreterResult<()> {
            let mut address_mutex = MALLOC_VALUE_INDEX.lock().expect("Lock failed");

            let address = *address_mutex;
            *address_mutex = address + 1;

            context.memory
                .borrow_mut()
                .insert(address, HashMap::new());


            context.put(&self.result, address);

            Ok(())
        }
    }
}

pub mod putchar {
    use crate::rtl::interpreter::context::Context;
    use crate::rtl::interpreter::error::RtlInterpreterError;
    use crate::rtl::interpreter::{RtlInterpFun, RtlInterpreterResult};
    use crate::rtl::structure::Fresh;
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

    impl<'a> RtlInterpFun<'a> for Putchar {
        fn fun_result(&self) -> &Register {
            &self.result
        }

        fn fun_arguments(&self) -> &Vec<Register> {
            &self.args
        }

        fn interp_fun(&self, context: &Context<'a>) -> RtlInterpreterResult<()> {
            let arg = self.args
                .first()
                .ok_or(RtlInterpreterError::Other("Pas d'argument pour la fonction putchar"))?;

            let val = context.get(arg);

            context.stdout.putchar(val as u8 as char);

            Ok(())
        }
    }
}
