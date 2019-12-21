mod ana;
mod bou;

// 迷路の大きさ 奇数で
const SIZE: usize = 31;

fn main() {
    let maze1 = ana::make_maze();
    display(&maze1);

    print!("\n");

    let maze2 = bou::make_maze();
    display(&maze2);
}

fn display(maze: &[[i32; SIZE]; SIZE]) {
    for v1 in maze.iter() {
        for v2 in v1.iter() {
            if *v2 == 1 {
                print!("■");
            } else {
                print!("□");
            }
        }
        print!("\n");
    }
}
