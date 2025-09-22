use {
    crate::{
        global_struct::{GS, environment::Environment, settings::Settings},
        pb_core::{
            close_r, dup2_r, execve_r, exit_r, fork_r, read_to_end_file_from_raw, read_write_fd,
            wait_pid_r, write_r,
        },
        pipeline::{Delimeter, Pipeline},
        program::Program,
    },
    std::{
        i32, panic, ptr,
        thread::{self},
    },
};

#[derive(Default)]
pub struct Executor {
    last_output: Vec<u8>,
    last_status: i32,
}

impl Executor {
    pub unsafe fn execute_pipeline_linear(&mut self, mut pipeline: Pipeline, gs: &mut GS) {
        loop {
            match pipeline.next(&self.last_output, self.last_status, gs) {
                Ok(Some((delimeter, program))) => {
                    if delimeter == Delimeter::Start
                        && self.last_status == 0
                        && self.last_output.len() > 0
                    {
                        print!("{}", String::from_utf8_lossy(&self.last_output))
                    }

                    match self.program_output(gs, program) {
                        Ok((r_code, stdout, stderr)) => {
                            if r_code == 0 {
                                if stderr.len() > 0 {
                                    eprintln!("{}", String::from_utf8_lossy(&stderr));
                                }
                                self.last_status = 0;
                                self.last_output = stdout
                            } else {
                                self.last_status = -1;
                                self.last_output.clear();
                                eprintln!("Error: {}", String::from_utf8_lossy(&stderr));
                            }
                        }
                        Err(e) => {
                            self.last_status = -1;
                            self.last_output.clear();
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                Ok(None) => {
                    if self.last_status == 0 && self.last_output.len() > 0 {
                        print!("{}", String::from_utf8_lossy(&self.last_output))
                    }
                    self.last_output.clear();
                    self.last_status = 0;
                    return;
                }
                Err(e) => {
                    self.last_status = -1;
                    self.last_output.clear();
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    fn program_output(
        &mut self,
        gs: &mut GS,
        program: Program,
    ) -> anyhow::Result<(i32, Vec<u8>, Vec<u8>)> {
        match program.is_default() {
            true => Ok(gs
                .default_utils
                .execute(program, &mut gs.settings, &mut gs.environment)),
            false => unsafe {
                Self::execute_program_in_thread(
                    program,
                    gs.environment.clone(),
                    gs.settings.clone(),
                )
                .join()
            }
            .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?,
        }
    }

    unsafe fn execute_program_in_thread(
        mut program: Program,
        mut environment: Environment,
        settings: Settings,
    ) -> thread::JoinHandle<anyhow::Result<(i32, Vec<u8>, Vec<u8>)>> {
        thread::spawn(move || {
            let [stdin_read_fd, stdin_write_fd] = unsafe { read_write_fd() }?;
            let [stdout_read_fd, stdout_write_fd] = unsafe { read_write_fd() }?;
            let [stderr_read_fd, stderr_write_fd] = unsafe { read_write_fd() }?;
            let interactive = settings.mode.interactive;

            let mut stdin_data = program.flush_stdin_data();
            let mut args_prt: Vec<*const i8> = program.into_iter().collect();
            args_prt.push(ptr::null());

            let env = environment.get_env()?;
            let mut env_ptr: Vec<*const i8> = env.iter().map(|item| item.as_ptr()).collect();
            env_ptr.push(ptr::null());

            match unsafe { fork_r() }? {
                0 => {
                    panic::set_hook(Box::new(|info| {
                        eprintln!("{}", info);
                        unsafe { exit_r(-1) }
                    }));

                    if !interactive {
                        unsafe { dup2_r(stdin_read_fd, libc::STDIN_FILENO) }.unwrap();
                    }
                    unsafe { close_r(stdin_read_fd) }.unwrap();
                    unsafe { close_r(stdin_write_fd) }.unwrap();

                    if !interactive {
                        unsafe { dup2_r(stdout_write_fd, libc::STDOUT_FILENO) }.unwrap();
                    }
                    unsafe { close_r(stdout_read_fd) }.unwrap();
                    unsafe { close_r(stdout_write_fd) }.unwrap();

                    if !interactive {
                        unsafe { dup2_r(stderr_write_fd, libc::STDERR_FILENO) }.unwrap();
                    }
                    unsafe { close_r(stderr_read_fd) }.unwrap();
                    unsafe { close_r(stderr_write_fd) }.unwrap();

                    unsafe { execve_r(args_prt[0], args_prt.as_ptr(), env_ptr.as_ptr()) }.unwrap();
                    unreachable!()
                }
                pid @ _ => {
                    unsafe { close_r(stdout_write_fd) }?;
                    unsafe { close_r(stderr_write_fd) }?;
                    unsafe { close_r(stdin_read_fd) }?;

                    unsafe { write_r(stdin_write_fd, &mut stdin_data) }?;
                    unsafe { close_r(stdin_write_fd) }?;

                    let r_code = unsafe { wait_pid_r(pid) }?;
                    let mut stdout_buffer = Vec::new();
                    let mut stderr_buffer = Vec::new();

                    if !interactive {
                        unsafe { read_to_end_file_from_raw(stdout_read_fd, &mut stdout_buffer) }?;
                        unsafe { read_to_end_file_from_raw(stderr_read_fd, &mut stderr_buffer) }?;
                    } else {
                        unsafe { close_r(stdout_read_fd) }?;
                        unsafe { close_r(stderr_read_fd) }?;
                    }

                    Ok((r_code, stdout_buffer, stderr_buffer))
                }
            }
        })
    }
}
