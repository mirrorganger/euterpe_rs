use std::ops::Index;

pub trait CircularBuffer: Index<usize> {
    fn size(&self) -> usize;
    fn push(&mut self, value: f64);
    fn back(&self) -> &f64;
}

pub struct DelayLine {
    buffer: Vec<f64>,
    write_index: usize,
    mask: usize,
}

impl DelayLine {
    pub fn new(size: usize) -> Self {
        assert!(size.is_power_of_two(), "Size must be a power of two");
        DelayLine {
            buffer: vec![0.0; size],
            write_index: 0,
            mask: size - 1,
        }
    }
}

impl Index<usize> for DelayLine {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        assert!(index < self.buffer.len());
        let offset = self.write_index.wrapping_sub(index + 1) & self.mask;
        &self.buffer[offset]
    }
}

impl CircularBuffer for DelayLine {
    fn size(&self) -> usize {
        self.buffer.len()
    }

    fn push(&mut self, value: f64) {
        self.buffer[self.write_index & self.mask] = value;
        self.write_index += 1;
    }

    fn back(&self) -> &f64 {
        &self[self.size() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_line() {
        let test_sizes: Vec<usize> = vec![4, 16, 64, 256];
        for size in test_sizes {
            let mut delay_line = DelayLine::new(size);

            assert_eq!(delay_line.size(), size);

            for i in 0..size {
                delay_line.push(i as f64);
            }
            assert_eq!(*delay_line.back(), 0.0);

            for i in 0..size {
                assert_eq!(delay_line[i], (size - 1 - i) as f64);
            }
        }
    }
}
