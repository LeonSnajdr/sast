<template>
    <Teleport to="#window-bar-content">
        <div class="d-flex align-center ga-2">
            <VIconBtn size="x-small">
                <VIcon icon="mdi-chevron-down" size="small" />
                <ProjectMenuSelect />
            </VIconBtn>
            <VSlideGroup showArrows>
                <VSlideGroupItem v-for="project in allProjects" :key="project.id">
                    <VBtn :to="getSwitchProjectLocationRef(project).value" density="compact">
                        <template v-if="project.favorite" #prepend>
                            <VIcon class="mr-n1 opacity-1" color="warning" icon="mdi-star" />
                        </template>
                        <span class="text-truncate" style="max-width: 150px">{{ project.name }}</span>
                    </VBtn>
                </VSlideGroupItem>
            </VSlideGroup>
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
