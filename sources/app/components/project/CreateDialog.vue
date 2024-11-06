<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800">
        <VCard>
            <VCardTitle>
                <VIcon color="success" icon="mdi-folder-plus" />
                {{ $t("project.create.title") }}
            </VCardTitle>
            <VCardText>
                <VForm v-model="isFormValid">
                    <VTextField v-model="project.name" autofocus />
                </VForm>
            </VCardText>
            <VCardActions>
                <VBtn @click="createProject()">{{ $t("action.create") }}</VBtn>
                <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts">
import type { CreateProjectContract } from "~/types/tauriBindings";

const i18n = useI18n();
const notify = useNotify();

const isDialogOpen = ref(false);
const isFormValid = ref(false);

const project = ref({} as CreateProjectContract);

const createProject = async () => {
    notify.success(i18n.t("action.close"));

    /*const createResult = await commands.createProject(project.value);

    console.log(createResult);*/
};
</script>
