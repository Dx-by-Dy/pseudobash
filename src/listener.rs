use {
    crate::{executor::Executor, global_struct::GS, pipeline::Pipeline},
    std::io::{Write, stdin, stdout},
};

pub struct Listener {}

impl Listener {
    pub fn start(gs: &mut GS) {
        let mut input = String::new();
        loop {
            print!("{} ", gs.settings.get_invitation_input());

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
                unsafe {
                    Executor::execute_pipeline_linear(Pipeline::new(input.as_bytes().to_vec()), gs)
                }
            }

            //println!("{:?}", gs.environment.get_env());

            input.clear();
        }
    }
}
