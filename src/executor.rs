use {
    crate::{
        global_struct::{GS, environment::Environment, settings::Settings},
        pb_core::{
            close_r, dup2_r, execve_r, exit_r, fork_r, read_to_end_file_from_raw, read_write_fd,
            wait_pid_r,
        },
        pipeline::{Delimeter, Pipeline},
        program::Program,
    },
    std::{
        i32, panic, ptr,
        thread::{self},
    },
};

pub struct Executor {}

impl Executor {
    pub unsafe fn execute_pipeline_linear(mut pipeline: Pipeline, gs: &mut GS) {
        let mut last_output = Vec::new();
        let mut last_status = true;

        loop {
            match pipeline.next(
                &last_output,
                last_status,
                &mut gs.environment,
                &gs.default_utils,
            ) {
                Ok(Some((delimeter, program))) => {
                    if delimeter == Delimeter::Start && last_status && last_output.len() > 0 {
                        println!("{}", String::from_utf8_lossy(&last_output))
                    }

                    last_output = match program.is_default() {
                        true => match gs.default_utils.execute(
                            program,
                            &mut gs.settings,
                            &mut gs.environment,
                        ) {
                            Ok(output) => {
                                last_status = true;
                                output
                            }
                            Err(e) => {
                                last_status = false;
                                eprintln!("Program exited with error: {}", e);
                                vec![]
                            }
                        },
                        false => {
                            let thread_handle = unsafe {
                                Self::execute_program_in_thread(
                                    program,
                                    gs.environment.clone(),
                                    gs.settings.clone(),
                                )
                            };
                            match thread_handle.join() {
                                Ok(Ok((_r_code, output))) => {
                                    last_status = true;
                                    output
                                }
                                Ok(Err(e)) => {
                                    last_status = false;
                                    eprintln!("Program exited with error: {}", e);
                                    vec![]
                                }
                                Err(e) => {
                                    last_status = false;
                                    eprintln!("Executor error: {:?}", e);
                                    vec![]
                                }
                            }
                        }
                    }
                }
                Ok(None) => {
                    if last_status && last_output.len() > 0 {
                        println!("{}", String::from_utf8_lossy(&last_output))
                    }
                    return;
                }
                Err(e) => {
                    last_status = false;
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    unsafe fn execute_program_in_thread(
        program: Program,
        mut environment: Environment,
        settings: Settings,
    ) -> thread::JoinHandle<anyhow::Result<(i32, Vec<u8>)>> {
        thread::spawn(move || {
            let [stdout_read_fd, stdout_write_fd] = unsafe { read_write_fd() }?;
            let [stderr_read_fd, stderr_write_fd] = unsafe { read_write_fd() }?;
            let interactive = settings.is_interactive();

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
                        unsafe { dup2_r(stdout_write_fd, libc::STDOUT_FILENO) }.unwrap();
                    }
                    unsafe { close_r(stdout_read_fd) }.unwrap();
                    unsafe { close_r(stdout_write_fd) }.unwrap();

                    if !interactive {
                        unsafe { dup2_r(stderr_write_fd, libc::STDERR_FILENO) }.unwrap();
                    }
                    unsafe { close_r(stderr_read_fd) }.unwrap();
                    unsafe { close_r(stderr_write_fd) }.unwrap();

                    let env_ptr: Vec<*const i8> = vec![ptr::null()];

                    unsafe { execve_r(args_prt[0], args_prt.as_ptr(), env_ptr.as_ptr()) }.unwrap();
                    unreachable!()
                }
                pid @ _ => {
                    unsafe { close_r(stdout_write_fd) }?;
                    unsafe { close_r(stderr_write_fd) }?;

                    match unsafe { wait_pid_r(pid) }? {
                        0 => {
                            unsafe { close_r(stderr_read_fd) }?;
                            let mut buffer = Vec::new();

                            if !interactive {
                                unsafe { read_to_end_file_from_raw(stdout_read_fd, &mut buffer) }?;
                            } else {
                                unsafe { close_r(stdout_read_fd) }?;
                            }

                            Ok((0, buffer))
                        }
                        r_code @ _ => {
                            unsafe { close_r(stdout_read_fd) }?;
                            let mut buffer = Vec::new();

                            if !interactive {
                                unsafe { read_to_end_file_from_raw(stderr_read_fd, &mut buffer) }?;
                            } else {
                                unsafe { close_r(stderr_read_fd) }?;
                            }

                            anyhow::bail!(format!(
                                "Exit code {} with output: {}",
                                r_code,
                                String::from_utf8_lossy(&buffer)
                            ))
                        }
                    }
                }
            }
        })
    }
}
