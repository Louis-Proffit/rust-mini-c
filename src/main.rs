use std::fs::{File, read_to_string};
use std::io::Write;
use clap::{Arg, ArgAction, Command};
use rust_mini_c::liveness::liveness_graph;
use rust_mini_c::liveness::structure::DisplayableLivenessGraph;
use rust_mini_c::coloring::color_graph;
use rust_mini_c::interference::interference_graph;
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
            Arg::new("output")
                .short('o')
                .long("output")
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
        .arg(
            Arg::new("debug-ertl")
                .long("debug-ertl")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug-liveness")
                .long("debug-liveness")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug-ltl")
                .long("debug-ltl")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let file_path = matches.get_one::<String>("file").expect("required");
    let debug_parser = matches.get_flag("debug-parser");
    let debug_typer = matches.get_flag("debug-typer");
    let debug_rtl = matches.get_flag("debug-rtl");
    let debug_ertl = matches.get_flag("debug-ertl");
    let debug_liveness = matches.get_flag("debug-liveness");
    let debug_ltl = matches.get_flag("debug-ltl");

    let output = matches.get_one::<String>("output").expect("required");

    let content = read_to_string(file_path).expect("Failed to read file");
    let _ = minic_parse(&content)
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
        .minic_ertl()
        .map(|file| {
            if debug_ertl {
                println!("ERTL file : {}", file)
            }

            if debug_liveness {
                println!("--------Liveness---------------------------");
                for (name, fun) in &file.funs {
                    let graph = liveness_graph(&fun.body).expect("Liveness failed");
                    println!("Liveness : \n{}:\n", name);
                    println!("{}", DisplayableLivenessGraph::new(&graph, &fun.entry));
                    let graph = interference_graph(&graph).expect("Interference failed");
                    println!("{}", graph);
                    let graph = color_graph(&graph).expect("Coloring failed");
                    println!("{}", graph)
                }
            }
            file
        }).expect("Failed to ertl file")
        .minic_ltl()
        .map(|file| {
            if debug_ltl {
                println!("LTL file :\n {}", file)
            }
            file
        }).expect("Failed to ltl file")
        .minic_linearise()
        .map(|file| {
            let mut output = File::create(output).expect("Failed to create a.out");
            writeln!(&mut output, "{}", file).expect("Failed to write to a.out")
        });
}