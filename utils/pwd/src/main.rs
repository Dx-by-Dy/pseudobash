fn main() {
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.to_string_lossy()),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(-1)
        }
    }
}
