//! Reap abandoned child processes from a PID 1-style application.
//!
//! On supported Unix platforms, [`reap_children`] waits for `SIGCHLD` and
//! drains all currently waitable children without blocking. On Windows and
//! Solaris it is a safe no-op, matching the behavior of the original
//! `go-reap` package.

use std::io;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

/// Returns whether child-process reaping is supported on this platform.
#[must_use]
pub const fn is_supported() -> bool {
    cfg!(all(unix, not(target_os = "solaris")))
}

/// Runs the long-lived child reaper until `shutdown` receives a message or is
/// disconnected.
///
/// Reaped process IDs are sent to `pids`, when supplied. Unexpected wait
/// errors are sent to `errors`, when supplied. Sending is intentionally
/// blocking, matching the original package's channel semantics.
///
/// If `reap_lock` is supplied, the reaper takes its write lock while draining
/// children. Callers that wait for subprocesses should hold a read lock during
/// that wait to prevent the reaper from claiming the exit status first.
#[allow(clippy::needless_pass_by_value)]
pub fn reap_children(
    pids: Option<Sender<libc::pid_t>>,
    errors: Option<Sender<io::Error>>,
    shutdown: Receiver<()>,
    reap_lock: Option<Arc<RwLock<()>>>,
) {
    #[cfg(all(unix, not(target_os = "solaris")))]
    platform::reap_children(
        pids.as_ref(),
        errors.as_ref(),
        &shutdown,
        reap_lock.as_ref(),
    );

    #[cfg(any(windows, target_os = "solaris", not(unix)))]
    {
        let _ = (pids, errors, shutdown, reap_lock);
    }
}

#[cfg(all(unix, not(target_os = "solaris")))]
mod platform {
    use super::{Arc, Receiver, RwLock, Sender, io};
    use signal_hook::consts::SIGCHLD;
    use signal_hook::iterator::Signals;
    use std::sync::mpsc::TryRecvError;
    use std::time::Duration;

    pub(super) fn reap_children(
        pids: Option<&Sender<libc::pid_t>>,
        errors: Option<&Sender<io::Error>>,
        shutdown: &Receiver<()>,
        reap_lock: Option<&Arc<RwLock<()>>>,
    ) {
        let Ok(mut signals) = Signals::new([SIGCHLD]) else {
            return;
        };

        loop {
            match shutdown.try_recv() {
                Ok(()) | Err(TryRecvError::Disconnected) => return,
                Err(TryRecvError::Empty) => {}
            }

            if signals.pending().next().is_some() {
                let _guard = reap_lock
                    .as_ref()
                    .map(|lock| lock.write().expect("reap lock poisoned"));
                drain_children(pids, errors);
                continue;
            }

            // Polling keeps shutdown responsive without another helper thread
            // or an async runtime.
            if shutdown.recv_timeout(Duration::from_millis(50)).is_ok() {
                return;
            }
        }
    }

    fn drain_children(pids: Option<&Sender<libc::pid_t>>, errors: Option<&Sender<io::Error>>) {
        loop {
            let mut status = 0;
            let pid = unsafe {
                // SAFETY: `status` is a valid out-pointer and the remaining
                // arguments are the documented wait4 values.
                libc::wait4(-1, &raw mut status, libc::WNOHANG, std::ptr::null_mut())
            };

            if pid > 0 {
                if let Some(sender) = pids {
                    let _ = sender.send(pid);
                }
                continue;
            }

            if pid == 0 {
                return;
            }

            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::EINTR) {
                continue;
            }
            match error.raw_os_error() {
                Some(code) if code == libc::ECHILD => return,
                _ => {
                    if let Some(sender) = errors {
                        let _ = sender.send(error);
                    }
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn support_matches_target() {
        assert!(is_supported());
    }

    #[test]
    #[cfg(all(unix, not(target_os = "solaris")))]
    #[allow(clippy::zombie_processes)]
    fn reaps_exited_child() {
        let (pid_tx, pid_rx) = mpsc::channel();
        let (error_tx, error_rx) = mpsc::channel();
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        let reaper = std::thread::spawn(move || {
            reap_children(Some(pid_tx), Some(error_tx), shutdown_rx, None);
        });

        let child = Command::new("sh")
            .args(["-c", "exit 0"])
            .spawn()
            .expect("spawn child");
        let expected_pid = child.id().cast_signed();

        assert_eq!(
            pid_rx.recv_timeout(Duration::from_secs(2)).unwrap(),
            expected_pid
        );
        assert!(error_rx.try_recv().is_err());
        shutdown_tx.send(()).unwrap();
        reaper.join().unwrap();
    }

    #[test]
    #[cfg(any(windows, target_os = "solaris", not(unix)))]
    fn unsupported_platform_is_a_noop() {
        let (pid_tx, pid_rx) = mpsc::channel();
        let (error_tx, error_rx) = mpsc::channel();
        let (_, shutdown_rx) = mpsc::channel();
        reap_children(Some(pid_tx), Some(error_tx), shutdown_rx, None);
        assert!(pid_rx.try_recv().is_err());
        assert!(error_rx.try_recv().is_err());
    }
}
