<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800">
        <VCard height="400">
            <VCardTitle>
                <VIcon color="warning" icon="mdi-folder-marker" />
                {{ $t("title.select", { type: $t("project.singular") }) }}
            </VCardTitle>
            <VCardText>
                <VTextField
                    v-model="search"
                    :label="$t('search.filter')"
                    :persistentPlaceholder="false"
                    appendInnerIcon="mdi-filter-outline"
                    autofocus
                    clearable
                />
                <VList>
                    <VListItem v-for="projectResult in projectResults" :key="projectResult.item.id" @click="switchProject(projectResult.item)">
                        <VListItemTitle>
                            <VIcon v-if="projectResult.item.favorite" class="mr-1" color="warning" icon="mdi-star" size="small" />
                            {{ projectResult.item.name }}
                        </VListItemTitle>
                        <VListItemSubtitle>
                            {{ $t("date.opened", { date: useLocaleTimeAgo(projectResult.item.dateLastOpened).value }) }}
                        </VListItemSubtitle>
                        <template #append>
                            <VIcon icon="mdi-arrow-right" />
                        </template>
                    </VListItem>

                    <VListSubheader v-if="projectResults.length == 0" class="text-center">{{ $t("search.noResults") }}</VListSubheader>
                </VList>
            </VCardText>
            <VCardActions>
                <BaseBtnIcon @click="isDialogOpen = false">{{ $t("action.close") }}</BaseBtnIcon>
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts">
import { useFuse } from "@vueuse/integrations/useFuse";

const { switchProject } = useProject();

const projectStore = useProjectStore();

const { allProjects } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const search = ref<string>("");

onBeforeMount(() => {
    projectStore.loadAllProjects();
});

const { results: projectResults } = useFuse(search, allProjects, {
    fuseOptions: {
        keys: ["name"],
        isCaseSensitive: false
    },
    matchAllWhenSearchEmpty: true
});

watch(isDialogOpen, () => {
    search.value = "";
});
</script>
