<template>
    <VRowSingle>
        <BaseChipSelect v-model="visibility" :items="items" itemText="translation" itemValue="visibility" />
    </VRowSingle>
</template>

<script setup lang="ts">
const visibility = defineModel<PlaceholderVisibility>({ required: true });

const { t } = useI18n();

const items = computed((): { visibility: PlaceholderVisibility; translation: string }[] => [
    {
        visibility: "Global",
        translation: t("placeholder.field.visibility.global")
    },
    {
        visibility: "Project",
        translation: t("placeholder.field.visibility.project")
    }
]);
</script>
