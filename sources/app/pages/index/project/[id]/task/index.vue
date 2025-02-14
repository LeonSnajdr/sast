<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("task.title") }}</VAppBarTitle>
        <TaskActionCreate @created="loadTasks()" />
    </VAppBar>

    <VContainer class="h-100">
        <VCard>
            <VCardText>
                <TaskTable :tasks="tasks" />
            </VCardText>
        </VCard>
    </VContainer>
</template>

<script setup lang="ts">
const projectStore = useProjectStore();

const notify = useNotify();
const { t } = useI18n();

const { selectedProject } = storeToRefs(projectStore);

const isLoading = ref(false);
const tasks = ref<TaskInfoContract[]>([]);

onBeforeMount(() => {
    loadTasks();
});

const loadTasks = async () => {
    isLoading.value = true;

    const taskResult = await commands.taskGetInfoAllProject(selectedProject.value.id);

    isLoading.value = false;

    if (taskResult.status === "error") {
        notify.error(t("task.load.error"));
        return;
    }

    tasks.value = taskResult.data;
};
</script>
