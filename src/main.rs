use std::time::Duration;

// ☼
fn main() {
    let mut x=6;
    let mut y=15;
    let mut map = vec![vec![0; 50]; 12];
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if i == 0 || i == 11 || j == 0 || j == 49 {
                print!("█");
            }
            else if i==x && j==y {
                print!("☼"); 
            }
             else {
                print!(" ");
            }
        }
        println!("");
    }
    
    loop{
        x-=1;
        std::thread::sleep(Duration::from_secs(1));
        for (i, row) in map.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if i == 0 || i == 11 || j == 0 || j == 49 {
                    print!("█");
                }
                else if i==x && j==y {
                    print!("☼"); 
                }
                 else {
                    print!(" ");
                }
            }
            println!("");
        }
        if x==49 || x==0 {
            break;
        }
    }
}
