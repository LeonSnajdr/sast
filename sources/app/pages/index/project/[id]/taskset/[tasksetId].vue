<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("taskset.singular") }}</VAppBarTitle>
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
                                <VTextField label="Tabname">
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
                                <VSwitch label="Geöffnet lassen" />
                            </VCol>
                        </VRow>
                    </VListItem>
                </VList>
            </VRowSingle>
        </VContainer>
    </div>
</template>

<script setup lang="ts"></script>
