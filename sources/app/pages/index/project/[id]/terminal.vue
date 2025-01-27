<template>
    <VAppBar>
        <VSlideGroup showArrows>
            <VSlideGroupItem v-for="terminal in terminals" :key="terminal.sessionId">
                <VBtn
                    :to="{ name: 'index-project-id-terminal-terminalId', params: { id: route.params.id, terminalId: terminal.sessionId } }"
                    class="ml-2 px-2"
                    density="comfortable"
                >
                    <template #prepend>
                        <VIcon color="info" icon="mdi-powershell" />
                    </template>
                    <span class="text-truncate" style="max-width: 150px">{{ terminal.name }}</span>
                    <template #append>
                        <VDefaultsProvider :defaults="{ VIcon: { size: '16' } }">
                            <VBtn @click.prevent.stop="closeTerminal(terminal.sessionId)" icon="mdi-close" size="20" variant="plain" />
                        </VDefaultsProvider>
                    </template>
                </VBtn>
            </VSlideGroupItem>
        </VSlideGroup>
        <VSpacer />
        <IconBtn @click="spawnTerminal()" color="secondary" icon="mdi-plus" variant="flat" />
    </VAppBar>

    <div class="h-100">
        <NuxtPage />
    </div>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-terminal");

const projectService = useProjectStore();

const { selectedProject } = storeToRefs(projectService);

const terminals = ref<PtyInfoContract[]>([]);

onBeforeMount(() => {
    getTerminals();
});

const getTerminals = async () => {
    const tabResult = await commands.ptyGetSessions(selectedProject.value.id);

    if (tabResult.status === "error") {
        return;
    }

    terminals.value = tabResult.data;
};

const spawnTerminal = async () => {
    const spawnContract: PtySpawnContract = {
        name: "Test",
        projectId: selectedProject.value.id
    };

    await commands.ptySpawn(spawnContract);

    await getTerminals();
};

const closeTerminal = async (terminalId: string) => {
    await commands.ptyKill(terminalId);
};
/*const closeTab = (tab: string) => {
    const index = lodIndexOf(terminal.value, tab);

    lodRemove(terminal.value, (x) => x === tab);

    if (terminal.value.length === 0) {
        navigateTo({ name: "index-project-id-tabs", params: { id: route.params.id } });
        return;
    }

    const nextIndex = index > 0 ? index - 1 : 0;
    navigateTo({ name: "index-project-id-tabs-tabId", params: { id: route.params.id, tabId: terminal.value[nextIndex] } });
};*/
</script>
