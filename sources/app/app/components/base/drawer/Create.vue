<template>
    <BaseDrawer v-model="isDrawerOpen">
        <template #title>
            {{ $t("title.create", { type }) }}
        </template>
        <template #actions>
            <slot name="actions" />
        </template>
        <template #content>
            <slot name="content" />
        </template>
    </BaseDrawer>
</template>

<script setup lang="ts" generic="T">
import type { VForm } from "vuetify/components";

const props = defineProps<{
    type: string;
    emptyElement: T;
}>();

const isDrawerOpen = defineModel<boolean>({ required: true });
const element = defineModel<T>("element", { required: true });

const form = ref<VForm>();

onBeforeMount(() => {
    resetDrawer();
});

const resetDrawer = () => {
    form.value?.resetValidation();
    element.value = lodCloneDeep(props.emptyElement);
};

watch(isDrawerOpen, () => {
    resetDrawer();
});
</script>
