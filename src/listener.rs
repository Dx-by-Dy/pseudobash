#[derive(Default)]
pub struct Listener {}

impl Listener {
    pub fn listen(&self) -> String {
        let mut result = String::new();
        std::io::stdin()
            .read_line(&mut result)
            .map_err(|e| eprintln!("Input error: {}", e))
            .unwrap();
        result
    }
}
