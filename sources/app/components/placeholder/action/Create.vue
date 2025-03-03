<template>
    <BaseBtnIcon @click="createPlaceholder()" :loading="isLoading" color="success" icon="mdi-plus" variant="flat">
        {{ $t("action.create") }}
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [id: string];
}>();

const props = defineProps<{
    placeholder: PlaceholderCreateContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const placeholderStore = usePlaceholderStore();

const isLoading = ref(false);

const createPlaceholder = async () => {
    isLoading.value = true;

    const createResult = await commands.placeholderCreate(props.placeholder);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(t("action.create.error", { type: t("placeholder.singular"), name: props.placeholder.name }), { error: createResult.error });
        return;
    }

    notify.success(t("action.create.success", { type: t("placeholder.singular"), name: props.placeholder.name }));

    placeholderStore.loadAll();

    emit("created", createResult.data.id);
};
</script>
