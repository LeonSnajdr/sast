<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800" eager>
        <VCard>
            <VCardTitle>
                <VIcon color="success" icon="mdi-folder-plus" />
                {{ $t("title.create", { type: $t("project.singular") }) }}
            </VCardTitle>
            <VCardText>
                <VForm ref="form" v-model="isFormValid">
                    <VTextField
                        id="input"
                        v-model.trim="project.name"
                        :label="$t('project.field.name')"
                        :rules="[
                            required($t('validation.rule.required', { field: $t('project.field.name') })),
                            validName($t('validation.rule.validName', { field: $t('placeholder.field.name') }))
                        ]"
                    />
                </VForm>
            </VCardText>
            <VCardActions>
                <VBtn @click="createProject()" :disabled="!isFormValid" :loading="isLoading">{{ $t("action.create") }}</VBtn>
                <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts">
const i18n = useI18n();
const notify = useNotify();
const { t } = useI18n();

const form = ref();

const isDialogOpen = ref(false);
const isFormValid = ref(false);
const isLoading = ref(false);

const project = ref({} as ProjectCreateContract);

const createProject = async () => {
    isLoading.value = true;

    const createResult = await commands.projectCreate(project.value);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(i18n.t("action.create.error", { type: t("project.singular"), name: project.value.name }), { error: createResult.error });
        return;
    }

    notify.success(i18n.t("action.create.success", { type: t("project.singular"), name: project.value.name }));

    isDialogOpen.value = false;

    navigateTo({ name: "index-project-id-home", params: { id: createResult.data.id } });
};

watch(isDialogOpen, () => {
    form.value!.reset();
});
</script>
