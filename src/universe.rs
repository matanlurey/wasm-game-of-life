#[derive(Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    /// Creates a [`Universe`] from the provided `cells` and `width`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `cells` is empty
    /// - `width` is zero
    /// - the length of `cells` is not a multiple of `width`
    pub fn try_with_cells(cells: impl Into<Vec<Cell>>, width: u32) -> Result<Self, &'static str> {
        let cells = cells.into();
        if cells.is_empty() {
            Err("Cells cannot be empty")
        } else if width == 0 {
            Err("Width cannot be zero")
        } else if cells.len() % width as usize != 0 {
            Err("Cells length must be a multiple of width")
        } else {
            #[allow(clippy::cast_possible_truncation)]
            let height = cells.len() as u32 / width;
            Ok(Universe {
                width,
                height,
                cells,
            })
        }
    }

    /// Creates a [`Universe`] from the provided dimensions and `fill` function.
    #[must_use]
    pub fn with_size_filled(
        width: u32,
        height: u32,
        fill: &mut impl FnMut(u32, u32) -> Cell,
    ) -> Self {
        let mut cells = Vec::with_capacity((width * height) as usize);
        for row in 0..height {
            for col in 0..width {
                cells.push(fill(row, col));
            }
        }
        Universe {
            width,
            height,
            cells,
        }
    }

    /// Returns the width of the universe.
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the universe.
    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Ticks the universe forward by one step.
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for (cell, row, col) in self.iter_cells() {
            let live_neighbors = self.live_neighbor_count(row, col);

            next[self.get_index(row, col)] = match (cell, live_neighbors) {
                (Cell::Dead, 3) | (Cell::Alive, 2..=3) => Cell::Alive,
                _ => Cell::Dead,
            };
        }
        self.cells = next;
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn iter_cells(&self) -> impl Iterator<Item = (Cell, u32, u32)> {
        self.cells.iter().enumerate().map(move |(index, &cell)| {
            let row = (index / self.width as usize) as u32;
            let col = (index % self.width as usize) as u32;
            (cell, row, col)
        })
    }

    /// Returns the cell at the provided position.
    ///
    /// If the position is out of bounds, returns `None`.
    fn get_cell(&self, row: u32, col: u32) -> Option<Cell> {
        self.cells.get(self.get_index(row, col)).copied()
    }

    /// Returns the index of the cell at the provided position.
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// Returns an iterator around the neighbors of the given cell dimensions.
    ///
    /// For a given cell, `◼`:
    /// ```txt
    /// 123
    /// 4◼5
    /// 678
    /// ```
    fn iter_neighbors(&self, row: u32, col: u32) -> impl Iterator<Item = Cell> {
        Neighbors {
            universe: self,
            current: (row, col),
            deltaidx: 0,
        }
    }

    /// Returns the alive neighbor count of the cell at the given position.
    #[allow(clippy::cast_possible_truncation)]
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        self.iter_neighbors(row, col)
            .filter(|&cell| cell.is_alive())
            .count() as u8
    }
}

struct Neighbors<'a> {
    universe: &'a Universe,
    current: (u32, u32),
    deltaidx: usize,
}

impl Neighbors<'_> {
    #[rustfmt::skip]
    const DELTAS: [(i32, i32); 8] = [
      (-1, -1), (-1, 0), (-1, 1),
      (0, -1),           (0, 1),
      (1, -1),  (1, 0),  (1, 1),
    ];
}

impl Iterator for Neighbors<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.deltaidx >= Self::DELTAS.len() {
            return None;
        }

        let (delta_row, delta_col) = Self::DELTAS[self.deltaidx];
        self.deltaidx += 1;

        let (row, col) = self.current;
        let neighbor_row = row.checked_add_signed(delta_row)?;
        let neighbor_col = col.checked_add_signed(delta_col)?;
        self.universe.get_cell(neighbor_row, neighbor_col)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    /// Returns whether the cell is considered alive.
    #[must_use]
    pub fn is_alive(self) -> bool {
        matches!(self, Cell::Alive)
    }

    /// Returns whether the cell is considered dead.
    #[must_use]
    pub fn is_dead(self) -> bool {
        matches!(self, Cell::Dead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_with_cells() {
        let cells = vec![Cell::Dead, Cell::Alive, Cell::Dead, Cell::Alive];
        let universe = Universe::try_with_cells(cells, 2).unwrap();
        assert_eq!(universe.width, 2);
        assert_eq!(universe.height, 2);
        assert_eq!(
            universe.cells,
            &[Cell::Dead, Cell::Alive, Cell::Dead, Cell::Alive]
        );
    }

    #[test]
    fn try_with_cells_cells_cannot_be_empty() {
        let result = Universe::try_with_cells(vec![], 5);
        assert_eq!(result.unwrap_err(), "Cells cannot be empty");
    }

    #[test]
    fn try_with_cells_width_cannot_be_zero() {
        let result = Universe::try_with_cells(vec![Cell::Dead], 0);
        assert_eq!(result.unwrap_err(), "Width cannot be zero");
    }

    #[test]
    fn try_with_cells_cells_length_must_be_multiple_of_width() {
        let result = Universe::try_with_cells(vec![Cell::Dead, Cell::Alive], 3);
        assert_eq!(
            result.unwrap_err(),
            "Cells length must be a multiple of width"
        );
    }

    #[test]
    fn with_size_filled_every_other() {
        let universe = Universe::with_size_filled(3, 3, &mut |row, col| {
            if (row + col) % 2 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        });

        assert_eq!(universe.width(), 3);
        assert_eq!(universe.height(), 3);
        assert_eq!(
            universe.cells,
            vec![
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                Cell::Dead,
                Cell::Alive
            ]
        );
    }
}
