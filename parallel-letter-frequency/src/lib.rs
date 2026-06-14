use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_size = (input.len() / worker_count).max(1);
    let (tx, rx) = mpsc::channel();

    thread::scope(|scope| {
        for chunk in input.chunks(chunk_size) {
            let tx = tx.clone();
            scope.spawn(move || {
                let map = chunk
                    .iter()
                    .flat_map(|s| s.chars())
                    .filter(|c| c.is_alphabetic())
                    .map(|c| c.to_ascii_lowercase())
                    .fold(HashMap::new(), |mut map, c| {
                        *map.entry(c).or_insert(0) += 1;
                        map
                    });
                tx.send(map).unwrap();
            });
        }
        drop(tx);
    });

    // merge results
    let mut map = HashMap::new();
    for partial in rx {
        map = merge_maps(map, partial);
    }

    map
}

fn merge_maps(a: HashMap<char, usize>, b: HashMap<char, usize>) -> HashMap<char, usize> {
    let (mut dest, source) = if a.len() >= b.len() {(a,b)} else {(b,a)};
    for (k, v) in source {
        *dest.entry(k).or_insert(0) += v;
    }
    dest
}
