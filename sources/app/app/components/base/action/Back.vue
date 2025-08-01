<template>
    <VBtn @click="back()" prependIcon="mdi-arrow-left" variant="flat" v-tooltip="$t('keybind.controlB.tooltip')">
        {{ $t("action.back") }}
    </VBtn>
</template>

<script setup lang="ts">
const router = useRouter();

useKeybind(["control", "b"], () => back());

const back = () => {
    router.back();
};
</script>
