<template>
    <BaseDrawerForm v-model="isDrawerOpen">
        <template #title>
            {{ $t("title.edit", { type }) }}
        </template>
        <template #actions="{ isFormValid }">
            <slot :isFormValid name="actions" />
        </template>
        <template #fields="{ isFormValid }">
            <slot :isFormValid name="fields" />
        </template>
    </BaseDrawerForm>
</template>

<script setup lang="ts">
defineProps<{
    type: string;
}>();

const isDrawerOpen = defineModel<boolean>({ required: true });
</script>
