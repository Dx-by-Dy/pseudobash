use {
    libc::{
        WEXITSTATUS, WIFEXITED, WIFSIGNALED, WTERMSIG, close, dup2, execve, exit, fork, pipe,
        waitpid,
    },
    std::{fs::File, io::Read, os::fd::FromRawFd},
};

pub unsafe fn read_write_fd() -> anyhow::Result<[i32; 2]> {
    unsafe {
        let mut fds = [0; 2];
        match pipe(fds.as_mut_ptr()) {
            0 => Ok(fds),
            _ => anyhow::bail!("Pipe failed with errno: {}", *libc::__errno_location()),
        }
    }
}

pub unsafe fn fork_r() -> anyhow::Result<i32> {
    unsafe {
        match fork() {
            -1 => anyhow::bail!("Fork failed with errno: -1"),
            pid @ _ => Ok(pid),
        }
    }
}

pub unsafe fn dup2_r(src: i32, dst: i32) -> anyhow::Result<i32> {
    unsafe {
        match dup2(src, dst) {
            -1 => anyhow::bail!("Dup2 failed with errno: -1"),
            fd @ _ => Ok(fd),
        }
    }
}

pub unsafe fn close_r(fd: i32) -> anyhow::Result<()> {
    unsafe {
        match close(fd) {
            0 => Ok(()),
            _ => anyhow::bail!(
                "Close {fd} failed with errno: {}",
                *libc::__errno_location()
            ),
        }
    }
}

pub unsafe fn wait_pid_r(pid: i32) -> anyhow::Result<i32> {
    unsafe {
        let mut proc_status = 0;
        match waitpid(pid, &mut proc_status, 0) {
            p if p == pid => {
                if WIFEXITED(proc_status) {
                    Ok(WEXITSTATUS(proc_status))
                } else if WIFSIGNALED(proc_status) {
                    anyhow::bail!(format!(
                        "Process killed by signal: {}",
                        WTERMSIG(proc_status)
                    ))
                } else {
                    anyhow::bail!("Process stopped")
                }
            }
            errno @ _ => anyhow::bail!("Waitpid failed with errno: {errno}"),
        }
    }
}

pub unsafe fn execve_r(
    prog: *const i8,
    argv: *const *const i8,
    envp: *const *const i8,
) -> anyhow::Result<()> {
    unsafe {
        match execve(prog, argv, envp) {
            -1 => anyhow::bail!("Execve failed with errno: {}", *libc::__errno_location()),
            _ => unreachable!(),
        }
    }
}

pub unsafe fn read_to_end_file_from_raw(fd: i32, buf: &mut Vec<u8>) -> anyhow::Result<()> {
    unsafe {
        match Read::read_to_end(&mut File::from_raw_fd(fd), buf) {
            Ok(_) => Ok(()),
            Err(e) => anyhow::bail!("Read file error: {e}"),
        }
    }
}

pub unsafe fn exit_r(status: i32) {
    unsafe { exit(status) }
}
