// const EMPTY_ENV_PTR: [*const c_char; 1] = [ptr::null()];

use {
    libc::{c_char, close, dup2, execve, fork, pipe},
    pseudobash::pb_core::pipe_r,
    std::{ffi::CString, os::fd::FromRawFd, ptr, thread},
};

struct ExecveCapture {
    command: CString,
    args: Vec<CString>,
}

impl ExecveCapture {
    fn new(command: CString, args: Vec<CString>) -> Self {
        Self { command, args }
    }

    fn execute_in_thread(self) -> thread::JoinHandle<anyhow::Result<Vec<u8>>> {
        thread::spawn(move || {
            let mut stdout_fds = [0; 2];
            pipe_r(&mut stdout_fds)?;
            let (stdout_read_fd, stdout_write_fd) = (stdout_fds[0], stdout_fds[1]);

            let mut stderr_fds = [0; 2];
            pipe_r(&mut stderr_fds)?;
            let (stderr_read_fd, stderr_write_fd) = (stderr_fds[0], stderr_fds[1]);

            let pid = unsafe { fork() };
            match pid {
                -1 => anyhow::bail!("Fork failed with errno: -1"),
                0 => {
                    match unsafe { dup2(stdout_write_fd, libc::STDOUT_FILENO) } {
                        -1 => anyhow::bail!("Dup2 failed with errno: -1"),
                        _ => {}
                    }

                    match unsafe { close(stdout_read_fd) } {
                        0 => {}
                        _ => anyhow::bail!("Close failed with errno: {}", unsafe {
                            *libc::__errno_location()
                        }),
                    }

                    match unsafe { close(stdout_write_fd) } {
                        0 => {}
                        _ => anyhow::bail!("Close failed with errno: {}", unsafe {
                            *libc::__errno_location()
                        }),
                    }

                    match unsafe { dup2(stderr_write_fd, libc::STDERR_FILENO) } {
                        -1 => anyhow::bail!("Dup2 failed with errno: -1"),
                        _ => {}
                    }

                    match unsafe { close(stderr_read_fd) } {
                        0 => {}
                        _ => anyhow::bail!("Close failed with errno: {}", unsafe {
                            *libc::__errno_location()
                        }),
                    }

                    match unsafe { close(stderr_write_fd) } {
                        0 => {}
                        _ => anyhow::bail!("Close failed with errno: {}", unsafe {
                            *libc::__errno_location()
                        }),
                    }

                    let mut args_prt: Vec<*const c_char> = vec![self.command.as_ptr()];
                    for arg in &self.args {
                        args_prt.push(arg.as_ptr());
                    }
                    args_prt.push(ptr::null());

                    let env: Vec<*const c_char> = vec![ptr::null()];

                    match unsafe { execve(args_prt[0], args_prt.as_ptr(), env.as_ptr()) } {
                        -1 => anyhow::bail!("Execve failed with errno: {}", unsafe {
                            *libc::__errno_location()
                        }),
                        _ => unreachable!(),
                    }
                }
                _ => {
                    match unsafe { close(stdout_write_fd) } {
                        0 => {}
                        _ => anyhow::bail!("Close failed with errno: {}", unsafe {
                            *libc::__errno_location()
                        }),
                    }

                    match unsafe { close(stderr_write_fd) } {
                        0 => {}
                        _ => anyhow::bail!("Close failed with errno: {}", unsafe {
                            *libc::__errno_location()
                        }),
                    }

                    let mut proc_status = 0;
                    match unsafe { libc::waitpid(pid, &mut proc_status, 0) } {
                        p if p == pid => {}
                        errno @ _ => anyhow::bail!("Waitpid failed with errno: {errno}"),
                    }

                    match proc_status {
                        0 => {
                            match unsafe { close(stderr_read_fd) } {
                                0 => {}
                                _ => anyhow::bail!("Close failed with errno: {}", unsafe {
                                    *libc::__errno_location()
                                }),
                            }

                            let mut strout_file =
                                unsafe { std::fs::File::from_raw_fd(stdout_read_fd) };
                            let mut buffer = Vec::new();

                            match std::io::Read::read_to_end(&mut strout_file, &mut buffer) {
                                Ok(_) => match unsafe { close(stdout_read_fd) } {
                                    0 => {}
                                    _ => anyhow::bail!("Close failed with errno: {}", unsafe {
                                        *libc::__errno_location()
                                    }),
                                },
                                Err(e) => anyhow::bail!("Read file error: {e}"),
                            }

                            Ok(buffer)
                        }
                        _ => {
                            match unsafe { close(stdout_read_fd) } {
                                0 => {}
                                _ => anyhow::bail!("Close failed with errno: {}", unsafe {
                                    *libc::__errno_location()
                                }),
                            }

                            let mut strerr_file =
                                unsafe { std::fs::File::from_raw_fd(stderr_read_fd) };
                            let mut buffer = Vec::new();

                            match std::io::Read::read_to_end(&mut strerr_file, &mut buffer) {
                                Ok(_) => match unsafe { close(stderr_read_fd) } {
                                    0 => {}
                                    _ => anyhow::bail!("Close failed with errno: {}", unsafe {
                                        *libc::__errno_location()
                                    }),
                                },
                                Err(e) => anyhow::bail!("Read file error: {e}"),
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

fn main() {
    let captures = vec![
        ExecveCapture::new(
            CString::new("/home/none/Rust_test/target/release/Rust_test").unwrap(),
            vec![],
        ),
        // ExecveCapture::new(
        //     CString::new("/usr/bin/echo").unwrap(),
        //     vec![
        //         CString::new("goodbye").unwrap(),
        //         CString::new("world").unwrap(),
        //     ],
        // ),
        //ExecveCapture::new("/usr/bin/whoami", &[]),
    ];

    let mut threads = Vec::new();

    for capture in captures {
        threads.push(capture.execute_in_thread());
    }

    for (i, thread) in threads.into_iter().enumerate() {
        match thread.join() {
            Ok(Ok(output)) => println!("Thread {}:\n{:?}", i, CString::new(output)),
            Ok(Err(e)) => eprintln!("Thread {} execution error: \n{}", i, e),
            Err(_) => eprintln!("Thread {} killed", i),
        }
    }
}

// fn main() -> anyhow::Result<()> {

//     let args = [CString::new("/usr/bin/fish")?];
//     let mut args_ptr: Vec<*const c_char> = args.iter().map(|c_string| c_string.as_ptr()).collect();
//     args_ptr.push(ptr::null());

//     let mut env: Vec<CString> = std::env::vars()
//         .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
//         .collect();
//     env.push(CString::new("ZZZ=value")?);
//     let mut env_ptr: Vec<*const c_char> = env.iter().map(|c_string| c_string.as_ptr()).collect();
//     env_ptr.push(ptr::null());

//     unsafe {
//         match execve(args[0].as_ptr(), args_ptr.as_ptr(), EMPTY_ENV_PTR.as_ptr()) {
//             -1 => {
//                 let err = *libc::__errno_location();
//                 println!("Error: {err}");
//             }
//             _ => unreachable!(),
//         }
//     }

//     Ok(())
// }
