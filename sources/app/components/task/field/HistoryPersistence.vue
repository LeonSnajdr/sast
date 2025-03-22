<template>
    <VRowSingle>
        <VCombobox
            v-bind="$attrs"
            v-model="historyPersistence"
            :items
            :label="$t('task.field.historyPersistence')"
            :returnObject="false"
            :rules="[required($t('validation.rule.required', { field: $t('task.field.historyPersistence') }))]"
            class="required"
            itemTitle="translation"
            itemValue="value"
        />
    </VRowSingle>
</template>

<script setup lang="ts">
const historyPersistence = defineModel<TerminalHistoryPersistence>();

const { t } = useI18n();

const items = computed((): { value: TerminalHistoryPersistence; translation: string }[] => {
    return [
        {
            value: "Always",
            translation: t("task.field.historyPersistence.always")
        },
        {
            value: "Never",
            translation: t("task.field.historyPersistence.never")
        },
        {
            value: "OnError",
            translation: t("task.field.historyPersistence.onError")
        },
        {
            value: "OnSuccess",
            translation: t("task.field.historyPersistence.onSuccess")
        }
    ];
});
</script>
