<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800">
        <VCard>
            <VCardTitle>
                <VIcon :icon />
                {{ $t("title.edit", { type }) }}
            </VCardTitle>
            <VCardText>
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
}>();

const isDialogOpen = defineModel<boolean>({ required: true });

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
