<template>
    <VForm ref="form">
        <PlaceholderFieldValue v-bind="$attrs" v-model="cloned.value" @blur="save">
            <template v-for="(_, name) in $slots" #[name]="slotData">
                <slot :name="name" v-bind="slotData" />
            </template>
            <template #append-inner>
                <VProgressCircular v-show="isPlaceholderSaving" size="x-small" width="2" indeterminate />
            </template>
        </PlaceholderFieldValue>
    </VForm>
</template>

<script setup lang="ts">
const placeholder = defineModel<PlaceholderContract>({ required: true });

const form = useTemplateRef("form");
const { savePlaceholder, isPlaceholderSaving } = usePlaceholder();

const { cloned, sync } = useCloned(placeholder);

const save = async () => {
    if (!form.value?.isValid) {
        sync();
        return;
    }

    await savePlaceholder(cloned.value);
};
</script>
