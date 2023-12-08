<template>
    <v-card>
        <v-card-title>Settings</v-card-title>
        <v-card-text>
            <v-row-single>
                <v-autocomplete label="Sprache" v-model="settings.language" :items="availableLocales" @update:modelValue="save"></v-autocomplete>
            </v-row-single>
            <v-row-single>
                <v-autocomplete label="Theme" v-model="settings.theme" :items="keys(theme.themes.value)" @update:modelValue="save"></v-autocomplete>
            </v-row-single>
            <v-row-single>
                <v-text-field label="Default dir" v-model="settings.default_dir" @update:modelValue="save"></v-text-field>
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

const theme = useTheme();
const { availableLocales } = useI18n();
const settingsStore = useSettingsStore();

const { settings } = storeToRefs(settingsStore);

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
