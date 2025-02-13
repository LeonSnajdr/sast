<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("task.edit.title") }}</VAppBarTitle>
    </VAppBar>
    <VContainer>
        <VCard :loading="isLoading">
            <VCardText v-if="task">
                <VForm v-model="isFormValid">
                    <TaskFieldName v-model="task.name" />
                    <TaskFieldBlocking v-model="task.blocking" />
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
