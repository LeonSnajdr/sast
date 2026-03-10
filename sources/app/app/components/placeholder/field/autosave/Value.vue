<template>
    <BaseFieldAutosave v-model="placeholder" :isSaving="isPlaceholderSaving" :save="savePlaceholder">
        <template #default="{ cloned, isSaving, save }">
            <PlaceholderFieldValue v-bind="$attrs" v-model="cloned.value" @blur="save">
                <template v-for="(_, name) in $slots" #[name]="slotData">
                    <slot :name="name" v-bind="slotData" />
                </template>
                <template #append-inner>
                    <VProgressCircular v-show="isSaving" size="x-small" width="2" indeterminate />
                </template>
            </PlaceholderFieldValue>
        </template>
    </BaseFieldAutosave>
</template>

<script setup lang="ts">
const placeholder = defineModel<PlaceholderContract>({ required: true });

const { savePlaceholder, isPlaceholderSaving } = usePlaceholder();
</script>
