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
    if length == 0 || length == 1 {
        return 1;
    }
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
        assert!(index < self.buffer.len(), "Index out of bounds");
        let offset = self.write_index.wrapping_sub(index + 1) & self.mask;
        &self.buffer[offset]
    }
}

impl DelayLine {
    pub fn get(&self, frac_index: f64) -> f64 {
        let index = frac_index.floor() as usize;
        assert!(index < self.buffer.len(), "Index out of bounds");
        let frac = frac_index - index as f64;
        let offset = self.write_index.wrapping_sub(index + 1) & self.mask;
        let next_offset = self.write_index.wrapping_sub(index + 2) & self.mask;
        self.buffer[offset] * (1.0 - frac) + self.buffer[next_offset] * frac
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
        let test_sizes: Vec<usize> = vec![1, 2, 3, 4, 7, 16, 23, 64, 111, 256];
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

    #[test]
    fn test_wrap_arround() {
        let mut delay_line = DelayLine::new(2);
        delay_line.push(1.0);
        delay_line.push(2.0);
        delay_line.push(3.0);
        delay_line.push(4.0);
        assert_eq!(delay_line[0], 4.0);
        assert_eq!(delay_line[1], 3.0);
    }

    #[test]
    fn test_get() {
        let mut delay_line = DelayLine::new(4);
        let test_values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        test_values.iter().for_each(|value| delay_line.push(*value));

        for i in 0..test_values.len() - 1 {
            assert_eq!(
                delay_line.get(i as f64),
                test_values[test_values.len() - 1 - i],
            );
        }

        assert_eq!(delay_line.get(0.5), 3.5);
        assert_eq!(delay_line.get(1.5), 2.5);
        assert_eq!(delay_line.get(2.5), 1.5);
        assert_eq!(delay_line.get(3.5), 2.5);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_out_of_bounds() {
        let delay_line = DelayLine::new(1);
        let _x = delay_line[2];
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_out_of_bounds_frac() {
        let delay_line = DelayLine::new(1);
        let _x = delay_line.get(2.0);
    }
}
