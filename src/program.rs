use {
    crate::{
        environment::ENVIRONMENT,
        pb_core::{
            close_r, dup2_r, execve_r, fork_r, read_to_end_file_from_raw, read_write_fd, wait_pid_r,
        },
    },
    std::{
        ffi::CString,
        ptr,
        thread::{self},
    },
};

pub struct Program {
    command: CString,
    args: Vec<CString>,
    interactive: bool,
}

impl Program {
    pub fn new(command: CString, args: Vec<CString>, interactive: bool) -> Self {
        Self {
            command,
            args,
            interactive,
        }
    }

    pub fn execute_in_thread(self) -> thread::JoinHandle<anyhow::Result<Vec<u8>>> {
        thread::spawn(move || {
            let [stdout_read_fd, stdout_write_fd] = unsafe { read_write_fd() }?;
            let [stderr_read_fd, stderr_write_fd] = unsafe { read_write_fd() }?;

            match unsafe { fork_r() }? {
                0 => {
                    if !self.interactive {
                        unsafe { dup2_r(stdout_write_fd, libc::STDOUT_FILENO) }?;
                    }
                    unsafe { close_r(stdout_read_fd) }?;
                    unsafe { close_r(stdout_write_fd) }?;

                    if !self.interactive {
                        unsafe { dup2_r(stderr_write_fd, libc::STDERR_FILENO) }?;
                    }
                    unsafe { close_r(stderr_read_fd) }?;
                    unsafe { close_r(stderr_write_fd) }?;

                    let mut args_prt: Vec<*const i8> = vec![self.command.as_ptr()];
                    for arg in &self.args {
                        args_prt.push(arg.as_ptr());
                    }
                    args_prt.push(ptr::null());

                    let env = ENVIRONMENT.get_env()?;
                    let mut env_ptr: Vec<*const i8> =
                        env.iter().map(|item| item.as_ptr()).collect();
                    env_ptr.push(ptr::null());

                    //let env_ptr: Vec<*const i8> = vec![ptr::null()];

                    unsafe { execve_r(args_prt[0], args_prt.as_ptr(), env_ptr.as_ptr()) }?;
                    unreachable!()
                }
                pid @ _ => {
                    unsafe { close_r(stdout_write_fd) }?;
                    unsafe { close_r(stderr_write_fd) }?;

                    match unsafe { wait_pid_r(pid) }? {
                        0 => {
                            unsafe { close_r(stderr_read_fd) }?;

                            let mut buffer = Vec::new();

                            if !self.interactive {
                                unsafe { read_to_end_file_from_raw(stdout_read_fd, &mut buffer) }?;
                            } else {
                                unsafe { close_r(stdout_read_fd) }?;
                            }

                            Ok(buffer)
                        }
                        _ => {
                            unsafe { close_r(stdout_read_fd) }?;

                            let mut buffer = Vec::new();

                            if !self.interactive {
                                unsafe { read_to_end_file_from_raw(stderr_read_fd, &mut buffer) }?;
                            } else {
                                unsafe { close_r(stderr_read_fd) }?;
                            }

                            anyhow::bail!(
                                "Process exited with error: {}",
                                CString::new(buffer)?.into_string()?
                            )
                        }
                    }
                }
            }
        })
    }
}
