use std::cmp::Ordering;

pub fn search<T: Ord>(data : &[T], target : &T) -> Option<usize> {
    _search(data, target, 0, data.len()-1)
}

fn _search<T: Ord>(data : &[T], target : &T, first : usize, last : usize) -> Option<usize> {
    if first > last {
        return None;
    }
    let mid = (first + last) / 2;
    match data[mid].cmp(target) {
        Ordering::Equal => Some(mid),
        Ordering::Greater => _search(data, target, first, mid - 1),
        Ordering::Less => _search(data, target, mid + 1, last)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let data = vec![2,4,5,8,10,14,23,36];
        let mut result = search(&data, &23);
        assert_eq!(result,Some(6));

        result = search(&data, &24);
        assert_eq!(result,None);
    }
}