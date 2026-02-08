use nix::pty::openpty;
use nix::unistd::{fork, ForkResult, execvp};
use std::ffi::CString;
use std::os::unix::io::{RawFd, AsRawFd};

pub struct PtySession {
    pub fd: RawFd,
    pub child_pid: nix::unistd::Pid,
}

impl PtySession {
    pub fn new(command: &str, args: &[&str]) -> Result<Self, Box<dyn std::error::Error>> {
        let pty = openpty(None, None)?;
        let master_fd = pty.master.as_raw_fd();
        let slave_fd = pty.slave.as_raw_fd();

        match unsafe { fork() }? {
            ForkResult::Parent { child } => {
                // In parent, we must keep master_fd. 
                // Note: OwnedFd will close on drop, so we might need to leak it or store it differently.
                // For MVP, we'll convert and manage raw fds, but technically pty.master should be kept alive.
                std::mem::forget(pty.master); 
                Ok(Self { fd: master_fd, child_pid: child })
            }
            ForkResult::Child => {
                unsafe {
                    libc::login_tty(slave_fd);
                }
                
                let cmd = CString::new(command)?;
                let mut c_args: Vec<CString> = Vec::new();
                c_args.push(cmd.clone());
                for arg in args {
                    c_args.push(CString::new(*arg)?);
                }

                execvp(&cmd, &c_args)?;
                unreachable!()
            }
        }
    }
}
