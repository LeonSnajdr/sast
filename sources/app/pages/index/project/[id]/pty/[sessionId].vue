<template>
    <VContainer class="h-100">
        <div ref="termElement" class="h-100 overflow-hidden" />
    </VContainer>
</template>

<script setup lang="ts">
import "xterm/css/xterm.css";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { WebLinksAddon } from "@xterm/addon-web-links";
import { open } from "@tauri-apps/plugin-shell";
import { getCurrentWindow } from "@tauri-apps/api/window";

const route = useRoute("index-project-id-pty-sessionId");

const theme = useTheme();
const notify = useNotify();
const { t } = useI18n();

const termElement = ref<HTMLDivElement>();

let terminal: Terminal;
let fitAddon: FitAddon;
let webLinksAddon: WebLinksAddon;

let cleanup: () => void;

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
    webLinksAddon = new WebLinksAddon((_, uri) => {
        open(uri);
    });

    terminal.loadAddon(fitAddon);
    terminal.loadAddon(webLinksAddon);

    await restoreHistory();

    terminal.onData((data) => writeToPtySession(data));
    terminal.onResize((data) => resizePtySession(data));

    terminal.open(termElement.value!);

    const unlistenResize = await getCurrentWindow().onResized(() => {
        fitAddon.fit();
    });

    const unlistenData = await events.ptySessionEvent.listen((eventData) => {
        if (eventData.payload.sessionId !== route.params.sessionId) return;

        terminal.write(eventData.payload.data);
    });

    cleanup = () => {
        unlistenResize();
        unlistenData();
        terminal.dispose();
    };

    nextTick(() => {
        terminal.focus();
        fitAddon.fit();
    });
});

onBeforeUnmount(() => {
    cleanup();
});

const restoreHistory = async () => {
    const readHistoryResult = await commands.ptySessionGetReadHistory(route.params.sessionId);

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
    const writeResult = await commands.ptySessionWrite(route.params.sessionId, data);
    if (writeResult.status === "error") {
        console.error("failed to write data");
    }
};

async function resizePtySession(resizeContract: PtySessionResizeContract) {
    const resizeResult = await commands.ptySessionResize(route.params.sessionId, resizeContract);
    if (resizeResult.status === "error") {
        console.error("Error while resizing pty");
    }
}
</script>
