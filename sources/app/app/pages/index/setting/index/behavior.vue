<template>
    <VAppBar>
        <VAppBarTitle>
            {{ $t("setting.behavior") }}
        </VAppBarTitle>
    </VAppBar>
    <VContainer>
        <SettingBehaviorFieldOpenWelcome v-model="setting.behaviorOpenWelcome" />
    </VContainer>
</template>

<script setup lang="ts">
const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);
</script>
