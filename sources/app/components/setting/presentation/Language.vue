<template>
    <VRowSingle>
        <VCard title="Sprache">
            <template #prepend>
                <VIcon color="info" icon="mdi-web" />
            </template>

            <VCardText>
                <div class="v-chip-group">
                    <VChip v-for="language in availableLanguages" :key="language.value" density="comfortable" variant="outlined">
                        {{ language.title }}
                    </VChip>
                </div>
            </VCardText>
        </VCard>
    </VRowSingle>
</template>

<script setup lang="ts">
const { t, availableLocales } = useI18n();

const availableLanguages = computed(() => availableLocales.map((locale) => ({ title: t(`setting.presentation.language.${locale}`), value: locale })));
</script>
