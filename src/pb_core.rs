use {
    libc::{c_char, close, dup2, execve, fork, pipe},
    std::{ffi::CString, os::fd::FromRawFd, ptr, thread},
};

pub fn pipe_r(buf: &mut [i32; 2]) -> anyhow::Result<()> {
    unsafe {
        match pipe(buf.as_mut_ptr()) {
            0 => Ok(()),
            _ => anyhow::bail!("Pipe failed with errno: {}", *libc::__errno_location()),
        }
    }
}
