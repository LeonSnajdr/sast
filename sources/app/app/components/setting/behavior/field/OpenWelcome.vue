<template>
    <VRowSingle>
        <VCard>
            <VCardTitle>
                <VIcon color="success" icon="mdi-application-import" />
                {{ $t("setting.behavior.openWelcome.title") }}
            </VCardTitle>
            <VCardText>
                <BaseChipSelect v-model="openWelcome" :items="laguageItems" itemText="translation" itemValue="value" />
            </VCardText>
        </VCard>
    </VRowSingle>
</template>

<script setup lang="ts">
const openWelcome = defineModel<boolean>({ required: true });

const { t } = useI18n();

const laguageItems = computed(() => [
    {
        value: true,
        translation: t("setting.behavior.openWelcome.true")
    },
    {
        value: false,
        translation: t("setting.behavior.openWelcome.false")
    }
]);
</script>
