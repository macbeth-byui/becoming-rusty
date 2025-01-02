
// Merge Sort the data in place
pub fn sort<T: Ord + Clone>(data : &mut [T]) {
    // Empty or sublist size 1 already sorted
    if data.len() <= 1 {
        return;
    }

    // Divide in half and recursively sort
    let mid = data.len() / 2;
    sort(&mut data[..mid]);
    sort(&mut data[mid..]);

    // Merge the two sorted halves together
    merge(data);
}

fn merge<T: Ord + Clone>(data : &mut [T]) {
    let mid = data.len() / 2;
    let left = data[..mid].to_vec();
    let right = data[mid..].to_vec();
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();

    for item in data {
        *item = match (left_iter.peek(), right_iter.peek()) {
            (None, Some(_)) => right_iter.next(),
            (Some(_), None) => left_iter.next(),
            (Some(l), Some(r)) 
                if l <= r   => left_iter.next(),
            _               => right_iter.next()
        }.unwrap().clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_merge_even_length() {
        let mut data = vec![1, 4, 6, 7, 8, 2, 3, 5, 9, 10];
        merge(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test2_merge_odd_length() {
        let mut data = vec![1, 4, 6, 7, 8, 2, 3, 5, 9, 10, 11];
        merge(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn test3_merge_already_sorted() {
        let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        merge(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test4_merge_small_2() {
        let mut data = vec![2, 1];
        merge(&mut data);
        assert_eq!(data, vec![1, 2]);
    }

    #[test]
    fn test5_merge_small_1() {
        let mut data = vec![1];
        merge(&mut data);
        assert_eq!(data, vec![1]);
    }

    #[test]
    fn test6_merge_empty() {
        let mut data: Vec<i32> = vec![];
        merge(&mut data);
        assert_eq!(data, vec![]);
    }

    #[test]
    fn test8_sort_even() {
        let mut data = vec![3, 5, 2, 6, 1, 4];
        sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test9_sort_odd() {
        let mut data = vec![3, 5, 7, 2, 6, 1, 4];
        sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test10_already_sorted() {
        let mut data = vec![1, 2, 3, 4, 5, 6];
        sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test11_empty() {
        let mut data: Vec<i32> = vec![];
        sort(&mut data);
        assert_eq!(data, vec![]);
    }
    

}