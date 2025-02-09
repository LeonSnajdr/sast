<template>
    <IconBtn @click="ptySpawn()" color="secondary" icon="mdi-plus" variant="flat" />
</template>

<script setup lang="ts">
const notify = useNotify();
const { t } = useI18n();

const projectService = useProjectStore();

const { selectedProject } = storeToRefs(projectService);

const ptySpawn = async () => {
    const spawnContract: PtySessionSpawnContract = {
        name: null,
        projectId: selectedProject.value.id,
        taskSetId: null,
        command: "dir",
        noExit: true,
        workingDirectory: null
    };

    const spawnResult = await commands.ptySessionSpawn(spawnContract);

    if (spawnResult.status === "error") {
        notify.error(t("ptySession.spawn.error"));
        return;
    }

    navigateTo({ name: "index-project-id-pty-sessionId", params: { id: selectedProject.value.id, sessionId: spawnResult.data } });
};
</script>
