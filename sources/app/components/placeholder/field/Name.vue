<template>
    <VRowSingle>
        <VTextField
            v-bind="$attrs"
            v-model.trim="name"
            :label="$t('placeholder.field.name')"
            :rules="[
                required($t('validation.rule.required', { field: $t('placeholder.field.name') })),
                validName($t('validation.rule.validName', { field: $t('placeholder.field.name') }))
            ]"
        />
    </VRowSingle>
</template>

<script setup lang="ts">
const name = defineModel<string>();
</script>
