use {
    crate::{config::CONFIG, executor::Executor, pipeline::Pipeline},
    std::{
        io::{Write, stdin, stdout},
        thread::sleep,
        time::Duration,
    },
};

pub struct Listener {}

impl Listener {
    pub fn start() {
        let mut buffer = String::new();
        let mut input = Vec::new();
        loop {
            match CONFIG.get_invitation_input() {
                Ok(invitation_input) => print!("{} ", invitation_input),
                Err(e) => eprintln!("{}", e),
            }

            match stdout().flush() {
                Ok(_) => {}
                Err(e) => eprintln!("{}", e),
            }

            match stdin().read_line(&mut buffer) {
                Ok(_) => input.append(unsafe { buffer.as_mut_vec() }),
                Err(e) => {
                    eprintln!("{}", e);
                }
            }

            if input.len() > 1 && *input.last().unwrap() == b'\n' {
                match Pipeline::try_from(&mut input) {
                    Ok(pipeline) => match unsafe { Executor::execute_pipeline_linear(pipeline) } {
                        Ok(result) => match result.len() {
                            0 => {}
                            _ => println!("{}", String::from_utf8_lossy(&result)),
                        },
                        Err(e) => eprintln!("{}", e),
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }

            buffer.clear();
            input.clear();

            sleep(Duration::from_millis(100));
        }
    }
}
