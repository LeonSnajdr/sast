<template>
    <VChip @click="isSelected = !isSelected" :color="isSelected ? 'primary' : ''" :variant="isSelected ? 'flat' : 'tonal'" density="comfortable">
        <slot v-if="!text" />
        {{ text }}
        <VIcon v-if="infoText" class="ml-2" icon="mdi-information-outline" v-tooltip:top="infoText" />
    </VChip>
</template>

<script setup lang="ts" generic="T">
defineProps<{
    text?: string;
    infoText?: string;
}>();

const isSelected = defineModel<boolean>({ required: true });
</script>
