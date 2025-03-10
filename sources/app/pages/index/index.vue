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
                    <VCard @click="openLastProject()" :disabled="!lastOpenedProjectId">
                        <VCardTitle>
                            <VIcon color="info" icon="mdi-folder" />
                            {{ $t("project.openLast.title") }}
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

const lastOpenedProjectId = ref<string | null>();

onBeforeMount(async () => {
    await loadLastOpenedProject();
});

const loadLastOpenedProject = async () => {
    const lastOpenedProjectIdResult = await commands.projectGetIdLastOpened();

    if (lastOpenedProjectIdResult.status == "error") {
        notify.error(t("project.load.failed"), { error: lastOpenedProjectIdResult.error });
        return;
    }

    lastOpenedProjectId.value = lastOpenedProjectIdResult.data;
};

const openLastProject = () => {
    navigateTo({ name: "index-project-id-home", params: { id: lastOpenedProjectId.value! } });
};

const openSetting = () => {
    navigateTo({ name: "index-setting-index-presentation" });
};
</script>
