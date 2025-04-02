<template>
    <VAppBar>
        <VAppBarTitle>
            {{ t("setting.behavior") }}
        </VAppBarTitle>
    </VAppBar>
    <SettingBehaviorFieldOpenWelcome v-model="setting.behaviorOpenWelcome" />
</template>

<script setup lang="ts">
const { t } = useI18n();
const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);
</script>
