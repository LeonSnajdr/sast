<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("title.edit", { type: $t("task.singular") }) }}</VAppBarTitle>
        <TaskActionDelete v-if="task" :disabled="!isFormValid" :task class="mr-2" />
        <TaskActionSave v-if="task" @saved="taskSaved()" :disabled="!isFormValid" :task />
    </VAppBar>
    <VContainer>
        <VCard :loading="isTaskLoading">
            <VCardText v-if="task">
                <VForm v-model="isFormValid">
                    <TaskFieldName v-model="task.name" />
                    <VRow>
                        <VCol cols="9">
                            <TaskFieldTabName v-model="task.tabName" />
                        </VCol>
                        <VCol cols="3">
                            <TaskFieldNoExit v-model="task.noExit" />
                        </VCol>
                    </VRow>
                    <TaskFieldCommandTiles v-model="task.commandTiles" />
                    <TaskFieldWorkingDirTiles v-model="task.workingDirTiles" />
                </VForm>
            </VCardText>
        </VCard>
    </VContainer>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-task-taskId");

const { isTaskLoading, task, loadTask } = useTask();

const isFormValid = ref(false);

onBeforeMount(() => {
    loadTask(route.params.taskId);
});

const taskSaved = () => {
    navigateTo({ name: "index-project-id-task", params: { id: route.params.id } });
};
</script>
