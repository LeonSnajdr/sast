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
import { openUrl } from "@tauri-apps/plugin-opener";
import { getCurrentWindow } from "@tauri-apps/api/window";

const route = useRoute("index-project-id-terminal-terminalId");

const theme = useTheme();
const notify = useNotify();
const { t } = useI18n();

const termElement = ref<HTMLDivElement>();

let terminal: Terminal;
let fitAddon: FitAddon;
let webLinksAddon: WebLinksAddon;

let cleanup = () => {};

onMounted(async () => {
    const openContract = await loadOpenContract();

    if (!openContract) return;

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
        },
        cols: openContract.size.cols,
        rows: openContract.size.rows
    });

    fitAddon = new FitAddon();
    webLinksAddon = new WebLinksAddon((_, uri) => {
        openUrl(uri);
    });

    terminal.loadAddon(fitAddon);
    terminal.loadAddon(webLinksAddon);

    terminal.attachCustomKeyEventHandler((event: KeyboardEvent) => {
        const isNewTabAction = event.ctrlKey && event.code === "KeyT" && event.type === "keydown";
        const isCloseTabAction = event.ctrlKey && event.code === "KeyW" && event.type === "keydown";

        const isCtrlC = event.ctrlKey && event.code === "KeyC" && event.type === "keydown";
        const isTextSelected = terminal.getSelection();
        const isCopyAction = isCtrlC && isTextSelected;

        return !isCopyAction && !isNewTabAction && !isCloseTabAction;
    });

    terminal.open(termElement.value!);

    await write(openContract.history);

    terminal.onResize((data) => resizeTerminal(data));
    terminal.onData((data) => writeToTerminal(data));

    const unlistenResize = await getCurrentWindow().onResized(
        lodDebounce(() => {
            terminal.focus();
            fitAddon.fit();
            terminal.scrollToBottom();
        }, 500)
    );

    const unlistenTerminalShellReadEvent = await events.terminalShellReadEvent.listen(async (eventData) => {
        if (eventData.payload.id !== route.params.terminalId) return;

        await write(eventData.payload.data);
    });

    const unlistenTerminalClosedEvent = await events.terminalClosedEvent.listen((event) => {
        if (event.payload !== route.params.terminalId) return;

        navigateTo({ name: "index-project-id-terminal" });
    });

    cleanup = () => {
        unlistenResize();
        unlistenTerminalShellReadEvent();
        unlistenTerminalClosedEvent();
        terminal.dispose();
    };

    nextTick(() => {
        terminal.focus();

        const dims = fitAddon.proposeDimensions();
        const sizeChanged = dims?.cols != openContract.size.cols || dims.rows != openContract.size.rows;
        if (sizeChanged) {
            fitAddon.fit();
        }
    });
});

onBeforeUnmount(() => {
    cleanup();
});

const write = async (data: string): Promise<void> => {
    return new Promise((resolve, _) => {
        terminal.write(data, () => {
            resolve();
        });
    });
};

const loadOpenContract = async () => {
    const openResult = await commands.terminalGetOneOpen(route.params.terminalId);

    if (openResult.status === "error") {
        notify.error(t("terminal.open.error"), { error: openResult.error });
        return;
    }

    return { history: openResult.data.history, size: openResult.data.shellSize };
};

const writeToTerminal = async (data: string) => {
    const writeResult = await commands.terminalShellWrite(route.params.terminalId, data);
    if (writeResult.status === "error") {
        console.error("failed to write data");
    }
};

async function resizeTerminal(resizeContract: ShellSizeContract) {
    const resizeResult = await commands.terminalShellResize(route.params.terminalId, resizeContract);
    if (resizeResult.status === "error") {
        console.error("Error while resizing terminal");
    }

    console.debug("Resizing terminal", resizeContract);
}
</script>
