use std::collections::VecDeque;
fn main() {
    let input = "#.##.##.
.##..#..
....#..#
.##....#
#..##...
.###..#.
..#.#..#
.....#..";
    let lines: VecDeque<VecDeque<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut space: VecDeque<VecDeque<VecDeque<char>>> = VecDeque::new();
    space.push_back(lines);
    println!("{:?}", space);
    
    for _cycle in 0..6 {
        let mut active_total = 0;
        let mut next_space: VecDeque<VecDeque<VecDeque<char>>> = space.clone();
        next_space.push_front(next_space[0].clone());
        next_space.push_back(next_space[0].clone());
        for z in 0..next_space.len() {
            let clone = next_space[z].get(0).unwrap().clone();
            next_space[z].push_front(clone.clone());
            next_space[z].push_back(clone);
            for y in 0..next_space[z].len() {
                next_space[z][y].push_front('.');
                next_space[z][y].push_back('.');
                for x in 0..next_space[z][y].len() {
                    let mut active_count = 0;
                    for dz in -1..=1 {
                        let zz = z as i32-1+dz;
                        if zz < 0 || zz >= space.len() as i32 {
                            continue;
                        }
                        for dy in -1..=1 {
                            let yy = y as i32-1+dy;
                            if yy < 0 || yy >= space[0].len() as i32 {
                                continue;
                            }
                            for dx in -1..=1 {
                                let xx = x as i32-1+dx;
                                if xx < 0 || xx >= space[0][0].len() as i32 || dz == 0 && dy == 0 && dx == 0 {
                                    continue;
                                }
                                if space[zz as usize][yy as usize][xx as usize] == '#' {
                                    active_count += 1;
                                }
                            }
                        }
                    }
                    let zs = next_space.len()-1;
                    let ys = next_space[0].len()-1;
                    let xs = next_space[0][0].len()-1;
                    let curr = match (z,y,x) {
                        (0,_,_) | (_,0,_) | (_,_,0) => '.',
                        (zz,yy,xx) => if zz == zs || yy == ys || xx == xs {'.'} else {next_space[zz][yy][xx]}
                    };
                    next_space[z][y][x] = match (curr, active_count) {
                        ('#',2) | (_,3) => { active_total += 1; '#' },
                        _ => '.'
                    };
                    print!("{}", next_space[z][y][x]);
                }
                println!();
            }
            println!("\n");
        }
        println!("active total {}\n\n", active_total);
        space = next_space;
    }
}

// ans 273