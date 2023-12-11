<template>
    <v-card>
        <v-card-title>{{ $t("settingsAppearance.title") }}</v-card-title>
        <v-card-text>
            <v-row-single>
                <v-autocomplete
                    v-model="settings.language"
                    :label="$t('settingsAppearance.input.language')"
                    :items="languages"
                    @update:modelValue="save"
                ></v-autocomplete>
            </v-row-single>
            <v-row-single>
                <v-autocomplete
                    v-model="settings.theme"
                    :label="$t('settingsAppearance.input.theme')"
                    :items="themes"
                    @update:modelValue="save"
                ></v-autocomplete>
            </v-row-single>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import { keys } from "lodash";
import { useI18n } from "vue-i18n";
import { useTheme } from "vuetify";
import * as commands from "@/bindings";
import type { UpdateSettingsContract } from "@/bindings";

const i18n = useI18n();
const theme = useTheme();
const { availableLocales } = useI18n();
const settingsStore = useSettingsStore();

const { settings } = storeToRefs(settingsStore);

const languages = computed(() => availableLocales.map((locale) => ({ title: i18n.t(`settingsAppearance.input.language.${locale}`), value: locale })));

const themes = computed(() => keys(theme.themes.value).map((theme) => ({ title: i18n.t(`settingsAppearance.input.theme.${theme}`), value: theme })));

const save = async () => {
    //TODO error handling

    const updateContract: UpdateSettingsContract = {
        id: settings.value.id,
        language: settings.value.language,
        theme: settings.value.theme,
        default_dir: settings.value.default_dir
    };

    await commands.updateSettings(updateContract);
    await settingsStore.initializeAppWithSettings();
};
</script>
