<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800">
        <VCard>
            <VCardTitle>
                <VIcon color="warning" icon="mdi-folder-marker" />
                {{ $t("project.select.title") }}
            </VCardTitle>
            <VCardText>
                <VTextField v-model="search" :placeholder="$t('search.filter')" appendInnerIcon="mdi-filter-outline" autofocus />
                <VList>
                    <VListItem v-for="project in filteredProjects" :key="project.id" :to="{ name: 'project-id', params: { id: project.id } }">
                        <VListItemTitle>{{ project.name }}</VListItemTitle>
                        <template #append>
                            <VIcon icon="mdi-arrow-right" />
                        </template>
                    </VListItem>

                    <VListSubheader v-if="filteredProjects.length == 0" class="text-center">{{ $t("search.noResults") }}</VListSubheader>
                </VList>
            </VCardText>
            <VCardActions>
                <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts">
const i18n = useI18n();
const notify = useNotify();

const isDialogOpen = ref(false);
const isLoading = ref(false);

const search = ref("");

const projects = ref<ProjectContract[]>([]);

onBeforeMount(() => {
    loadProjects();
});

const loadProjects = async () => {
    isLoading.value = true;

    const projectsResult = await commands.getAllProjects();

    isLoading.value = false;

    if (projectsResult.status == "error") {
        notify.error(i18n.t("project.load.failed"));
        return;
    }

    projects.value = projectsResult.data;
};

const filteredProjects = computed(() => {
    return projects.value.filter((x) => x.name.toLowerCase().includes(search.value.toLowerCase()));
});
</script>
