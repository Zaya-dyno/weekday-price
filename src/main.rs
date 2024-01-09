mod fin_data;
mod graph;
use fin_data::*;
use std::ffi::OsStr;
use graph::*;
use std::env;
use std::cmp::Ordering;

fn higher(a:f64,b:f64) -> Option<Ordering> { a.partial_cmp(&b)}
fn lower(a:f64,b:f64) -> Option<Ordering> { b.partial_cmp(&a)}

fn do_everything(data: &OsStr, data_label: &str, dest: &OsStr, label: &str, highest: bool)
{
    let mut data = read_data(&data, data_label);
    let ret = analyse_data(&mut data,highest);
    draw_graph(ret, &dest, label);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("{:#?}",args);
    if args.len() != 5 {
        eprintln!("Incorrect number of arguments");
        std::process::exit(1);
    }
    let data_loc = OsStr::new(&args[0]);
    let dest_loc = OsStr::new(&args[2]);
    let highest = match args[4].as_str() {
        "high" => true,
        "low" => false,
        _ => panic!("wrong argument"),
    };
    do_everything(&data_loc,&args[1],&dest_loc,&args[3],highest);
}
