<template>
    <Teleport to="#window-bar-content">
        <div class="d-flex align-center ga-2">
            <VIconBtn size="x-small">
                <VIcon icon="mdi-chevron-down" size="small" />
                <ProjectMenuSelect />
            </VIconBtn>
            <VTabs color="text" centerActive showArrows>
                <VTab
                    v-for="project in allProjects"
                    :key="project.id"
                    :to="getSwitchProjectLocationRef(project).value"
                    class="border-b border-opacity-0"
                    density="compact"
                    height="34"
                    selectedClass="border-text border-opacity-100"
                >
                    <template v-if="project.favorite" #prepend>
                        <VIcon class="opacity-100" color="warning" icon="mdi-star" size="small" style="margin-top: 2px; margin-right: -2px" />
                    </template>
                    <span class="font-weight-medium text-truncate mx-n1" style="max-width: 150px">{{ project.name }}</span>
                    <template #append>
                        <TerminalCountChip :project />
                    </template>
                </VTab>
            </VTabs>
        </div>
    </Teleport>
</template>
<script setup lang="ts">
const { getSwitchProjectLocationRef } = useProject();

const projectStore = useProjectStore();

const { allProjects } = storeToRefs(projectStore);
</script>

<style lang="scss" scoped>
@use "@/assets/styles/settings";

:deep(.v-slide-group__prev) {
    margin-left: -(settings.$spacer * 5);
    margin-right: -(settings.$spacer * 2);
}

:deep(.v-slide-group__next) {
    margin-right: -(settings.$spacer * 3);
    margin-left: -(settings.$spacer * 2);
}
</style>
