use std::io;
use std::time::Duration;
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut direction: Option<Direction> = None;
    let mut x = 6;
    let mut y = 15;
    let mut map = vec![vec![0; 50]; 12];
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if i == 0 || i == 11 || j == 0 || j == 49 {
                print!("█");
            } else if i == x && j == y {
                print!("☼");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    match direction {
        None => {
            let mut a = String::new();
            io::stdin().read_line(&mut a).expect("Couldn't read line");
        }
        Some(_) => loop {
            x -= 1;
            std::thread::sleep(Duration::from_secs(1));
            for (i, row) in map.iter().enumerate() {
                for (j, _) in row.iter().enumerate() {
                    if i == 0 || i == 11 || j == 0 || j == 49 {
                        print!("█");
                    } else if i == x && j == y {
                        print!("☼");
                    } else {
                        print!(" ");
                    }
                }
                println!("");
            }
            if x == 49 || x == 0 {
                break;
            }
        },
    }
}
