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
        command: "dotnet build",
        noExit: false,
        workingDirectory: "C:\\Repos\\metis\\sources\\ControlCenter.Api"
    };

    const spawnResult = await commands.ptySessionSpawn(spawnContract);

    if (spawnResult.status === "error") {
        notify.error(t("ptySession.spawn.error"));
        return;
    }
};
</script>
