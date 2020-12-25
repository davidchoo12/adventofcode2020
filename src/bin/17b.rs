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
    let mut space: VecDeque<VecDeque<VecDeque<VecDeque<char>>>> = VecDeque::new();
    let mut v: VecDeque<VecDeque<VecDeque<char>>> = VecDeque::new();
    v.push_back(lines);
    space.push_back(v);
    println!("{:?}", space);
    
    for _cycle in 0..6 {
        let mut active_total = 0;
        let mut next_space: VecDeque<VecDeque<VecDeque<VecDeque<char>>>> = space.clone();
        next_space.push_front(next_space[0].clone());
        next_space.push_back(next_space[0].clone());
        for w in 0..next_space.len() {
            let clone = next_space[w].get(0).unwrap().clone();
            next_space[w].push_front(clone.clone());
            next_space[w].push_back(clone);
            for z in 0..next_space[w].len() {
                println!("w{} z{}", w, z);
                let clone = next_space[w][z].get(0).unwrap().clone();
                next_space[w][z].push_front(clone.clone());
                next_space[w][z].push_back(clone);
                for y in 0..next_space[w][z].len() {
                    next_space[w][z][y].push_front('.');
                    next_space[w][z][y].push_back('.');
                    for x in 0..next_space[w][z][y].len() {
                        let mut active_count = 0;
                        for dw in -1..=1 {
                            let ww = w as i32-1+dw;
                            if ww < 0 || ww >= space.len() as i32 {
                                continue;
                            }
                            for dz in -1..=1 {
                                let zz = z as i32-1+dz;
                                if zz < 0 || zz >= space[0].len() as i32 {
                                    continue;
                                }
                                for dy in -1..=1 {
                                    let yy = y as i32-1+dy;
                                    if yy < 0 || yy >= space[0][0].len() as i32 {
                                        continue;
                                    }
                                    for dx in -1..=1 {
                                        let xx = x as i32-1+dx;
                                        if xx < 0 || xx >= space[0][0][0].len() as i32 || dw == 0 && dz == 0 && dy == 0 && dx == 0 {
                                            continue;
                                        }
                                        if space[ww as usize][zz as usize][yy as usize][xx as usize] == '#' {
                                            active_count += 1;
                                        }
                                    }
                                }
                            }
                        }
                        let ws = next_space.len()-1;
                        let zs = next_space[0].len()-1;
                        let ys = next_space[0][0].len()-1;
                        let xs = next_space[0][0][0].len()-1;
                        let curr = match (w,z,y,x) {
                            (0,_,_,_) | (_,0,_,_) | (_,_,0,_) | (_,_,_,0) => '.',
                            (ww,zz,yy,xx) => if ww == ws || zz == zs || yy == ys || xx == xs {'.'} else {next_space[ww][zz][yy][xx]}
                        };
                        next_space[w][z][y][x] = match (curr, active_count) {
                            ('#',2) | (_,3) => { active_total += 1; '#' },
                            _ => '.'
                        };
                        print!("{}", next_space[w][z][y][x]);
                    }
                    println!();
                }
                println!("\n");
            }
            println!("\n\n");
        }
        println!("active total {}\n\n", active_total);
        space = next_space;
    }
}

// ans 1504