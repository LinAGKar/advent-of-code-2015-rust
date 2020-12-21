use std::io::Read;
use std::sync::{Arc, Mutex};

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    let orig_len = input.len();

    let answer_mutex = Arc::new(Mutex::new(None));

    let cores = num_cpus::get();

    let threads: Vec<_> = (0..cores).map(|i| {
        let mut data = input.clone();
        let answer_mutex = Arc::clone(&answer_mutex);

        std::thread::spawn(move || {
            if let Some(Some(answer)) = (0..).map(|x| x * cores + i).find_map(|x| {
                if x % (100 * cores + i) == 0 {
                    if let Some(found_answer) = *answer_mutex.lock().unwrap() {
                        if x > found_answer {
                            return Some(None)
                        }
                    }
                }

                data.append(&mut format!("{}", x).into_bytes());
                let hash = md5::compute(&data);
                if (0..3).all(|y| hash.0[y] == 0) {
                    Some(Some(x))
                } else {
                    data.truncate(orig_len);
                    None
                }
            }) {
                let mut found_answer = answer_mutex.lock().unwrap();
                if (*found_answer).unwrap_or(usize::MAX) > answer {
                    *found_answer = Some(answer);
                }
            }
        })
    }).collect();

    for thread in threads {
        thread.join().unwrap();
    }

    println!("{}", answer_mutex.lock().unwrap().unwrap());
}
