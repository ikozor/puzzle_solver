use std::fmt;

#[derive(Debug, Clone)]
pub struct Board {
    pub size: usize,
    pub blank_index: usize,
    pub children: Vec<Board>,
    pub depth: usize,
    pub puzzle: String,
}

impl Board {
    pub fn is_board_solvable(&self, solution: &String) -> bool {
        if self.size == 2 {
            if self.puzzle == *solution {
                return true
            }
        }
        if self.size % 2 == 1 {
            return self.get_inversions() % 2 == 0
        }
        return ((((self.puzzle.find(' ').unwrap() as f32) / (self.size as f32))
            .ceil() as usize) - 1 + self.get_inversions()) % 2 == 1
    }
    
    fn get_inversions(&self) -> usize {
        let mut inversions: usize = 0; 
        let row_major_order: String = self.puzzle.clone().replace(" ", ""); 
        for (i, ichar) in row_major_order.char_indices() {
            for (_, jchar) in row_major_order.chars().skip(i + 1).enumerate() {
                if ichar > jchar {
                    inversions += 1;
                }
            }
        }
        return inversions; 
    }
    
    pub fn solved(&self, solution: &String) -> bool{
        return self.puzzle == *solution;
        
    }

    pub fn get_heuristic(&self, solution: &String) -> usize {
        let mut heuristic: usize = 0;
        for (i, v) in self.puzzle.char_indices() {
            if v == ' ' {
                let i: isize = i as isize;
                let sol: isize = solution.find(' ').unwrap() as isize;
                let size: isize = self.size as isize;

                let x_distance: isize = (sol % size) - (i % size);
                let y_distance: isize = (sol / size) - (i / size);
                heuristic = x_distance.abs_diff(0) + y_distance.abs_diff(0);
            }
        }
        return heuristic;
    }
    
    pub fn create_branches(&mut self) {
        let mut dirs: Vec<&str> = Vec::new();
        
        if self.blank_index % self.size < self.size - 1 {
            dirs.push("right");
        }
        if self.blank_index % self.size > 0 {
            dirs.push("left");
        }
        if self.blank_index > self.size - 1 {
            dirs.push("up");
        }
        if self.blank_index < self.size * (self.size - 1) {
            dirs.push("down");
        }
        
        for d in dirs {
            let branch: Board;
            match Some(d) {
                Some("right") => branch = create_branch(self, self.blank_index + 1),
                Some("left") => branch = create_branch(self, self.blank_index - 1),
                Some("up") => branch = create_branch(self, self.blank_index - self.size),
                Some("down") => branch = create_branch(self, self.blank_index + self.size),
                Some(&_) => todo!(),
                None => todo!(),
            };
            self.children.push(branch);
        }
    }
}
 impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = String::from("");
        for i in 0..self.size {
            result.push('|');
            for j in 0..self.size {
                result.push(self.puzzle.chars().nth((i*self.size)+j).unwrap());
            result.push('|');
            }
            result.push('\n');
        }
        return write!(f,"{}", result);
    }
 }

fn create_branch(board: &Board, blank_index: usize) -> Board { 
    let mut puzzle: String = board.puzzle.clone();
    let moved_char: String = puzzle.chars().nth(blank_index).unwrap().to_string();
    puzzle = puzzle.replace(" ",&moved_char);
    puzzle.insert(blank_index,' ');
    puzzle.remove(blank_index+1);

    let new_board = Board {
        blank_index, 
        puzzle,
        depth:  board.depth + 1,
        children: Vec::new(),
        ..*board
    };
    
    return new_board;
}
