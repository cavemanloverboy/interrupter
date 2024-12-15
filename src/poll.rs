use nix::libc::{poll, pollfd, POLLIN};
use nix::unistd;
use std::os::unix::io::RawFd;

static PIPE: (RawFd, RawFd) = (-1, -1);

pub unsafe fn poll_ctrl_c() -> bool {
    let mut fds = [pollfd {
        fd: PIPE.0,
        events: POLLIN,
        revents: 0,
    }];

    // Poll with 0ms timeout for a non-blocking check.
    let ret = poll(fds.as_mut_ptr(), 1, 0);
    if ret > 0 && (fds[0].revents & POLLIN) != 0 {
        let mut buf = [0u8; 1];
        if let Ok(1) = unistd::read(PIPE.0, &mut buf[..]) {
            return true;
        }
    }

    false
}
