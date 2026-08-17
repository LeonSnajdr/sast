<template>
    <VChip @click="cycle">
        <template #prepend>
            <VExpandXTransition>
                <span v-if="currentState.icon" class="d-inline-flex overflow-hidden mr-1 ml-n1">
                    <VIcon :color="currentState.color" :icon="currentState.icon" />
                </span>
            </VExpandXTransition>
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
