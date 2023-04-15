pub struct BitSet {
    pub state: usize,
}

impl BitSet {
    pub fn new() -> Self {
        BitSet { state: 0 }
    }

    pub fn set(&mut self, bit: usize) {
        self.state |= 1 << bit;
    }
    pub fn unset(&mut self, bit: usize) {
        self.state &= !(1 << bit);
    }
}
