// 穴掘り法
use super::SIZE;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn make_maze() -> [[i32; SIZE]; SIZE] {
    // 迷路の２次元配列
    let mut maze = [[1; SIZE]; SIZE];

    // 地点用
    let mut stack: Vec<(usize, usize)> = vec![];

    // ランダムな奇数の数字rowとcolの二つ作成
    let mut rng = thread_rng();

    let mut row: usize = rng.gen_range(1, SIZE - 1);
    if row % 2 == 0 {
        row += 1;
    }
    let mut col: usize = rng.gen_range(1, SIZE - 1);
    if col % 2 == 0 {
        col += 1;
    }

    // その地点を 0（通路）にします
    maze[row][col] = 0;

    // 現在の地点として保持
    stack.push((row, col));

    loop {
        // stack 全部戻ったら終わり
        if stack.len() == 0 {
            break;
        }

        // 2つ先地点が 1（壁）なら ok
        let f = move_maze(&mut maze, row, col);
        if f.0 == false {
            let p = stack.pop().unwrap();
            row = p.0;
            col = p.1;
            continue;
        }

        row = f.1;
        col = f.2;

        stack.push((row, col));
    }

    maze
}

fn move_maze(maze: &mut [[i32; SIZE]; SIZE], row: usize, col: usize) -> (bool, usize, usize) {
    let mut lst: Vec<&str> = vec!["up", "down", "left", "right"];

    loop {
        let d = lst.choose(&mut rand::thread_rng()).unwrap();

        // 1つ先
        let mut r1 = row as i32;
        let mut c1 = col as i32;

        // 2つ先
        let mut r2 = row as i32;
        let mut c2 = col as i32;

        if *d == "up" {
            r1 = r1 - 1;
            r2 = r2 - 2;
        } else if *d == "down" {
            r1 = r1 + 1;
            r2 = r2 + 2;
        } else if *d == "left" {
            c1 = c1 - 1;
            c2 = c2 - 2;
        } else if *d == "right" {
            c1 = c1 + 1;
            c2 = c2 + 2;
        }

        // 2つ先地点が 1（壁）なら ok
        if r2 < SIZE as i32 && c2 < SIZE as i32 && r2 >= 0 && c2 >= 0 {
            if maze[r2 as usize][c2 as usize] == 1 {
                maze[r1 as usize][c1 as usize] = 0;
                maze[r2 as usize][c2 as usize] = 0;
                return (true, r2 as usize, c2 as usize);
            }
        }

        // dを削除
        lst = lst.iter().filter(|x| x != &d).cloned().collect();
        if lst.len() == 0 {
            break;
        }
    }

    (false, 0, 0)
}
