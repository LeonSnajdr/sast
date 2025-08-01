<template>
    <VRowSingle>
        <VCard>
            <VCardTitle>
                <VIcon color="info" icon="mdi-web" />
                {{ $t("setting.presentation.language.title") }}
            </VCardTitle>
            <VCardText>
                <VBtnToggle v-model="selectedLangauge" @click.stop.prevent density="compact">
                    <VBtn v-for="languageItem of laguageItems" :key="languageItem.language" :value="languageItem.language">
                        {{ languageItem.translation }}
                    </VBtn>
                </VBtnToggle>
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
