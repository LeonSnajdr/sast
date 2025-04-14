<template>
    <BaseBtnIcon @click="createTerminal()" :loading="isLoading" color="secondary" icon="mdi-plus" variant="flat" />
</template>

<script setup lang="ts">
const notify = useNotify();
const { t } = useI18n();

const projectService = useProjectStore();

const { selectedProject } = storeToRefs(projectService);

const isLoading = ref(false);

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

    isLoading.value = true;
    const spawnResult = await commands.terminalCreate(createContract, spawnContract);
    isLoading.value = false;

    if (spawnResult.status === "error") {
        notify.error(t("terminal.spawn.error"), { error: spawnResult.error });
        return;
    }

    navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: selectedProject.value.id, terminalId: spawnResult.data } });
};
</script>
