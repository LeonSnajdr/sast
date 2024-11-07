<template>
    <VEmptyState :headline="$t('welcome.headline')" :title="$t('welcome.title')">
        <VContainer>
            <VRow>
                <VCol cols="12" sm="6">
                    <VCard :text="$t('project.openLast.description')" :title="$t('project.openLast.title')">
                        <template #prepend>
                            <VIcon color="info" icon="mdi-folder" />
                        </template>
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard :text="$t('project.select.description')" :title="$t('project.select.title')">
                        <template #prepend>
                            <VIcon color="warning" icon="mdi-folder-marker" />
                        </template>

                        <ProjectSelectDialog />
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard :text="$t('project.create.description')" :title="$t('project.create.title')">
                        <template #prepend>
                            <VIcon color="success" icon="mdi-folder-plus" />
                        </template>

                        <ProjectCreateDialog />
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard :text="$t('settings.description')" :title="$t('settings.title')">
                        <template #prepend>
                            <VIcon color="secondary" icon="mdi-cog" />
                        </template>
                    </VCard>
                </VCol>
            </VRow>
        </VContainer>
    </VEmptyState>
</template>
