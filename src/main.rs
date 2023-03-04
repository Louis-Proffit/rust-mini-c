use std::fs::read_to_string;
use clap::{Arg, ArgAction, Command};
use rust_mini_c::minic_parse;

fn main() {
    let matches = Command::new("minic")
        .version("1.0")
        .author("Louis P. <louisproffit86@gmail.com>")
        .about("Mini-c compiler, INF564")
        .arg(
            Arg::new("file")
                .required(true)
        )
        .arg(
            Arg::new("debug-parser")
                .long("debug-parser")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug-typer")
                .long("debug-typer")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug-rtl")
                .long("debug-rtl")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let file_path = matches.get_one::<String>("file").expect("required");
    let debug_parser = matches.get_flag("debug-parser");
    let debug_typer = matches.get_flag("debug-typer");
    let debug_rtl = matches.get_flag("debug-rtl");

    let content = read_to_string(file_path).expect("Failed to read file");
    let stdout = minic_parse(&content)
        .map(|file| {
            if debug_parser {
                println!("Parsed file : {:?}", file);
            }
            file
        })
        .expect("Failed to parse file")
        .minic_typ()
        .map(|file| {
            if debug_typer {
                println!("Typed file : {:?}", file);
            }
            file
        })
        .expect("Failed to parse file")
        .minic_rtl()
        .map(|file| {
            if debug_rtl {
                println!("RTL file : {}", file);
            }
            file
        })
        .expect("Failed to rtl file")
        .minic_interp()
        .expect("Failed to interp RTL");

    println!("------------------------------------");
    println!("{}", stdout)
}