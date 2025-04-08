<template>
    <VForm ref="form" v-model="isValid">
        <PlaceholderFieldName v-model="placeholder.name" autofocus />
        <PlaceholderFieldValue v-model="placeholder.value" />
        <PlaceholderFieldVisibility v-model="placeholder.visibility" />
    </VForm>
</template>

<script setup lang="ts">
const placeholder = defineModel<PlaceholderCreateContract | PlaceholderContract>({ required: true });
const isValid = defineModel<boolean | null>("isValid", { required: true });
</script>
