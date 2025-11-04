<template>
    <VBtn @click="cloneTask()" :loading="isLoading" color="success" prependIcon="mdi-content-duplicate" variant="flat">
        {{ $t("action.clone") }}
    </VBtn>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    cloned: [task: TaskContract];
}>();

const props = defineProps<{
    taskClone: TaskCloneContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const taskStore = useTaskStore();

const isLoading = ref(false);

const cloneTask = async () => {
    isLoading.value = true;

    const cloneResult = await commands.taskClone(props.taskClone);

    isLoading.value = false;

    if (cloneResult.status == "error") {
        notify.error(t("action.clone.error", { type: t("task.singular"), name: props.taskClone.newName }), { error: cloneResult.error });
        return;
    }

    notify.success(t("action.clone.success", { type: t("task.singular"), name: props.taskClone.newName }));

    taskStore.loadAll();

    emit("cloned", cloneResult.data);
};
</script>
