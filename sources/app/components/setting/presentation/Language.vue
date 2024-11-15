<template>
    <VRowSingle>
        <VCard title="Sprache">
            <template #prepend>
                <VIcon color="info" icon="mdi-web" />
            </template>

            <VCardText>
                <div class="v-chip-group">
                    <VChip
                        v-for="language in availableLanguages"
                        :key="language.language"
                        @click="selectLanguage(language.language)"
                        :variant="language.language === locale ? 'flat' : 'outlined'"
                        density="comfortable"
                    >
                        {{ language.translation }}
                    </VChip>
                </div>
            </VCardText>
        </VCard>
    </VRowSingle>
</template>

<script setup lang="ts">
const { t, locale } = useI18n();

const availableLanguages = computed(() => [
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
    locale.value = language;
};
</script>
