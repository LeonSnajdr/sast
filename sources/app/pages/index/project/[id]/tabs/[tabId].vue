<template>
    <div ref="termElement" class="flex-grow-1" />
</template>

<script setup lang="ts">
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import "xterm/css/xterm.css";

// const route = useRoute("index-project-id-tabs-tabId");

const termElement = ref<HTMLDivElement>();
const sessionId = ref("");

let terminal: Terminal;
let fitAddon: FitAddon;

onMounted(async () => {
    terminal = new Terminal({});
    fitAddon = new FitAddon();

    terminal.loadAddon(fitAddon);
    terminal.open(termElement.value!);

    fitAddon.fit();

    await spawn();
});

onActivated(() => {
    terminal.scrollToBottom();
});

useResizeObserver(termElement, () => {
    fitAddon.fit();
});

const spawn = async () => {
    console.log("try to spawn");

    const spawnResult = await commands.spawnTask({ cols: terminal.cols, rows: terminal.rows });

    console.log(spawnResult);

    if (spawnResult.status === "error") {
        console.error("Failed to create pty");
        return;
    }

    sessionId.value = spawnResult.data;

    console.log("spawned process", sessionId.value);

    terminal.onData((data) => writeData(data));

    await readData();
};

const writeData = async (data: string) => {
    console.log("write");

    const writeResult = await commands.writeToTask(sessionId.value, data);

    if (writeResult.status === "error") {
        console.error("failed to write data");
        return;
    }
};

const readData = async () => {
    while (true) {
        const readResult = await commands.readFromTask(sessionId.value);

        if (readResult.status === "error") {
            console.error("Error while reading data");
            break;
        }

        terminal.write(readResult.data);
    }
};
</script>
