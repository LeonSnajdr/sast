<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800">
        <VCard>
            <VCardTitle>
                <VIcon :icon color="success" />
                {{ $t("title.create", { type }) }}
            </VCardTitle>
            <VCardText>
                <slot name="content" />
            </VCardText>
            <VCardActions>
                <BaseBtnIcon @click="isDialogOpen = false" variant="flat">{{ $t("action.close") }}</BaseBtnIcon>
                <slot name="actions" />
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts" generic="T">
import type { VForm } from "vuetify/components";

const props = defineProps<{
    icon: string;
    type: string;
    emptyElement: T;
}>();

const isDialogOpen = defineModel<boolean>({ required: true });
const element = defineModel<T>("element", { required: true });

const form = ref<VForm>();

onBeforeMount(() => {
    resetDialog();
});

const resetDialog = () => {
    form.value?.resetValidation();
    element.value = lodCloneDeep(props.emptyElement);
};

watch(isDialogOpen, () => {
    resetDialog();
});
</script>
