#![feature(assert_matches)]

use std::fs::{read_to_string};
use std::rc::Rc;
use logos_nom_bridge::Tokens;
use rust_mini_c::interpreter::typer::interp_file;
use rust_mini_c::parser::parse_file;
use rust_mini_c::typer::context::FileContext;
use rust_mini_c::typer::typ_file;
/*
macro_rules! test_typing_bad {
    ($($name:ident: $path:literal,)*) => {
        $(
        #[test]
        fn $name() {
            _test_typing_bad($path);
        }
        )*

    };
}*/

macro_rules! test_exec_good {
    ($($name:ident: $path:literal->$expected_path:literal,)*) => {
        $(
        #[test]
        fn $name() {
            _test_exec_good($path, $expected_path);
        }
        )*

    };
}

/*
fn _test_exec_bad(path: &str) {
    println!("File {}", path);

    let content = read_to_string(path).unwrap();
    let input = Tokens::new(&content);
    let parsed = parse_file(input);

    assert_matches!(parsed, Ok(_));

    let (_, file) = parsed.unwrap();
    let context = Context::default();
    let typed = typ_file(&context, &file);

    assert_matches!(typed, Err(_))
}*/

fn _test_exec_good(path: &str, expected_path: &str) {
    println!("File {}, expected output : {}", path, expected_path);

    let content = read_to_string(path).unwrap();
    let input = Tokens::new(&content);
    let (_, file) = parse_file(input).unwrap();

    let context = FileContext::default();
    let file = typ_file(Rc::new(context), &file).unwrap();

    let interp = interp_file(file).unwrap();
    let output = interp.to_str();

    let expected = read_to_string(expected_path).unwrap();

    assert_eq!(expected, output);
}


mod good {
    use crate::_test_exec_good;

    test_exec_good!(
    abr: "tests/source/exec/abr.c" -> "tests/source/exec/abr.out",
    and_1: "tests/source/exec/and1.c" -> "tests/source/exec/and1.out",
    and_2: "tests/source/exec/and2.c" -> "tests/source/exec/and2.out",
    exec_arith_1: "tests/source/exec/arith1.c" -> "tests/source/exec/arith1.out",
    exec_assign_1: "tests/source/exec/assign1.c" -> "tests/source/exec/assign1.out",
    exec_assign_2: "tests/source/exec/assign2.c" -> "tests/source/exec/assign2.out",
    exec_assign_3: "tests/source/exec/assign3.c" -> "tests/source/exec/assign3.out",
    exec_assign_6: "tests/source/exec/assign6.c" -> "tests/source/exec/assign6.out",
    exec_assign_7: "tests/source/exec/assign7.c" -> "tests/source/exec/assign7.out",
    block_1 : "tests/source/exec/block1.c" -> "tests/source/exec/block1.out",
    block_2: "tests/source/exec/block2.c" -> "tests/source/exec/block2.out",
    exec_call_1: "tests/source/exec/call1.c" -> "tests/source/exec/call1.out",
    exec_call_2: "tests/source/exec/call2.c" -> "tests/source/exec/call2.out",
    char_1: "tests/source/exec/char1.c" -> "tests/source/exec/char1.out",
    char_2: "tests/source/exec/char2.c" -> "tests/source/exec/char2.out",
    char_3: "tests/source/exec/char3.c" -> "tests/source/exec/char3.out",
    char_4: "tests/source/exec/char4.c" -> "tests/source/exec/char4.out",
    comp_1: "tests/source/exec/comp1.c" -> "tests/source/exec/comp1.out",
    dllist: "tests/source/exec/dllist.c" -> "tests/source/exec/dllist.out",
    fact: "tests/source/exec/fact.c" -> "tests/source/exec/fact.out",
    fact_imp: "tests/source/exec/fact_imp.c" -> "tests/source/exec/fact_imp.out",
    fact_rec: "tests/source/exec/fact_rec.c" -> "tests/source/exec/fact_rec.out",
    field_2: "tests/source/exec/field2.c" -> "tests/source/exec/field2.out",
    field_4: "tests/source/exec/field4.c" -> "tests/source/exec/field4.out",
    field_7: "tests/source/exec/field7.c" -> "tests/source/exec/field7.out",
    field_8: "tests/source/exec/field8.c" -> "tests/source/exec/field8.out",
    for_1: "tests/source/exec/for1.c" -> "tests/source/exec/for1.out",
    for_2: "tests/source/exec/for2.c" -> "tests/source/exec/for2.out",
    for_3: "tests/source/exec/for3.c" -> "tests/source/exec/for3.out",
    for_4: "tests/source/exec/for4.c" -> "tests/source/exec/for4.out",
    for_5: "tests/source/exec/for5.c" -> "tests/source/exec/for5.out",
    for_6: "tests/source/exec/for6.c" -> "tests/source/exec/for6.out",
    hello_world: "tests/source/exec/hello_world.c" -> "tests/source/exec/hello_world.out",
    if_1: "tests/source/exec/if1.c" -> "tests/source/exec/if1.out",
    init_1: "tests/source/exec/init1.c" -> "tests/source/exec/init1.out",
    josephus: "tests/source/exec/josephus.c" -> "tests/source/exec/josephus.out",
    lazy_1: "tests/source/exec/lazy1.c" -> "tests/source/exec/lazy1.out",
    local_1: "tests/source/exec/local1.c" -> "tests/source/exec/local1.out",
    local_2: "tests/source/exec/local2.c" -> "tests/source/exec/local2.out",
    local_3: "tests/source/exec/local3.c" -> "tests/source/exec/local3.out",
    mandelbrot: "tests/source/exec/mandelbrot.c" -> "tests/source/exec/mandelbrot.out",
    many: "tests/source/exec/many.c" -> "tests/source/exec/many.out",
    not_1: "tests/source/exec/not1.c" -> "tests/source/exec/not1.out",
    not_2: "tests/source/exec/not2.c" -> "tests/source/exec/not2.out",
    or_1: "tests/source/exec/or1.c" -> "tests/source/exec/or1.out",
    pascal: "tests/source/exec/pascal.c" -> "tests/source/exec/pascal.out",
    print_int: "tests/source/exec/print_int.c" -> "tests/source/exec/print_int.out",
    putchar_octal: "tests/source/exec/putchar-octal1.c" -> "tests/source/exec/putchar-octal1.out",
    putchar: "tests/source/exec/putchar1.c" -> "tests/source/exec/putchar1.out",
    putchar_hexa: "tests/source/exec/putchar_hexa1.c" -> "tests/source/exec/putchar_hexa1.out",
    return_1: "tests/source/exec/return1.c" -> "tests/source/exec/return1.out",
    return_2: "tests/source/exec/return2.c" -> "tests/source/exec/return2.out",
    shadow_1: "tests/source/exec/shadow1.c" -> "tests/source/exec/shadow1.out",
    sizeof_2: "tests/source/exec/sizeof2.c" -> "tests/source/exec/sizeof2.out",
    spilled_1: "tests/source/exec/spilled1.c" -> "tests/source/exec/spilled1.out",
    exec_uminus_1: "tests/source/exec/uminus1.c" -> "tests/source/exec/uminus1.out",
    while_1: "tests/source/exec/while1.c" -> "tests/source/exec/while1.out",
    while_2: "tests/source/exec/while2.c" -> "tests/source/exec/while2.out",
    while_3: "tests/source/exec/while3.c" -> "tests/source/exec/while3.out",
    while_4: "tests/source/exec/while4.c" -> "tests/source/exec/while4.out",
    );
}

/*
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
}*/