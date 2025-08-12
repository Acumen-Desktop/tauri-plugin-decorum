use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, WebviewWindow,
};

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

mod commands;

/// Extensions to [`WebviewWindow`] to access the window management APIs.
pub trait WebviewWindowExt<R: Runtime> {
    /// Create a custom titlebar overlay.
    /// This will remove the default titlebar and create a draggable area for the titlebar.
    fn create_overlay_titlebar(&self) -> tauri::Result<()>;

    /// Set the inset of the traffic lights.
    /// This will move the traffic lights to the specified position.
    /// This is only available on macOS.
    #[cfg(target_os = "macos")]
    fn set_traffic_lights_inset(&self, x: f64, y: f64) -> tauri::Result<()>;

    /// Set the window background to transparent.
    #[cfg(target_os = "macos")]
    fn make_transparent(&self) -> tauri::Result<()>;

    /// Set the window level.
    /// This is only available on macOS.
    #[cfg(target_os = "macos")]
    fn set_window_level(&self, level: i32) -> tauri::Result<()>;
}

impl<R: Runtime> WebviewWindowExt<R> for WebviewWindow<R> {
    fn create_overlay_titlebar(&self) -> tauri::Result<()> {
        // Remove native decorations
        #[cfg(target_os = "windows")]
        {
            self.set_decorations(false)?;
            windows::create_custom_controls(self)?;
        }

        #[cfg(target_os = "linux")]
        {
            self.set_decorations(false)?;
            linux::create_custom_controls(self)?;
        }

        #[cfg(target_os = "macos")]
        {
            // macOS keeps decorations but creates overlay titlebar
            macos::create_overlay_titlebar(self)?;
        }

        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn set_traffic_lights_inset(&self, x: f64, y: f64) -> tauri::Result<()> {
        macos::set_traffic_lights_position(self, x, y)
    }

    #[cfg(target_os = "macos")]
    fn make_transparent(&self) -> tauri::Result<()> {
        macos::make_window_transparent(self)
    }

    #[cfg(target_os = "macos")]
    fn set_window_level(&self, level: i32) -> tauri::Result<()> {
        macos::set_window_level(self, level)
    }
}

/// Initialize the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("decorum")
        .invoke_handler(tauri::generate_handler![commands::show_snap_overlay])
        .setup(|_app, _api| {
            println!("Decorum plugin initialized");
            Ok(())
        })
        .build()
}