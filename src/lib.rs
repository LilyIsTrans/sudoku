#![feature(portable_simd)]

#[cfg(test)]
mod test;

use std::{
    fmt::{Display, Write},
    marker::PhantomData,
    simd::prelude::*,
};

// Because it must align to power-of-two boundaries for SIMD convenience anyway, all types are actually capable of 16x16 variant sudoku. This might at some point get actually implemented.

/// Represents the solver's knowledge about a single cell of the sudoku puzzle (each 1 bit represents a possible value for the cell).
pub type Cell = u16;

pub type Region = u16x16;

mod seal {
    pub trait Seal {}
    impl Seal for super::Sudoku16 {}
    impl Seal for super::Sudoku9 {}
}

pub trait Game: seal::Seal {
    const TOTAL_CELLS: usize;
    const CELLS_PER_REGION: usize;
    const POSSIBILITIES_PER_CELL: usize;
    const ROW_COUNT: usize;

    fn display_cell(cell: Cell) -> char;
}

pub struct Sudoku9;

pub struct Sudoku16;

impl Game for Sudoku9 {
    const TOTAL_CELLS: usize = 81;

    const CELLS_PER_REGION: usize = 9;

    const POSSIBILITIES_PER_CELL: usize = 9;

    const ROW_COUNT: usize = 9;

    #[inline(always)]
    fn display_cell(cell: Cell) -> char {
        match cell.count_ones() {
            0 => '!',
            1 => match cell {
                0b0000000000000001 => '1',
                0b0000000000000010 => '2',
                0b0000000000000100 => '3',
                0b0000000000001000 => '4',
                0b0000000000010000 => '5',
                0b0000000000100000 => '6',
                0b0000000001000000 => '7',
                0b0000000010000000 => '8',
                0b0000000100000000 => '9',
                _ => unreachable!("Non sudoku digit in board!"),
            },
            _ => ' ',
        }
    }
}
impl Game for Sudoku16 {
    const TOTAL_CELLS: usize = 256;

    const CELLS_PER_REGION: usize = 16;

    const POSSIBILITIES_PER_CELL: usize = 16;

    const ROW_COUNT: usize = 16;

    #[inline(always)]
    fn display_cell(cell: Cell) -> char {
        match cell.count_ones() {
            0 => '!',
            1 => match cell.trailing_zeros() + 1 {
                1 => '0',
                2 => '1',
                3 => '2',
                4 => '3',
                5 => '4',
                6 => '5',
                7 => '6',
                8 => '7',
                9 => '8',
                10 => '9',
                11 => 'A',
                12 => 'B',
                13 => 'C',
                14 => 'D',
                15 => 'E',
                16 => 'F',
                _ => unreachable!("Invalid Sudoku16 hexadigit!"),
            },
            _ => ' ',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board<T: Game> {
    data: [Region; 16],
    _state_marker: PhantomData<T>,
}

impl<GAME: Game> Display for Board<GAME> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data {
            for cell in row.as_array() {
                f.write_char(GAME::display_cell(*cell))?;
            }
        }
        Ok(())
    }
}

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
