use crate::domain::AppResult;
use crate::state::{AppState, Notification};
use std::future::Future;

/// Spawns a local future that returns an AppResult.
/// Automatically handles the Err case by showing an error notification.
/// The Ok case is passed to the on_success callback.
pub fn spawn_result_task<F, T, S>(future: F, app_state: AppState, mut on_success: S)
where
    F: Future<Output = AppResult<T>> + 'static,
    T: 'static,
    S: FnMut(T) + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        match future.await {
            Ok(data) => on_success(data),
            Err(e) => {
                app_state.ui.notify(Notification::error(e.user_message()));
            }
        }
    });
}
