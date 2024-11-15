<template>
    <VRow>
        <VCol v-for="theme in availableThemes" :key="theme.name" cols="12" sm="6">
            <VThemeProvider :theme="theme.name">
                <VCard @click="selectTheme(theme.name)" :class="{ selected: theme.name === currentThemeName }" :text="theme.description" :title="theme.title">
                    <template #prepend>
                        <VIcon :color="theme.color" :icon="theme.icon" />
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
const { t } = useI18n();
const { global, name: currentThemeName } = useTheme();

const availableThemes = computed(() => [
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
    global.name.value = name;
};
</script>

<style lang="scss">
.v-card {
    &.selected {
        border: 1px solid rgb(var(--v-theme-info));
    }
}
</style>
