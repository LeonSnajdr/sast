<template>
    <VBtn @click.prevent.stop="terminalClose()" size="20" variant="plain" v-tooltip="$t('keybind.controlW.tooltip')">
        <VIcon icon="mdi-close" />
    </VBtn>
</template>

<script setup lang="ts">
const props = defineProps<{
    terminal: TerminalInfoContract;
}>();

const route = useRoute("index-project-id-terminal-terminalId");
const notify = useNotify();
const { t } = useI18n();

useKeybind(["control", "w"], async () => {
    if (route.params.terminalId != props.terminal.id) return;

    await terminalClose();
});

const terminalClose = async () => {
    const closeResult = await commands.terminalClose(props.terminal.id);

    if (closeResult.status === "error") {
        notify.error(t("action.close.error", { type: t("terminal.singular"), name: props.terminal.name }), { error: closeResult.error });
    }
};
</script>
