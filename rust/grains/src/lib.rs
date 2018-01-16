struct ChessBoard {
    pub squares: Vec<u64>
}

impl ChessBoard {
    pub fn new_with_grains() -> ChessBoard {
        let mut sqs: Vec<u64> = Vec::new();
        for x in 0..64 {
            sqs.push(2u64.pow(x));
        }
        ChessBoard {
            squares: sqs
        }
    }
}

pub fn square(s: u32) -> u64 {
   if s < 1 || s > 64 {
       panic!("Square must be between 1 and 64");
   }
   let board = ChessBoard::new_with_grains();
   board.squares[(s-1) as usize].clone()
}

pub fn total() -> u64 {
    let board = ChessBoard::new_with_grains();
    board.squares.iter().fold(0, |sum, curr| sum + curr)
}
