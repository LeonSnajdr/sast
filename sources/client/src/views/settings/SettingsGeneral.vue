<template>
    <v-card>
        <v-card-title>{{ $t("settingsGeneral.title") }} </v-card-title>
        <v-card-text>
            <v-row-single>
                <v-text-field :label="$t('settingsGeneral.input.defaultDirectory')" v-model="settings.default_dir" @update:modelValue="save"></v-text-field>
            </v-row-single>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import * as commands from "@/bindings";
import type { UpdateSettingsContract } from "@/bindings";

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
