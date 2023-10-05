<template>
    <v-navigation-drawer :rail="rail" @update:rail="(value) => (hover = !value)" width="200" permanent left floating expandOnHover>
        <template #default> </template>
        <template #append v-if="expanded">
            <div class="d-flex pa-2">
                <v-btn variant="flat" color="primary" width="80%" density="comfortable">
                    <v-icon icon="mdi-plus"></v-icon>
                </v-btn>
                <v-spacer />
                <v-btn variant="text" @click.prevent="rail = !rail" density="compact" icon>
                    <v-icon size="small">{{ rail ? "mdi-pin-outline" : "mdi-pin-off-outline" }}</v-icon>
                </v-btn>
            </div>
        </template>
    </v-navigation-drawer>
    <!--<Card>
        <template #title>
            <h1>Sast</h1>
        </template>
        <template #content>
            <div v-for="listProject in listProjects" :key="listProject.id">
                <Btn :label="listProject.name" @click="switchProject(listProject)" class="w-full mb-1" />
            </div>
        </template>
        <template #footer>
            <div class="flex">
                <InputText v-model="addProjectName" class="w-11" />
                <Btn @click="addProject">Moin</Btn>
            </div>
        </template>
    </Card>-->
</template>

<script setup lang="ts">
import type { CreateProjectContract, ListProjectContract } from "@/bindings";
import { createProject, getListProjects } from "@/bindings";
import { useNotificationStore } from "@/stores/notificationStore";
import { useProjectListStore } from "@/stores/projectListStore";
import { storeToRefs } from "pinia";
import { computed, onBeforeMount, ref } from "vue";
import { useRouter } from "vue-router";
import { VNavigationDrawer } from "vuetify/components";

const notify = useNotificationStore();
const pageStore = useProjectListStore();
const router = useRouter();

const { rail } = storeToRefs(pageStore);

const hover = ref(false);
const listProjects = ref<ListProjectContract[]>([]);
const createProjectName = ref("");

onBeforeMount(() => {
    loadListProjects();
});

const loadListProjects = async () => {
    try {
        listProjects.value = await getListProjects();
    } catch (error) {
        console.error("Loading list projects failed", error);
        notify.error("TODO");
    }
};

const addProject = async () => {
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
};

const switchProject = (listProject: ListProjectContract) => {
    router.push({ name: "project", params: { projectId: listProject.id } });
};

const expanded = computed(() => {
    return !rail.value || hover.value;
});
</script>
