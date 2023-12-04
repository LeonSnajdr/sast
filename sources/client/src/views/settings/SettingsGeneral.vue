<template>
    <v-card>
        <v-card-title>Settings</v-card-title>
        <v-card-text>
            <v-row-single>
                <v-autocomplete label="Sprache" :items="availableLocales" @update:modelValue="setLanguage"></v-autocomplete>
            </v-row-single>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";

const { availableLocales, locale } = useI18n();

const setLanguage = () => {
    locale.value = "en";
};
</script>
