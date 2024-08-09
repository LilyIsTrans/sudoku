use crate::Game;

use crate::Sudoku16;

use std::marker::PhantomData;
use std::ops::BitXorAssign;
use std::simd::cmp::SimdOrd;

use crate::Region;

use crate::Cell;

use crate::Sudoku9;

use crate::Board;

impl Board<Sudoku9> {
    pub fn new(cells: [[Cell; 9]; 9]) -> Self {
        Self {
            data: [
                Region::load_or_default(&cells[0]),
                Region::load_or_default(&cells[1]),
                Region::load_or_default(&cells[2]),
                Region::load_or_default(&cells[3]),
                Region::load_or_default(&cells[4]),
                Region::load_or_default(&cells[5]),
                Region::load_or_default(&cells[6]),
                Region::load_or_default(&cells[7]),
                Region::load_or_default(&cells[8]),
                Region::default(),
                Region::default(),
                Region::default(),
                Region::default(),
                Region::default(),
                Region::default(),
                Region::default(),
            ],
            _state_marker: PhantomData,
        }
    }
}

impl Board<Sudoku16> {
    pub fn new(cells: [[Cell; 16]; 16]) -> Self {
        Self {
            data: cells.map(Region::from_array),
            _state_marker: PhantomData,
        }
    }
}

impl<GAME: Game> Board<GAME> {
    /// Finds a `Cell` in `self` with the multiple possible values,
    /// and returns a pair, with mutually exclusive possibilities
    /// for that cell in each board of the pair, such that together
    /// they represent all possibilities for the cell. The exact way
    /// the cell is selected and the way the possibilities are partitioned
    /// is left intentionally unspecified, and consistent behaviour in
    /// that regard should not be relied upon.
    /// Returns `Err(self)`` if `self` is solved.
    pub fn branching_assume(self) -> Result<(Self, Self), Self> {
        let branch_cell = self.select_branch_cell();
        if self.data[branch_cell.0].as_array()[branch_cell.1] == 1 {
            Err(self)
        } else if self.data[branch_cell.0].as_array()[branch_cell.1] > 1 {
            let mut possibilities = self.data[branch_cell.0].as_array()[branch_cell.1].clone();
            let mut possibility_indices = arrayvec::ArrayVec::<u32, 16>::new();
            while (possibilities.trailing_zeros() as usize) < GAME::POSSIBILITIES_PER_CELL {
                possibility_indices.push(possibilities.trailing_zeros());
                possibilities &= !(1u16 << possibilities.trailing_zeros());
            }
            let (left, right) = possibility_indices.split_at(possibility_indices.len() / 2);

            todo!()
        } else {
            unreachable!("It should be impossible for a board to contain 0-cells!")
        }
    }

    fn select_branch_cell(self) -> (usize, usize) {
        let mut min_positive_entropy_idx = (0usize, 0usize);
        let mut min_positive_entropy = u32::MAX;
        for (rowidx, row) in self.data[0..GAME::ROW_COUNT].iter().enumerate() {
            for (colidx, cell) in row.as_array()[0..GAME::CELLS_PER_REGION].iter().enumerate() {
                if cell.count_ones() < min_positive_entropy {
                    min_positive_entropy = cell.count_ones();
                    min_positive_entropy_idx = (rowidx, colidx);
                }
            }
        }
        min_positive_entropy_idx
    }
}
