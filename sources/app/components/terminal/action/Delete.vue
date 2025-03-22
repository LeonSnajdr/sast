<template>
    <VBtn @click.prevent.stop="deleteTerminal()" size="20" variant="plain">
        <VIcon icon="mdi-minus" />
    </VBtn>
</template>

<script setup lang="ts">
const props = defineProps<{
    id: string;
}>();

const notify = useNotify();
const { t } = useI18n();

const deleteTerminal = async () => {
    const killResult = await commands.terminalDelete(props.id);

    if (killResult.status === "error") {
        //TODO adjust error messag
        notify.error(t("terminal.kill.error"), { error: killResult.error });
    }
};
</script>
