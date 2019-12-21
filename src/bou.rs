// 棒倒し法
use super::SIZE;
use rand::seq::SliceRandom;

pub fn make_maze() -> [[i32; SIZE]; SIZE] {
    // 迷路の２次元配列
    let mut maze: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    // 基本形
    let mut c = 2;
    for i1 in 0..SIZE {
        for i2 in 0..SIZE {
            if i1 == 0 {
                maze[i1][i2] = 1;
            }

            if i1 == SIZE - 1 {
                maze[i1][i2] = 1;
            }

            if i2 == 0 {
                maze[i1][i2] = 1;
            }

            if i2 == SIZE - 1 {
                maze[i1][i2] = 1;
            }

            if i1 != 0 && i1 != SIZE - 1 && i2 != 0 && i2 != SIZE - 1 {
                if i1 % 2 == 0 && i2 % 2 == 0 {
                    maze[i1][i2] = c;
                    c += 1;
                }
            }
        }
    }

    // 2以上の場所を見る
    for i1 in 0..SIZE {
        for i2 in 0..SIZE {
            if i1 == 2 && maze[i1][i2] >= 2 {
                down_bou1(&mut maze, i1, i2);
            } else if i1 > 2 && maze[i1][i2] >= 2 {
                down_bou2(&mut maze, i1, i2);
            }
        }
    }

    maze
}

fn down_bou1(maze: &mut [[i32; SIZE]; SIZE], row: usize, col: usize) {
    let mut lst: Vec<&str> = vec!["up", "down", "left", "right"];

    loop {
        // 倒す方角
        let d = lst.choose(&mut rand::thread_rng()).unwrap();

        // 1つ先
        let mut r1 = row as i32;
        let mut c1 = col as i32;

        if *d == "up" {
            r1 = r1 - 1;
        } else if *d == "down" {
            r1 = r1 + 1;
        } else if *d == "left" {
            c1 = c1 - 1;
        } else if *d == "right" {
            c1 = c1 + 1;
        }

        // 1つ先が 0 なら ok
        if maze[r1 as usize][c1 as usize] == 0 {
            maze[r1 as usize][c1 as usize] = 1;
            maze[row as usize][col as usize] = 1;
            return;
        }

        // dを削除
        lst = lst.iter().filter(|x| x != &d).cloned().collect();
        if lst.len() == 0 {
            break;
        }
    }
}

fn down_bou2(maze: &mut [[i32; SIZE]; SIZE], row: usize, col: usize) {
    let mut lst: Vec<&str> = vec!["down", "left", "right"];

    loop {
        // 倒す方角
        let d = lst.choose(&mut rand::thread_rng()).unwrap();

        // 1つ先
        let mut r1 = row as i32;
        let mut c1 = col as i32;

        if *d == "down" {
            r1 = r1 + 1;
        } else if *d == "left" {
            c1 = c1 - 1;
        } else if *d == "right" {
            c1 = c1 + 1;
        }

        // 1つ先が 0 なら ok
        if maze[r1 as usize][c1 as usize] == 0 {
            maze[r1 as usize][c1 as usize] = 1;
            maze[row as usize][col as usize] = 1;
            return;
        }

        // dを削除
        lst = lst.iter().filter(|x| x != &d).cloned().collect();
        if lst.len() == 0 {
            break;
        }
    }
}
