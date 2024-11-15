<template>
    <slot v-if="isSettingInitalized" />

    <VStepper class="h-100 d-flex flex-column">
        <template #default="{ prev, next }">
            <VStepperHeader class="elevation-0">
                <VStepperItem :value="StepType.Language" title="Sprache" />
                <VDivider />
                <VStepperItem :value="StepType.Theme" title="Theme" />
                <VDivider />
                <VStepperItem :value="StepType.Verfiy" title="Validieren" />
            </VStepperHeader>
            <VStepperWindow class="flex-grow-1">
                <VStepperWindowItem :value="StepType.Language" class="mt-2">
                    <VAutocomplete v-model="initSetting.presentationLanguage" :items="['de', 'en']" label="Language" />
                </VStepperWindowItem>
                <VStepperWindowItem :value="StepType.Theme"> Hallo step2 </VStepperWindowItem>
                <VStepperWindowItem :value="StepType.Verfiy"> Hallo step3 </VStepperWindowItem>
            </VStepperWindow>
            <VStepperActions @click:next="next" @click:prev="prev" />
        </template>
    </VStepper>
</template>

<script setup lang="ts">
import type { InitSettingContract } from "~/utils/tauriBindings";

const isLoading = ref(true);
const isSettingInitalized = ref(false);

const initSetting = ref<InitSettingContract>({} as InitSettingContract);

enum StepType {
    Language = 1,
    Theme = 2,
    Verfiy = 3
}

onBeforeMount(async () => {
    isLoading.value = true;
    await loadIsSettingInitalized();
    isLoading.value = false;
});

const loadIsSettingInitalized = async () => {
    const isSettingInitalizedResult = await commands.getIsSettingInitialized();

    if (isSettingInitalizedResult.status == "error") {
        return;
    }

    isSettingInitalized.value = true;
};
</script>
