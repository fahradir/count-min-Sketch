extern crate siphasher;

use siphasher::sip::SipHasher13;

use std::hash::{Hash, Hasher};

struct CountMinSketch {
    hashes: Vec<(u64, u64)>,
    field: Vec<Vec<u64>>,
    row_size: u64,
}

impl CountMinSketch {

    /// used to get a new Instance of CountMinSketch with a number of Hashfunctions and Rowsize
    fn new(hasher_c: u64, row_size: u64) -> CountMinSketch {
        let mut rng = rand::thread_rng();
        let hashes = (0..hasher_c)
            .map(|_| (rng.next_u64(), rng.next_u64()))
            .collect();
        let field = vec![vec![0u64; row_size as usize]; hasher_c as usize];
        CountMinSketch {
            hashes,
            field,
            row_size,
        }
    }

    /// updates the CountMinSketch with a new element,
    /// element has to implement Hash
    fn update<T: Hash>(&mut self, element: T) {
        for i in 0..self.hashes.len() {
            let mut hasher = SipHasher13::new_with_keys(self.hashes[i].0, self.hashes[i].1);
            element.hash(&mut hasher);
            let hash = hasher.finish();
            self.field[i][(hash % self.row_size) as usize] += 1;
        }
    }

    /// returns the current maximal occurence count of a specific element
    fn count<T: Hash>(&self, element: T) -> u64 {
        let mut vec = Vec::new();
        for i in 0..self.hashes.len() {
            let mut hasher = SipHasher13::new_with_keys(self.hashes[i].0, self.hashes[i].1);
            element.hash(&mut hasher);
            let hash = hasher.finish();
            vec.push(self.field[i][(hash % self.row_size) as usize]);
        }
        *vec.iter().min().unwrap()
    }
}
