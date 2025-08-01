<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("projectSetting.singular") }}</VAppBarTitle>
    </VAppBar>

    <VContainer>
        <VRowSingle>
            <VCard>
                <VCardText>
                    <VListItem>
                        <template #prepend>
                            <VIcon @click="onFolderIconClick" :color="folderColor" class="cursor-pointer" icon="mdi-folder" size="70" />
                        </template>
                        <VListItemTitle>
                            {{ selectedProject.name }}
                        </VListItemTitle>
                        <VListItemSubtitle>{{ $t("date.created", { date: useLocaleTimeAgo(selectedProject.dateCreated).value }) }}</VListItemSubtitle>

                        <template #append>
                            <VIconBtn
                                @click="selectedProject.favorite = !selectedProject.favorite"
                                :color="selectedProject.favorite ? 'warning' : 'secondary'"
                                :icon="selectedProject.favorite ? 'mdi-star' : 'mdi-star-outline'"
                                class="mr-8"
                            />
                        </template>
                    </VListItem>
                </VCardText>
            </VCard>
        </VRowSingle>
        <VRowSingle>
            <VCard>
                <VCardText>
                    <VList>
                        <VListItem>
                            <VListItemTitle>{{ $t("action.rename") }}</VListItemTitle>
                            <span class="text-medium-emphasis">
                                {{ $t("projectSetting.delete.rename") }}
                            </span>
                            <template #append>
                                <VBtn variant="tonal" width="100">
                                    {{ $t("action.edit") }}
                                    <ProjectSettingEditName v-model="selectedProject" />
                                </VBtn>
                            </template>
                        </VListItem>
                        <VListItem>
                            <VListItemTitle>
                                {{ $t("projectSetting.quickSwitch") }}
                                <VChip class="ml-2" color="secondary" density="compact" variant="outlined" label>
                                    {{ keybind ? keybind : $t("keybind.none") }}
                                </VChip>
                            </VListItemTitle>
                            <span class="text-medium-emphasis">
                                {{ $t("projectSetting.quickSwitch.description") }}
                            </span>
                            <template #append>
                                <div v-if="!isCapturing" class="d-flex ga-2 ml-2">
                                    <VBtn v-if="keybind" @click="keybind = null">{{ $t("action.reset") }}</VBtn>
                                    <VBtn @click="capture()" variant="tonal" width="100">{{ $t("action.capture") }}</VBtn>
                                </div>
                                <VBtn v-else @click="cancel()" class="ml-2" variant="tonal" width="100">{{ $t("action.cancel") }}</VBtn>
                            </template>
                        </VListItem>
                        <VListItem>
                            <VListItemTitle>{{ $t("action.delete") }}</VListItemTitle>
                            <span class="text-medium-emphasis">
                                {{ $t("projectSetting.delete.description") }}
                            </span>
                            <template #append>
                                <ProjectActionDelete :project="selectedProject" class="ml-2" width="100" />
                            </template>
                        </VListItem>
                    </VList>
                </VCardText>
            </VCard>
        </VRowSingle>
    </VContainer>
</template>

<script setup lang="ts">
const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const colors = ["info", "success", "warning", "secondary", "primary", "error"];
const folderColor = ref("info");

const keybind = computed({
    get() {
        return selectedProject.value.quickSwitchKeybind;
    },
    set(newVal) {
        selectedProject.value.quickSwitchKeybind = newVal;
    }
});

const { isCapturing, capture, cancel } = useKeybindCapture(keybind);

watch(
    selectedProject,
    async () => {
        const updateContract: ProjectUpdateContract = {
            id: selectedProject.value.id,
            name: selectedProject.value.name,
            favorite: selectedProject.value.favorite,
            quickSwitchKeybind: selectedProject.value.quickSwitchKeybind
        };

        const saveResult = await commands.projectUpdateOne(updateContract);

        if (saveResult.status == "error") {
            notify.error(t("action.save.error", { type: t("project.singular"), name: selectedProject.value.name }), { error: saveResult.error });

            // Restore the project
            projectStore.openProject(selectedProject.value.id);
            return;
        }

        await projectStore.loadAllProjects();
    },
    { deep: true }
);

const onFolderIconClick = lodAfter(10, () => {
    const currentIndex = colors.indexOf(folderColor.value);
    folderColor.value = colors[(currentIndex + 1) % colors.length]!;
});

definePageMeta({
    projectSwitchName: "index-project-id-setting"
});
</script>
