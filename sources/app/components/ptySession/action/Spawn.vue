<template>
    <IconBtn @click="ptySpawn()" color="secondary" icon="mdi-plus" variant="flat" />
</template>

<script setup lang="ts">
const notify = useNotify();
const { t } = useI18n();

const projectService = useProjectStore();
const ptySessionStore = usePtySessionStore();

const { selectedProject } = storeToRefs(projectService);

const ptySpawn = async () => {
    const spawnContract: PtySessionSpawnContract = {
        name: "Test",
        projectId: selectedProject.value.id
    };

    const spawnResult = await commands.ptySessionSpawn(spawnContract);

    if (spawnResult.status === "error") {
        notify.error(t("ptySession.spawn.error"));
        return;
    }

    await ptySessionStore.loadAll();

    navigateTo({ name: "index-project-id-pty-sessionId", params: { id: selectedProject.value.id, sessionId: spawnResult.data } });
};
</script>
