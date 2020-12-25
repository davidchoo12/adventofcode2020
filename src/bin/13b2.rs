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
    // Chinese Remainder Theorem following this tutorial https://www.youtube.com/watch?v=zIFehsBHB8o
    let prod_n = ids.iter().fold(1, |a, e| a * e.1);
    println!("{}", prod_n);
    let mut sum_x = 0;
    while ids.len() > 0 {
        let (a, n) = ids.pop().unwrap(); // pop removes last element, not first
        let ni = prod_n / n;
        // modInverse(ni, n) using Extended Euclidean Algo, adapted from method 2 of https://www.geeksforgeeks.org/multiplicative-inverse-under-modulo-m/
        let mut dividend = ni;
        let mut divisor = n;
        let mut y = 0;
        let mut x = 1;
        while dividend > 1 {
            let q = dividend / divisor;
            let mut t = divisor;
            divisor = dividend % divisor;
            dividend = t;
            t = y;
            y = x - q * y;
            x = t;
        }
        let xi = if x<0 {x+n} else {x};
        sum_x += a * ni * xi;
        println!("{}", sum_x % prod_n);
    }
}

// ans 725169163285238