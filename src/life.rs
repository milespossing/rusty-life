const ALIVE: char = '■';
const DEAD: char = '□';

pub struct Board {
    width: usize,
    height: usize,
    pub cells: Vec<Vec<u8>>,
}

impl Board {
  pub fn new(width: usize, height: usize) -> Board {
    Board { width, height, cells: vec![vec![0; width]; height] }
  }
}

// TODO: There must be a more performant way to do this
fn count_neighbors(board: &Board, idx: usize, idy: usize) -> u8 {
    let mut count = 0;
    if idy > 0 {
        count += board.cells[idy - 1][idx];
    }
    if idx > 0 {
        count += board.cells[idy][idx - 1];
    }
    if idx > 0 && idy > 0 {
        count += board.cells[idy - 1][idx - 1];
    }
    if idy < board.width - 1 {
        count += board.cells[idy + 1][idx];
    }
    if idx < board.height - 1 {
        count += board.cells[idy][idx + 1];
    }
    if idx < board.width - 1 && idy < board.height - 1 {
        count += board.cells[idy + 1][idx + 1]
    }
    return count;
}

pub fn update_cells(board: &Board) -> Board {
  let mut next = Board::new(board.width, board.height);
  for (idy, v) in board.cells.iter().enumerate() {
    for (idx, cell) in v.iter().enumerate() {
      let count = count_neighbors(board, idx, idy);
      next.cells[idy][idx] = match (cell, count) {
        (1, c) if c < 2 => 0u8,
        (1, c) if c > 3 => 0u8,
        (0, c) if c == 3 => 1u8,
        (cell, _) => *cell
      }
    }
  }
  next
}
