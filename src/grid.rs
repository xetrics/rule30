use crate::cell::Cell;

const RULE: u8 = 30;

#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    // number of rows simulated so far
    rows: usize,
    // full infinite tape for the current row; grows by 2 each step
    tape: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width != 0 && height != 0);
        let size = width.checked_mul(height).expect("too big");
        let mut cells = vec![Cell::default(); size];

        // first row: a single live cell in the middle
        cells[width / 2].set_alive(true);

        let tape = cells[..width].to_vec();

        Grid {
            cells,
            width,
            height,
            rows: 1,
            tape,
        }
    }

    pub fn step(&mut self) {
        // grow tape by one dead cell on each side, then compute the next row
        let mut padded = vec![Cell::default(); self.tape.len() + 2];
        padded[1..self.tape.len() + 1].copy_from_slice(&self.tape);

        let mut next_tape = vec![Cell::default(); padded.len()];
        Self::compute_row(&padded, &mut next_tape);

        // extract the center `width` cells for display
        let start = (next_tape.len() - self.width) / 2;
        let visible = &next_tape[start..start + self.width];

        if self.rows < self.height {
            let row_start = self.rows * self.width;
            self.cells[row_start..row_start + self.width].copy_from_slice(visible);
            self.rows += 1;
        } else {
            // shift all rows up by one, write new bottom row
            self.cells.copy_within(self.width.., 0);
            let last_row = (self.height - 1) * self.width;
            self.cells[last_row..last_row + self.width].copy_from_slice(visible);
        }

        self.tape = next_tape;
    }

    fn compute_row(prev: &[Cell], next: &mut [Cell]) {
        let alive = |x: Option<usize>| x.and_then(|x| prev.get(x)).is_some_and(Cell::is_alive);
        for (x, cell) in next.iter_mut().enumerate() {
            let left = alive(x.checked_sub(1)) as u8;
            let center = alive(Some(x)) as u8;
            let right = alive(x.checked_add(1)) as u8;
            let pattern = left << 2 | center << 1 | right;
            cell.set_alive(RULE >> pattern & 1 == 1);
        }
    }

    pub fn draw(&self, screen: &mut [u8]) {
        debug_assert_eq!(screen.len(), 4 * self.cells.len());
        for (c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            pix.copy_from_slice(&c.color().0);
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.width, self.height);
    }
}
