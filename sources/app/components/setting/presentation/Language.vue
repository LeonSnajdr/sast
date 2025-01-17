<template>
    <VRowSingle>
        <VCard title="Sprache">
            <template #prepend>
                <VIcon color="info" icon="mdi-web" />
            </template>

            <VCardText>
                <ChipSelect v-model="selectedLangauge" :items="laguageItems" itemText="translation" itemValue="language" />
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
