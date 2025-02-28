<template>
    <VBtn @click="createTaskSet()">
        {{ $t("action.create") }}
    </VBtn>
</template>

<script setup lang="ts">
const emit = defineEmits(["created"]);

const props = defineProps<{
    taskSet: TaskSetCreateContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const isLoading = ref(false);

const createTaskSet = async () => {
    isLoading.value = true;

    const createResult = await commands.taskSetCreate(props.taskSet);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(t("action.create.error", { type: t("taskSet.singular"), name: props.taskSet.name }), { error: createResult.error });
        return;
    }

    notify.success(t("action.create.success", { type: t("taskSet.singular"), name: props.taskSet.name }));

    emit("created");
};
</script>
