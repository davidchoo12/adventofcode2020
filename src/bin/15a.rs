use std::collections::HashMap;
fn main() {
    let input = "13,0,10,12,1,5,8";
    let mut nums: HashMap<i32, usize> = HashMap::new();
    let nums_str: Vec<&str> = input.split(",").collect();
    for (i, s) in nums_str[..nums_str.len() - 1].iter().enumerate() {
        nums.insert(s.parse().expect("nan"), i+1);
    }
    println!("{:?}", nums);
    let mut prev: i32 = nums_str.last().unwrap().parse().expect("nan");
    for i in nums_str.len()..2020 {
        let curr;
        if let Some(&last_pos) = nums.get(&prev) {
            curr = i as i32 - last_pos as i32;
        } else {
            curr = 0
        }
        nums.insert(prev, i);
        prev = curr;
        println!("{}", curr);
    }
}

// ans 260