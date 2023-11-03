<template>
    <v-card>
        <v-card-title>Settings</v-card-title>
        <v-card-text>
            <v-row-single>
                <v-select label="Sprache" :items="availableLocales"> </v-select>
            </v-row-single>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";

const { availableLocales } = useI18n();
</script>
