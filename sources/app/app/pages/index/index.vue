<template>
    <VEmptyState v-if="ready" :headline="$t('welcome.headline')" :title="$t('welcome.title')">
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
                        <ProjectDialogCreate />
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
                        <ProjectDialogSelect />
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard @click="openLastProject()" :disabled="!lastOpenedProject">
                        <VCardTitle>
                            <VIcon color="info" icon="mdi-folder" />
                            <span v-if="lastOpenedProject" class="text-truncate" style="max-width: 300px">
                                {{ $t("project.openLast.title", { name: lastOpenedProject.name }) }}
                            </span>
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
const router = useRouter();

const projectStore = useProjectStore();
const settingStore = useSettingStore();

const { lastOpenedProject } = storeToRefs(projectStore);
const { setting } = storeToRefs(settingStore);

const ready = ref(false);

onBeforeMount(async () => {
    await projectStore.loadLastOpenedProject();

    const isInitialMount = router.options.history.state.back === null;
    if (isInitialMount && !setting.value.behaviorOpenWelcome && lastOpenedProject.value) {
        await navigateTo({ name: "index-project-id", params: { id: lastOpenedProject.value.id } });
    }

    ready.value = true;
});

const openLastProject = () => {
    navigateTo({ name: "index-project-id", params: { id: lastOpenedProject.value!.id } });
};

const openSetting = () => {
    navigateTo({ name: "index-setting-index-presentation" });
};
</script>
