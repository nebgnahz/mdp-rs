use std::os::unix::io::RawFd;

use termios::{Termios, tcsetattr, ICANON, ECHO, TCSANOW};

pub struct ImmediateInput {
    stdin: RawFd,
    /// The original termios is needed so that we can reset the terminal in the
    /// `Drop` trait.
    original_termios: Termios,
}

impl ImmediateInput {
    pub fn new(fd: RawFd) -> ImmediateInput {
        let termios = match Termios::from_fd(fd) {
            Ok(t) => t,
            Err(e) => panic!("failed to get term from {}, err: {}", fd, e),
        };
        ImmediateInput {
            stdin: fd,
            original_termios: termios,
        }
    }

    pub fn set_immediate(&self) {
        let mut new_termios = self.original_termios.clone();
        new_termios.c_lflag &= !(ICANON | ECHO);
        match tcsetattr(self.stdin, TCSANOW, &new_termios) {
            Ok(_) => {}
            Err(e) => panic!("failed to set attritbute, err: {}", e),
        }
    }
}

impl Drop for ImmediateInput {
    fn drop(&mut self) {
        match tcsetattr(self.stdin, TCSANOW, &self.original_termios) {
            Ok(_) => {}
            Err(_) => {
                panic!("failed to reset the input terminal");
            }
        }
    }
}
