<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("taskset.title") }}</VAppBarTitle>
    </VAppBar>
    <div class="fill-height d-flex flex-column">
        <VContainer>
            <VRow>
                <VCol>
                    <VCard>
                        <VCardTitle>
                            <VIcon icon="mdi-console" />
                            Terminal
                        </VCardTitle>
                        <VCardText>
                            <VCombobox :items="['SastTerm', 'WindowsTerminal', 'Powershell']" />
                        </VCardText>
                    </VCard>
                </VCol>
                <VCol>
                    <VCard>
                        <VCardTitle>
                            <VIcon color="warning" icon="mdi-pencil" />
                            Name des Tasksets
                        </VCardTitle>
                        <VCardText>
                            <VTextField modelValue="ControlCenter App" />
                        </VCardText>
                    </VCard>
                </VCol>
            </VRow>

            <VRowSingle>
                <VList rounded>
                    <VListItem prependIcon="mdi-drag">
                        <VRow class="pt-2">
                            <VCol>
                                <VTextField v-model="createTest.name" label="Tabname">
                                    <template #prepend-inner>
                                        <VIcon color="warning" icon="mdi-tab" />
                                    </template>
                                </VTextField>
                            </VCol>
                            <VCol>
                                <VTextField label="Startverzeichniss">
                                    <template #prepend-inner>
                                        <VIcon color="tertiary" icon="mdi-folder-outline" />
                                    </template>
                                </VTextField>
                            </VCol>
                            <VCol>
                                <VSwitch v-model="createTest.blocking" label="Geöffnet lassen" />
                            </VCol>
                        </VRow>
                        <VRowSingle>
                            <VField class="pa-2 mb-2" variant="outlined" active>
                                <VueTextInsertEditor v-model="createTest.commandTiles" :editorOptions="options" />
                            </VField>
                        </VRowSingle>
                        <VRowSingle>
                            <VBtn @click="createTask()">Create</VBtn>
                        </VRowSingle>
                    </VListItem>
                </VList>
            </VRowSingle>
        </VContainer>
    </div>
</template>

<script setup lang="ts">
import { PlaceholderInsertChip, PlaceholderInsertMenu } from "#components";
import type { EditorOptions } from "vue-text-insert";
import { VueTextInsertEditor } from "vue-text-insert";

const route = useRoute("index-project-id-taskset-tasksetId");

const createTest = ref<TaskCreateContract>({
    name: "Test",
    projectId: route.params.id,
    blocking: false,
    workingDirTiles: [] as PlaceholderInsertTileContract[],
    commandTiles: [] as PlaceholderInsertTileContract[]
} as TaskCreateContract);

const createTask = async () => {
    console.log("try to create", createTest.value);

    const createResult = await commands.taskCreate(createTest.value);

    if (createResult.status === "error") {
        console.error(createResult);
        return;
    }

    console.log("created", createResult.data);
};

const options: EditorOptions<PlaceholderInsertTileContract> = {
    textType: "Text",
    typeField: "kind",
    valueField: "textValue",
    insertOptions: {
        Placeholder: {
            trigger: "@",
            insertComponent: PlaceholderInsertChip,
            menuComponent: PlaceholderInsertMenu
        }
    }
};
</script>
