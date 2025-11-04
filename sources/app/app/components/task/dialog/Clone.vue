<template>
    <BaseDialogClone v-model="isDialogOpen" v-model:element="taskClone" :loading :type="$t('task.singular')" icon="mdi-checkbox-marked-circle-outline">
        <template #content>
            <VForm ref="form" v-model="isTaskValid">
                <VRowSingle>
                    <VCombobox
                        v-model="projectId"
                        :items="allProjects"
                        :label="$t('task.field.project')"
                        :returnObject="false"
                        :rules="[required($t('validation.rule.required', { field: $t('task.field.project') }))]"
                        class="required"
                        itemTitle="name"
                        itemValue="id"
                        autofocus
                    />
                </VRowSingle>
                <TaskFieldName v-model="taskClone.newName" :disabled="loading" autofocus />
                <VRowSingle>
                    <VDataTable
                        :headers="headers"
                        :items="taskClone.placeholderMappings"
                        :loading
                        style="max-height: 200px"
                        hideDefaultFooter
                        hideDefaultHeader
                    >
                        <template #[`item.from`]="{ item }">
                            <PlaceholderIcon :visibility="item.from.visibility" />
                            {{ item.from.name }}
                        </template>
                        <template #[`item.to`]="{ item }">
                            <VAutocomplete
                                v-model="item.to"
                                :items="taskClone.projectPlaceholders"
                                :returnObject="true"
                                density="compact"
                                itemTitle="name"
                                clearable
                            >
                                <template #prepend-inner>
                                    <PlaceholderIcon v-if="item.to?.visibility" :visibility="item.to.visibility" />
                                </template>
                                <template #item="{ props: autocompleteProps, item: autocompleteItem }">
                                    <VListItem v-bind="autocompleteProps">
                                        <template #prepend>
                                            <PlaceholderIcon :visibility="autocompleteItem.raw.visibility" />
                                        </template>
                                    </VListItem>
                                </template>
                            </VAutocomplete>
                        </template>
                    </VDataTable>
                </VRowSingle>
            </VForm>
        </template>
        <template #actions>
            <TaskActionClone @cloned="taskCloned" :disabled="!isTaskValid" :taskClone />
        </template>
    </BaseDialogClone>
</template>

<script setup lang="ts">
import type { DataTableHeader } from "vuetify";

const emit = defineEmits<{
    cloned: [task: TaskContract];
}>();

const props = defineProps<{
    taskId: string;
}>();

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject, allProjects } = storeToRefs(projectStore);

const loading = ref(false);
const projectId = ref(selectedProject.value.id);
const taskClone = ref({} as TaskCloneContract);
const isTaskValid = ref<boolean | null>(false);
const isDialogOpen = ref(false);

const headers: DataTableHeader[] = [
    {
        key: "from",
        width: "50%"
    },
    {
        key: "to",
        width: "50%"
    }
];

whenever(isDialogOpen, () => buildDefaultTaskClone());
watch(projectId, () => buildDefaultTaskClone());

const buildDefaultTaskClone = async () => {
    taskClone.value = {} as TaskCloneContract;

    loading.value = true;

    const buildTaskCloneResult = await commands.taskBuildEmptyTaskClone(props.taskId, projectId.value);

    loading.value = false;

    if (buildTaskCloneResult.status === "error") {
        notify.error(t("action.load.error", { type: t("task.singular") }), { error: buildTaskCloneResult.error });
        return;
    }

    taskClone.value = buildTaskCloneResult.data;
};

const taskCloned = (task: TaskContract) => {
    isDialogOpen.value = false;
    emit("cloned", task);
};
</script>
