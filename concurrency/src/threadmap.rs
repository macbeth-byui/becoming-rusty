use std::sync::{Arc, Mutex};

pub type SharedData<T> = Arc<Mutex<Vec<T>>>;

pub fn thread_map<F, T>(lambda : F, data : Vec<T>) -> Vec<T>
where 
    F: Fn(T) -> T + Clone + Send + 'static,
    T: Clone + Send + 'static,
{
    let shared_data = Arc::new(Mutex::new(data.clone()));
    let mut threads = Vec::new();
    for index in 0..data.len() {
        let shared_data = shared_data.clone();
        let lambda = lambda.clone();
        let handle = std::thread::spawn(
            move || one_map(index, lambda, &shared_data));
        threads.push(handle);
    }
    for thread in threads {
        thread.join().unwrap();
    } 
    // This works because all of my references to shared data are out of scope now
    Arc::into_inner(shared_data).unwrap().into_inner().unwrap()
}

fn one_map<F, T>(index : usize, lambda : F, data : &SharedData<T>)
where 
    F: Fn(T) -> T + Clone + Send + 'static,
    T: Clone + Send + 'static,
{
    let input = {
        let data = data.lock().unwrap();
        match data.get(index) {
            Some(input) => input.clone(),
            None => return
        }
    };
    let result = lambda(input);
    let mut data = data.lock().unwrap();
    data[index] = result;
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_f64() {
        let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0];        
        data = thread_map(|x| x * 2.0, data);
        assert_eq!(data, vec![2.0, 4.0, 6.0, 8.0, 10.0])
    }

    #[test]
    fn test_string() {
        let mut data = vec!["cat".to_string(), "dog".to_string(), "pig".to_string(), "cow".to_string()];        
        data = thread_map(|x| x.to_uppercase(), data);
        assert_eq!(data, vec!["CAT".to_string(), "DOG".to_string(), "PIG".to_string(), "COW".to_string()])
    }
}