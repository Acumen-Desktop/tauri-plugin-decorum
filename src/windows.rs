use tauri::{Runtime, WebviewWindow};
use windows::{
    core::Result,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::*,
    },
};

/// Create custom window controls for Windows
pub fn create_custom_controls<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    // Inject custom controls JavaScript
    let script = r#"
        // Create custom window controls
        const controls = document.createElement('div');
        controls.id = 'decorum-controls';
        controls.style.cssText = `
            position: fixed;
            top: 0;
            right: 0;
            height: 32px;
            display: flex;
            align-items: center;
            z-index: 10001;
            -webkit-app-region: no-drag;
        `;
        
        const minimize = document.createElement('button');
        minimize.innerHTML = '🗕';
        minimize.style.cssText = `
            width: 45px; height: 32px; border: none; background: transparent;
            font-size: 12px; cursor: pointer;
        `;
        minimize.onclick = () => window.__TAURI__.window.getCurrentWindow().minimize();
        
        const maximize = document.createElement('button');
        maximize.innerHTML = '🗖';
        maximize.style.cssText = `
            width: 45px; height: 32px; border: none; background: transparent;
            font-size: 12px; cursor: pointer;
        `;
        maximize.onclick = () => window.__TAURI__.window.getCurrentWindow().toggleMaximize();
        
        const close = document.createElement('button');
        close.innerHTML = '🗙';
        close.style.cssText = `
            width: 45px; height: 32px; border: none; background: transparent;
            font-size: 12px; cursor: pointer; color: #fff;
        `;
        close.onmouseover = () => close.style.backgroundColor = '#e81123';
        close.onmouseout = () => close.style.backgroundColor = 'transparent';
        close.onclick = () => window.__TAURI__.window.getCurrentWindow().close();
        
        controls.appendChild(minimize);
        controls.appendChild(maximize);
        controls.appendChild(close);
        
        if (document.body) {
            document.body.appendChild(controls);
        } else {
            document.addEventListener('DOMContentLoaded', () => {
                document.body.appendChild(controls);
            });
        }
        
        // Add titlebar drag area
        const titlebar = document.createElement('div');
        titlebar.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            right: 135px;
            height: 32px;
            -webkit-app-region: drag;
            z-index: 10000;
        `;
        
        if (document.body) {
            document.body.appendChild(titlebar);
        } else {
            document.addEventListener('DOMContentLoaded', () => {
                document.body.appendChild(titlebar);
            });
        }
    "#;
    
    window.eval(script)?;
    Ok(())
}

/// Show Windows snap overlay (Win + Z functionality)
pub fn show_snap_overlay() -> Result<()> {
    unsafe {
        // Simulate Win + Z key combination
        keybd_event(VK_LWIN.0 as u8, 0, 0, 0);
        keybd_event(b'Z', 0, 0, 0);
        keybd_event(b'Z', 0, KEYEVENTF_KEYUP, 0);
        keybd_event(VK_LWIN.0 as u8, 0, KEYEVENTF_KEYUP, 0);
        
        // Small delay then press Alt to hide numbers
        std::thread::sleep(std::time::Duration::from_millis(50));
        keybd_event(VK_MENU.0 as u8, 0, 0, 0);
        keybd_event(VK_MENU.0 as u8, 0, KEYEVENTF_KEYUP, 0);
    }
    
    Ok(())
}