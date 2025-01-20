<template>
    <VRowSingle>
        <VTextField
            v-model="value"
            :label="$t('placeholder.field.value')"
            :rules="[required($t('validation.rule.required', { field: $t('placeholder.field.value') }))]"
        />
    </VRowSingle>
</template>

<script setup lang="ts">
const value = defineModel<string>();
</script>
