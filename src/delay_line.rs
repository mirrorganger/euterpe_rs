use std::ops::Index;

pub trait CircularBuffer: Index<usize> {
    fn size(&self) -> usize;
    fn push(&mut self, value: f64);
    fn back(&self) -> &f64;
    fn clear(&mut self);
}

pub struct DelayLine {
    buffer: Vec<f64>,
    write_index: usize,
    mask: usize,
}

fn adjust_length_to_power_of_two(length: usize) -> usize {
    let mut power_of_two = 1;
    while power_of_two < length {
        power_of_two *= 2;
    }
    power_of_two
}

impl DelayLine {
    pub fn new(size: usize) -> Self {
        let adjusted_size = adjust_length_to_power_of_two(size);
        DelayLine {
            buffer: vec![0.0; adjusted_size],
            write_index: 0,
            mask: adjusted_size - 1,
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

    fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_line() {
        let test_sizes: Vec<usize> = vec![3, 4, 7, 16, 23, 64, 111, 256];
        for size in test_sizes {
            let adjust_size = adjust_length_to_power_of_two(size);
            let mut delay_line = DelayLine::new(adjust_size);

            assert_eq!(delay_line.size(), adjust_size);

            for i in 0..adjust_size {
                delay_line.push(i as f64);
            }
            assert_eq!(*delay_line.back(), 0.0);

            for i in 0..adjust_size {
                assert_eq!(delay_line[i], (adjust_size - 1 - i) as f64);
            }

            delay_line.clear();
            for i in 0..adjust_size {
                assert_eq!(delay_line[i], 0.0);
            }
        }
    }
}
