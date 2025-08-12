use tauri::{Runtime, WebviewWindow};

/// Create custom window controls for Linux
pub fn create_custom_controls<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    // Inject custom controls for Linux
    let script = r#"
        // Create custom window controls for Linux
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
        minimize.innerHTML = '−';
        minimize.style.cssText = `
            width: 40px; height: 32px; border: none; background: transparent;
            font-size: 16px; cursor: pointer; font-weight: bold;
        `;
        minimize.onclick = () => window.__TAURI__.window.getCurrentWindow().minimize();
        
        const maximize = document.createElement('button');
        maximize.innerHTML = '□';
        maximize.style.cssText = `
            width: 40px; height: 32px; border: none; background: transparent;
            font-size: 14px; cursor: pointer;
        `;
        maximize.onclick = () => window.__TAURI__.window.getCurrentWindow().toggleMaximize();
        
        const close = document.createElement('button');
        close.innerHTML = '×';
        close.style.cssText = `
            width: 40px; height: 32px; border: none; background: transparent;
            font-size: 18px; cursor: pointer; font-weight: bold;
        `;
        close.onmouseover = () => close.style.backgroundColor = '#cc4125';
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
            right: 120px;
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