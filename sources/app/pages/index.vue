<template>
    <VEmptyState headline="Willkommen," title="was möchtest du heute erledigen?">
        <VContainer>
            <VRow>
                <VCol cols="12" sm="6">
                    <VCard text="Beginne dort, wo du aufgehört hast" title="Letztes Projekt öffnen">
                        <template #prepend>
                            <VIcon color="info" icon="mdi-folder" />
                        </template>
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard text="Wähle eines deiner Projekte aus" title="Projekt auswählen">
                        <template #prepend>
                            <VIcon color="warning" icon="mdi-folder-marker" />
                        </template>
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard text="Erstelle ein neues Projekt" title="Projekt erstellen">
                        <template #prepend>
                            <VIcon color="success" icon="mdi-folder-plus" />
                        </template>
                    </VCard>
                </VCol>

                <VCol cols="12" sm="6">
                    <VCard text="Konfigurierte alles nach deinen Vorstellungen" title="Einstellungen öffnen">
                        <template #prepend>
                            <VIcon color="secondary" icon="mdi-cog" />
                        </template>
                    </VCard>
                </VCol>
            </VRow>
        </VContainer>
    </VEmptyState>
</template>
