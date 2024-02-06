#![feature(assert_matches)]

use std::fs::read_to_string;

use rust_mini_c::parser::parse_file;

macro_rules! test_typing_bad {
    ($($name:ident: $path:literal,)*) => {
        $(
        #[test]
        fn $name() {
            _test_typing_bad($path);
        }
        )*

    };
}

macro_rules! test_typing_good {
    ($($name:ident: $path:literal,)*) => {
        $(
        #[test]
        fn $name() {
            _test_typing_good($path);
        }
        )*

    };
}

fn _test_typing_bad(path: &str) {
    println!("File {}", path);

    let file = read_to_string(path).expect("Failed to read file");
    match parse_file(&file)
        .expect("Failed to parse")
        .minic_typ() {
        Ok(file) => {
            println!("{:?}", file);
            assert!(false);
        }
        Err(_) => {}
    }
}

fn _test_typing_good(path: &str) {
    println!("File {}", path);

    let file = read_to_string(path).expect("Failed to read file");
    match parse_file(&file)
        .expect("Failed to parse")
        .minic_typ() {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", err);
            assert!(false);
        }
    }
}


mod good {
    use crate::_test_typing_good;

    test_typing_good!(
    recursive_1: "tests/source/typing/good/testfile-recursive-1.c",
    scope_1: "tests/source/typing/good/testfile-scope-1.c",
    subtype_1: "tests/source/typing/good/testfile-subtype-1.c",
    subtype_2: "tests/source/typing/good/testfile-subtype-2.c",
    unique_1: "tests/source/typing/good/testfile-unique-1.c",
    deref_null: "tests/source/exec-fail/deref_null.c",
    division_by_zero: "tests/source/exec-fail/division_by_zero1.c",
    abr: "tests/source/exec/abr.c",
    and_1: "tests/source/exec/and1.c",
    and_2: "tests/source/exec/and2.c",
    exec_arith_1: "tests/source/exec/arith1.c",
    exec_assign_1: "tests/source/exec/assign1.c",
    exec_assign_2: "tests/source/exec/assign2.c",
    exec_assign_3: "tests/source/exec/assign3.c",
    exec_assign_6: "tests/source/exec/assign6.c",
    exec_assign_7: "tests/source/exec/assign7.c",
    block_1 : "tests/source/exec/block1.c",
    block_2: "tests/source/exec/block2.c",
    exec_call_1: "tests/source/exec/call1.c",
    exec_call_2: "tests/source/exec/call2.c",
    char_1: "tests/source/exec/char1.c",
    char_2: "tests/source/exec/char2.c",
    char_3: "tests/source/exec/char3.c",
    char_4: "tests/source/exec/char4.c",
    comp_1: "tests/source/exec/comp1.c",
    dllist: "tests/source/exec/dllist.c",
    fact: "tests/source/exec/fact.c",
    fact_imp: "tests/source/exec/fact_imp.c",
    fact_rec: "tests/source/exec/fact_rec.c",
    field_2: "tests/source/exec/field2.c",
    field_4: "tests/source/exec/field4.c",
    field_7: "tests/source/exec/field7.c",
    field_8: "tests/source/exec/field8.c",
    for_1: "tests/source/exec/for1.c",
    for_2: "tests/source/exec/for2.c",
    for_3: "tests/source/exec/for3.c",
    for_4: "tests/source/exec/for4.c",
    for_5: "tests/source/exec/for5.c",
    for_6: "tests/source/exec/for6.c",
    hello_world: "tests/source/exec/hello_world.c",
    if_1: "tests/source/exec/if1.c",
    init_1: "tests/source/exec/init1.c",
    josephus: "tests/source/exec/josephus.c",
    lazy_1: "tests/source/exec/lazy1.c",
    local_1: "tests/source/exec/local1.c",
    local_2: "tests/source/exec/local2.c",
    local_3: "tests/source/exec/local3.c",
    mandelbrot: "tests/source/exec/mandelbrot.c",
    many: "tests/source/exec/many.c",
    not_1: "tests/source/exec/not1.c",
    not_2: "tests/source/exec/not2.c",
    or_1: "tests/source/exec/or1.c",
    pascal: "tests/source/exec/pascal.c",
    print_int: "tests/source/exec/print_int.c",
    putchar_octal: "tests/source/exec/putchar-octal1.c",
    putchar: "tests/source/exec/putchar1.c",
    putchar_hexa: "tests/source/exec/putchar_hexa1.c",
    return_1: "tests/source/exec/return1.c",
    return_2: "tests/source/exec/return2.c",
    shadow_1: "tests/source/exec/shadow1.c",
    sizeof_2: "tests/source/exec/sizeof2.c",
    spilled_1: "tests/source/exec/spilled1.c",
    exec_uminus_1: "tests/source/exec/uminus1.c",
    while_1: "tests/source/exec/while1.c",
    while_2: "tests/source/exec/while2.c",
    while_3: "tests/source/exec/while3.c",
    while_4: "tests/source/exec/while4.c",
    );
}

mod bad {
    use crate::_test_typing_bad;

    test_typing_bad!(
    arith_1: "tests/source/typing/bad/testfile-arith-1.c",
    arith_2: "tests/source/typing/bad/testfile-arith-2.c",
    arith_3: "tests/source/typing/bad/testfile-arith-3.c",
    arrow_1: "tests/source/typing/bad/testfile-arrow-1.c",
    arrow_2: "tests/source/typing/bad/testfile-arrow-2.c",
    arrow_3: "tests/source/typing/bad/testfile-arrow-3.c",
    arrow_4: "tests/source/typing/bad/testfile-arrow-4.c",
    call_1: "tests/source/typing/bad/testfile-call-1.c",
    call_2: "tests/source/typing/bad/testfile-call-2.c",
    missing_main_1: "tests/source/typing/bad/testfile-missing_main-1.c",
    missing_main_2: "tests/source/typing/bad/testfile-missing_main-2.c",
    redef_1: "tests/source/typing/bad/testfile-redef-1.c",
    redef_2: "tests/source/typing/bad/testfile-redef-2.c",
    redef_3: "tests/source/typing/bad/testfile-redef-3.c",
    redef_4: "tests/source/typing/bad/testfile-redef-4.c",
    redef_5: "tests/source/typing/bad/testfile-redef-5.c",
    redef_6: "tests/source/typing/bad/testfile-redef-6.c",
    redef_7: "tests/source/typing/bad/testfile-redef-7.c",
    redef_8: "tests/source/typing/bad/testfile-redef-8.c",
    typing_scope_1: "tests/source/typing/bad/testfile-scope-1.c",
    typing_scope_2: "tests/source/typing/bad/testfile-scope-2.c",
    typing_scope_3: "tests/source/typing/bad/testfile-scope-3.c",
    uminus_1: "tests/source/typing/bad/testfile-unary_minus-1.c",
    undef_field_1: "tests/source/typing/bad/testfile-undef_field-1.c",
    undef_fun_1: "tests/source/typing/bad/testfile-undef_fun-1.c",
    undef_fun_2: "tests/source/typing/bad/testfile-undef_fun-2.c",
    undef_struct_1: "tests/source/typing/bad/testfile-undef_struct-1.c",
    undef_struct_2: "tests/source/typing/bad/testfile-undef_struct-2.c",
    undef_struct_3: "tests/source/typing/bad/testfile-undef_struct-3.c",
    undef_struct_4: "tests/source/typing/bad/testfile-undef_struct-4.c",
    undef_var_1: "tests/source/typing/bad/testfile-undef_var-1.c",
    undef_var_2: "tests/source/typing/bad/testfile-undef_var-2.c",
    undef_var_3: "tests/source/typing/bad/testfile-undef_var-3.c",
);
}