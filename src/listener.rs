use {
    crate::{SETTINGS, executor::Executor, pipeline::Pipeline},
    std::io::{Write, stdin, stdout},
};

pub struct Listener {}

impl Listener {
    pub fn start() {
        let mut input = String::new();
        loop {
            print!("{} ", SETTINGS.lock().unwrap().get_invitation_input());

            match stdout().flush() {
                Ok(_) => {}
                Err(e) => eprintln!("{}", e),
            }

            match stdin().read_line(&mut input) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            }

            if input.len() > 1 {
                match Pipeline::try_from(&mut input.as_bytes().to_vec()) {
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

            input.clear();
        }
    }
}
