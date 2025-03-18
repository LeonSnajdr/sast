<template>
    <BaseBtnIcon @click="ptySpawn()" color="secondary" icon="mdi-plus" variant="flat" />
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
        taskId: null,
        taskSetId: null,
        command: "volta run yarn migrate-mongo up --brand cores",
        noExit: false,
        workingDir: "C:\\Repos\\selfserviceportal\\mongodb",
        forceKill: false,
        historyPersistence: "OnError"
    };

    const spawnResult = await commands.ptySessionSpawn(spawnContract);

    if (spawnResult.status === "error") {
        notify.error(t("ptySession.spawn.error"), { error: spawnResult.error });
        return;
    }
};
</script>
