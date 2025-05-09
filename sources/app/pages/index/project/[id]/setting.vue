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
                                <BaseBtnIcon variant="tonal" width="100">
                                    {{ $t("action.edit") }}
                                    <ProjectDialogEdit @saved="projectSaved" :projectId="selectedProject.id" />
                                </BaseBtnIcon>
                            </template>
                        </VListItem>
                        <VListItem>
                            <VListItemTitle class="d-flex align-center">
                                {{ $t("projectSetting.quickSwitch") }}
                                <VChip class="ml-2" color="secondary" density="compact" variant="outlined" label>{{
                                    keybind ? keybind : $t("keybind.none")
                                }}</VChip>
                            </VListItemTitle>
                            <span class="text-medium-emphasis">
                                {{ $t("projectSetting.quickSwitch.description") }}
                            </span>
                            <template #append>
                                <div v-if="!isCapturing" class="d-flex ga-2">
                                    <BaseBtnIcon v-if="keybind" @click="keybind = ''">{{ $t("action.reset") }}</BaseBtnIcon>
                                    <BaseBtnIcon @click="capture()" variant="tonal" width="100">{{ $t("action.capture") }}</BaseBtnIcon>
                                </div>
                                <BaseBtnIcon v-else @click="cancel()" variant="tonal" width="100">{{ $t("action.cancel") }}</BaseBtnIcon>
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
const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const projectSaved = (project: ProjectContract) => {
    selectedProject.value = project;
};

const colors = ["info", "success", "warning", "secondary", "primary", "error"];
const folderColor = ref("info");

const keybind = ref("");

const { isCapturing, capture, cancel } = useKeybindCapture(keybind);

const onFolderIconClick = lodAfter(10, () => {
    const currentIndex = colors.indexOf(folderColor.value);
    folderColor.value = colors[(currentIndex + 1) % colors.length];
});

definePageMeta({
    projectSwitchName: "index-project-id-setting"
});
</script>
