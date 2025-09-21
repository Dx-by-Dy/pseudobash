use {
    crate::{executor::Executor, global_struct::GS, pipeline::Pipeline},
    std::io::{Write, stdin, stdout},
};

#[derive(Default)]
pub struct Listener {
    executor: Executor,
}

impl Listener {
    pub fn start(&mut self, gs: &mut GS) {
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
                    self.executor
                        .execute_pipeline_linear(Pipeline::new(input.as_bytes().to_vec()), gs)
                }
            }

            //println!("{:?}", gs.environment.get_env());

            input.clear();
        }
    }
}
