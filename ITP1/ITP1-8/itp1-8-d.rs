fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();
    let mut p = String::new();
    std::io::stdin().read_line(&mut p).unwrap();
    let p = p.trim();
    let ss = format!("{}{}", s, s);

    for i in 0..s.len() {
        if p == &ss[i..i + p.len()]{
            println!("Yes");
            return ;
        }
    }
    println!("No");

}
