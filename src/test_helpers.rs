#[cfg(test)]
pub fn is_bounded_buffer(vec: &[f32], min: f32, max: f32) -> bool {
    vec.iter().all(|&x| x >= min && x <= max)
}

#[cfg(test)]
pub fn is_increasing_buffer(vec: &[f32]) -> bool {
    vec.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
pub fn is_decreasing_buffer(vec: &[f32]) -> bool {
    vec.windows(2).all(|w| w[0] >= w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect_bounder_buffer() {
        let data = [0.2, 3.5, 5.9, 10.7];
        assert!(is_bounded_buffer(&data, 0.0, 10.7));
        assert!(!is_bounded_buffer(&data, 0.0, 10.6));
    }

    #[test]
    fn test_increasing_buffer() {
        let increasing = [2.6, 6.8, 9.1, 10.0];
        let non_increasing = [2.6, 6.8, 9.1, 9.0];
        assert!(is_increasing_buffer(&increasing));
        assert!(!is_increasing_buffer(&non_increasing));
    }

    #[test]
    fn test_decreasing_buffer() {
        let increasing = [10.0, 9.1, 6.8, 2.6];
        let non_decreasing = [10.0, 9.1, 6.8, 6.9];
        assert!(is_decreasing_buffer(&increasing));
        assert!(!is_decreasing_buffer(&non_decreasing));
    }
}
