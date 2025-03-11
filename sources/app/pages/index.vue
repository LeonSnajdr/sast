<template>
    <div class="h-100 overflow-auto">
        <NuxtPage v-if="isInitialized" />
    </div>
</template>

<script setup lang="ts">
const route = useRoute();
const presentation = usePresentation();
const { current: pressedKeys } = useMagicKeys();

const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);

const isInitialized = ref(false);

onBeforeMount(async () => {
    initialize();
});

const initialize = async () => {
    const settingResult = await commands.settingGetDefault();

    if (settingResult.status == "error") {
        return navigateTo({ name: "error" });
    }

    if (settingResult.data == null) {
        return navigateTo({ name: "initialize" });
    }

    setting.value = settingResult.data!;

    presentation.applySetting();

    isInitialized.value = true;
};

whenever(
    () => pressedKeys.has("control") && pressedKeys.has("+"),
    () => {
        if (route.name === "initialize") return;

        return navigateTo({ name: "index-setting-index-presentation" });
    }
);
</script>
