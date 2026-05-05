<template>
    <VRowSingle v-if="mode === 'row'">
        <VListItem class="px-0">
            <VListItemTitle>{{ $t("taskSet.field.favorite") }}</VListItemTitle>

            <template #append>
                <BaseBtnToggle v-model="favorite" :color="favorite ? 'warning' : 'secondary'" :icon="favorite ? 'mdi-star' : 'mdi-star-outline'" />
            </template>
        </VListItem>
    </VRowSingle>

    <BaseBtnToggle v-else v-model="favorite" :color="favorite ? 'warning' : 'secondary'" :icon="favorite ? 'mdi-star' : 'mdi-star-outline'" />
</template>

<script setup lang="ts">
const { mode = "row" } = defineProps<{
    mode?: "button" | "row";
}>();

const favorite = defineModel<boolean>({ required: true });
</script>
