<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800" eager>
        <VCard>
            <VCardTitle>
                <VIcon color="success" icon="mdi-folder-plus" />
                {{ $t("project.create.title") }}
            </VCardTitle>
            <VCardText>
                <VForm ref="form" v-model="isFormValid">
                    <VTextField
                        v-model="project.name"
                        :label="$t('project.field.name')"
                        :rules="[required($t('validation.rule.required', { field: $t('project.field.name') }))]"
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

const form = ref();

const isDialogOpen = ref(false);
const isFormValid = ref(false);
const isLoading = ref(false);

const project = ref({} as CreateProjectContract);

const createProject = async () => {
    isLoading.value = true;

    const createResult = await commands.createProject(project.value);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(i18n.t("project.create.error"));

        return;
    }

    notify.success(i18n.t("project.create.sucess", { projectName: createResult.data.name }));

    isDialogOpen.value = false;

    navigateTo({ name: "project-id-home", params: { id: createResult.data.id } })
};

watch(isDialogOpen, () => {
    form.value!.reset();
});
</script>
