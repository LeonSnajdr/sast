<template>
    <VChip @click="cycle">
        <template v-if="currentState.icon" #prepend>
            <VIcon :color="currentState.color" :icon="currentState.icon" class="mr-1 ml-n1" />
        </template>
        <slot />
    </VChip>
</template>

<script lang="ts">
export type ChipCycleState<T> = {
    value: T;
    icon?: string;
    color?: string;
};
</script>

<script setup lang="ts" generic="T">
const props = defineProps<{
    states: ChipCycleState<T>[];
}>();

const current = defineModel<T>({ required: true });

const currentIndex = computed(() => {
    return props.states.findIndex((state) => state.value === current.value);
});

const currentState = computed(() => {
    return props.states[currentIndex.value] ?? props.states[0]!;
});

const cycle = () => {
    const nextIndex = (currentIndex.value + 1) % props.states.length;

    current.value = props.states[nextIndex]!.value;
};
</script>
