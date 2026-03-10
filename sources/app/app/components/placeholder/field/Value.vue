<template>
    <VRowSingle>
        <VTextField
            v-bind="$attrs"
            v-model.trim="value"
            :label="$t('placeholder.field.value')"
            :rules="[required($t('validation.rule.required', { field: $t('placeholder.field.value') }))]"
            class="required"
        >
            <template v-for="(_, name) in $slots" #[name]="slotData">
                <slot :name="name" v-bind="slotData" />
            </template>
        </VTextField>
    </VRowSingle>
</template>

<script setup lang="ts">
const value = defineModel<string>();
</script>
