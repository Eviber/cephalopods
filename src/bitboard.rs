#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Bitboard(u32);

impl Bitboard {
    /// Creates a new empty board (all cells set to 0)
    fn new() -> Self {
        Bitboard(0)
    }

    /// Gets the value of a cell at (row, col)
    fn get(&self, row: usize, col: usize) -> u8 {
        let shift = (row * 3 + col) * 3;
        ((self.0 >> shift) & 0b111) as u8
    }

    /// Sets a cell at (row, col) with a value (1-6, 0 for empty)
    fn set(&mut self, row: usize, col: usize, value: u8) {
        assert!(value <= 6, "Value must be between 0 and 6");
        let shift = (row * 3 + col) * 3;
        self.0 &= !(0b111 << shift); // Clear bits
        self.0 |= (value as u32) << shift; // Set new value
    }

    /// Prints the board in a human-readable format
    fn print(&self) {
        for row in 0..3 {
            for col in 0..3 {
                print!("{} ", self.get(row, col));
            }
            println!();
        }
    }

    pub fn from_state(state: [[u32; 3]; 3]) -> Bitboard {
        let mut bitboard = Bitboard::new();
        for (r, row) in state.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                bitboard.set(r, c, *val as u8);
            }
        }

        bitboard
    }
}

#[cfg(test)]
mod tests {

}
