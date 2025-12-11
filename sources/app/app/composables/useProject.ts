import type { RouteLocationRaw } from "vue-router";

export default function useProject() {
    const route = useRoute();

    const getSwitchProjectLocationRef = (project: ProjectContract) => computed(() => getSwitchProjectLocation(project));

    const getSwitchProjectLocation = (project: ProjectContract): RouteLocationRaw => {
        const isProjectRoute = route.matched.some((match) => match.name === "index-project-id");

        if (!isProjectRoute) return { name: "index-project-id", params: { id: project.id } };

        const match = lodFindLast(route.matched, (match) => {
            return match.meta.projectSwitchName != undefined;
        });

        const name = match ? match.meta.projectSwitchName : "index-project-id";

        return { name: name as "index", params: { id: project.id } };
    };

    const switchProject = async (project: ProjectContract) => {
        await navigateTo(getSwitchProjectLocation(project));
    };

    const switchProjectDebounced = lodDebounce(switchProject, 250);

    return { getSwitchProjectLocation, getSwitchProjectLocationRef, switchProject, switchProjectDebounced };
}
