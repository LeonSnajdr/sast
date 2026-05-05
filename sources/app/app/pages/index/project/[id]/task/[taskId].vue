<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("title.edit", { type: $t("task.singular") }) }}</VAppBarTitle>
        <BaseActionBack />
        <TaskActionDelete v-if="task" :task />
        <VBtn v-if="task" class="mr-2" prependIcon="mdi-content-duplicate">
            <TaskDialogClone @cloned="taskCloned" :taskId="task.id" />
            {{ $t("action.clone") }}
        </VBtn>
        <TaskActionSave v-if="task" @saved="taskSaved()" :disabled="!isTaskValid" :task />
    </VAppBar>
    <VContainer>
        <VCard :loading="isTaskLoading">
            <VCardText v-if="task">
                <TaskFieldContainer v-model="task" v-model:isValid="isTaskValid" />
            </VCardText>
        </VCard>
    </VContainer>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-task-taskId");

const { isTaskLoading, task, loadTask } = useTask();

const isTaskValid = ref<boolean | null>(false);

onBeforeMount(() => {
    loadTask(route.params.taskId);
});

const taskSaved = () => {
    navigateTo({ name: "index-project-id-task", params: { id: route.params.id } });
};

const taskCloned = (task: TaskContract) => {
    navigateTo({ name: "index-project-id-task-taskId", params: { id: task.projectId, taskId: task.id } });
};
</script>
