use nix::pty::{openpty, Winsize};
use nix::unistd::{read, write, fork, ForkResult, execvp};
use std::ffi::CString;
use std::os::unix::io::RawFd;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct PtySession {
    pub fd: RawFd,
    pub child_pid: nix::unistd::Pid,
}

impl PtySession {
    pub fn new(command: &str, args: &[&str]) -> Result<Self, Box<dyn std::error::Error>> {
        let pty = openpty(None, None)?;
        let master_fd = pty.master;
        let slave_fd = pty.slave;

        match unsafe { fork() }? {
            ForkResult::Parent { child } => {
                Ok(Self { fd: master_fd, child_pid: child })
            }
            ForkResult::Child => {
                // In child: setup slave PTY as stdin/stdout/stderr
                unsafe {
                    libc::login_tty(slave_fd);
                }
                
                let cmd = CString::new(command)?;
                let c_args: Vec<CString> = args.iter()
                    .map(|&s| CString::new(s).unwrap())
                    .collect();
                let mut c_args_ptr: Vec<*const libc::c_char> = c_args.iter()
                    .map(|s| s.as_ptr())
                    .collect();
                c_args_ptr.push(std::ptr::null());

                execvp(&cmd, &c_args_ptr)?;
                unreachable!()
            }
        }
    }
}
