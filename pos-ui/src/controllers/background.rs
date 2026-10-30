//! Runs repository calls off the UI thread and hands the result back to it.
//!
//! WHY a thread per call: repository methods block (the mock sleeps, a real
//! client waits for HTTP). Calls are rare and user-driven, so a short-lived
//! thread is simpler than a pool or an async runtime.

use std::thread;

/// Runs `work` on a worker thread, then `on_done` with its result on the UI
/// thread. `on_done` may capture UI-thread-only values (`Rc`, models).
pub fn run_in_background<T, Work, Done>(work: Work, on_done: Done)
where
    T: Send + 'static,
    Work: FnOnce() -> T + Send + 'static,
    Done: FnOnce(T) + 'static,
{
    let (sender, receiver) = futures_channel::oneshot::channel();
    thread::spawn(move || {
        // The receiver is gone only if the event loop has shut down.
        let _ = sender.send(work());
    });
    let spawn_result = slint::spawn_local(async move {
        if let Ok(result) = receiver.await {
            on_done(result);
        }
    });
    if let Err(error) = spawn_result {
        eprintln!("could not schedule UI update: {error}");
    }
}
