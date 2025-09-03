export default function useProject() {
    const route = useRoute();

    const switchProject = async (project: ProjectContract) => {
        const isProjectRoute = route.matched.some((match) => match.name === "index-project-id");

        if (!isProjectRoute) return;

        const match = lodFindLast(route.matched, (match) => {
            return match.meta.projectSwitchName != undefined;
        });

        const name = match ? match.meta.projectSwitchName : "index-project-id";

        await navigateTo({ name: name as "index", params: { id: project.id } });
    };

    const switchProjectDebounced = lodDebounce(switchProject, 250);

    return { switchProject, switchProjectDebounced };
}
