<template>
    <VRow>
        <VCol v-for="themeItem in themeItems" :key="themeItem.name" cols="12" sm="6">
            <VThemeProvider :theme="themeItem.name">
                <VCard
                    @click="selectTheme(themeItem.name)"
                    :class="{ selected: themeItem.name === selectedTheme }"
                    :text="themeItem.description"
                    :title="themeItem.title"
                >
                    <template #prepend>
                        <VIcon :color="themeItem.color" :icon="themeItem.icon" />
                    </template>
                    <VCardText class="d-flex flex-column ga-2">
                        <div class="d-flex ga-2 align-center">
                            <VChip :color="''" class="w-50" variant="tonal" />
                            <VChip :color="''" class="w-50" density="compact" variant="tonal" />
                        </div>
                        <VChip :color="''" class="w-100" density="compact" variant="tonal" />
                        <VChip :color="''" class="w-100" density="compact" variant="tonal" />
                    </VCardText>
                </VCard>
            </VThemeProvider>
        </VCol>
    </VRow>
</template>

<script setup lang="ts">
const selectedTheme = defineModel<string>({ required: true });

const { t } = useI18n();
const { global: uiTheme } = useTheme();

const themeItems = computed(() => [
    {
        title: t("setting.presentation.theme.dark"),
        description: t("setting.presentation.theme.dark.description"),
        name: "dark",
        icon: "mdi-weather-night",
        color: "secondary"
    },
    {
        title: t(`setting.presentation.theme.light`),
        description: t(`setting.presentation.theme.light.description`),
        name: "light",
        icon: "mdi-weather-sunny",
        color: "success"
    }
]);

const selectTheme = (name: string) => {
    uiTheme.name.value = name;
    selectedTheme.value = name;
};
</script>

<style lang="scss">
.v-card {
    border: 1px solid transparent;
    &.selected {
        border-color: rgb(var(--v-theme-info));
    }
}
</style>
