<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800" eager>
        <VCard>
            <VCardTitle>
                <VIcon color="success" icon="mdi-label" />
                {{ $t("placeholder.create.title") }}
            </VCardTitle>
            <VCardText>
                <VForm ref="form" v-model="isFormValid">
                    <PlaceholderFieldName v-model="placeholder.name" />
                    <PlaceholderFieldValue v-model="placeholder.value" />
                    <PlaceholderFieldProjectId v-model="placeholder.projectId" />
                </VForm>
            </VCardText>
            <VCardActions>
                <VBtn @click="createPlaceholder()" :disabled="!isFormValid" :loading="isLoading">{{ $t("action.create") }}</VBtn>
                <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [];
}>();

const notify = useNotify();
const { t } = useI18n();

const form = ref();
const isDialogOpen = ref(false);
const isFormValid = ref(false);
const isLoading = ref(false);

const placeholder = ref({} as CreatePlaceholderContract);

const createPlaceholder = async () => {
    isLoading.value = true;

    const createResult = await commands.createPlaceholder(placeholder.value);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(t("placeholder.create.error"));

        return;
    }

    notify.success(t("placeholder.create.success", { placeholderName: createResult.data.name }));

    isDialogOpen.value = false;

    emit("created");
};

watch(isDialogOpen, () => {
    form.value!.reset();
});
</script>
