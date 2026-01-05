<script lang="ts">
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"; // 确保导入
	import { LogicalSize } from "@tauri-apps/api/window";
    import { invoke } from "@tauri-apps/api/core";

    import { onMount } from "svelte";

    let translated_text = $state("");
    let is_loading = $state(false);
	const appWindow = getCurrentWebviewWindow();

    function applyTheme(theme: string) {
        if (theme === 'dark') {
            document.documentElement.classList.add('dark');
        } else if (theme === 'light') {
            document.documentElement.classList.remove('dark');
        } else {
            // system
            if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
                document.documentElement.classList.add('dark');
            } else {
                document.documentElement.classList.remove('dark');
            }
        }
    }

    onMount(() => {
        let unlisten: UnlistenFn; 
        let unlistenSettings: UnlistenFn;
        let unlistenTheme: UnlistenFn;
        
        // 初始化主题
        invoke("get_settings").then((settings: any) => {
            applyTheme(settings.theme);
        });

        const setupListen = async () => {
            unlisten = await listen("shortcut-event", (event) => {
                // 每次触发时，先清空旧内容或显示加载状态
                is_loading = false; 
                if (typeof event.payload === "string") {
                    translated_text = event.payload;
                } else {
                    translated_text = "解析错误";
                }
				requestAnimationFrame(() => {
					setTimeout(async () => {
						const container = document.querySelector(".window-container");
						if (container) {
							const rect = container.getBoundingClientRect();
							console.log("测量到的尺寸: ", rect.width, rect.height);

							await appWindow.setSize(new LogicalSize(
								Math.ceil(rect.width) + 4,
								Math.ceil(rect.height) + 4,
							));
						}
					}, 40)
				})
            });

            unlistenSettings = await listen("go-to-settings", () => {
                window.location.href = "/settings";
            });

            unlistenTheme = await listen("theme-changed", (event) => {
                applyTheme(event.payload as string);
            });
        }
        
        setupListen();

        return () => {
            if (unlisten) unlisten();
            if (unlistenSettings) unlistenSettings();
            if (unlistenTheme) unlistenTheme();
        };
    });
</script>

<main class="window-container" data-tauri-drag-region>
    <div class="card" data-tauri-drag-region>
        {#if translated_text}
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="result-content" onmousedown={(e) => e.stopPropagation()}>
                {translated_text}
            </div>
        {:else}
            <div class="placeholder" data-tauri-drag-region>正在等待划词...</div>
        {/if}
    </div>
</main>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        background-color: transparent;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
        overflow: hidden; /* 隐藏滚动条 */
    }

    .window-container {
        display: inline-block;
		width: max-content; /* 关键：使用 max-content 让内容撑开宽度，而不是受限于初始窗口大小 */
		max-width: 600px;   /* 调整为 600px，通常这是一个更适合阅读的宽度，太宽会像横幅 */
        height: auto;
        padding: 5px; 
        box-sizing: border-box;
    }

    .card {
        background: rgba(255, 255, 255, 0.98);
        border: 1px solid rgba(0, 0, 0, 0.1);
        border-radius: 6px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
        padding: 12px; /* 增加一点内边距让文字不那么拥挤 */
        width: 100%;   /* 填满容器 */
		max-width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        transition: background-color 0.3s, color 0.3s;
        box-sizing: border-box;
    }

    /* 深色模式样式 */
    :global(html.dark) .card {
        background: rgba(30, 30, 30, 0.98);
        border-color: rgba(255, 255, 255, 0.1);
        color: #e0e0e0;
    }

    .result-content {
		-webkit-app-region: no-drag; /* 明确禁止拖拽，允许选择 */
        color: #2c3e50;
        font-size: 14px;
        line-height: 1.6;
        word-break: break-all;
        white-space: pre-wrap; 
        user-select: text; /* 允许文本选择 */
        -webkit-user-select: text;
        cursor: text;
    }

    /* 深色模式文字颜色 */
    :global(html.dark) .result-content {
        color: #e0e0e0;
    }

    .placeholder {
        color: #999;
        font-size: 12px;
        text-align: center;
    }

    /* 深色模式占位符颜色 */
    :global(html.dark) .placeholder {
        color: #aaa;
    }
</style>