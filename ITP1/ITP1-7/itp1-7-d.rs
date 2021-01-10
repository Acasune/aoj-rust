fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let vec: Vec<usize> = s.split_whitespace().map(|a| a.parse().unwrap()).collect();
    let N = vec[0];
    let M = vec[1];
    let L = vec[2];

    let mut a: Vec<Vec<i32>> = Vec::new();
    let mut b: Vec<Vec<i32>> = Vec::new();
    for _ in 0..N {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let row: Vec<i32> = s.split_whitespace().map(|a| a.parse().unwrap()).collect();
        a.push(row);
    }
    for _ in 0..M {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let row: Vec<i32> = s.split_whitespace().map(|a| a.parse().unwrap()).collect();
        b.push(row);
    }
    for i in 0..N {
        for k in 0..L {
            let mut cnt:i128 =0;
            for j in 0..M {
                cnt = cnt + a[i][j] as i128 * b[j][k] as i128;
            }
            if k == 0 {
                print!("{}", cnt);
            } else {
                print!(" {}", cnt);
            }
        }
        println!("");
    }
}
