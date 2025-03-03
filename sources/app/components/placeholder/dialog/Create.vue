<template>
    <BaseDialogCreate v-model="isDialogOpen" :emptyElement :type="$t('placeholder.singular')" icon="mdi-label">
        <template #actions="{ isFormValid, element }">
            <PlaceholderActionCreate @created="placeholderCreated" :disabled="!isFormValid" :placeholder="element" />
        </template>
        <template #fields="{ element: placeholder }">
            <PlaceholderFieldName v-model="placeholder.name" />
            <PlaceholderFieldValue v-model="placeholder.value" />
            <PlaceholderFieldVisibility v-model="placeholder.visibility" />
        </template>
    </BaseDialogCreate>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [id: string];
}>();

const isDialogOpen = defineModel<boolean>({ required: true });

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const emptyElement: PlaceholderCreateContract = {
    projectId: selectedProject.value.id,
    name: "",
    value: "",
    visibility: "Project",
    kind: "Text",
    source: "Static"
};

const placeholderCreated = (id: string) => {
    isDialogOpen.value = false;
    emit("created", id);
};
</script>
