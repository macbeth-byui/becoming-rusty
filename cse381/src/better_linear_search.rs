fn better_linear_search<T: Ord>(data : &[T], target : &T) -> Option<usize> {
    for index in 0..data.len() {
        if data.get(index).unwrap() == target {
            return Some(index);
        }
    }
    None
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_found_first() {
        let data = [5, 2, 6, 4, 1, 3, 5, 9, 7];
        let target = 5;
        assert_eq!(better_linear_search(&data, &target), Some(0));
    }

    #[test]
    fn test_target_found_middle() {
        let data = [5, 2, 6, 4, 1, 3, 5, 9, 7];
        let target = 1;
        assert_eq!(better_linear_search(&data, &target), Some(4));
    }

    #[test]
    fn test_target_found_last() {
        let data = [5, 2, 6, 4, 1, 3, 5, 9, 7];
        let target = 7;
        assert_eq!(better_linear_search(&data, &target), Some(8));
    }

    #[test]
    fn test_target_not_found() {
        let data = [5, 2, 6, 4, 1, 3, 5, 9, 7];
        let target = 8;
        assert_eq!(better_linear_search(&data, &target), None);
    }
}