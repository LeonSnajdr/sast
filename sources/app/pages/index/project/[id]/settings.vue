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
                                <BaseBtnIcon color="secondary" variant="flat">
                                    {{ $t("action.edit") }}
                                    <ProjectDialogEdit @saved="projectSaved" :projectId="selectedProject.id" />
                                </BaseBtnIcon>
                            </template>
                        </VListItem>
                        <VListItem>
                            <VListItemTitle>{{ $t("action.delete") }}</VListItemTitle>
                            <span class="text-medium-emphasis">
                                {{ $t("projectSetting.delete.description") }}
                            </span>
                            <template #append>
                                <ProjectActionDelete :project="selectedProject" class="ml-2" />
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

const onFolderIconClick = lodAfter(10, () => {
    const currentIndex = colors.indexOf(folderColor.value);
    folderColor.value = colors[(currentIndex + 1) % colors.length];
});
</script>
