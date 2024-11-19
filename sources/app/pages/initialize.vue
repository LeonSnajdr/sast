<template>
    <VContainer class="fill-height" maxWidth="800">
        <VRowSingle>
            <SettingPresentationLanguage v-model="setting.presentationLanguage" />
            <SettingPresentationTheme v-model="setting.presentationTheme" />
            <VRowSingle colClass="d-flex">
                <VSpacer />
                <VBtn @click="finish()" appendIcon="mdi-arrow-right" color="primary">{{ $t("initialize.finish") }}</VBtn>
            </VRowSingle>
        </VRowSingle>
    </VContainer>
</template>

<script setup lang="ts">
import type { InitializeSettingContract } from "~/utils/tauriBindings";

const { locale: uiLanguage } = useI18n();
const { name: uiTheme } = useTheme();

const setting = ref<InitializeSettingContract>({
    presentationLanguage: uiLanguage.value,
    presentationTheme: uiTheme.value
});

const finish = async () => {
    const keckw = await commands.initializeSetting(setting.value);

    if (keckw.status == "error") {
        return;
    }

    navigateTo({ name: "index" });
};

definePageMeta({
    middleware: "not-initialized"
});
</script>
