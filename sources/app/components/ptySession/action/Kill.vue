<template>
    <VBtn @click.prevent.stop="killSession()" icon="mdi-close" size="20" variant="plain" />
</template>

<script setup lang="ts">
const props = defineProps<{
    sessionId: string;
}>();

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();
const ptySessionStore = usePtySessionStore();

const { selectedProject } = storeToRefs(projectStore);
const { ptySessions } = storeToRefs(ptySessionStore);

const killSession = async () => {
    const index = lodFindIndex(ptySessions.value, (x) => x.sessionId === props.sessionId);

    const killResult = await commands.ptySessionKill(props.sessionId);

    if (killResult.status === "error") {
        notify.error(t("ptySession.kill.error"));
        console.error("failed to kill pty session", killResult);
    }

    await ptySessionStore.loadAll();

    if (ptySessions.value.length === 0) {
        navigateTo({ name: "index-project-id-pty", params: { id: selectedProject.value.id } });
        return;
    }

    const nextIndex = index > 0 ? index - 1 : 0;
    navigateTo({ name: "index-project-id-pty-sessionId", params: { id: selectedProject.value.id, sessionId: ptySessions.value[nextIndex].sessionId } });
};
</script>
