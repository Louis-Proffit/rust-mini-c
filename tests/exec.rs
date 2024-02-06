#![feature(exit_status_error)]

use std::fs::{File, read_to_string};
use std::process::Command;
use std::io::Write;

use rust_mini_c::parser::parse_file;

macro_rules! test_bad {
    ($($name:ident: $path:literal,)*) => {
        $(
        // #[serial]
        #[test]
        #[should_panic]
        fn $name() {
            _test_bad($path);
        }
        )*

    };
}

macro_rules! test_good {
    ($($name:ident: $path:literal,)*) => {
        $(
        // #[serial]
        #[test]
        fn $name() {
            _test_good($path);
        }
        )*

    };
}


fn _test_bad(path: &str) {
    println!("File {}", path);

    let file = read_to_string(path).expect("Failed to read file");
    parse_file(&file)
        .expect("Failed to parse file")
        .minic_typ()
        .expect("Failed to typ file")
        .minic_rtl()
        .expect("Failed to rtl file")
        .minic_interp()
        .expect("Failed to interp");
}

fn _test_good(base_path: &str) {
    println!("File {}", base_path);

    let file = read_to_string(format!("{}.c", base_path)).expect("Failed to read source file");
    let expected = read_to_string(format!("{}.out", base_path)).expect("Failed to read expected file");

    let parsed = parse_file(&file)
        .expect("Failed to parse file");

    let typed = parsed.minic_typ()
        .expect("Failed to typ file");

    let rtl = typed.minic_rtl()
        .map(|file| {
            println!("{}", file);
            file
        })
        .expect("Failed to rtl file");

    let ertl = rtl.minic_ertl()
        .map(|file| {
            println!("{}", file);
            file
        })
        .expect("Failed to ertl file");


    let ltl = ertl.minic_ltl()
        .map(|file| {
            println!("{}", file);
            file
        })
        .expect("Failed to ltl file");

    let x86 = ltl.minic_linearise()
        .expect("Failed to linearise file");

    let mut tmp = File::create(format!("{}.s", base_path)).expect("Failed to create assembly file");
    writeln!(&mut tmp, "{}", x86).expect("Failed to write to assembly");

    let exec = Command::new("gcc")
        .arg(format!("{}.s", base_path))
        .arg("-o")
        .arg(format!("{}.o", base_path))
        .output()
        .expect("Failed to start compilation");

    match exec.status.exit_ok() {
        Ok(_) => {}
        Err(_) => {
            panic!("Compilation failed : {}", String::from_utf8_lossy(&exec.stderr));
        }
    }

    let exec = Command::new(format!("{}.o", base_path))
        .output()
        .expect("Failed to execute");

    assert_eq!(expected, String::from_utf8_lossy(&exec.stdout))
}


mod good {
    use crate::_test_good;
    // use serial_test::serial;

    // for_1: "tests/source/exec/for1.c" -> "tests/source/exec/for1.out",
    // mandelbrot: "tests/source/exec/mandelbrot.c" -> "tests/source/exec/mandelbrot.out",
    // pascal: "tests/source/exec/pascal.c" -> "tests/source/exec/pascal.out",

    test_good!(
        fact_imp: "tests/source/exec/fact_imp",
        fact_rec: "tests/source/exec/fact_rec",
        abr: "tests/source/exec/abr",
        and_1: "tests/source/exec/and1",
        and_2: "tests/source/exec/and2",
        exec_arith_1: "tests/source/exec/arith1",
        exec_assign_1: "tests/source/exec/assign1",
        exec_assign_2: "tests/source/exec/assign2",
        exec_assign_3: "tests/source/exec/assign3",
        exec_assign_6: "tests/source/exec/assign6",
        exec_assign_7: "tests/source/exec/assign7",
        block_1 : "tests/source/exec/block1",
        block_2: "tests/source/exec/block2",
        exec_call_1: "tests/source/exec/call1",
        exec_call_2: "tests/source/exec/call2",
        char_1: "tests/source/exec/char1",
        char_2: "tests/source/exec/char2",
        char_3: "tests/source/exec/char3",
        char_4: "tests/source/exec/char4",
        comp_1: "tests/source/exec/comp1",
        dllist: "tests/source/exec/dllist",
        fact: "tests/source/exec/fact",
        field_2: "tests/source/exec/field2",
        field_4: "tests/source/exec/field4",
        field_7: "tests/source/exec/field7",
        field_8: "tests/source/exec/field8",
        for_2: "tests/source/exec/for2",
        for_3: "tests/source/exec/for3",
        for_4: "tests/source/exec/for4",
        for_5: "tests/source/exec/for5",
        for_6: "tests/source/exec/for6",
        hello_world: "tests/source/exec/hello_world",
        if_1: "tests/source/exec/if1",
        init_1: "tests/source/exec/init1",
        josephus: "tests/source/exec/josephus",
        lazy_1: "tests/source/exec/lazy1",
        local_1: "tests/source/exec/local1",
        local_2: "tests/source/exec/local2",
        local_3: "tests/source/exec/local3",
        many: "tests/source/exec/many",
        not_1: "tests/source/exec/not1",
        not_2: "tests/source/exec/not2",
        or_1: "tests/source/exec/or1",
        print_int: "tests/source/exec/print_int",
        putchar_octal: "tests/source/exec/putchar-octal1",
        putchar: "tests/source/exec/putchar1",
        putchar_hexa: "tests/source/exec/putchar_hexa1",
        return_1: "tests/source/exec/return1",
        return_2: "tests/source/exec/return2",
        shadow_1: "tests/source/exec/shadow1",
        sizeof_2: "tests/source/exec/sizeof2",
        spilled_1: "tests/source/exec/spilled1",
        exec_uminus_1: "tests/source/exec/uminus1",
        while_1: "tests/source/exec/while1",
        while_2: "tests/source/exec/while2",
        while_3: "tests/source/exec/while3",
        while_4: "tests/source/exec/while4",
    );
}

mod bad {
    use crate::_test_bad;
    // use serial_test::serial;

    test_bad!(
        deref_null: "tests/source/exec-fail/deref_null.c",
        division_by_zero: "tests/source/exec-fail/division_by_zero1.c",
    );
}
