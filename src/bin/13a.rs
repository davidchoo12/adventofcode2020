fn main() {
    let input = "1006401
17,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,449,x,x,x,x,x,x,x,23,x,x,x,x,13,x,x,x,x,x,19,x,x,x,x,x,x,x,x,x,x,x,607,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29";
    let lines: Vec<&str> = input.lines().collect();
    let dep = lines[0].parse::<i32>().expect("nan");
    let buses = lines[1];
    let ids: Vec<i32> = buses
        .split(",")
        .filter(|&x| x != "x")
        .map(|x| x.parse::<i32>().expect("nan"))
        .collect();
    println!("{:?}", ids);
    let mut max_diff = 0;
    let mut max_id = 0;
    for id in ids {
        if dep % id > max_diff {
            max_diff = dep % id;
            max_id = id;
        }
    }
    println!("max_diff {}, max_id {}, ans ((max_id-max_diff)*max_id) {}", max_diff, max_id, (max_id-max_diff)*max_id)
}

// ans 3035