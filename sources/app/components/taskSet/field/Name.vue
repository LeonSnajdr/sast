<template>
    <VRowSingle>
        <VTextField
            v-model.trim="name"
            :label="$t('taskSet.field.name')"
            :rules="[
                required($t('validation.rule.required', { field: $t('taskSet.field.name') })),
                validName($t('validation.rule.validName', { field: $t('taskSet.field.name') }))
            ]"
        />
    </VRowSingle>
</template>

<script setup lang="ts">
const name = defineModel<string>();
</script>
