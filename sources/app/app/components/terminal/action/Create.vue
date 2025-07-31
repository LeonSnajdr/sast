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
    const createContract = {
        projectId: selectedProject.value.id,
        historyPersistence: "OnError",
        jumpInto: true
    } as TerminalCreateContract;

    const spawnContract = {
        noExit: false,
        forceKill: false
    } as ShellSpawnContract;

    isLoading.value = true;
    const spawnResult = await commands.terminalCreate(createContract, spawnContract);
    isLoading.value = false;

    if (spawnResult.status === "error") {
        notify.error(t("action.create.error", { type: t("terminal.singular") }), { error: spawnResult.error });
        return;
    }
};
</script>
