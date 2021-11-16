use std::hash::Hasher;
use std::{collections::hash_map::DefaultHasher, hash::Hash};
use std::fmt;

use rand::{
  RngCore,
  Rng,
};

///
/// Individual organism capable of moving and reproducing.
///
#[derive(Debug, Clone)]
pub struct Organism {
  pub x: usize,
  pub y: usize,
  pub genome: Genome,
}

impl Organism {
  ///
  /// Create a new organism with a random genome of given size and the given x, y coordinates.
  /// 
  pub fn new(x: usize, y: usize, genome_size: usize) -> Organism {
    Organism {
      x,
      y,
      genome: Genome::new_random(genome_size),
    }
  }

  ///
  /// Generate a color based on the organism's genome.
  /// 
  pub fn as_color(&self) -> [u8; 3] {
    let mut rng = rand::thread_rng();
    let mut color: [u8; 3] = [0, 0, 0];
    let mut s = DefaultHasher::new();
    self.genome.hash(&mut s);
    let bytes = s.finish().to_ne_bytes();
    [
      bytes[0], bytes[1], bytes[2]
    ]
  }
}

///
/// Genom of an organism.
///
#[derive(Clone, Hash)]
pub struct Genome {
  pub genes: Vec<Gene>,
}

impl fmt::Debug for Genome {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[Genome: {}]", self.to_string())
  }
}

impl ToString for Genome {
  fn to_string(&self) -> String {
    self.genes.clone().into_iter().map(|g| g.to_string()).collect::<Vec<String>>().join(" ")
  }
}

impl Genome {
  pub fn new_random(size: usize) -> Genome {
    let mut genes = Vec::new();
    for _ in 0..size {
      genes.push(Gene::new_random());
    }
    Genome { genes }
  }

  fn mutate(&mut self) {
    let size = self.genes.len();
    self.genes[rand::thread_rng().gen_range(0..size)]
      .mutate();
  }

  
}

/// 
/// Gene of an organism.
/// 
#[derive(Clone, Hash)]
pub struct Gene {
  pub value: u32,
}

impl fmt::Debug for Gene {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[Gene: {}]", self.to_string())
  }
}

impl ToString for Gene {
  fn to_string(&self) -> String {
    format!("{:X}", self.value)
  }
}

impl Gene {
    fn new_random() -> Gene {
        let mut rng = rand::thread_rng();
        Gene {
            value: rng.next_u32(),
        }
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        self.value ^= 2^rng.gen_range(0..32);
    }
}
