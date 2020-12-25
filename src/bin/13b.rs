fn main() {
    let input = "1006401
17,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,449,x,x,x,x,x,x,x,23,x,x,x,x,13,x,x,x,x,x,19,x,x,x,x,x,x,x,x,x,x,x,607,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29";
    let lines: Vec<&str> = input.lines().collect();
    let buses = lines[1];
    let mut ids: Vec<(i64, i64)> = buses
        .split(",")
        .enumerate()
        .filter(|(_i, x)| *x != "x")
        .map(|(i, x)| {
            let x_int = x.parse::<i64>().expect("nan");
            ((x_int-(i as i64%x_int))%x_int, x_int)
        })
        .collect();
    ids.sort_by(|a, b| a.1.cmp(&b.1)); // sort ascending by modulo
    println!("{:?}", ids);
    // solve for t where:
    // t = 0 mod 17
    // t = 26 mod 37
    // ...
    // search by sieving, src: https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving
    let (mut a, mut n) = ids.pop().unwrap(); // pop removes last element, not first
    let mut t = a;
    while ids.len() > 0 {
        let (next_a, next_n) = ids.pop().unwrap();
        while t % next_n != next_a {
            t += n;
        }
        println!("{} {} {} {} {}", t, a, n, next_a, next_n);
        a = next_a;
        n *= next_n;
    }
}

// ans 725169163285238