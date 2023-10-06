<template>
    <v-navigation-drawer :rail="rail" @update:rail="(value) => (hover = !value)" width="200" permanent left floating expandOnHover>
        <template #default>
            <div v-for="listProject in listProjects" :key="listProject.id" class="pa-2">
                <v-btn @click="switchProject(listProject)" :variant="project && listProject.id == project.id ? 'flat' : 'text'" color="secondary" block>
                    {{ listProject.name }}</v-btn
                >
            </div>
        </template>
        <template #append v-if="expanded">
            <div class="d-flex pa-2">
                <v-btn color="primary" width="80%" density="comfortable">
                    <v-icon icon="mdi-plus"></v-icon>
                    <ProjectCreateDialog />
                </v-btn>
                <v-spacer />
                <v-btn variant="text" @click.prevent="rail = !rail" density="compact" icon>
                    <v-icon size="small">{{ rail ? "mdi-pin-outline" : "mdi-pin-off-outline" }}</v-icon>
                </v-btn>
            </div>
        </template>
    </v-navigation-drawer>
</template>

<script setup lang="ts">
import ProjectCreateDialog from "./ProjectCreateDialog.vue";
import { useProjectListStore } from "@/stores/projectListStore";
import { useProjectStore } from "@/stores/projectStore";
import type { ListProjectContract } from "@/bindings";
import { useRouter } from "vue-router";

const pageStore = useProjectListStore();
const projectStore = useProjectStore();
const router = useRouter();

const { rail } = storeToRefs(pageStore);
const { listProjects, project } = storeToRefs(projectStore);

const hover = ref(false);

onBeforeMount(() => {
    projectStore.loadListProjects();
});

// TODO Remove or reuse
/*const addProject = async () => {
    const createContract: CreateProjectContract = {
        name: createProjectName.value
    };

    try {
        await createProject(createContract);
        await loadListProjects();
        notify.error("TODO");
    } catch (error) {
        console.error("Project creation failed", error);
        notify.error("TODO");
    }
};*/

const switchProject = async (listProject: ListProjectContract) => {
    await router.push({ name: "project", params: { projectId: listProject.id } });
};

const expanded = computed(() => {
    return !rail.value || hover.value;
});
</script>
