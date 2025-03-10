<template>
    <VRowSingle>
        <VTextField
            v-bind="$attrs"
            v-model.trim="tabName"
            :label="$t('task.field.tabName')"
            :rules="[validName($t('validation.rule.validName', { field: $t('task.field.tabName') }))]"
        />
    </VRowSingle>
</template>

<script setup lang="ts">
const tabName = defineModel<string | null>();
</script>
