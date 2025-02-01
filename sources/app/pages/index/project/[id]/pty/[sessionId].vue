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
        cursorStyle: "bar",
        cursorInactiveStyle: "none",
        fontSize: 16,
        theme: {
            background: theme.current.value.colors["terminal-background"],
            foreground: theme.current.value.colors["terminal-foreground"],
            cursor: theme.current.value.colors["terminal-cursor"],
            selectionForeground: theme.current.value.colors["terminal-selectionForeground"],
            selectionBackground: theme.current.value.colors["terminal-selectionBackground"],
            selectionInactiveBackground: theme.current.value.colors["terminal-selectionInactiveBackground"],
            white: theme.current.value.colors["terminal-white"],
            brightWhite: theme.current.value.colors["terminal-brightWhite"],
            black: theme.current.value.colors["terminal-black"],
            brightBlack: theme.current.value.colors["terminal-brightBlack"],
            blue: theme.current.value.colors["terminal-blue"],
            brightBlue: theme.current.value.colors["terminal-brightBlue"],
            cyan: theme.current.value.colors["terminal-cyan"],
            brightCyan: theme.current.value.colors["terminal-brightCyan"],
            green: theme.current.value.colors["terminal-green"],
            brightGreen: theme.current.value.colors["terminal-brightGreen"],
            magenta: theme.current.value.colors["terminal-magenta"],
            brightMagenta: theme.current.value.colors["terminal-brightMagenta"],
            red: theme.current.value.colors["terminal-red"],
            brightRed: theme.current.value.colors["terminal-brightRed"],
            yellow: theme.current.value.colors["terminal-yellow"],
            brightYellow: theme.current.value.colors["terminal-brightYellow"]
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
