<template>
    <VContainer class="h-100">
        <div ref="termElement" class="h-100" />
    </VContainer>
</template>

<script setup lang="ts">
import "xterm/css/xterm.css";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { WebLinksAddon } from "@xterm/addon-web-links";
import { open } from "@tauri-apps/plugin-shell";

const route = useRoute("index-project-id-pty-sessionId");
const theme = useTheme();
const notify = useNotify();
const { t } = useI18n();

const termElement = ref<HTMLDivElement>();

let terminal: Terminal;
let fitAddon: FitAddon;
let webLinksAddon: WebLinksAddon;
let isActive = false;

onMounted(async () => {
    isActive = true;

    terminal = new Terminal({
        theme: {
            background: theme.current.value.colors.background,
            foreground: theme.current.value.colors["on-surface"],
            cursor: theme.current.value.colors["on-surface"],
            selectionForeground: theme.current.value.colors.primary,
            selectionBackground: theme.current.value.colors["primary-lighten-3"]
        }
    });

    fitAddon = new FitAddon();
    webLinksAddon = new WebLinksAddon((_, uri) => {
        open(uri);
    });

    terminal.loadAddon(fitAddon);
    terminal.loadAddon(webLinksAddon);

    terminal.open(termElement.value!);

    fitAddon.fit();
    await resizePtySession({ rows: terminal.rows, cols: terminal.cols });

    await restoreHistory();

    terminal.onData((data) => writeToPtySession(data));
    terminal.onResize((data) => resizePtySession(data));

    await startPtySessionReadLoop();
});

onBeforeUnmount(() => {
    isActive = false;

    resizeObserver.stop();

    fitAddon.dispose();
    webLinksAddon.dispose();
    terminal.dispose();
});

const resizeObserver = useResizeObserver(termElement, () => {
    if (!isActive) return;

    fitAddon.fit();
});

async function restoreHistory() {
    const readHistoryResult = await commands.ptySessionGetReadHistory(route.params.sessionId);

    if (readHistoryResult.status === "error") {
        notify.error(t("ptySession.open.error"));
        return;
    }

    if (!readHistoryResult.data) {
        return;
    }

    terminal.write(readHistoryResult.data);
}

async function writeToPtySession(data: string) {
    if (!isActive) return;

    const writeResult = await commands.ptySessionWrite(route.params.sessionId, data);
    if (writeResult.status === "error") {
        console.error("failed to write data");
    }

    console.log("write", data);
}

async function startPtySessionReadLoop() {
    while (isActive) {
        const readResult = await commands.ptySessionRead(route.params.sessionId);

        if (!isActive) break;

        if (readResult.status === "error") {
            console.error("Error while reading data");
            break;
        }

        console.log("read");

        terminal.write(readResult.data);
    }
}

async function resizePtySession(resizeContract: PtySessionResizeContract) {
    if (!isActive) return;

    const resizeResult = await commands.ptySessionResize(route.params.sessionId, resizeContract);
    if (resizeResult.status === "error") {
        console.error("Error while resizing pty");
    }

    /*console.log("TerminalRows / Pty", terminal.rows, resizeContract.rows);
    console.log("TerminalCols / Pty", terminal.cols, resizeContract.cols);*/
}
</script>
