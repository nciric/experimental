use std::sync::{Mutex, Arc};
use std::convert::TryInto;
use std::collections::HashMap;
use scoped_threadpool::Pool;

// Assume ASCII input since slicing on utf8 strings won't work like this.
type CharCountMap = HashMap<char, i64>;

pub fn str_frequency(input: &str) -> (char, i64) {
    let mut map : CharCountMap = HashMap::new();

    for ch in input.chars() {
        if let Some(count) = map.get_mut(&ch) {
            *count += 1;
        } else {
            map.insert(ch, 1);
        }
    }

    let mut max_count: i64 = -1;
    let mut max_char: char = ' ';
    for (ch, count) in map {
        if count > max_count {
            max_char = ch;
            max_count = count;
        }
    }

    (max_char, max_count)
}

// There is a centralized hash and threads write to it with blocking.
pub fn str_frequency_multicore(input: &str, cores: usize) -> (char, i64) {
    if input.len() <= cores || cores == 0 || cores == 1 {
        return str_frequency(input);
    }

    let map = Arc::new(Mutex::new(CharCountMap::new()));
    let mut pool = Pool::new(cores.try_into().unwrap());
    pool.scoped(|scope| {
        let chunk_size: usize = (input.len() as f64 / cores as f64).round() as usize;
        for i in 0..cores {
            let map = Arc::clone(&map);
            scope.execute(move || {
                let slice;
                if i + 1 == cores {
                    slice = &input[i*chunk_size..];
                } else {
                    slice = &input[i*chunk_size..(i+1)*chunk_size];
                }

                for ch in slice.chars() {
                    let mut hash = map.lock().unwrap();
                    if let Some(count) = hash.get_mut(&ch) {
                        *count += 1;
                    } else {
                        hash.insert(ch, 1);
                    }
                }
            });
        }
    });

    let mut max_count: i64 = -1;
    let mut max_char: char = ' ';
    let hash = &*map.lock().unwrap();
    for (ch, count) in hash {
        if count > &max_count {
            max_char = *ch;
            max_count = *count;
        }
    }

    (max_char, max_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!((' ', -1), str_frequency(""));
        assert_eq!((' ', -1), str_frequency_multicore("", 4));
    }

    #[test]
    fn single_char_value_string() {
        assert_eq!(('a', 4), str_frequency("aaaa"));
        assert_eq!(('a', 4), str_frequency_multicore("aaaa", 4));        
    }

    #[test]
    fn random_string() {
        assert_eq!(('a', 4), str_frequency("aabaccbac"));
        assert_eq!(('a', 4), str_frequency_multicore("aabaccbac", 4));
    }
}