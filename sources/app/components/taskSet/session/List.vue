<template>
    <DefineSessionTask v-slot="{ $slots, sessionTask, icon, color, indeterminate }">
        <VListItem>
            <template #prepend>
                <BaseIconIndeterminate :color :icon :indeterminate class="mr-2" />
            </template>
            <VListItemTitle class="text-body-2">{{ sessionTask.taskName }}</VListItemTitle>
            <VListItemSubtitle class="text-caption">
                <component :is="$slots.default" />
            </VListItemSubtitle>
            <template #append>
                <TaskActionTerminalLink :disabled="!isLatestSessionSelected" :taskId="sessionTask.taskId" :taskSetId="taskSetId" />
            </template>
        </VListItem>
    </DefineSessionTask>

    <VList v-if="selectedSession">
        <VListSubheader v-if="!alwaysShowLatestRun">
            <VSelect
                v-model="selectedSessionId"
                @click:clear="selectedSessionId = LATEST"
                :clearable="!isLatestSessionSelected"
                :items="items"
                class="mr-2"
                clearIcon="mdi-history"
                density="compact"
                itemTitle="name"
                itemValue="id"
                variant="underlined"
                persistentClear
            />
        </VListSubheader>
        <template v-for="sessionTask in selectedSession.tasks" :key="sessionTask.taskId">
            <SessionTask v-if="sessionTask.status === 'NotStarted'" :sessionTask color="secondary" icon="mdi-checkbox-blank-circle-outline">
                {{ $t("taskSetSession.task.status.notStarted") }}
            </SessionTask>
            <SessionTask v-if="sessionTask.status === 'Running'" :sessionTask color="warning" icon="mdi-circle" indeterminate>
                {{ $t("taskSetSession.task.status.running", { seconds: dateFormatter.getDiff(now, sessionTask.dateStarted!, "seconds") }) }}
            </SessionTask>
            <SessionTask v-if="sessionTask.status === 'Completed'" :sessionTask color="success" icon="mdi-check-circle">
                {{
                    $t("taskSetSession.task.status.completed", {
                        seconds: dateFormatter.getDiff(sessionTask.dateFinished!, sessionTask.dateStarted!, "seconds")
                    })
                }}
            </SessionTask>
            <SessionTask v-if="sessionTask.status === 'Failed'" :sessionTask color="error" icon="mdi-close-circle">
                {{
                    $t("taskSetSession.task.status.failed", {
                        seconds: dateFormatter.getDiff(sessionTask.dateFinished!, sessionTask.dateStarted!, "seconds")
                    })
                }}
            </SessionTask>
            <SessionTask v-if="sessionTask.status === 'Skipped'" :sessionTask color="secondary" icon="mdi-arrow-right-thin-circle-outline">
                {{ $t("taskSetSession.task.status.skipped") }}
            </SessionTask>
        </template>
    </VList>
    <VEmptyState v-else height="50">
        {{ $t("search.noResults") }}
    </VEmptyState>
</template>

<script setup lang="ts">
const props = defineProps<{
    taskSetId: string;
    alwaysShowLatestRun?: boolean;
}>();

const [DefineSessionTask, SessionTask] = createReusableTemplate<{
    sessionTask: TaskSetSessionTaskContract;
    icon: string;
    color: string;
    indeterminate?: boolean;
}>();

const LATEST = "Latest";

const taskSetSessionStore = useTaskSetSessionStore();

const { sessions } = storeToRefs(taskSetSessionStore);

const { t } = useI18n();
const dateFormatter = useDate();
const now = useNow();

const selectedSessionId = ref<string>(LATEST);

const filteredSessions = computed(() => {
    return lodFilter(sessions.value, (session) => session.taskSetId === props.taskSetId);
});

const latestSession = computed(() => {
    return lodMaxBy(filteredSessions.value, (session) => new Date(session.dateStarted));
});

const items = computed(() => {
    const items = [
        {
            name: t("taskSetSession.run.latest"),
            id: LATEST
        }
    ];

    const orderedSessions = lodOrderBy(filteredSessions.value, (x) => new Date(x.dateStarted), "desc").filter((x) => x.id !== latestSession.value?.id);

    const otherItems = orderedSessions.map((session, index) => {
        return {
            name: t("taskSetSession.run", { run: orderedSessions.length - index }),
            id: session.id
        };
    });

    return items.concat(otherItems);
});

const isLatestSessionSelected = computed(() => {
    return selectedSessionId.value === LATEST;
});

const selectedSession = computed(() => {
    if (isLatestSessionSelected.value) {
        return latestSession.value;
    }

    return filteredSessions.value.find((x) => x.id === selectedSessionId.value);
});
</script>

<style scoped>
:deep(.v-list-subheader__text) {
    display: flex;
    width: 100%;
}
</style>
