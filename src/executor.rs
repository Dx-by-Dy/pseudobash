use {
    crate::{
        config::CONFIG,
        pb_core::{
            close_r, dup2_r, execve_r, fork_r, read_to_end_file_from_raw, read_write_fd, wait_pid_r,
        },
        pipeline::Pipeline,
        program::Program,
    },
    libc::exit,
    std::{
        ffi::CString,
        i32, panic, ptr,
        thread::{self},
    },
};

pub struct Executor {}

impl Executor {
    pub unsafe fn execute_pipeline_linear(pipeline: Pipeline) -> anyhow::Result<CString> {
        let mut last_output = CString::new("")?;
        for program in pipeline {
            let thread_handle = unsafe { Self::execute_program_in_thread(program) };
            match thread_handle.join() {
                Ok(Ok((r_code, output))) => match r_code {
                    _ => last_output = CString::new(output)?,
                },
                Ok(Err(e)) => anyhow::bail!("Program exited with error: \n{}", e),
                Err(e) => anyhow::bail!("Executor error: {:?}", e),
            }
        }

        Ok(last_output)
    }

    unsafe fn execute_program_in_thread(
        program: Program,
    ) -> thread::JoinHandle<anyhow::Result<(i32, Vec<u8>)>> {
        thread::spawn(move || {
            let [stdout_read_fd, stdout_write_fd] = unsafe { read_write_fd() }?;
            let [stderr_read_fd, stderr_write_fd] = unsafe { read_write_fd() }?;

            match unsafe { fork_r() }? {
                0 => {
                    panic::set_hook(Box::new(|info| {
                        eprintln!("{}", info);
                        unsafe { exit(-1) }
                    }));

                    if !program.interactive {
                        unsafe { dup2_r(stdout_write_fd, libc::STDOUT_FILENO) }.unwrap();
                    }
                    unsafe { close_r(stdout_read_fd) }.unwrap();
                    unsafe { close_r(stdout_write_fd) }.unwrap();

                    if !program.interactive {
                        unsafe { dup2_r(stderr_write_fd, libc::STDERR_FILENO) }.unwrap();
                    }
                    unsafe { close_r(stderr_read_fd) }.unwrap();
                    unsafe { close_r(stderr_write_fd) }.unwrap();

                    let mut args_prt: Vec<*const i8> = vec![program.command.as_ptr()];
                    for arg in &program.args {
                        args_prt.push(arg.as_ptr());
                    }
                    args_prt.push(ptr::null());

                    let env = CONFIG.current_env().unwrap();
                    let mut env_ptr: Vec<*const i8> =
                        env.iter().map(|item| item.as_ptr()).collect();
                    env_ptr.push(ptr::null());

                    // let env_ptr: Vec<*const i8> = vec![ptr::null()];

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

                            if !program.interactive {
                                unsafe { read_to_end_file_from_raw(stdout_read_fd, &mut buffer) }?;
                            } else {
                                unsafe { close_r(stdout_read_fd) }?;
                            }

                            Ok((0, buffer))
                        }
                        _ => {
                            unsafe { close_r(stdout_read_fd) }?;

                            let mut buffer = Vec::new();

                            if !program.interactive {
                                unsafe { read_to_end_file_from_raw(stderr_read_fd, &mut buffer) }?;
                            } else {
                                unsafe { close_r(stderr_read_fd) }?;
                            }

                            anyhow::bail!(CString::new(buffer)?.into_string()?)
                        }
                    }
                }
            }
        })
    }
}
