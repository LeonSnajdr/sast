<template>
    <BaseBtnIcon color="secondary" icon="mdi-plus" variant="flat">
        {{ $t("action.create") }}
        <VDialog v-model="isDialogOpen" activator="parent" width="800" eager>
            <VCard>
                <VCardTitle>
                    <VIcon color="success" icon="mdi-checkbox-multiple-marked-circle-outline" />
                    {{ $t("action.create.title", { type: $t("taskSet.singular") }) }}
                </VCardTitle>
                <VCardText>
                    <VForm v-model="isFormValid">
                        <TaskSetFieldName v-model="taskSet.name" />
                    </VForm>
                </VCardText>
                <VCardActions>
                    <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
                    <VBtn @click="createdTaskSet()" :disabled="!isFormValid" :loading="isLoading">{{ $t("action.create") }}</VBtn>
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

const taskSet = ref<TaskSetCreateContract>({} as TaskSetCreateContract);

onBeforeMount(() => {
    setEmptyTaskSet();
});

const setEmptyTaskSet = () => {
    taskSet.value = {
        projectId: selectedProject.value.id,
        name: ""
    };
};

const createdTaskSet = async () => {
    isLoading.value = true;

    const createResult = await commands.taskSetCreate(taskSet.value);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(t("action.create.error", { type: t("taskSet.singular"), name: taskSet.value.name }), { error: createResult.error });
        return;
    }

    notify.success(t("action.create.success", { type: t("taskSet.singular"), name: taskSet.value.name }));

    isDialogOpen.value = false;

    emit("created");
};

watch(isDialogOpen, () => {
    setEmptyTaskSet();
});
</script>
