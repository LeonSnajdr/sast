<template>
    <VForm ref="form">
        <slot :cloned="cloned" :isSaving="isSaving" :save="handleSave" />
    </VForm>
</template>

<script setup lang="ts" generic="T">
const emit = defineEmits<{
    save: [item: T];
}>();

defineProps<{
    isSaving: boolean;
}>();

const element = defineModel<T>({ required: true });

const form = useTemplateRef("form");
const { cloned, sync } = useCloned(element);

const handleSave = async () => {
    if (!form.value?.isValid) {
        sync();
        return;
    }

    emit("save", cloned.value);
};
</script>
