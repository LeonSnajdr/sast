<template>
    <VNavigationDrawer v-model="isDrawerOpen" location="right" width="450" disableResizeWatcher>
        <VList>
            <VListItem height="48">
                <slot :isFormValid name="title" />
                <template #append>
                    <BaseBtnIcon @click="isDrawerOpen = false" class="mr-2" icon="mdi-close">{{ $t("action.close") }}</BaseBtnIcon>
                    <slot :isFormValid name="actions" />
                </template>
            </VListItem>
            <VDivider />
            <VContainer>
                <VForm ref="form" v-model="isFormValid">
                    <slot :isFormValid name="fields" />
                </VForm>
            </VContainer>
        </VList>
    </VNavigationDrawer>
</template>

<script setup lang="ts" generic="T">
import { VForm } from "vuetify/components";

const isDrawerOpen = defineModel<boolean>({ required: true });

const form = ref<VForm>();
const isFormValid = ref(false);

onBeforeMount(() => {
    resetDrawer();
});

const resetDrawer = () => {
    form.value?.resetValidation();
};

watch(isDrawerOpen, () => {
    resetDrawer();
});
</script>
