use std::io::Read;

fn main() {
    let args = std::env::args().enumerate();
    let args_len = args.len() - 1;

    let mut buf = String::new();
    for (idx, arg) in args {
        if idx == 0 {
            continue;
        }
        buf.clear();
        match std::fs::File::open(&arg) {
            Ok(mut file) => match file.read_to_string(&mut buf) {
                Ok(size) => match idx {
                    val if val == args_len => print!(
                        "{} {} {}",
                        buf.split(|sym| sym == '\n').count(),
                        buf.split_whitespace().count(),
                        size
                    ),
                    _ => println!(
                        "{} {} {}",
                        buf.split(|sym| sym == '\n').count(),
                        buf.split_whitespace().count(),
                        size
                    ),
                },
                Err(e) => eprintln!("{}: {}", e, arg),
            },
            Err(e) => eprintln!("{}: {}", e, arg),
        }
    }
}
