<template>
    <TaskDrawerCreate v-model="isTaskCreateDrawerOpen" @created="addTask" />
    <TaskDrawerEdit v-model="isTaskEditDrawerOpen" @saved="savedTask" :taskId="editTaskId" />

    <VAppBar>
        <VAppBarTitle>{{ $t("title.edit", { type: $t("taskSet.singular") }) }}</VAppBarTitle>
        <TaskSetActionDelete v-if="taskSet" :taskSet class="mr-2" />
        <TaskSetActionSave v-if="taskSet" :disabled="!isFormValid" :taskSet />
    </VAppBar>

    <VContainer v-if="taskSet" class="h-100 d-flex flex-column ga-4">
        <VCard :loading="isLoading" class="overflow-visible">
            <VCardText>
                <VForm v-model="isFormValid">
                    <TaskSetFieldName v-model="taskSet.name" />
                </VForm>
            </VCardText>
        </VCard>

        <VCard class="overflow-visible">
            <VCardText class="d-flex ga-2 align-center">
                <TaskSetTaskSearch @add="addTask" :taskSetTasks="taskSet.tasks" />
                <VDivider vertical />
                <BaseBtnIcon @click.stop.prevent="toggleTaskCreateDrawer()" :active="isTaskCreateDrawerOpen" icon="mdi-dock-right">
                    {{ $t("action.create") }} & {{ $t("action.add") }}
                </BaseBtnIcon>
            </VCardText>
        </VCard>

        <div class="flex-grow-1 overflow-hidden">
            <Draggable v-model="taskSet.tasks" class="h-100 overflow-auto" handle=".drag-handle">
                <VListItem v-for="taskSetTask in taskSet.tasks" :key="taskSetTask.taskId">
                    <template #prepend>
                        <VIcon class="drag-handle" icon="mdi-drag" />
                    </template>
                    <div class="d-flex ga-4 align-center">
                        <div>
                            <p>{{ taskSetTask.taskName }}</p>
                            <p class="text-caption text-medium-emphasis">
                                {{ $t("task.table.column.dateLastUpdated") }} {{ useLocaleTimeAgo(taskSetTask.taskDateLastUpdated) }}
                            </p>
                        </div>
                        <VSpacer />
                        <div>
                            <BaseChipSwitch v-model="taskSetTask.blocking" @click.stop.prevent :infoText="$t('taskSetTask.field.blocking.info')">
                                {{ $t("taskSetTask.field.blocking") }}
                            </BaseChipSwitch>
                        </div>
                        <div>
                            <BaseBtnIcon
                                @click.stop.prevent="toggleTaskEditDrawer(taskSetTask.taskId)"
                                :active="taskSetTask.taskId === editTaskId"
                                icon="mdi-dock-right"
                            />
                            <BaseBtnIcon @click.stop.prevent="removeTask(taskSetTask.taskId)" icon="mdi-close" />
                        </div>
                    </div>
                </VListItem>
            </Draggable>
        </div>
    </VContainer>
</template>

<script setup lang="ts">
import { VueDraggable as Draggable } from "vue-draggable-plus";

const route = useRoute("index-project-id-taskSet-taskSetId");

const notify = useNotify();
const { t } = useI18n();

const isLoading = ref(false);
const isFormValid = ref(false);
const taskSet = ref<TaskSetContract>();
const editTaskId = ref<string>();

const isTaskEditDrawerOpen = computed({
    get() {
        return editTaskId.value != undefined;
    },
    set() {
        editTaskId.value = undefined;
    }
});

const isTaskCreateDrawerOpen = ref(false);

const toggleTaskCreateDrawer = () => {
    isTaskEditDrawerOpen.value = false;
    isTaskCreateDrawerOpen.value = !isTaskCreateDrawerOpen.value;
};

const toggleTaskEditDrawer = (taskId: string) => {
    if (editTaskId.value === taskId) {
        editTaskId.value = undefined;
        return;
    }

    isTaskCreateDrawerOpen.value = false;
    editTaskId.value = taskId;
};

onBeforeMount(() => {
    loadTaskSet();
});

const loadTaskSet = async () => {
    isLoading.value = true;

    const taskSetResult = await commands.taskSetGetOne(route.params.taskSetId);

    isLoading.value = false;

    if (taskSetResult.status === "error") {
        notify.error(t("action.load.error", { type: t("taskSet.singular") }), { error: taskSetResult.error });
        return;
    }

    taskSet.value = taskSetResult.data;
};

const savedTask = (task: { id: string; name: string; dateLastUpdated: string }) => {
    if (!taskSet.value) return;

    const taskSetTask = taskSet.value.tasks.find((x) => x.taskId === task.id);

    if (!taskSetTask) return;

    taskSetTask.taskName = task.name;
    taskSetTask.taskDateLastUpdated = task.dateLastUpdated;
};

const addTask = (task: { id: string; name: string; dateCreated: string; dateLastUpdated: string }) => {
    if (!taskSet.value) return;

    taskSet.value.tasks.push({
        taskId: task.id,
        taskName: task.name,
        taskDateCreated: task.dateCreated,
        taskDateLastUpdated: task.dateLastUpdated,
        blocking: false
    });
};

const removeTask = (taskId: string) => {
    if (!taskSet.value) return;

    taskSet.value.tasks = taskSet.value.tasks.filter((x) => x.taskId !== taskId);
};
</script>

<style lang="scss" scoped>
:deep(.sortable-ghost) {
    opacity: 0.1;
}
</style>
