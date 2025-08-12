<template>
    <VRowSingle>
        <VCard class="border" variant="outlined">
            <VCardTitle>
                <VIcon icon="mdi-shape-outline" />
                {{ $t("setting.presentation.theme.title") }}
            </VCardTitle>
            <VCardText>
                <VRow>
                    <VCol v-for="themeItem in themeItems" :key="themeItem.theme" :cols="themeItem.cols">
                        <VCard
                            @click="selectTheme(themeItem.theme)"
                            :class="{ selected: themeItem.theme === selectedTheme, border: themeItem.outlined }"
                            :theme="themeItem.theme"
                            :variant="themeItem.outlined ? 'outlined' : 'flat'"
                            density="compact"
                        >
                            <VCardTitle>
                                <VIcon :color="themeItem.color" :icon="themeItem.icon" />
                                {{ themeItem.title }}
                            </VCardTitle>
                            <VCardText>
                                {{ themeItem.description }}
                            </VCardText>
                        </VCard>
                    </VCol>
                </VRow>
            </VCardText>
        </VCard>
    </VRowSingle>
</template>

<script setup lang="ts">
const selectedTheme = defineModel<string>({ required: true });

const presentation = usePresentation();
const { t } = useI18n();

const themeItems = computed(() => [
    {
        title: t("setting.presentation.theme.system"),
        description: t("setting.presentation.theme.system.description"),
        theme: "system",
        icon: "mdi-laptop",
        outlined: true,
        cols: 12
    },
    {
        title: t("setting.presentation.theme.dark"),
        description: t("setting.presentation.theme.dark.description"),
        theme: "dark",
        icon: "mdi-weather-night",
        color: "secondary",
        cols: 6
    },
    {
        title: t(`setting.presentation.theme.light`),
        description: t(`setting.presentation.theme.light.description`),
        theme: "light",
        icon: "mdi-weather-sunny",
        color: "success",
        cols: 6
    }
]);

const selectTheme = (theme: string) => {
    presentation.applyTheme(theme);
    selectedTheme.value = theme;
};
</script>

<style lang="scss">
.v-card {
    border: 1px solid transparent;
    &.selected {
        border-color: rgb(var(--v-theme-info)) !important;
    }
}
</style>
