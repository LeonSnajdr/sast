<template>
    <VRowSingle>
        <VTextField
            v-bind="$attrs"
            v-model.trim="name"
            :label="$t('task.field.name')"
            :rules="[
                required($t('validation.rule.required', { field: $t('task.field.name') })),
                validName($t('validation.rule.validName', { field: $t('task.field.name') }))
            ]"
        />
    </VRowSingle>
</template>

<script setup lang="ts">
const name = defineModel<string>();
</script>
