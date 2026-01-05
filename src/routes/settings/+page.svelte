<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount, onDestroy } from "svelte";
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';

    let apiKey = $state("");
    let appId = $state("");
    let shortcut = $state("");
    let theme = $state("system");
    let autostart = $state(false);
    let statusMessage = $state("");
    let isRecording = $state(false);

    function handleKeyDown(e: KeyboardEvent) {
        if (!isRecording) return;
        
        e.preventDefault();
        e.stopPropagation();
        
        if (e.key === "Escape") {
            stopRecording();
            return;
        }
        
        const modifiers = [];
        if (e.ctrlKey) modifiers.push("Ctrl");
        if (e.altKey) modifiers.push("Alt");
        if (e.shiftKey) modifiers.push("Shift");
        if (e.metaKey) modifiers.push("Super");
        
        const code = e.code;
        let finalKey = "";

        // 优先使用 code，因为它对应物理按键，且格式固定 (KeyA, Digit1)
        // 这能避免输入法或不同键盘布局导致的字符不匹配问题
        if (code.startsWith("Key")) {
            finalKey = code.slice(3);
        } else if (code.startsWith("Digit")) {
            finalKey = code.slice(5);
        } else if (code === "Space") {
            finalKey = "Space";
        } else if (code.startsWith("Arrow")) {
            finalKey = code.slice(5);
        } else if (code.startsWith("F") && code.length <= 3 && !isNaN(Number(code.slice(1)))) {
             // F1 - F12
             finalKey = code;
        } else {
            const key = e.key.toUpperCase();
            if (["CONTROL", "ALT", "SHIFT", "META"].includes(key)) return;
            
            if (key.length === 1) {
                finalKey = key;
            } else {
                finalKey = e.key; 
            }
        }
        
        if (!finalKey) return;
        
        // 组合快捷键字符串
        shortcut = [...modifiers, finalKey].join("+");
        stopRecording();
    }

    function startRecording() {
        isRecording = true;
        window.addEventListener("keydown", handleKeyDown);
    }

    function stopRecording() {
        isRecording = false;
        window.removeEventListener("keydown", handleKeyDown);
    }

    onDestroy(() => {
        if (typeof window !== 'undefined') {
            window.removeEventListener("keydown", handleKeyDown);
        }
    });

    onMount(async () => {
        try {
            const settings: any = await invoke("get_settings");
            apiKey = settings.apiKey;
            appId = settings.appId;
            shortcut = settings.shortcut || "Ctrl+Q";
            theme = settings.theme || "system";
            
            autostart = await isEnabled();
        } catch (e) {
            console.error("Failed to load settings", e);
        }
    });

    async function toggleAutostart() {
        try {
            if (autostart) {
                await disable();
                autostart = false;
            } else {
                await enable();
                autostart = true;
            }
        } catch (e) {
            console.error("Failed to toggle autostart", e);
            statusMessage = "设置自启动失败: " + e;
        }
    }

    async function saveSettings() {
        try {
            await invoke("save_api_key", { key: apiKey });
            await invoke("save_app_id", { appId: appId });
            await invoke("save_shortcut", { shortcut: shortcut });
            await invoke("save_theme", { theme: theme });
            statusMessage = "保存成功！";
            setTimeout(async () => {
                statusMessage = "";
                const appWindow = getCurrentWebviewWindow();
                await appWindow.hide();
            }, 1000);
        } catch (e) {
            statusMessage = "保存失败: " + e;
        }
    }

    async function goBack() {
        const appWindow = getCurrentWebviewWindow();
        await appWindow.hide();
    }

    function startDrag() {
        getCurrentWebviewWindow().startDragging();
    }
</script>

<main class="settings-container">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="header" data-tauri-drag-region>
        <h2 data-tauri-drag-region>设置</h2>
    </div>
    
    <div class="content">
        <div class="form-group">
            <label for="appId">App ID</label>
            <input id="appId" type="text" bind:value={appId} placeholder="输入 App ID" />
        </div>

        <div class="form-group">
            <label for="apiKey">API Key</label>
            <input id="apiKey" type="password" bind:value={apiKey} placeholder="输入 API Key" />
        </div>

        <div class="form-group">
            <label for="shortcut">快捷键</label>
            <button 
                id="shortcut" 
                class="shortcut-btn {isRecording ? 'recording' : ''}" 
                onclick={startRecording}
                type="button"
            >
                {isRecording ? "请按下快捷键 (Esc 取消)..." : (shortcut || "点击设置快捷键")}
            </button>
        </div>

        <div class="form-group">
            <label for="theme">主题</label>
            <select id="theme" bind:value={theme}>
                <option value="light">浅色模式</option>
                <option value="dark">深色模式</option>
                <option value="system">跟随系统</option>
            </select>
        </div>

        <div class="form-group row">
            <label for="autostart">开机自启动</label>
            <button class="toggle-switch {autostart ? 'active' : ''}" onclick={toggleAutostart} aria-label="Toggle Autostart">
                <div class="toggle-knob"></div>
            </button>
        </div>

        <div class="actions">
            <button onclick={saveSettings}>保存</button>
            <button class="secondary" onclick={goBack}>返回</button>
        </div>

        {#if statusMessage}
            <div class="status">{statusMessage}</div>
        {/if}
    </div>
</main>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        overflow: hidden; /* 隐藏滚动条 */
    }

    .settings-container {
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        height: 100vh; /* Use vh to be sure */
        width: 100vw;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
    }

    .header {
        cursor: move;
        padding: 20px 20px 10px 20px;
        border-bottom: 1px solid #eee;
        flex-shrink: 0; /* Prevent header from shrinking */
    }

    .content {
        padding: 20px;
        display: flex;
        flex-direction: column;
        gap: 15px;
        overflow-y: auto; /* Allow scrolling if content is too long */
        flex-grow: 1;
    }

    h2 {
        margin: 0;
        font-size: 18px;
        color: #333;
        cursor: move;
        user-select: none;
        -webkit-user-select: none;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    .form-group.row {
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
    }

    label {
        font-size: 14px;
        color: #666;
    }

    input, select, .shortcut-btn {
        padding: 8px;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-size: 14px;
    }

    .shortcut-btn {
        background: white;
        color: #333;
        text-align: left;
        cursor: pointer;
        width: 100%;
        box-sizing: border-box;
    }

    .shortcut-btn.recording {
        border-color: #007bff;
        background-color: #e7f1ff;
        color: #007bff;
    }

    /* Toggle Switch Styles */
    .toggle-switch {
        width: 40px;
        height: 20px;
        background-color: #ccc;
        border-radius: 20px;
        position: relative;
        cursor: pointer;
        transition: background-color 0.3s;
        padding: 0;
        border: none;
    }

    .toggle-switch.active {
        background-color: #007bff;
    }

    .toggle-knob {
        width: 16px;
        height: 16px;
        background-color: white;
        border-radius: 50%;
        position: absolute;
        top: 2px;
        left: 2px;
        transition: transform 0.3s;
        box-shadow: 0 1px 3px rgba(0,0,0,0.3);
    }

    .toggle-switch.active .toggle-knob {
        transform: translateX(20px);
    }

    .actions {
        display: flex;
        gap: 10px;
        margin-top: 10px;
    }

    button {
        padding: 8px 16px;
        background: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 14px;
    }

    button:hover {
        background: #0056b3;
    }

    button.secondary {
        background: #6c757d;
    }

    button.secondary:hover {
        background: #545b62;
    }

    .status {
        margin-top: 10px;
        font-size: 12px;
        color: green;
    }
</style>