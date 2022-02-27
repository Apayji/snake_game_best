fn main() {
    let map = [[0; 50]; 12];
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if i == 0 || i == 11 || j == 0 || j == 49 {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
