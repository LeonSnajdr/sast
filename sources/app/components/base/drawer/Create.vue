<template>
    <BaseDrawerForm v-model="isDrawerOpen">
        <template #title>
            {{ $t("title.create", { type }) }}
        </template>
        <template #actions="{ isFormValid }">
            <slot :element :isFormValid name="actions" />
        </template>
        <template #fields="{ isFormValid }">
            <slot :element :isFormValid name="fields" />
        </template>
    </BaseDrawerForm>
</template>

<script setup lang="ts" generic="T">
const props = defineProps<{
    type: string;
    emptyElement: T;
}>();

const isDrawerOpen = defineModel<boolean>({ required: true });

const element = ref<T>({} as T);

onBeforeMount(() => {
    resetDrawer();
});

const resetDrawer = () => {
    element.value = lodCloneDeep(props.emptyElement);
};

watch(isDrawerOpen, () => {
    resetDrawer();
});
</script>
