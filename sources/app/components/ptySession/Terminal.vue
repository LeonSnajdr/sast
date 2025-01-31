<template>
    <div ref="termElement" class="h-100 overflow-hidden" />
</template>

<script setup lang="ts">
import "xterm/css/xterm.css";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { WebLinksAddon } from "@xterm/addon-web-links";
import { open } from "@tauri-apps/plugin-shell";

const props = defineProps<{
    sessionId: string;
}>();

const theme = useTheme();
const notify = useNotify();
const { t } = useI18n();

const termElement = ref<HTMLDivElement>();

let terminal: Terminal;
let fitAddon: FitAddon;
let webLinksAddon: WebLinksAddon;

onMounted(async () => {
    console.log("test");

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

    await restoreHistory();

    terminal.onData((data) => writeToPtySession(data));
    terminal.onResize((data) => resizePtySession(data));

    fitAddon.fit();

    //await startPtySessionReadLoop();

    console.log("listen for data");

    const unlisten = await events.ptySessionEvent.listen((data) => {
        console.log("Got data");

        terminal.write(data.payload);
        console.log(data);
    });

    onBeforeUnmount(() => {
        unlisten();
    });
});

onBeforeUnmount(() => {
    resizeObserver.stop();
    terminal.dispose();
});

const resizeObserver = useResizeObserver(termElement, () => {
    fitAddon.fit();
});

const isTerminalVisible = useElementVisibility(termElement);
watch(isTerminalVisible, () => {
    fitAddon.fit();
    terminal.focus();
});

const restoreHistory = async () => {
    const readHistoryResult = await commands.ptySessionGetReadHistory(props.sessionId);

    if (readHistoryResult.status === "error") {
        notify.error(t("ptySession.open.error"));
        return;
    }

    if (!readHistoryResult.data) {
        return;
    }

    terminal.write(readHistoryResult.data);
};

const writeToPtySession = async (data: string) => {
    const writeResult = await commands.ptySessionWrite(props.sessionId, data);
    if (writeResult.status === "error") {
        console.error("failed to write data");
    }
};

async function resizePtySession(resizeContract: PtySessionResizeContract) {
    const resizeResult = await commands.ptySessionResize(props.sessionId, resizeContract);
    if (resizeResult.status === "error") {
        console.error("Error while resizing pty");
    }
}
</script>
