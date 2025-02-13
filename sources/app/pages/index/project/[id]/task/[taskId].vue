<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("task.edit.title") }}</VAppBarTitle>
        <TaskActionDelete v-if="task" :disabled="!isFormValid" :task="task" class="mr-2"></TaskActionDelete>
        <TaskActionSave v-if="task" :disabled="!isFormValid" :task="task"></TaskActionSave>
    </VAppBar>
    <VContainer>
        <VCard :loading="isLoading">
            <VCardText v-if="task">
                <VForm v-model="isFormValid">
                    <VRow>
                        <VCol cols="8">
                            <TaskFieldName v-model="task.name" />
                        </VCol>
                        <VCol cols="4">
                            <TaskFieldBlocking v-model="task.blocking" />
                        </VCol>
                    </VRow>
                    <TaskFieldCommandTiles v-model="task.workingDirTiles" />
                    <TaskFieldWorkingDirTiles v-model="task.commandTiles" />
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
