use std::io::Read;

fn main() {
    let args = std::env::args().enumerate();

    let mut buf = String::new();
    if args.len() == 1 {
        match std::io::stdin().read_to_string(&mut buf) {
            Ok(_) => print!("{}", buf),
            Err(e) => eprintln!("{}", e),
        }
    } else {
        for (idx, arg) in args {
            if idx == 0 {
                continue;
            }
            buf.clear();
            match std::fs::File::open(&arg).and_then(|mut file| file.read_to_string(&mut buf)) {
                Ok(_) => print!("{}", buf),
                Err(e) => eprintln!("{}: {}", e, arg),
            }
        }
    }
}
