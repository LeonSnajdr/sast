<template>
    <VBtnToggle v-model="options" @click.stop.prevent density="compact" style="height: 28px" multiple>
        <VBtn :value="TaskSetTaskOptions.Blocking" v-tooltip="$t('taskSetTask.field.blocking.info')">
            <VIcon icon="mdi-timer-sand" />
        </VBtn>
        <VBtn :value="TaskSetTaskOptions.JumpInto">
            <VIcon icon="mdi-debug-step-into" />
        </VBtn>
    </VBtnToggle>
</template>

<script setup lang="ts">
const taskSetTask = defineModel<TaskSetTaskInfoContract>({ required: true });

enum TaskSetTaskOptions {
    Blocking = "Blocking",
    JumpInto = "JumpInto"
}

const options = computed({
    get() {
        const values: string[] = [];
        if (taskSetTask.value.blocking) values.push(TaskSetTaskOptions.Blocking);
        if (taskSetTask.value.jumpInto) values.push(TaskSetTaskOptions.JumpInto);

        return values;
    },
    set(newValues) {
        taskSetTask.value.blocking = newValues.includes(TaskSetTaskOptions.Blocking);
        taskSetTask.value.jumpInto = newValues.includes(TaskSetTaskOptions.JumpInto);
    }
});
</script>
