use std::sync::{Arc, Mutex, Condvar};
use std::{thread, time};

pub type SharedData<T> = Arc<Mutex<Vec<T>>>;

fn factory_init(count : u8, requests : &SharedData<()>) -> u64 {
    let x = Arc::new(Condvar::new());
    let mut threads = Vec::new();
    for _ in 0..count {
        let requests = requests.clone();
        let handle = thread::spawn(move || {
            let sleep_time = time::Duration::from_secs(1);
            let mut completed = 0;
            loop {
                let job = {
                    let mut requests = requests.lock().unwrap();
                    requests.pop()
                };
                match job {
                    Some(_) => {
                        thread::sleep(sleep_time);
                        completed += 1;
                    }
                    None => break
                }
            }
            completed
        });
        threads.push(handle);
    }
    let mut completed = 0;
    for thread in threads {
        completed += thread.join().unwrap();
    }
    completed
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_factory1() {
        let mut requests = Vec::new();
        for _ in 0..10 {
            requests.push(());
        }
        let requests = Arc::new(Mutex::new(requests));
        let completed = factory_init(1, &requests);
        assert_eq!(completed, 10);
    }

    #[test]
    fn test_factory25() {
        let mut requests = Vec::new();
        for _ in 0..1000 {
            requests.push(());
        }
        let requests = Arc::new(Mutex::new(requests));
        let completed = factory_init(25, &requests);
        assert_eq!(completed, 1000);
    }

}