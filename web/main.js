const { invoke } = window.__TAURI__.tauri;

document.getElementById('refresh-button').addEventListener('click', () => {
    invoke('refresh_app');
});