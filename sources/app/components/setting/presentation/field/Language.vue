<template>
    <VRowSingle>
        <VCard>
            <VCardTitle>
                <VIcon color="info" icon="mdi-web" />
                {{ $t("setting.presentation.language.title") }}
            </VCardTitle>
            <VCardText>
                <BaseChipSelect v-model="selectedLangauge" :items="laguageItems" itemText="translation" itemValue="language" />
            </VCardText>
        </VCard>
    </VRowSingle>
</template>

<script setup lang="ts">
const selectedLangauge = defineModel<string>({ required: true });

const presentation = usePresentation();
const { t } = useI18n();

const laguageItems = computed(() => [
    {
        language: "en",
        translation: t("setting.presentation.language.en")
    },
    {
        language: "de",
        translation: t("setting.presentation.language.de")
    }
]);

watch(selectedLangauge, () => {
    presentation.applyLangauge(selectedLangauge.value);
});
</script>
