<template>
    <div @click.prevent.stop class="d-flex">
        <TaskActionStart :task />
        <TaskActionRestart :task />
        <TaskActionStop :task />
        <TaskActionTerminalLink :task />
    </div>
</template>

<script setup lang="ts">
defineProps<{
    task: TaskInfoContract;
}>();
</script>
