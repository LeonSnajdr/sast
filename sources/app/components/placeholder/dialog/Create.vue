<template>
    <BaseDialogCreate v-model="isDialogOpen" v-model:element="placeholder" :emptyElement :type="$t('placeholder.singular')" icon="mdi-label">
        <template #content>
            <PlaceholderFieldContainer v-model="placeholder" v-model:isValid="isPlaceholderValid" />
        </template>
        <template #actions>
            <PlaceholderActionCreate @created="placeholderCreated" :disabled="!isPlaceholderValid" :placeholder />
        </template>
    </BaseDialogCreate>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [id: string];
}>();

const placeholder = ref({} as PlaceholderCreateContract);
const isPlaceholderValid = ref<boolean | null>(false);
const isDialogOpen = ref(false);

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
