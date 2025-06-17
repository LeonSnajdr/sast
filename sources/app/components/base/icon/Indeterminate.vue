<template>
    <DefineIcon v-slot="{ inSpinner }">
        <VIcon :color :icon :size="inSpinner ? size - 8 : size" />
    </DefineIcon>

    <div v-bind="$attrs">
        <VProgressCircular v-if="indeterminate" :color :indeterminate :size width="2">
            <Icon :inSpinner="true" />
        </VProgressCircular>

        <Icon v-else />
    </div>
</template>

<script setup lang="ts">
withDefaults(
    defineProps<{
        icon: string;
        color?: string;
        size?: number;
        indeterminate?: boolean;
    }>(),
    {
        color: undefined,
        size: 20,
        indeterminate: false
    }
);

const [DefineIcon, Icon] = createReusableTemplate<{ inSpinner?: boolean }>();
</script>
