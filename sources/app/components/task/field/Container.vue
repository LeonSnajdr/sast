<template>
    <VForm ref="form" v-model="isValid">
        <TaskFieldName v-model="task.name" autofocus />
        <TaskFieldTabName v-model="task.tabName" />
        <VRow>
            <VCol>
                <TaskFieldNoExit v-model="task.noExit" />
            </VCol>
            <VCol>
                <TaskFieldForceKill v-model="task.forceKill" />
            </VCol>
        </VRow>
        <TaskFieldHistoryPersistence v-model="task.historyPersistence" />
        <TaskFieldCommandTiles v-model="task.commandTiles" />
        <TaskFieldWorkingDirTiles v-model="task.workingDirTiles" />
    </VForm>
</template>

<script setup lang="ts">
const task = defineModel<TaskCreateContract | TaskContract>({ required: true });
const isValid = defineModel<boolean | null>("isValid", { required: true });
</script>
