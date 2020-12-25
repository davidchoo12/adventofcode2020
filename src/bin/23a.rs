fn main() {
    let input = "193467258";
    let mut nums: Vec<u32> = input.chars().map(|c| char::to_digit(c, 10).unwrap()).collect();
    println!("{:?}", nums);
    // let mut it = nums.into_iter().cycle();
    let mut curr_i = 0;
    // rust linked list cursor is still nightly :( so had to go with manual indices tracking
    for i in 0..100 {
        println!("i {}", i);
        println!("nums {:?}", nums);
        let curr = nums[curr_i];
        // let curr = it.next().unwrap();
        println!("curr {}", curr);
        let mut removed = vec![];
        for _ in 0..3 {
            let next = (curr_i+1)%nums.len();
            removed.push(nums.remove(next));
            if next == 0 { // removing at 0 will shift all remaining nums index -1
                curr_i -= 1;
            }
        }
        // let removed: Vec<u32> = nums.drain((i+1)%nums.len()..(i+4)%nums.len()).collect();
        println!("removed {:?}", removed);
        // println!("{:?}", nums);
        let mut max = 0;
        let mut max_lt_curr = 0;
        for &n in &nums {
            if n > max {
                max = n;
            }
            if n > max_lt_curr && n < curr {
                max_lt_curr = n;
            }
        }
        let target = if max_lt_curr == 0 {max} else {max_lt_curr};
        println!("target {}", target);
        let pos = nums.iter().position(|&n| n == target).unwrap();
        for j in 0..3 {
            let pos_to_insert = pos+j+1;
            if curr_i >= pos_to_insert {
                curr_i += 1;
            }
            nums.insert(pos_to_insert, removed[j]);
        }
        // curr may have moved after removing and inserting, need to find curr
        // let curr_pos = nums.iter().position(|&n| n == curr).unwrap();
        // println!("nums {:?}", nums);
        // println!("curr {} curr_pos {} curr_i {}", curr, curr_pos, curr_i);
        // if curr_pos != curr_i {
        //     return;
        // }
        curr_i = (curr_i + 1) % nums.len();
    }
    println!("nums {:?}", nums);
    let pos1 = nums.iter().position(|&n| n == 1).unwrap();
    for i in 1..nums.len() {
        print!("{}", nums[(pos1+i)%nums.len()]);
    }
    println!();
}
// ans 25468379