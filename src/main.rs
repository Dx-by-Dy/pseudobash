// const EMPTY_ENV_PTR: [*const c_char; 1] = [ptr::null()];

use libc::{c_char, close, dup2, execve, fork, pipe};
use std::ffi::CString;
use std::os::fd::FromRawFd;
use std::ptr;
use std::thread;

struct ExecveCapture {
    command: CString,
    args: Vec<CString>,
}

impl ExecveCapture {
    fn new(command: CString, args: Vec<CString>) -> Self {
        Self { command, args }
    }

    fn execute_in_thread(self) -> thread::JoinHandle<anyhow::Result<CString>> {
        thread::spawn(move || {
            let mut fds = [0; 2];
            unsafe {
                if pipe(fds.as_mut_ptr()) == -1 {
                    todo!()
                }
            }

            let (read_fd, write_fd) = (fds[0], fds[1]);

            let pid = unsafe { fork() };
            match pid {
                -1 => todo!(),
                0 => unsafe {
                    dup2(write_fd, libc::STDOUT_FILENO);
                    close(read_fd);
                    close(write_fd);

                    let mut args_prt: Vec<*const c_char> = vec![self.command.as_ptr()];
                    for arg in &self.args {
                        args_prt.push(arg.as_ptr());
                    }
                    args_prt.push(ptr::null());

                    let env: Vec<*const c_char> = vec![ptr::null()];

                    match execve(args_prt[0], args_prt.as_ptr(), env.as_ptr()) {
                        -1 => {
                            let err = *libc::__errno_location();
                            panic!("Error: {err}");
                        }
                        _ => unreachable!(),
                    }
                },
                _ => {
                    unsafe {
                        close(write_fd);
                    }

                    unsafe {
                        libc::waitpid(pid, &mut 0, 0);
                    }

                    let mut file = unsafe { std::fs::File::from_raw_fd(read_fd) };
                    let mut buffer = Vec::new();

                    match std::io::Read::read_to_end(&mut file, &mut buffer) {
                        Ok(_) => {}
                        Err(e) => panic!("Read error: {e}"),
                    }

                    unsafe { close(read_fd) };
                    CString::new(buffer).map_err(|e| anyhow::Error::new(e))
                }
            }
        })
    }
}

fn main() {
    let captures = vec![
        ExecveCapture::new(CString::new("/usr/bin/echo").unwrap(), vec![]),
        ExecveCapture::new(
            CString::new("/usr/bin/echo").unwrap(),
            vec![
                CString::new("goodbye").unwrap(),
                CString::new("world").unwrap(),
            ],
        ),
        //ExecveCapture::new("/usr/bin/whoami", &[]),
    ];

    let mut threads = Vec::new();

    for capture in captures {
        threads.push(capture.execute_in_thread());
    }

    for (i, thread) in threads.into_iter().enumerate() {
        match thread.join() {
            Ok(Ok(output)) => println!("Thread {}:\n{:?}", i, output),
            Ok(Err(e)) => eprintln!("Thread {} execution error: {}", i, e),
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
