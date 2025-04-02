<template>
    <BaseBtnIcon @click="ptySpawn()" color="secondary" icon="mdi-plus" variant="flat" />
</template>

<script setup lang="ts">
const notify = useNotify();
const { t } = useI18n();

const projectService = useProjectStore();

const { selectedProject } = storeToRefs(projectService);

const ptySpawn = async () => {
    const spawnContract: TerminalSpawnContract = {
        name: null,
        projectId: selectedProject.value.id,
        taskId: null,
        command: null,
        noExit: false,
        workingDir: null,
        forceKill: false,
        historyPersistence: "OnError"
    };

    const spawnResult = await commands.terminalSpawn(spawnContract);

    if (spawnResult.status === "error") {
        notify.error(t("terminal.spawn.error"), { error: spawnResult.error });
        return;
    }
};
</script>
