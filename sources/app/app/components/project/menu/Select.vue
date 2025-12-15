<template>
    <VMenu v-model="isMenuOpen" :transition="false" activator="parent">
        <VSheet>
            <VTextField
                v-model="query"
                @click.stop.prevent
                @keydown.enter="onEnter"
                :placeholder="$t('search.filter')"
                class="ma-2 mt-3"
                density="compact"
                prependIcon="mdi-magnify"
                variant="plain"
                autofocus
            />
            <VDivider />
            <VList maxHeight="400" maxWidth="400" minWidth="250">
                <VListItem v-for="projectResult in projectResults" :key="projectResult.item.id" @click="onItemClick(projectResult.item)">
                    <VListItemTitle class="d-flex align-center ga-1">
                        <VIcon v-if="projectResult.item.favorite" color="warning" icon="mdi-star" size="small" />
                        <span class="text-truncate" style="max-width: 300px">{{ projectResult.item.name }}</span>
                        <TerminalCountChip :project="projectResult.item" density="default" />
                    </VListItemTitle>
                    <VListItemSubtitle>
                        {{ $t("date.opened", { date: useLocaleTimeAgo(projectResult.item.dateLastOpened).value }) }}
                    </VListItemSubtitle>
                    <template #append>
                        <VHotkey v-if="projectResult.item.quickSwitchKeybind" :keys="projectResult.item.quickSwitchKeybind" class="ml-2" />
                    </template>
                </VListItem>
                <VListSubheader v-if="projectResults.length == 0" class="text-center">{{ $t("search.noResults") }}</VListSubheader>
            </VList>
        </VSheet>
    </VMenu>
</template>

<script setup lang="ts">
const { switchProject } = useProject();
const { query, projectResults } = useProjectSearch();

const isMenuOpen = ref(false);

watch(isMenuOpen, () => {
    query.value = "";
});

const onItemClick = async (project: ProjectContract) => {
    await switchProject(project);
};

const onEnter = async () => {
    const firstItem = projectResults.value[0]?.item;

    if (!firstItem) return;

    isMenuOpen.value = false;

    await switchProject(firstItem);
};
</script>
