mod board;
mod solver;

use clap::Parser;
use board::Board;


#[derive(Parser)]
struct Cli {
    size: u32,
    initial_stage: String,
    search_method: String,
}

fn main() {
//    let args = Cli::parse();
//    
//    if (args.initial_stage.len() as u32) % args.size != 0 {
//        println!("Stage incompatble with size");
//        return
//    }
//    if !args.initial_stage.contains(' ') {
//        println!("board missing empty block");
//        return
//    }
//    let search_methods = ["BFS","DFS","GBFS","Astar"];
//    if !search_methods.contains(&args.search_method.as_str()){
//        println!("Invalid search method");
//        return
//    }
    let solution:String = String::from("213 ");
    let mut board = Board{
            size: 2,
            puzzle: String::from("32 1") ,
            blank_index: 1,
            children: Vec::new(),
            depth: 1,
    };

    //println!("{:?}", board);
    if board.is_board_solvable(&solution) {
        //println!("{}",board.get_heuristic(&solution));
    }
    else {
        println!("nah");
    }
    
    println!("{}", board);
    board.create_branches();
    for i in board.children { 
        println!("{}",i);
    }
}
