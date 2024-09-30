pub enum Statement {
    
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    println!("{}", std::fs::read_to_string(path).unwrap());
}
