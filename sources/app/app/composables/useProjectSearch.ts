import { useFuse } from "@vueuse/integrations/useFuse";

export default function useProjectSearch() {
    const projectStore = useProjectStore();

    const { allProjects } = storeToRefs(projectStore);

    const query = ref<string>("");

    const { results: projectResults } = useFuse(query, allProjects, {
        fuseOptions: {
            keys: ["name"],
            isCaseSensitive: false
        },
        matchAllWhenSearchEmpty: true
    });

    return { query, projectResults };
}
