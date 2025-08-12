use tauri::{command, Runtime, WebviewWindow};

#[command]
pub async fn show_snap_overlay<R: Runtime>(
    _window: WebviewWindow<R>,
) -> Result<(), String> {
    // Windows snap overlay functionality
    #[cfg(target_os = "windows")]
    {
        crate::windows::show_snap_overlay().map_err(|e| e.to_string())?;
    }
    
    Ok(())
}
