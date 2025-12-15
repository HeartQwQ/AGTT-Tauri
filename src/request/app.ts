import { fetch } from '@tauri-apps/plugin-http';

await fetch('http://127.0.0.1:14242/health', {
    method: 'GET',
});