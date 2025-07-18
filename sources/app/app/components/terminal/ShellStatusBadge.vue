<template>
    <VBadge :color="indicator.color" :modelValue="Boolean(shellStatus)" v-bind="$attrs" location="bottom right" dot>
        <slot />
        <VTooltip v-if="indicator.tooltip" :text="indicator.tooltip" activator="parent" location="left" />
    </VBadge>
</template>

<script setup lang="ts">
const props = defineProps<{
    shellStatus?: TerminalShellStatus;
}>();

const indicator = computed(() => {
    const status = props.shellStatus;

    if (typeof status === "object" && "Crashed" in status) {
        return { color: "error", tooltip: `${status.Crashed.code} ${status.Crashed.message}` };
    } else if (status == "Restarting") {
        return { color: "warning" };
    } else if (status == "Running") {
        return { color: "success" };
    }

    return { color: undefined };
});
</script>
