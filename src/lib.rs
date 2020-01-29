#[derive(Debug, Clone)]
pub struct MemBlock {
    slc: Box<[u8]>,
    size: (usize, usize),
}

impl MemBlock {
    /// Create a new [`MemBlock`] from a size in pixel (x,y)
    pub fn new(size: (usize, usize)) -> Self {
        MemBlock {
            slc: vec![0; size.0 * size.1 * 4].into_boxed_slice(),
            size,
        }
    }
    /// Create a new [`MemBlock`] from a size in pixel (x,y) with a given default value;
    pub fn new_with_value(size: (usize, usize), value: u32) -> Self {
        MemBlock {
            slc: vec![
                ((value >> 24) & 0xFF) as u8,
                ((value >> 16) & 0xFF) as u8,
                ((value >> 8) & 0xFF) as u8,
                (value & 0xFF) as u8,
            ]
            .repeat(size.0 * size.1)
            .into_boxed_slice(),
            size,
        }
    }
    ///Returns the size (in pixel) of the [`MemBlock`]
    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    /// Read a single value from the [`MemBlock`]
    /// The index is the number of u8's from the start, it is a 1d index
    fn read_u8(&self, index: usize) -> u8 {
        assert!(index < self.size.0 * self.size.1 * 4);
        assert!(index <= isize::max_value() as usize);
        self.slc[index]
    }

    /// Write a single value to the [`MemBlock`]
    /// The index is the number of u8's from the start, it is a 1d index
    fn write_u8(&mut self, index: usize, data: u8) {
        assert!(index < self.size.0 * self.size.1 * 4);
        assert!(index <= isize::max_value() as usize);
        self.slc[index] = data;
    }

    /// Read a single pixel from the [`MemBlock`]
    /// The index is the (x,y) position of the pixel in the memory
    pub fn read(&self, index: (usize, usize)) -> u32 {
        assert!(index.0 < self.size.0);
        assert!(index.1 < self.size.1);
        let r_index = (index.0 + index.1 * self.size.0) * 4;
        let mut data = 0u32;
        for i in 0..4 {
            data += (self.read_u8(r_index + i) as u32) << (24 - 8 * i);
        }
        data
    }
    /// Write a single pixel to the [`MemBlock`]
    /// The index is the (x,y) position of the pixel in the memory
    pub fn write(&mut self, index: (usize, usize), data: u32) {
        assert!(index.0 < self.size.0);
        assert!(index.1 < self.size.1);
        let r_index = (index.0 + index.1 * self.size.0) * 4;
        for i in 0..4 {
            self.write_u8(r_index + i, (data >> (24 - 8 * i)) as u8);
        }
    }

    /// Copy `source` onto `self` at given index
    pub fn dma(&mut self, index: (usize, usize), source: &Self) {
        for y in 0..source.size.1 {
            for x in 0..source.size.0 {
                if x + index.0 >= self.size.0 || y + index.1 >= self.size.1 {
                } else {
                    self.write((index.0 + x, index.1 + y), source.read((x, y)));
                }
            }
        }
    }

    /// Print the [`MemBlock`] with nice formating;
    pub fn table(&self) {
        for y in 0..(self.size.1) {
            for x in 0..(self.size.0) {
                print!(" {:08X}", self.read((x, y)));
            }
            println!();
        }
    }
}

impl std::ops::Deref for MemBlock {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.slc
    }
}
impl std::ops::DerefMut for MemBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slc
    }
}

impl std::borrow::Borrow<[u8]> for MemBlock {
    fn borrow(&self) -> &[u8] {
        &self[..]
    }
}

impl std::borrow::BorrowMut<[u8]> for MemBlock {
    fn borrow_mut(&mut self) -> &mut [u8] {
        &mut self[..]
    }
}
