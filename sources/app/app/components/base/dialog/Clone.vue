<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800">
        <VCard :loading class="h-100">
            <VCardTitle>
                <VIcon :icon color="success" />
                {{ $t("title.clone", { type }) }}
            </VCardTitle>
            <VCardText class="h-100 overflow-hidden">
                <slot name="content" />
            </VCardText>
            <VCardActions>
                <VBtn @click="isDialogOpen = false" variant="flat">{{ $t("action.close") }}</VBtn>
                <slot name="actions" />
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts">
import type { VForm } from "vuetify/components";

defineProps<{
    icon: string;
    type: string;
    loading?: boolean;
}>();

const isDialogOpen = defineModel<boolean>({ required: true });

// Form that is inside the content slot
const form = ref<VForm>();

onBeforeMount(() => {
    resetDialog();
});

const resetDialog = () => {
    form.value?.resetValidation();
};

watch(isDialogOpen, () => {
    resetDialog();
});
</script>
