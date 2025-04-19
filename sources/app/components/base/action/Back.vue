<template>
    <BaseBtnIcon @click="back()" icon="mdi-arrow-left" variant="flat" v-tooltip="$t('keybind.controlB.tooltip')">
        {{ $t("action.back") }}
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const router = useRouter();

useKeybind(["control", "b"], () => back());

const back = () => {
    router.back();
};
</script>
