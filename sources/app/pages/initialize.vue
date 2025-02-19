<template>
    <VContainer class="fill-height" maxWidth="800">
        <VRowSingle>
            <SettingPresentationFieldLanguage v-model="setting.presentationLanguage" />
            <SettingPresentationFieldTheme v-model="setting.presentationTheme" />
            <VRowSingle colClass="d-flex">
                <VSpacer />
                <VBtn @click="finish()" appendIcon="mdi-arrow-right" color="primary">{{ $t("initialize.finish") }}</VBtn>
            </VRowSingle>
        </VRowSingle>
    </VContainer>
</template>

<script setup lang="ts">
const { t } = useI18n();
const notify = useNotify();

const { locale: uiLanguage } = useI18n();
const { name: uiTheme } = useTheme();

const setting = ref<SettingInitializeContract>({
    presentationLanguage: uiLanguage.value,
    presentationTheme: uiTheme.value
});

const finish = async () => {
    const initializeResult = await commands.settingInitialize(setting.value);

    if (initializeResult.status == "error") {
        if (initializeResult.error === "AlreadyExists") {
            notify.error(t("initialize.error.alreadyExists"), { error: initializeResult.error });
        }
        return;
    }

    navigateTo({ name: "index" });
};
</script>
