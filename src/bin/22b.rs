use std::collections::VecDeque;
use std::collections::HashSet;

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
    let mut p1: VecDeque<usize> = split[0].lines().skip(1).map(|s| s.parse::<usize>().expect("nan")).collect();
    let mut p2: VecDeque<usize> = split[1].lines().skip(1).map(|s| s.parse::<usize>().expect("nan")).collect();
    println!("{:?}", p1);
    println!("{:?}", p2);
    let winner;
    let mut total_games = 0;
    let mut total_rounds = 0;
    if play(&mut p1, &mut p2, 1, &mut total_games, &mut total_rounds) {
        winner = p1;
    } else {
        winner = p2;
    }
    let mut sum = 0;
    for (i, card) in winner.iter().enumerate() {
        sum += card * (winner.len() - i);
    }
    println!("TOTAL GAMES {}", total_games);
    println!("TOTAL ROUNDS {}", total_rounds);
    println!("ans {}", sum)
}

fn play(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>, game: i32, mut total_games: &mut usize, mut total_rounds: &mut usize) -> bool { // true if p1 wins
    println!("STARTING GAME");
    let mut round = 0;
    let mut round_cards: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new(); // takes 24min
    // tried different datatypes for the hashset to optimize, but still just as slow
    // let mut round_cards: HashSet<(String, String)> = HashSet::new(); // takes 28min
    // let mut round_cards: HashSet<String> = HashSet::new(); // takes 29mins
    while p1.len() > 0 && p2.len() > 0 {
        if !round_cards.insert((p1.clone(), p2.clone())) {
        // if !round_cards.insert((p1.iter().map(|&id| id.to_string() + ",").collect(), p2.iter().map(|&id| id.to_string() + ",").collect())) {
        // if !round_cards.insert(p1.iter().map(|&id| id.to_string() + ",").collect::<String>() + "|" + &p2.iter().map(|&id| id.to_string() + ",").collect::<String>()) {
            println!("PLAYER 1 WINS BY SAME CARDS RULE");
            *total_rounds += round;
            *total_games += 1;
            return true;
        }
        round += 1;
        // println!("round_cards {:?}", round_cards);
        println!("game {} round {}", game, round);
        println!("p1 {}", p1.len());
        println!("p2 {}", p2.len());
        let p1first = p1.pop_front().unwrap();
        let p2first = p2.pop_front().unwrap();
        let mut p1wins = p1first > p2first;
        println!("p1first {}", p1first);
        println!("p2first {}", p2first);
        if p1first <= p1.len() && p2first <= p2.len() {
            let mut p1clone = p1.clone();
            p1clone.truncate(p1first);
            let mut p2clone = p2.clone();
            p2clone.truncate(p2first);
            p1wins = play(&mut p1clone, &mut p2clone, game + 1, &mut total_games, &mut total_rounds);
        }
        if p1wins {
            p1.push_back(p1first);
            p1.push_back(p2first);
        } else {
            p2.push_back(p2first);
            p2.push_back(p1first);
        }
    }
    if p1.len() > 0 {
        println!("PLAYER 1 WINS");
    } else {
        println!("PLAYER 2 WINS");
    }
    *total_rounds += round;
    *total_games += 1;
    p1.len() > 0
}
// ans 33441, ran for 24m 40s!!!
// PLAYER 1 WINS
// TOTAL GAMES 13702
// TOTAL ROUNDS 1123258