use tauri::{
    Invoke,
    plugin::Plugin,
    Runtime,
    window::Window,
};
mod com;

pub struct PenetrablePlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime>PenetrablePlugin<R> {
    pub fn init() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![]),
        }
    }
}

impl<R: Runtime> Plugin<R> for PenetrablePlugin<R> {
    fn name(&self) -> &'static str {
        "penetrable"
    }
    fn created(&mut self, window: Window<R>) {
        com::windows::penetrable(window)
    }
    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}
