<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800" eager>
        <VCard>
            <VCardTitle>
                <VIcon color="success" icon="mdi-label" />
                {{ $t("placeholder.create.title") }}
            </VCardTitle>
            <VCardText>
                <VForm ref="form" v-model="isFormValid">
                    <VRowSingle>
                        <VTextField
                            v-model="placeholder.name"
                            :label="$t('project.field.name')"
                            :rules="[required($t('validation.rule.required', { field: $t('placeholder.field.name') }))]"
                        />
                    </VRowSingle>
                    <VRowSingle>
                        <VTextField
                            v-model="placeholder.value"
                            :label="$t('project.field.value')"
                            :rules="[required($t('validation.rule.required', { field: $t('placeholder.field.name') }))]"
                        />
                    </VRowSingle>
                    <VRowSingle>
                        <ChipSelect v-model="placeholder.projectId" :items="projectIdItems" itemText="translation" itemValue="projectId" />
                    </VRowSingle>
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
const projectStore = useProjectStore();

const notify = useNotify();
const { t } = useI18n();

const { selectedProject } = storeToRefs(projectStore);
const form = ref();
const isDialogOpen = ref(false);
const isFormValid = ref(false);
const isLoading = ref(false);

const placeholder = ref({ projectId: selectedProject.value.id } as CreatePlaceholderContract);

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
};

const projectIdItems = computed(() => [
    {
        projectId: null,
        translation: t("project.field.projectId.global")
    },
    {
        projectId: selectedProject.value.id,
        translation: t("project.field.projectId.specific")
    }
]);

watch(isDialogOpen, () => {
    form.value!.reset();
});
</script>
