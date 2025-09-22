fn main() {
    for (idx, arg) in std::env::args().enumerate() {
        match idx {
            0 => {}
            1 => print!("{}", arg),
            2.. => print!(" {}", arg),
        }
    }
    print!("\n")
}
