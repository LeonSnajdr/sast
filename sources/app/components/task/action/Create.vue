<template>
    <IconBtn color="secondary" icon="mdi-plus" variant="flat">
        {{ $t("action.create") }}
        <VDialog v-model="isDialogOpen" activator="parent" width="800" eager>
            <VCard>
                <VCardTitle>
                    <VIcon color="success" icon="mdi-label" />
                    {{ $t("task.create.title") }}
                </VCardTitle>
                <VCardText>
                    <VForm v-model="isFormValid">
                        <TaskFieldName v-model="task.name" />
                        <TaskFieldBlocking v-model="task.blocking" />
                        <TaskFieldCommandTiles v-model="task.workingDirTiles" />
                        <TaskFieldWorkingDirTiles v-model="task.commandTiles" />
                    </VForm>
                </VCardText>
                <VCardActions>
                    <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
                    <VBtn @click="createTask()" :disabled="!isFormValid" :loading="isLoading">{{ $t("action.create") }}</VBtn>
                </VCardActions>
            </VCard>
        </VDialog>
    </IconBtn>
</template>

<script setup lang="ts">
const emit = defineEmits(["created"]);

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const isFormValid = ref(false);
const isLoading = ref(false);

const task = ref<TaskCreateContract>({} as TaskCreateContract);

onBeforeMount(() => {
    setEmptyTask();
});

const setEmptyTask = () => {
    task.value = {
        projectId: selectedProject.value.id,
        name: "",
        blocking: false,
        workingDirTiles: [] as PlaceholderInsertTileContract[],
        commandTiles: [] as PlaceholderInsertTileContract[]
    };
};

const createTask = async () => {
    isLoading.value = true;

    const createResult = await commands.taskCreate(task.value);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(t("task.create.error"));
        console.error(createResult);
        return;
    }

    notify.success(t("task.create.success", { name: task.value.name }));

    isDialogOpen.value = false;

    emit("created");
};

watch(isDialogOpen, () => {
    setEmptyTask();
});
</script>
