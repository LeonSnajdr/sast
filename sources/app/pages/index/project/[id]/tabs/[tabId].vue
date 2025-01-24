<template>
    <VContainer class="h-100">
        <div ref="termElement" />
    </VContainer>
</template>

<script setup lang="ts">
import { WebLinksAddon } from "@xterm/addon-web-links";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import "xterm/css/xterm.css";
import { open } from "@tauri-apps/plugin-shell";

// const route = useRoute("index-project-id-tabs-tabId");
const theme = useTheme();

const termElement = ref<HTMLDivElement>();

const sessionId = ref("");

const resultHistory = ref<string[]>([]);

let terminal: Terminal;
let fitAddon: FitAddon;

onMounted(async () => {
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
        console.log("link clicked", uri);
    });

    terminal.loadAddon(fitAddon);
    terminal.loadAddon(webLinksAddon);

    terminal.open(termElement.value!);

    fitTerminal();

    await spawn();
});

onActivated(() => {
    fitTerminal();
    reRender();
});

const fitTerminal = () => {
    fitAddon.fit();
};

const reRender = () => {
    terminal.reset();

    for (const result of resultHistory.value) {
        terminal.write(result);
    }
};

const spawn = async () => {
    console.log("try to spawn");

    const spawnResult = await commands.ptySpawn({ cols: terminal.cols, rows: terminal.rows });

    console.log(spawnResult);

    if (spawnResult.status === "error") {
        console.error("Failed to create pty");
        return;
    }

    sessionId.value = spawnResult.data;

    console.log("spawned process", sessionId.value);

    terminal.onData((data) => writeData(data));
    terminal.onResize((data) => ptyResize(data));

    await readData();
};

const writeData = async (data: string) => {
    const writeResult = await commands.ptyWrite(sessionId.value, data);

    if (writeResult.status === "error") {
        console.error("failed to write data");
        return;
    }
};

const readData = async () => {
    while (true) {
        const readResult = await commands.ptyRead(sessionId.value);

        if (readResult.status === "error") {
            console.error("Error while reading data");
            break;
        }

        terminal.write(readResult.data);

        resultHistory.value.push(readResult.data);
    }
};

const ptyResize = async (resize_contract: ResizePtyContract) => {
    const resizeResult = await commands.ptyResize(sessionId.value, resize_contract);

    if (resizeResult.status === "error") {
        console.error("Error while resizing pty");
        return;
    }
};
</script>
