fn main() {
    match std::env::current_dir() {
        Ok(path) => print!("{}", path.to_string_lossy()),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(-1)
        }
    }
}
