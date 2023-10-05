<template>
    <VNavigationDrawer permanent left floating expandOnHover>
        <template #prepend>
            <h2>Hallo</h2>
        </template>
    </VNavigationDrawer>
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
import { onBeforeMount, ref } from "vue";
import { useRouter } from "vue-router";
import { VNavigationDrawer } from "vuetify/components";

const notify = useNotificationStore();
const router = useRouter();

const listProjects = ref<ListProjectContract[]>([]);
const addProjectName = ref("");

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
        name: addProjectName.value
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
</script>
