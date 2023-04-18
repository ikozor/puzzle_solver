mod board;
mod solver;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    size: u32,
    initial_stage: String,
    search_method: String,
}

fn main() {
    let args = Cli::parse();
    
    if (args.initial_stage.len() as u32) % args.size != 0 {
        println!("Stage incompatble with size");
        return
    }
    if !args.initial_stage.contains(' ') {
        println!("board missing empty block");
        return
    }
    let search_methods = ["BFS","DFS","GBFS","Astar"];
    if !search_methods.contains(&args.search_method.as_str()){
        println!("Invalid search method");
        return
    }
    println!("passed");
    
}
