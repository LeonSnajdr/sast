<template>
    <VRowSingle>
        <VCard title="Sprache">
            <template #prepend>
                <VIcon color="info" icon="mdi-web" />
            </template>

            <VCardText>
                <div class="v-chip-group">
                    <VChip
                        v-for="languageItem in laguageItems"
                        :key="languageItem.language"
                        @click="selectLanguage(languageItem.language)"
                        :variant="languageItem.language === selectedLangauge ? 'flat' : 'outlined'"
                        density="comfortable"
                    >
                        {{ languageItem.translation }}
                    </VChip>
                </div>
            </VCardText>
        </VCard>
    </VRowSingle>
</template>

<script setup lang="ts">
const selectedLangauge = defineModel<string>({ required: true });

const { t, setLocale } = useI18n();

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

const selectLanguage = (language: string) => {
    setLocale(language);
    selectedLangauge.value = language;
};
</script>
