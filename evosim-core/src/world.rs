use rand::Rng;

use crate::{life::Organism, render::Color};

///
/// Cellular automata like grid, but with a fixed size.
///
pub struct Board {
  pub tiles: Vec<Vec<Tile>>,
}

impl Board {
  ///
  /// Create a new board with the given size.
  /// The board is filled with empty tiles.
  /// 
  pub fn new(width: usize, height: usize) -> Board {
    let mut tiles = Vec::new();
    for _ in 0..height {
      let mut row = Vec::new();
      for _ in 0..width {
        row.push(Tile::new());
      }
      tiles.push(row);
    }
    Board { tiles }
  }

  ///
  /// Clear the board.
  /// 
  pub fn clear(&mut self) {
    for row in self.tiles.iter_mut() {
      for tile in row.iter_mut() {
        tile.clear();
      }
    }
  }

  ///
  /// Populate the board with the given number of organisms placed at random position.
  /// 
  pub fn populate_with_random_organisms(&mut self, count: usize, genome_size: usize) {
    self.clear();
    let mut organisms_count = 0;
    while organisms_count < count {
      let x = rand::thread_rng().gen_range(0..self.tiles[0].len());
      let y = rand::thread_rng().gen_range(0..self.tiles.len());
      if self.tiles[x][y].organism.is_none() {
        self.tiles[x][y].organism = Some(Organism::new(x, y, genome_size));
        organisms_count += 1;
      }
    }
  }
  
  ///
  /// The function used to draw an image, taking as input an x and y coordinate.
  /// 
  pub fn color_at(&self, x: usize, y: usize) -> Color {
    match self.tiles[x][y].organism.clone() {
        Some(organism) => {
          let color = organism.as_color();
          Color { r: color[0], g: color[1], b: color[2] }
        },
        None => {
          Color { r: 255, g: 255, b: 255 }
        },
    }
  }

  ///
  /// Return the number of tiles on each row
  /// 
  pub fn width(&self) -> usize {
    self.tiles[0].len()
  }

  ///
  /// Return the number of tiles on each column
  /// 
  pub fn height(&self) -> usize {
    self.tiles.len()
  }
  
}

/// 
/// Individual cell of the board
/// 
pub struct Tile {
  pub organism: Option<crate::life::Organism>,
}

impl Tile {
  ///
  /// Create a new empty tile.
  /// 
  fn new() -> Tile {
    Tile { organism: None }
  }

  ///
  /// Clear the tile.
  /// 
  pub fn clear(&mut self) {
    self.organism = None;
  }
}