<template>
    <VTextField
        v-bind="$attrs"
        v-model.trim="name"
        :label="$t('project.field.name')"
        :rules="[
            required($t('validation.rule.required', { field: $t('project.field.name') })),
            validName($t('validation.rule.validName', { field: $t('placeholder.field.name') }))
        ]"
    />
</template>

<script setup lang="ts">
const name = defineModel<string>({ required: true });
</script>
