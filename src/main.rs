mod board;
mod solver;

use clap::Parser;


#[derive(Parser)]
struct Cli {
    size: usize,
    initial_stage: String,
    search_method: String,
}

fn main() {
    let args = Cli::parse();
    
    if args.initial_stage.len() % args.size != 0 {
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
    let solution:String = String::from("213 ");
    let mut board = board::create_board(args.size, 0, args.initial_stage);

    if board.is_board_solvable(&solution) {
        //println!("{}",board.get_heuristic(&solution));
    }
    else {
        println!("nah");
    }
    
    println!("{}", board);
    board.create_branches();
    for i in board.children{
        println!("{}", i);
    }
}
