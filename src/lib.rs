use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
mod com;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("penetrable")
        .invoke_handler(tauri::generate_handler![])
        .on_window_ready(|window| {
            com::windows::penetrable(window);
        })
        .build()
}

