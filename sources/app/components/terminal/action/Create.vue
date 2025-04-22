<template>
    <VBtn @click.prevent.stop="createTerminal()" class="ml-2" size="28" v-tooltip="$t('keybind.controlT.tooltip')">
        <VIcon icon="mdi-plus" size="small" />
    </VBtn>
</template>

<script setup lang="ts">
const notify = useNotify();
const { t } = useI18n();

const projectService = useProjectStore();

const { selectedProject } = storeToRefs(projectService);

const isLoading = ref(false);

useKeybind(["control", "t"], () => createTerminal());

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
        notify.error(t("action.create.error", { type: t("terminal.singular") }), { error: spawnResult.error });
        return;
    }

    navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: selectedProject.value.id, terminalId: spawnResult.data } });
};
</script>
