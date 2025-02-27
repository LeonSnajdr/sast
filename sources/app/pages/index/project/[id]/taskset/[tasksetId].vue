<template>
    <VNavigationDrawer v-model="isTaskCreateDrawerOpen" location="right" width="400" disableResizeWatcher>
        <VList>
            <VListItem height="48">
                Task - Create
                <template #append>
                    <IconBtn icon="mdi-content-save">Save</IconBtn>
                </template>
            </VListItem>

            <VDivider />

            <VContainer>
                <VRowSingle v-for="i in 5" :key="i">
                    <VTextField />
                </VRowSingle>
            </VContainer>
        </VList>
    </VNavigationDrawer>

    <VNavigationDrawer v-model="isTaskEditDrawerOpen" location="right" width="400" disableResizeWatcher>
        <VList>
            <VListItem height="48">
                Task - Edit{{ selectedTaskId }}
                <template #append>
                    <IconBtn icon="mdi-content-save">Save</IconBtn>
                </template>
            </VListItem>

            <VDivider />

            <VContainer>
                <VRowSingle v-for="i in 5" :key="i">
                    <VTextField />
                </VRowSingle>
            </VContainer>
        </VList>
    </VNavigationDrawer>

    <VAppBar>
        <VAppBarTitle>{{ $t("taskSet.plural") }} - {{ isTaskCreateDrawerOpen }} - {{ isTaskEditDrawerOpen }}</VAppBarTitle>
        <TaskSetActionDelete v-if="taskSet" :disabled="!isFormValid" :taskSet class="mr-2" />
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
                <VAutocomplete :persistentPlaceholder="false" label="Task suche" />
                <IconBtn icon="mdi-plus">Add</IconBtn>
                <VDivider vertical />
                <IconBtn @click.stop.prevent="toggleTaskCreateDrawer()" :active="isTaskCreateDrawerOpen" icon="mdi-dock-right">Create</IconBtn>
            </VCardText>
        </VCard>

        <div class="flex-grow-1 overflow-hidden">
            <Draggable v-model="taskSetTasks" class="h-100 overflow-auto" handle=".drag-handle">
                <VListItem v-for="taskSetTask in taskSetTasks" :key="taskSetTask.name">
                    <template #prepend>
                        <VIcon class="drag-handle" icon="mdi-drag" />
                    </template>
                    <div class="d-flex ga-4 align-center">
                        <div>
                            <p>Hallo</p>
                            <p class="text-caption text-medium-emphasis">erstellt vor 12Stunden</p>
                        </div>
                        <VSpacer />
                        <ChipSwitch v-model="taskSetTask.blocking" @click.stop.prevent>Blocking</ChipSwitch>
                        <div>
                            <IconBtn
                                @click.stop.prevent="toggleTaskEditDrawer(taskSetTask.id)"
                                :active="taskSetTask.id === selectedTaskId"
                                icon="mdi-dock-right"
                            />
                            <IconBtn @click.stop.prevent icon="mdi-close" />
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

const selectedTaskId = ref();

const isTaskEditDrawerOpen = computed({
    get() {
        return selectedTaskId.value != undefined;
    },
    set() {
        selectedTaskId.value = undefined;
    }
});

const isTaskCreateDrawerOpen = ref(false);

const toggleTaskCreateDrawer = () => {
    isTaskEditDrawerOpen.value = false;
    isTaskCreateDrawerOpen.value = !isTaskCreateDrawerOpen.value;
};

const toggleTaskEditDrawer = (taskId: string) => {
    if (selectedTaskId.value === taskId) {
        selectedTaskId.value = undefined;
        return;
    }

    isTaskCreateDrawerOpen.value = false;
    selectedTaskId.value = taskId;
};

const taskSetTasks = ref([
    { id: "taskId1", name: "test", blocking: false },
    { id: "taskId2", name: "test2", blocking: true }
]);

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
</script>

<style lang="scss" scoped>
:deep(.sortable-ghost) {
    opacity: 0.1;
}
</style>
