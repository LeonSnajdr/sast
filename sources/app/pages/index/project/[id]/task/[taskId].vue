<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("task.edit.title") }}</VAppBarTitle>
        <TaskActionDelete v-if="task" :disabled="!isFormValid" :task="task" class="mr-2" />
        <TaskActionSave v-if="task" :disabled="!isFormValid" :task="task" />
    </VAppBar>
    <VContainer>
        <VCard :loading="isLoading">
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

const isLoading = ref(false);
const isFormValid = ref(false);
const task = ref<TaskContract>();

onBeforeMount(() => {
    loadTask();
});

const loadTask = async () => {
    isLoading.value = true;

    const taskResult = await commands.taskGetOne(route.params.taskId);

    isLoading.value = false;

    if (taskResult.status === "error") {
        return;
    }

    task.value = taskResult.data;
};
</script>
