<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("task.plural") }}</VAppBarTitle>
        <BaseBtnIcon color="secondary" icon="mdi-plus" variant="flat">
            {{ $t("action.create") }}
            <TaskDialogCreate v-model="isCreateDialogOpen" @created="loadTasks()" />
        </BaseBtnIcon>
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
const isCreateDialogOpen = ref(false);

const tasks = ref<TaskInfoContract[]>([]);

onBeforeMount(() => {
    loadTasks();
});

const loadTasks = async () => {
    isLoading.value = true;

    const taskResult = await commands.taskGetManyInfo(selectedProject.value.id);

    isLoading.value = false;

    if (taskResult.status === "error") {
        notify.error(t("action.load.error", { type: t("task.plural") }), { error: taskResult.error });
        return;
    }

    tasks.value = taskResult.data;
};
</script>
