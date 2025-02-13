<template>
    <VRowSingle>
        <VTextField
            v-model.trim="name"
            :label="$t('placeholder.field.name')"
            :rules="[required($t('validation.rule.required', { field: $t('placeholder.field.name') }))]"
        />
    </VRowSingle>
</template>

<script setup lang="ts">
const name = defineModel<string>();
</script>
