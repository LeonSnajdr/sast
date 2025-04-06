<template>
    <BaseBtnIcon @click="createTerminal()" color="secondary" icon="mdi-plus" variant="flat" />
</template>

<script setup lang="ts">
const notify = useNotify();
const { t } = useI18n();

const projectService = useProjectStore();

const { selectedProject } = storeToRefs(projectService);

const createTerminal = async () => {
    const createContract: TerminalCreateContract = {
        name: null,
        projectId: selectedProject.value.id,
        taskId: null,
        historyPersistence: "OnError"
    };

    const spawnContract: ShellSpawnContract = {
        command: null,
        noExit: false,
        workingDir: null,
        forceKill: false
    };

    const spawnResult = await commands.terminalCreate(createContract, spawnContract);

    if (spawnResult.status === "error") {
        notify.error(t("terminal.spawn.error"), { error: spawnResult.error });
        return;
    }
};
</script>
