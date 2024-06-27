pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::{BinaryHeap, HashMap};
    let mut map = HashMap::new();
    for n in nums {
        *map.entry(n).or_insert(0) += 1;
    }
    let mut heap = BinaryHeap::new();
    for (n, c) in map {
        heap.push((c, n));
    }
    (0..k).map(|_| heap.pop().unwrap().1).collect()
}

pub fn top_k_frequent_2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for n in nums {
        *map.entry(n).or_insert(0) += 1;
    }

    let max_count = *map.values().max().unwrap_or(&0);
    let mut buckets = vec![Vec::new(); max_count + 1];

    for (num, count) in map {
        buckets[count].push(num);
    }

    let mut result = Vec::new();
    for bucket in buckets.into_iter().rev() {
        result.extend(bucket);
        if result.len() >= k as usize {
            result.truncate(k as usize);
            break;
        }
    }

    result
}
