<template>
    <VBtn @click="createProject()" :loading="isLoading" color="success" prependIcon="mdi-plus" variant="flat">
        {{ $t("action.create") }}
    </VBtn>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [id: string];
}>();

const props = defineProps<{
    project: ProjectCreateContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const isLoading = ref(false);

const createProject = async () => {
    isLoading.value = true;

    const createResult = await commands.projectCreate(props.project);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(t("action.create.error", { type: t("project.singular"), name: props.project.name }), { error: createResult.error });
        return;
    }

    notify.success(t("action.create.success", { type: t("project.singular"), name: props.project.name }));

    emit("created", createResult.data.id);
};
</script>
