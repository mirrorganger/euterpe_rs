pub trait CircularBuffer {
    fn push(&mut self, value: f64);
    fn read(&self) -> f64;
}

pub struct DelayLine {
    buffer: Vec<f64>,
    write_index: usize,
}

impl DelayLine {
    pub fn new(size: usize) -> Self {
        DelayLine {
            buffer: vec![0.0; size],
            write_index: 0,
        }
    }
}

impl CircularBuffer for DelayLine {
    fn push(&mut self, value: f64) {
        self.buffer[self.write_index] = value;
        if self.write_index == self.buffer.len() - 1 {
            self.write_index = 0;
        } else {
            self.write_index += 1;
        }
    }

    fn read(&self) -> f64 {
        self.buffer[self.write_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_line() {
        let test_sizes: Vec<usize> = vec![1, 2, 17, 62, 670];
        for size in test_sizes {
            let mut delay_line = DelayLine::new(size);
            for i in 0..size {
                assert_eq!(delay_line.read(), 0.0);
                delay_line.push(i as f64);
            }
            for i in 0..size {
                assert_eq!(delay_line.read(), i as f64);
                delay_line.push(0.0);
            }
        }
    }
}
