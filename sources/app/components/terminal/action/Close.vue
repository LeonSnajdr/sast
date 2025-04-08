<template>
    <VBtn @click.prevent.stop="terminalClose()" size="20" variant="plain">
        <VIcon icon="mdi-close" />
    </VBtn>
</template>

<script setup lang="ts">
const props = defineProps<{
    id: string;
}>();

const notify = useNotify();
const { t } = useI18n();

const terminalClose = async () => {
    const closeResult = await commands.terminalClose(props.id);

    if (closeResult.status === "error") {
        //TODO adjust error messag
        notify.error(t("terminal.kill.error"), { error: closeResult.error });
    }
};
</script>
