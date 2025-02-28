<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("taskSet.plural") }}</VAppBarTitle>
        <TaskSetDialogCreate />
    </VAppBar>

    <VContainer class="h-100">
        <VCard>
            <VCardText>
                <TaskSetTable :taskSets="taskSets" />
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
const taskSets = ref<TaskSetInfoContract[]>([]);

onBeforeMount(() => {
    loadTaskSets();
});

const loadTaskSets = async () => {
    isLoading.value = true;

    const taskResult = await commands.taskSetGetManyInfo(selectedProject.value.id);

    isLoading.value = false;

    if (taskResult.status === "error") {
        notify.error(t("action.load.error", { type: t("taskSet.plural") }), { error: taskResult.error });
        return;
    }

    taskSets.value = taskResult.data;
};
</script>
