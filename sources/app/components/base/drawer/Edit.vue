<template>
    <BaseDrawer v-model="isDrawerOpen">
        <template #title>
            {{ $t("title.edit", { type }) }}
        </template>
        <template #actions>
            <slot name="actions" />
        </template>
        <template #content>
            <slot name="content" />
        </template>
    </BaseDrawer>
</template>

<script setup lang="ts">
defineProps<{
    type: string;
}>();

const isDrawerOpen = defineModel<boolean>({ required: true });
</script>
