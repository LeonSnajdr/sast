<template>
    <BaseBtnIcon color="secondary" icon="mdi-plus" variant="flat">
        {{ $t("action.create") }}
        <VDialog v-model="isDialogOpen" activator="parent" width="800" eager>
            <VCard>
                <VCardTitle>
                    <VIcon color="success" icon="mdi-label" />
                    {{ $t("action.create.title", { type: $t("placeholder.singular") }) }}
                </VCardTitle>
                <VCardText>
                    <VForm ref="form" v-model="isFormValid">
                        <PlaceholderFieldName v-model="placeholder.name" />
                        <PlaceholderFieldValue v-model="placeholder.value" />
                        <PlaceholderFieldVisibility v-model="placeholder.visibility" />
                    </VForm>
                </VCardText>
                <VCardActions>
                    <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
                    <VBtn @click="createPlaceholder()" :disabled="!isFormValid" :loading="isLoading">{{ $t("action.create") }}</VBtn>
                </VCardActions>
            </VCard>
        </VDialog>
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();
const placeholderStore = usePlaceholderStore();

const { selectedProject } = storeToRefs(projectStore);

const form = ref();
const isDialogOpen = ref(false);
const isFormValid = ref(false);
const isLoading = ref(false);

const placeholder = ref({ projectId: selectedProject.value.id, visibility: "Project", kind: "Text", source: "Static" } as PlaceholderCreateContract);

const createPlaceholder = async () => {
    isLoading.value = true;

    const createResult = await commands.placeholderCreate(placeholder.value);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(t("action.create.error", { type: t("placeholder.singular"), name: placeholder.value.name }), { error: createResult.error });
        return;
    }

    notify.success(t("action.create.success", { type: t("placeholder.singular"), name: placeholder.value.name }));

    isDialogOpen.value = false;

    placeholderStore.loadAll();
};

watch(isDialogOpen, () => {
    form.value!.reset();
});
</script>
