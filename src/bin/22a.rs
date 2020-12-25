use std::collections::VecDeque;

fn main() {
    let input = "Player 1:
43
21
2
20
36
31
32
37
38
26
48
47
17
16
42
12
45
19
23
14
50
44
29
34
1

Player 2:
40
24
49
10
22
35
28
46
7
41
15
5
39
33
11
8
3
18
4
13
6
25
30
27
9";
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut p1: VecDeque<i32> = split[0].lines().skip(1).map(|s| s.parse::<i32>().expect("nan")).collect();
    let mut p2: VecDeque<i32> = split[1].lines().skip(1).map(|s| s.parse::<i32>().expect("nan")).collect();
    println!("{:?}", p1);
    println!("{:?}", p2);
    while p1.len() > 0 && p2.len() > 0 {
        let p1first = p1.pop_front().unwrap();
        let p2first = p2.pop_front().unwrap();
        if p1first > p2first {
            p1.push_back(p1first);
            p1.push_back(p2first);
        } else {
            p2.push_back(p2first);
            p2.push_back(p1first);
        }
        println!("{:?}", p1);
        println!("{:?}", p2);
    }
    let winner;
    if p1.len() > 0 {
        winner = p1;
    } else {
        winner = p2;
    }
    let mut sum = 0;
    for (i, card) in winner.iter().enumerate() {
        sum += card * (winner.len() - i) as i32;
    }
    println!("ans {}", sum)
}
// ans 33925