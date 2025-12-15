use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

use proconio::input;


const INF: usize = 1_000_000_000_000;
enum Action {
    L, U, R, D,
}

fn main() {
    input! { h: usize, w: usize, raw_board: [String; h] };
    let mut board: Vec<Vec<char>> = raw_board.iter().map(|s| s.chars().collect()).collect();
    let mut dist = vec![vec![INF; w]; h];

    let mut warps_by_alphabet: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            if board[i][j] == '.' {
                continue;
            }
            if board[i][j] == '#' {
                continue;
            }
            warps_by_alphabet.entry(board[i][j]).or_insert(vec![]).push((i, j));
        }
    }

    // remove isolated warp points
    for (_, xys) in warps_by_alphabet.iter() {
        if xys.len() == 1 {
            let (p, q) = xys[0];
            board[p][q] = '.';
        }
    }

    let nexts = |p: usize, q: usize, d: usize, dist: &mut Vec<Vec<usize>>, used_warp: &mut HashSet<char>| -> Vec<(usize, usize)> {
        let mut xys = vec![];
        if board[p][q] != '.' && board[p][q] != '#' && !used_warp.contains(&board[p][q]) {
            // println!("from ({}, {}) found warp!", p, q);
            for &(x, y) in warps_by_alphabet[&board[p][q]].iter() {
                if (p, q) == (x, y) {
                    continue;
                }
                if 1 + d >= dist[x][y] {
                    continue;
                }
                dist[x][y] = 1 + d;
                xys.push((x, y));
            }
            used_warp.insert(board[p][q]);
        }

        for action in [Action::L, Action::U, Action::R, Action::D] {
            let (x, y) = match action {
                Action::L => {
                    if q == 0 {
                        continue;
                    }
                    (p, q - 1)
                },
                Action::U => {
                    if p == 0 {
                        continue;
                    }
                    (p - 1, q)
                },
                Action::R => {
                    if q == w - 1 {
                        continue;
                    }
                    (p, q + 1)
                },
                Action::D => {
                    if p == h - 1 {
                        continue;
                    }
                    (p + 1, q)
                },
            };

            if board[x][y] == '#' {
                continue;
            }
            if 1 + d >= dist[x][y] {
                continue;
            }
            dist[x][y] = 1 + d;
            xys.push((x, y));
        }
        // println!("({}, {}) -> {:?}", p, q, xys);
        xys
    };

    dist[0][0] = 0;
    let mut used_warp: HashSet<char> = HashSet::new();

    let mut queue: BinaryHeap<(Reverse<usize>, (usize, usize))> = BinaryHeap::new();
    for (p, q) in nexts(0, 0, 0, &mut dist, &mut used_warp) {
        queue.push((Reverse(1), (p, q)));
    }

    while let Some((Reverse(d), (p, q))) = queue.pop() {
        for (x, y) in nexts(p, q, d, &mut dist, &mut used_warp) {
            queue.push((Reverse(1 + d), (x, y)));
        }
    }

    if dist[h-1][w-1] == INF {
        println!("-1");
    }
    else {
        println!("{}", dist[h-1][w-1]);
    }
}
