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

const route = useRoute("index-project-id-terminal-terminalId");
const theme = useTheme();

const termElement = ref<HTMLDivElement>();

let terminal: Terminal;
let fitAddon: FitAddon;
let isActive = true;

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
    const webLinksAddon = new WebLinksAddon((_, uri) => {
        open(uri);
    });

    terminal.loadAddon(fitAddon);
    terminal.loadAddon(webLinksAddon);
    terminal.open(termElement.value!);
    fitAddon.fit();
    await spawn();
});

onUnmounted(() => {
    isActive = false;
    terminal!.dispose();
    fitAddon!.dispose();
});

useResizeObserver(termElement, () => {
    fitAddon.fit();
});

async function spawn() {
    const readHistoryResult = await commands.ptyGetReadHistory(route.params.terminalId);

    if (readHistoryResult.status === "error") {
        console.error("Failed to create pty");
        return;
    }

    let combinedString = "";
    for (const result of readHistoryResult.data) {
        combinedString += result;
    }
    if (combinedString) {
        terminal.write(combinedString);
    }

    terminal.onData((data) => writeData(data));
    terminal.onResize((data) => ptyResize(data));
    await readData();
}

async function writeData(data: string) {
    if (!isActive) return;

    const writeResult = await commands.ptyWrite(route.params.terminalId, data);
    if (writeResult.status === "error") {
        console.error("failed to write data");
    }
}

async function readData() {
    while (isActive) {
        const readResult = await commands.ptyRead(route.params.terminalId);

        if (!isActive) break;

        if (readResult.status === "error") {
            console.error("Error while reading data");
            break;
        }

        terminal.write(readResult.data);
    }
}

async function ptyResize(resizeContract: PtyResizeContract) {
    if (!isActive) return;

    const resizeResult = await commands.ptyResize(route.params.terminalId, resizeContract);
    if (resizeResult.status === "error") {
        console.error("Error while resizing pty");
    }
}
</script>
