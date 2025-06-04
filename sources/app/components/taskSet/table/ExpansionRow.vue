<template>
    <DefineSessionTask v-slot="{ sessionTask, icon, color, indeterminate }">
        <VListItem>
            <template #prepend>
                <BaseIconIndeterminate :color :icon :indeterminate class="mr-2" />
            </template>
            <VListItemTitle class="text-body-2">{{ sessionTask.taskName }}</VListItemTitle>
            <VListItemSubtitle class="text-caption"> Started 20 minutes ago </VListItemSubtitle>
        </VListItem>
    </DefineSessionTask>

    <tr v-if="taskSetSession">
        <td :colspan="columnLength">
            <VList class="w-100">
                <template v-for="sessionTask in taskSetSession.tasks" :key="sessionTask.taskId">
                    <SessionTask v-if="sessionTask.status === 'Running'" :sessionTask color="warning" icon="mdi-circle" indeterminate />
                    <SessionTask v-if="sessionTask.status === 'Completed'" :sessionTask color="success" icon="mdi-check-circle" />
                    <SessionTask v-if="sessionTask.status === 'Failed'" :sessionTask color="error" icon="mdi-close-circle" />
                    <SessionTask v-if="sessionTask.status === 'NotStarted'" :sessionTask color="secondary" icon="mdi-checkbox-blank-circle-outline" />
                    <SessionTask v-if="sessionTask.status === 'Skipped'" :sessionTask color="secondary" icon="mdi-arrow-right-thin-circle-outline" />
                </template>
            </VList>
        </td>
    </tr>
</template>

<script setup lang="ts">
const props = defineProps<{
    columnLength: number;
    taskSet: TaskSetInfoContract;
}>();

const taskSetSessionStore = useTaskSetSessionStore();

const { sessions } = storeToRefs(taskSetSessionStore);

const [DefineSessionTask, SessionTask] = createReusableTemplate<{
    sessionTask: TaskSetSessionTaskContract;
    icon: string;
    color: string;
    indeterminate?: boolean;
}>();

const taskSetSession = computed(() => {
    const filteredSessions = lodFilter(sessions.value, (session) => session.taskSetId === props.taskSet.id);
    const newestSession = lodMaxBy(filteredSessions, (session) => new Date(session.dateStarted));

    return newestSession;
});
</script>
