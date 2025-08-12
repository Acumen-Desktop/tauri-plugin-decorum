use cocoa::{
    appkit::{NSView, NSWindow, NSWindowButton, NSColor},
    base::{id, nil, NO},
    foundation::{NSRect, NSPoint},
};
use objc::{msg_send, sel, sel_impl};
use tauri::{Runtime, WebviewWindow};

/// Create an overlay titlebar by injecting JavaScript for draggable regions
pub fn create_overlay_titlebar<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    // Inject draggable titlebar CSS and JS
    let script = r#"
        // Create draggable titlebar area
        const titlebar = document.createElement('div');
        titlebar.id = 'decorum-titlebar';
        titlebar.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            height: 28px;
            -webkit-app-region: drag;
            z-index: 10000;
            pointer-events: auto;
        `;
        
        // Insert at the beginning of body
        if (document.body) {
            document.body.insertBefore(titlebar, document.body.firstChild);
        } else {
            document.addEventListener('DOMContentLoaded', () => {
                document.body.insertBefore(titlebar, document.body.firstChild);
            });
        }
    "#;
    
    window.eval(script)?;
    Ok(())
}

/// Set traffic light button positions
pub fn set_traffic_lights_position<R: Runtime>(
    window: &WebviewWindow<R>, 
    x: f64, 
    y: f64
) -> tauri::Result<()> {
    unsafe {
        let ns_window = window.ns_window()? as id;
        position_traffic_lights(ns_window, x, y);
    }
    Ok(())
}

/// Make window background transparent
pub fn make_window_transparent<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    unsafe {
        let ns_window = window.ns_window()? as id;
        
        // Set window background to transparent
        let clear_color = NSColor::clearColor(nil);
        let _: () = msg_send![ns_window, setBackgroundColor: clear_color];
        let _: () = msg_send![ns_window, setOpaque: NO];
        
        // Make webview background transparent
        window.with_webview(|webview| {
            #[cfg(target_os = "macos")]
            {
                let wkwebview = webview.inner() as id;
                let _: () = msg_send![wkwebview, setDrawsBackground: NO];
            }
        })?;
    }
    Ok(())
}

/// Set window level (for keeping window above/below others)
pub fn set_window_level<R: Runtime>(window: &WebviewWindow<R>, level: i32) -> tauri::Result<()> {
    unsafe {
        let ns_window = window.ns_window()? as id;
        let _: () = msg_send![ns_window, setLevel: level];
    }
    Ok(())
}

/// Internal function to position traffic light buttons
unsafe fn position_traffic_lights(ns_window: id, x: f64, y: f64) {
    let close_button = ns_window.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
    let minimize_button = ns_window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
    let zoom_button = ns_window.standardWindowButton_(NSWindowButton::NSWindowZoomButton);
    
    if close_button.is_null() || minimize_button.is_null() || zoom_button.is_null() {
        return;
    }
    
    let button_spacing = 20.0;
    let buttons = [close_button, minimize_button, zoom_button];
    
    for (i, button) in buttons.iter().enumerate() {
        if !button.is_null() {
            let _frame: NSRect = msg_send![*button, frame];
            let new_origin = NSPoint {
                x: x + (i as f64 * button_spacing),
                y: y,
            };
            button.setFrameOrigin(new_origin);
        }
    }
}