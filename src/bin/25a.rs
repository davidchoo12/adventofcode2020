fn main() {
    let input = "12232269
19452773";
    let lines: Vec<&str> = input.lines().collect();
    let pk1: u32 = lines[0].parse().expect("nan");
    let pk2: u32 = lines[1].parse().expect("nan");
    let mut loopsize1 = 0;
    let mut loopsize2 = 0;
    let mut currloop = 0;
    let mut currnum = 1;
    loop {
        currnum = currnum * 7 % 20201227;
        currloop += 1;
        if loopsize1 == 0 && currnum == pk1 {
            loopsize1 = currloop;
            println!("found loopsize1 {}", loopsize1);
        }
        if loopsize2 == 0 && currnum == pk2 {
            loopsize2 = currloop;
            println!("found loopsize2 {}", loopsize2);
        }
        if loopsize1 > 0 && loopsize2 > 0 {
            break;
        }
    }
    let mut ans: u64 = 1;
    for _ in 0..loopsize1 {
        ans = ans * pk2 as u64 % 20201227;
    }
    let mut ans2: u64 = 1;
    for _ in 0..loopsize2 {
        ans2 = ans2 * pk1 as u64 % 20201227;
    }
    println!("ans {}, same ans using other public key {}", ans, ans2);
}
// ans 354320