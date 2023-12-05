<template>
    <v-card>
        <v-card-title>Settings</v-card-title>
        <v-card-text>
            <v-row-single>
                <v-autocomplete label="Sprache" :items="availableLocales" @update:modelValue="setLanguage"></v-autocomplete>
            </v-row-single>
            <v-row>
                <v-col v-for="themeName in keys(theme.themes.value)" @click="changeTheme(themeName)" :key="themeName">
                    <v-card> {{ themeName }} </v-card>
                </v-col>
            </v-row>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import { keys } from "lodash";
import { useI18n } from "vue-i18n";
import { useTheme } from "vuetify";

const theme = useTheme();
const { availableLocales, locale } = useI18n();

const setLanguage = () => {
    locale.value = "en";
};

const changeTheme = (value: string) => {
    theme.global.name.value = value;
};
</script>
