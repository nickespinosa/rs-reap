//! Reap abandoned child processes from a PID 1-style application.
//!
//! On supported Unix platforms, [`reap_children`] waits for `SIGCHLD` and
//! drains all currently waitable children without blocking. On Windows and
//! Solaris it is a safe no-op, matching the behavior of the original
//! `go-reap` package.

use std::io;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

/// Process id reported by the reaper.
///
/// On Unix this matches `libc::pid_t`. On other platforms it is `i32` so the
/// public API stays portable even though reaping is a no-op there.
#[cfg(all(unix, not(target_os = "solaris")))]
pub type Pid = libc::pid_t;

/// Process id reported by the reaper.
#[cfg(any(
    windows,
    target_os = "solaris",
    not(unix)
))]
pub type Pid = i32;

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
    pids: Option<Sender<Pid>>,
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

    #[cfg(any(
        windows,
        target_os = "solaris",
        not(unix)
    ))]
    {
        let _ = (pids, errors, shutdown, reap_lock);
    }
}

#[cfg(all(unix, not(target_os = "solaris")))]
mod platform {
    use super::{Arc, Pid, Receiver, RwLock, Sender, io};
    use signal_hook::consts::SIGCHLD;
    use signal_hook::iterator::Signals;
    use std::sync::mpsc::TryRecvError;
    use std::time::Duration;

    pub(super) fn reap_children(
        pids: Option<&Sender<Pid>>,
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
                Err(TryRecvError::Empty) => {},
            }

            if signals.pending().next().is_some() {
                let _guard = reap_lock.as_ref().map(|lock| {
                    lock.write()
                        .expect("reap lock poisoned")
                });
                drain_children(pids, errors);
                continue;
            }

            // Polling keeps shutdown responsive without another helper thread
            // or an async runtime.
            if shutdown
                .recv_timeout(Duration::from_millis(50))
                .is_ok()
            {
                return;
            }
        }
    }

    fn drain_children(pids: Option<&Sender<Pid>>, errors: Option<&Sender<io::Error>>) {
        loop {
            let mut status = 0;
            let pid = unsafe {
                // SAFETY: `status` is a valid out-pointer and the remaining
                // arguments are the documented wait4 values.
                libc::wait4(
                    -1,
                    &raw mut status,
                    libc::WNOHANG,
                    std::ptr::null_mut(),
                )
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
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    const REAP_TIMEOUT: Duration = Duration::from_secs(2);
    const SETTLE: Duration = Duration::from_millis(100);

    #[test]
    fn support_matches_target() {
        assert_eq!(
            is_supported(),
            cfg!(all(unix, not(target_os = "solaris")))
        );
    }

    #[test]
    fn pid_is_signed_32() {
        assert_eq!(
            std::mem::size_of::<Pid>(),
            std::mem::size_of::<i32>()
        );
    }

    #[cfg(all(unix, not(target_os = "solaris")))]
    mod unix {
        use super::*;
        use std::collections::HashSet;
        use std::process::{Child, Command};
        use std::sync::mpsc::{Receiver, Sender};
        use std::sync::{Mutex, MutexGuard, OnceLock};
        use std::thread::{self, JoinHandle};

        /// `wait4(-1, …)` is process-global, so reaper tests must not overlap.
        fn exclusive() -> MutexGuard<'static, ()> {
            static GATE: OnceLock<Mutex<()>> = OnceLock::new();
            let guard = GATE
                .get_or_init(|| Mutex::new(()))
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            drain_stray_children();
            guard
        }

        fn drain_stray_children() {
            loop {
                let mut status = 0;
                // SAFETY: `status` is a valid out-pointer; WNOHANG never blocks.
                let pid = unsafe {
                    libc::wait4(
                        -1,
                        &raw mut status,
                        libc::WNOHANG,
                        std::ptr::null_mut(),
                    )
                };
                if pid <= 0 {
                    return;
                }
            }
        }

        struct Reaper {
            pids: Receiver<Pid>,
            errors: Receiver<io::Error>,
            shutdown: Option<Sender<()>>,
            join: Option<JoinHandle<()>>,
        }

        impl Reaper {
            fn start(reap_lock: Option<Arc<RwLock<()>>>) -> Self {
                let (pid_tx, pids) = mpsc::channel();
                let (error_tx, errors) = mpsc::channel();
                let (shutdown_tx, shutdown_rx) = mpsc::channel();
                let join = thread::spawn(move || {
                    reap_children(
                        Some(pid_tx),
                        Some(error_tx),
                        shutdown_rx,
                        reap_lock,
                    );
                });
                thread::sleep(SETTLE);
                Self {
                    pids,
                    errors,
                    shutdown: Some(shutdown_tx),
                    join: Some(join),
                }
            }

            fn expect_pid(&self, expected: Pid) {
                let pid = self
                    .pids
                    .recv_timeout(REAP_TIMEOUT)
                    .expect("timed out waiting for reaped pid");
                assert_eq!(pid, expected);
                assert!(
                    self.errors.try_recv().is_err(),
                    "unexpected reaper error"
                );
            }

            fn expect_no_report(&self, quiet: Duration) {
                assert!(
                    self.pids.recv_timeout(quiet).is_err(),
                    "unexpected pid report"
                );
                assert!(
                    self.errors.try_recv().is_err(),
                    "unexpected reaper error"
                );
            }

            fn shutdown(mut self) {
                if let Some(tx) = self.shutdown.take() {
                    tx.send(()).expect("shutdown send");
                }
                self.join
                    .take()
                    .expect("join handle")
                    .join()
                    .expect("reaper thread");
            }

            fn disconnect_shutdown(mut self) {
                drop(self.shutdown.take());
                self.join
                    .take()
                    .expect("join handle")
                    .join()
                    .expect("reaper thread");
            }
        }

        impl Drop for Reaper {
            fn drop(&mut self) {
                if let Some(tx) = self.shutdown.take() {
                    let _ = tx.send(());
                }
                if let Some(join) = self.join.take() {
                    let _ = join.join();
                }
            }
        }

        fn spawn_sh(script: &str) -> Child {
            Command::new("sh")
                .args(["-c", script])
                .spawn()
                .expect("spawn sh")
        }

        fn child_pid(child: &Child) -> Pid {
            child.id().cast_signed()
        }

        #[test]
        #[allow(clippy::zombie_processes)]
        fn reaps_exited_child() {
            let _gate = exclusive();
            let reaper = Reaper::start(None);
            let child = spawn_sh("exit 0");
            let expected = child_pid(&child);
            reaper.expect_pid(expected);
            reaper.shutdown();
        }

        #[test]
        #[allow(clippy::zombie_processes)]
        fn reaps_non_zero_exit() {
            let _gate = exclusive();
            let reaper = Reaper::start(None);
            let child = spawn_sh("exit 7");
            let expected = child_pid(&child);
            reaper.expect_pid(expected);
            reaper.shutdown();
        }

        #[test]
        #[allow(clippy::zombie_processes)]
        fn reaps_killed_child() {
            let _gate = exclusive();
            let reaper = Reaper::start(None);
            let mut child = spawn_sh("exec sleep 30");
            let expected = child_pid(&child);
            child.kill().expect("kill child");
            reaper.expect_pid(expected);
            reaper.shutdown();
        }

        #[test]
        #[allow(clippy::zombie_processes)]
        fn reaps_multiple_children() {
            let _gate = exclusive();
            let reaper = Reaper::start(None);
            let children: Vec<Child> = (0..5)
                .map(|_| spawn_sh("exit 0"))
                .collect();
            let expected: HashSet<Pid> = children.iter().map(child_pid).collect();

            let mut seen = HashSet::new();
            for _ in 0..expected.len() {
                let pid = reaper
                    .pids
                    .recv_timeout(REAP_TIMEOUT)
                    .expect("timed out waiting for child");
                assert!(
                    expected.contains(&pid),
                    "unexpected pid {pid}"
                );
                assert!(seen.insert(pid), "duplicate pid {pid}");
            }
            assert!(reaper.errors.try_recv().is_err());
            reaper.shutdown();
        }

        #[test]
        #[allow(clippy::zombie_processes)]
        fn none_channels_still_reap() {
            let _gate = exclusive();
            let (shutdown_tx, shutdown_rx) = mpsc::channel();
            let join = thread::spawn(move || {
                reap_children(None, None, shutdown_rx, None);
            });
            thread::sleep(SETTLE);
            let _child = spawn_sh("exit 0");
            thread::sleep(SETTLE + SETTLE);
            shutdown_tx.send(()).expect("shutdown");
            join.join().expect("reaper thread");
        }

        #[test]
        fn shutdown_message_stops_reaper() {
            let _gate = exclusive();
            let reaper = Reaper::start(None);
            reaper.shutdown();
        }

        #[test]
        fn shutdown_disconnect_stops_reaper() {
            let _gate = exclusive();
            let reaper = Reaper::start(None);
            reaper.disconnect_shutdown();
        }

        #[test]
        #[allow(clippy::zombie_processes)]
        fn spurious_sigchld_reports_nothing() {
            let _gate = exclusive();
            let reaper = Reaper::start(None);
            let mut child = spawn_sh("exec sleep 30");

            // SAFETY: signaling the current process with SIGCHLD is valid.
            let rc = unsafe { libc::kill(libc::getpid(), libc::SIGCHLD) };
            assert_eq!(rc, 0, "kill(SIGCHLD) failed");

            reaper.expect_no_report(Duration::from_millis(400));

            let expected = child_pid(&child);
            child.kill().expect("kill child");
            reaper.expect_pid(expected);
            reaper.shutdown();
        }

        #[test]
        #[allow(clippy::zombie_processes)]
        fn reap_lock_read_blocks_reaper_until_released() {
            let _gate = exclusive();
            let lock = Arc::new(RwLock::new(()));
            let reaper = Reaper::start(Some(Arc::clone(&lock)));

            let mut child = spawn_sh("exec sleep 30");
            let expected = child_pid(&child);

            let guard = lock.read().expect("read lock");
            child.kill().expect("kill child");
            reaper.expect_no_report(Duration::from_millis(400));

            drop(guard);
            reaper.expect_pid(expected);
            reaper.shutdown();
        }

        #[test]
        #[allow(clippy::zombie_processes)]
        fn repeated_reap_cycles() {
            let _gate = exclusive();
            let reaper = Reaper::start(None);
            for _ in 0..3 {
                let mut child = spawn_sh("exec sleep 30");
                let expected = child_pid(&child);
                child.kill().expect("kill child");
                reaper.expect_pid(expected);
            }
            reaper.shutdown();
        }
    }

    #[cfg(any(
        windows,
        target_os = "solaris",
        not(unix)
    ))]
    mod unsupported {
        use super::*;

        #[test]
        fn unsupported_platform_is_a_noop() {
            let (pid_tx, pid_rx) = mpsc::channel();
            let (error_tx, error_rx) = mpsc::channel();
            let (_, shutdown_rx) = mpsc::channel();
            reap_children(
                Some(pid_tx),
                Some(error_tx),
                shutdown_rx,
                None,
            );
            assert!(pid_rx.try_recv().is_err());
            assert!(error_rx.try_recv().is_err());
        }

        #[test]
        fn unsupported_accepts_none_channels() {
            let (_, shutdown_rx) = mpsc::channel();
            reap_children(None, None, shutdown_rx, None);
        }
    }
}
