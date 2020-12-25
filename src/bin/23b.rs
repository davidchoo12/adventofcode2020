use std::collections::HashMap;

fn main() {
    let input = "193467258";
    let nums: Vec<u32> = input.chars().map(|c| char::to_digit(c, 10).unwrap()).collect();
    let mut n_next: HashMap<u32, u32> = (10..1000000).map(|n| (n, n+1)).collect();
    // let mut n_next: HashMap<u32, u32> = HashMap::new();
    for i in 0..nums.len()-1 {
        n_next.insert(nums[i], nums[i+1]);
    }
    n_next.insert(nums[nums.len()-1], 10);
    n_next.insert(1000000, nums[0]);
    // n_next.insert(nums[nums.len()-1], nums[0]);
    // for i in 1..11 {
    //     println!("n_next[{}] {}", i, n_next[&i]);
    // }
    // println!("n_next[{}] {}", 1000000, n_next[&1000000]);

    // let mut curr_i = 0;
    let mut curr = nums[0];
    // rust linked list cursor is still nightly :( so had to go with manual indices tracking
    for i in 0..10_000_000 {
        if i % 100000 == 0 {
            println!("i {}", i);
        }
        // println!("n_next {:?}", n_next);
        // let curr = nums[curr_i];
        // println!("curr {}", curr);
        let remove_start = n_next[&curr];
        // println!("remove_start {}", remove_start);
        let mut next = remove_start;
        let mut removed: [u32; 3] = [0;3];
        for i in 0..3 {
            removed[i] = next;
            next = n_next[&next]
        }
        n_next.insert(curr, next);
        let mut dest = curr - 1;
        if dest == 0 {
            dest = 1_000_000;
        }
        while removed.contains(&dest) {
            dest -= 1;
            if dest == 0 {
                dest = 1_000_000;
            }
        }
        let dest_next = n_next[&dest];
        n_next.insert(dest, remove_start);
        n_next.insert(removed[2], dest_next);
        // println!("dest {}", dest);
        // println!("remove_start {}", remove_start);
        // println!("next {}", next);
        curr = n_next[&curr];
    }
    // println!("n_next {:?}", n_next);
    // let pos1 = nums.iter().position(|&n| n == 1).unwrap();
    let ans: u64 = n_next[&1] as u64 * n_next[&n_next[&1]] as u64;
    println!("n_next[1] {}", n_next[&1]);
    println!("n_next[n_next[1]] {}", n_next[&n_next[&1]]);
    println!("ans {}", ans);
}
// ans 474747880250