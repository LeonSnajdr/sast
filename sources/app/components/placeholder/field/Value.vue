<template>
    <VRowSingle>
        <VTextField
            v-bind="$attrs"
            v-model.trim="value"
            :label="$t('placeholder.field.value')"
            :rules="[required($t('validation.rule.required', { field: $t('placeholder.field.value') }))]"
            class="required"
        />
    </VRowSingle>
</template>

<script setup lang="ts">
const value = defineModel<string>();
</script>
