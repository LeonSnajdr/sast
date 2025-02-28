<template>
    <BaseBtnIcon color="secondary" icon="mdi-plus" variant="flat">
        {{ $t("action.create") }}
        <VDialog v-model="isDialogOpen" activator="parent" width="800" eager>
            <VCard>
                <VCardTitle>
                    <VIcon color="success" icon="mdi-label" />
                    {{ $t("action.create.title", { type: $t("task.singular") }) }}
                </VCardTitle>
                <VCardText>
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
                <VCardActions>
                    <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
                    <VBtn @click="createTask()" :disabled="!isFormValid" :loading="isLoading">{{ $t("action.create") }}</VBtn>
                </VCardActions>
            </VCard>
        </VDialog>
    </BaseBtnIcon>
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
        tabName: null,
        noExit: true,
        workingDirTiles: [] as PlaceholderInsertTileContract[],
        commandTiles: [] as PlaceholderInsertTileContract[]
    };
};

const createTask = async () => {
    isLoading.value = true;

    const createResult = await commands.taskCreate(task.value);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(t("action.create.error", { type: t("task.singular"), name: task.value.name }), { error: createResult.error });
        return;
    }

    notify.success(t("action.create.success", { type: t("task.singular"), name: task.value.name }));

    isDialogOpen.value = false;

    emit("created");
};

watch(isDialogOpen, () => {
    setEmptyTask();
});
</script>
