<template>
    <div @click.prevent.stop class="d-flex">
        <IconBtn @click="start()" :loading="isStarting" color="success" icon="mdi-play" />
        <IconBtn color="info" icon="mdi-autorenew">
            <VBadge color="info" content="6" location="bottom" offsetY="5" />
            <VMenu activator="parent">
                <VList>
                    <VListItem>
                        <VBtn class="text-caption" color="secondary-lighten-1" density="compact" variant="outlined">Alle neu starten</VBtn>
                    </VListItem>
                    <VListItem subtitle="Gestartet vor 10 Minuten" title="CC Migration">
                        <template #append>
                            <IconBtn @click.prevent.stop class="ml-1" icon="mdi-autorenew" />
                            <IconBtn @click.prevent.stop class="ml-1" icon="mdi-tab" />
                        </template>
                    </VListItem>
                </VList>
            </VMenu>
        </IconBtn>
        <IconBtn color="error" icon="mdi-stop">
            <VBadge color="error" content="6" density="compact" location="bottom" offsetY="5" />
            <VMenu activator="parent">
                <VList>
                    <VListItemSubtitle>Sessions</VListItemSubtitle>
                    <VListItem />
                </VList>
            </VMenu>
        </IconBtn>
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{
    task: TaskInfoContract;
}>();

const isStarting = ref(false);

const start = async () => {
    isStarting.value = true;
    await commands.taskStartOne(props.task.projectId, props.task.id);
    isStarting.value = false;
};
</script>
