<template>
    <div @click.prevent.stop class="d-flex">
        <TaskActionStart :task />
        <TaskActionRestart :task />
        <TaskActionStop :task />
        <TaskActionTerminalLink :taskId="task.id" />
    </div>
</template>

<script setup lang="ts">
defineProps<{
    task: TaskInfoContract;
}>();
</script>
