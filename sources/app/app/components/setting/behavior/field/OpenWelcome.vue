<template>
    <VRowSingle>
        <VCard>
            <VCardTitle>
                <VIcon color="success" icon="mdi-application-import" />
                {{ $t("setting.behavior.openWelcome.title") }}
            </VCardTitle>
            <VCardText>
                <VBtnToggle v-model="openWelcome" @click.stop.prevent density="compact">
                    <VBtn v-for="option of options" :key="option.value + option.translation" :value="option.value">
                        {{ option.translation }}
                    </VBtn>
                </VBtnToggle>
            </VCardText>
        </VCard>
    </VRowSingle>
</template>

<script setup lang="ts">
const openWelcome = defineModel<boolean>({ required: true });

const { t } = useI18n();

const options = computed(() => [
    {
        value: true,
        translation: t("setting.behavior.openWelcome.true")
    },
    {
        value: false,
        translation: t("setting.behavior.openWelcome.false")
    }
]);
</script>
