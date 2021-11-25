pub mod k_means {
    use crate::preprocess::preprocess;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use rand::prelude::*;
    use std::f32;

    #[derive(Eq, Hash, PartialEq)]
    pub struct Pair {
        one: i32,
        two: i32
    }

    pub struct KMeans {
        text_blobs: Vec<Vec<String>>,
        k: i32,
        clusters: HashMap<i32, i32>,
        centroids: Vec<Vec<String>>
    }

    impl KMeans {
        fn jaccard_distance(set_one: HashSet<String>, set_two: HashSet<String>) -> f32 {
            1.0 - set_one.intersection(&set_two).collect::<Vec<&String>>().len() as f32 / set_one.union(&set_two).collect::<Vec<&String>>().len() as f32
        }
    
        fn to_set(vec: Vec<String>) -> HashSet<String> {
            HashSet::from_iter(vec)
        }
    
        pub fn cluster(mut self) {
            let mut centroids: Vec<Vec<String>> = Vec::new();
            let mut distances: HashMap<Pair, f32> = HashMap::new();
            let mut i = 0;
            let mut rng = rand::thread_rng();
            let mut nums: Vec<usize> = Vec::new();
            loop {
                let mut n = rng.gen_range(0..self.text_blobs.len());
                while nums.contains(&n) {
                    n = rng.gen_range(0..self.text_blobs.len());
                }
                nums.push(n);
                i += 1;
                if i == self.k {
                    break;
                }
            }
            for n in nums {
                centroids.push(self.text_blobs[n]);
            }
            let mut i = 0;
            for text in self.text_blobs {
                let mut j = 0;
                let bagOne = KMeans::to_set(text);
                for t in self.text_blobs {
                    let bagTwo = KMeans::to_set(t);
                    let d = KMeans::jaccard_distance(bagOne, bagTwo);
                    if i == j {
                        distances.insert(Pair {one: i, two: j}, 1.0);
                    } else if distances.contains_key(&Pair {one: i, two: j}) {
                        
                    } else {
                        distances.insert(Pair {one: i, two: j}, d);
                        distances.insert(Pair {one: j, two: i}, d);
                    }
                    j += 1;
                }
                i += 1;
            }
            while centroids != self.centroids {
                let mut clusters: HashMap<i32, i32> = HashMap::new();
                self.centroids = centroids.clone();
                clusters = self.clusters.clone();
                for c in self.clusters.keys() {
                    let mut d: Vec<f32> = Vec::new();
                    for n in nums {
                        d.push(*distances.get(&Pair {one: *c, two: n as i32}).unwrap());
                    }
                    let min_d = d.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                    let index = d.iter().position(|&r| r == min_d).unwrap();
                    clusters[c] = nums[index] as i32;
                }
                nums = Vec::new();
                self.clusters = clusters.clone();
                let mut k = 0;
                loop {
                    let c: Vec<i32> = Vec::new();
                    for (k, v) in self.clusters.iter() {
                        if v == k {
                            c.push(*k);
                        }
                    }
                    let min = 0.0;
                    let min_text = 0;
                    for n in c {
                        let cumulative = 0.0;
                        for nu in c {
                            cumulative += distances.get(&Pair{one: n, two: nu}).unwrap();
                        }
                        if min == 0.0 || cumulative < min {
                            min = cumulative;
                            min_text = n;
                        }
                    }
                    nums.push(min_text as usize);
                    centroids.push(self.text_blobs[min_text as usize]);
                    k += 1;
                    if k == self.k {
                        break;
                    }
                }
            }
            self.centroids = centroids;
        }
    
        pub fn new(vector: Vec<preprocess::DataRecord>, k: i32) -> KMeans {
            let mut map: HashMap<i32, i32> = HashMap::new();
            let mut vec: Vec<Vec<String>> = Vec::new();
            let mut i = 0;
            for record in vector {
                map.insert(i, -1);
                vec.push(record.answer_text);
                i += 1;
            }
            KMeans {
                text_blobs: vec,
                k: k,
                clusters: map,
                centroids: Vec::new()
            }
        }
    }
}