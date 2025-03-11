<template>
    <VEmptyState :headline="$t('welcome.headline')" :title="$t('welcome.title')">
        <VContainer>
            <VRow>
                <VCol cols="12" sm="6">
                    <VCard link>
                        <VCardTitle>
                            <VIcon color="success" icon="mdi-folder-plus" />
                            {{ $t("project.create.title") }}
                        </VCardTitle>
                        <VCardText>
                            {{ $t("project.create.description") }}
                        </VCardText>
                        <ProjectCreateDialog />
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard link>
                        <VCardTitle>
                            <VIcon color="warning" icon="mdi-folder-marker" />
                            {{ $t("project.select.title") }}
                        </VCardTitle>
                        <VCardText>
                            {{ $t("project.select.description") }}
                        </VCardText>
                        <ProjectSelectDialog />
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard @click="openLastProject()" :disabled="!lastOpenedProject">
                        <VCardTitle>
                            <VIcon color="info" icon="mdi-folder" />
                            <span v-if="lastOpenedProject" class="text-truncate"> {{ $t("project.openLast.title", { name: lastOpenedProject.name }) }}</span>
                            <span v-else>{{ $t("project.openLast.title.noProject") }}</span>
                        </VCardTitle>
                        <VCardText>
                            {{ $t("project.openLast.description") }}
                        </VCardText>
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard @click="openSetting()">
                        <VCardTitle>
                            <VIcon color="secondary" icon="mdi-cog" />
                            {{ $t("setting.title") }}
                        </VCardTitle>
                        <VCardText>
                            {{ $t("setting.description") }}
                        </VCardText>
                    </VCard>
                </VCol>
            </VRow>
        </VContainer>
    </VEmptyState>
</template>

<script setup lang="ts">
const { t } = useI18n();
const notify = useNotify();

const lastOpenedProject = ref<ProjectContract | null>();

onBeforeMount(async () => {
    await loadLastOpenedProject();
});

const loadLastOpenedProject = async () => {
    const lastOpenedProjectResult = await commands.projectGetLastOpened();

    if (lastOpenedProjectResult.status == "error") {
        notify.error(t("project.load.failed"), { error: lastOpenedProjectResult.error });
        return;
    }

    lastOpenedProject.value = lastOpenedProjectResult.data;
};

const openLastProject = () => {
    navigateTo({ name: "index-project-id-home", params: { id: lastOpenedProject.value!.id } });
};

const openSetting = () => {
    navigateTo({ name: "index-setting-index-presentation" });
};
</script>
