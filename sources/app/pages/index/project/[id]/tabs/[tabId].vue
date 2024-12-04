<template>
    <VContainer id="termContainer" class="flex-grow-1">
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

let terminal: Terminal;
let fitAddon: FitAddon;

onMounted(async () => {
    terminal = new Terminal({
        theme: {
            // background: theme.current.value.colors.background,
            background: "red",
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
    terminal.scrollToBottom();
});

useResizeObserver(termElement, () => fitTerminal());

const fitTerminal = () => {
    // TODO: The height 103 is currently hardcoded, if height of elements e.g. header change, this will break
    termElement.value!.style.height = window.innerHeight - 103 + "px";
    fitAddon.fit();

    //const dimensions = fitAddon.proposeDimensions()!;
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

const clear = () => {
    terminal.reset();
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

        console.log("read", readResult.data);

        terminal.write(readResult.data);
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
