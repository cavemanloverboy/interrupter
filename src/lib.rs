use nix::libc;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_sigint(_sig: i32) {
    INTERRUPTED.store(true, Ordering::Relaxed);
}

unsafe fn install_sigint_handler() -> Result<(), String> {
    let mut sa: libc::sigaction = mem::zeroed();
    sa.sa_sigaction = handle_sigint as usize;
    sa.sa_flags = 0;

    libc::sigemptyset(&mut sa.sa_mask);

    if libc::sigaction(libc::SIGINT, &sa, std::ptr::null_mut()) != 0 {
        return Err("Failed to register SIGINT handler".to_string());
    }

    Ok(())
}

pub struct Interrupter<Handler: FnOnce()> {
    pub handler: Option<Handler>,
}

impl<Handler: FnOnce()> Interrupter<Handler> {
    pub fn poll(&mut self) -> bool {
        if INTERRUPTED.load(Ordering::Relaxed) {
            // Handler should be run exactly once
            if let Some(handler) = self.handler.take() {
                handler();
                return true;
            }
        }
        false
    }
}

pub fn set_handler<Handler: FnOnce()>(
    handler: Handler,
) -> Result<Interrupter<Handler>, InterrupterError> {
    unsafe { install_sigint_handler()? };
    Ok(Interrupter {
        handler: Some(handler),
    })
}

#[derive(Debug)]
pub enum InterrupterError {
    HandlerSet,
    HandlerInstallFailed(String),
}

impl From<String> for InterrupterError {
    fn from(err: String) -> Self {
        InterrupterError::HandlerInstallFailed(err)
    }
}
