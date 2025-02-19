<template>
    <VBtn @click.prevent.stop="killSession()" size="20" variant="plain">
        <VIcon icon="mdi-close" />
    </VBtn>
</template>

<script setup lang="ts">
const props = defineProps<{
    sessionId: string;
}>();

const notify = useNotify();
const { t } = useI18n();

const killSession = async () => {
    const killResult = await commands.ptySessionKill(props.sessionId);

    if (killResult.status === "error") {
        notify.error(t("ptySession.kill.error"), { error: killResult.error });
    }
};
</script>
