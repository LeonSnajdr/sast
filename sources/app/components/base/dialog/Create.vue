<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800">
        <VCard>
            <VCardTitle>
                <VIcon :icon color="success" />
                {{ $t("title.create", { type }) }}
            </VCardTitle>
            <VCardText>
                <VForm ref="form" v-model="isFormValid">
                    <slot :element name="fields" />
                </VForm>
            </VCardText>
            <VCardActions>
                <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
                <slot :element :isFormValid name="actions" />
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts" generic="T">
import { VForm } from "vuetify/components";

const props = defineProps<{
    icon: string;
    type: string;
    emptyElement: T;
}>();

const isDialogOpen = defineModel<boolean>();

const form = ref<VForm>();
const isFormValid = ref(false);

const element = ref<T>({} as T);

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
