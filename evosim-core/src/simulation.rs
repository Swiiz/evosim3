use crate::world::Board;

pub struct Simulation {
  pub board: Board,
}

impl Simulation {
  ///
  /// Create a new simulation with a board of the given size.
  /// 
  pub fn new(width: usize, height: usize) -> Simulation {
    Simulation {
      board: Board::new(width, height),
    }
  }

  pub fn start(&mut self) {
    self.board.populate_with_random_organisms(10, 2);
  }

  pub fn step(&mut self) {
    
  }
    
}