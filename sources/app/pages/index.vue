<template>
    <VEmptyState :headline="$t('welcome.headline')" :title="$t('welcome.title')">
        <VContainer>
            <VRow>
                <VCol cols="12" sm="6">
                    <VCard :text="$t('project.create.description')" :title="$t('project.create.title')" link>
                        <template #prepend>
                            <VIcon color="success" icon="mdi-folder-plus" />
                        </template>

                        <ProjectCreateDialog />
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard :text="$t('project.select.description')" :title="$t('project.select.title')" link>
                        <template #prepend>
                            <VIcon color="warning" icon="mdi-folder-marker" />
                        </template>

                        <ProjectSelectDialog />
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard
                        @click="openLastProject()"
                        :disabled="!lastOpenedProject"
                        :text="$t('project.openLast.description')"
                        :title="$t('project.openLast.title')"
                    >
                        <template #prepend>
                            <VIcon color="info" icon="mdi-folder" />
                        </template>
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard :text="$t('setting.description')" :title="$t('setting.title')" disabled>
                        <template #prepend>
                            <VIcon color="secondary" icon="mdi-cog" />
                        </template>
                    </VCard>
                </VCol>
            </VRow>
        </VContainer>
    </VEmptyState>
</template>

<script setup lang="ts">
const { t } = useI18n();
const notify = useNotify();
const presentation = usePresentation();

const lastOpenedProject = ref<ProjectContract | null>();

onBeforeMount(async () => {
    presentation.applyUsingSetting();
    await loadLastOpenedProject();
});

const loadLastOpenedProject = async () => {
    const lastOpenedProjectResult = await commands.openLastProject();

    if (lastOpenedProjectResult.status == "error") {
        notify.error(t("project.load.failed"));
        return;
    }

    lastOpenedProject.value = lastOpenedProjectResult.data;
};

const openLastProject = () => {
    navigateTo({ name: "project-id-home", params: { id: lastOpenedProject.value!.id } });
};

definePageMeta({
    middleware: "initialized"
});
</script>
