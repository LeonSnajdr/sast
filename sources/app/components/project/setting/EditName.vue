<template>
    <BaseDialogEdit v-model="isDialogOpen" :type="$t('project.singular')" icon="mdi-folder">
        <template v-if="project.id" #content>
            <VConfirmEdit ref="confirmEdit" v-model="project">
                <!--  eslint-disable-next-line vue/no-unused-vars -> Needed to hide the default actions -->
                <template #default="{ model: proxyProject, actions }">
                    <ProjectFieldContainer v-model="proxyProject.value" v-model:isValid="isValid" />
                </template>
            </VConfirmEdit>
        </template>
        <template v-if="project.id" #actions>
            <BaseBtnIcon @click="save()" :disabled="!isValid" color="success" icon="mdi-content-save" variant="flat" v-tooltip="$t('keybind.controlS.tooltip')">
                {{ $t("action.save") }}
            </BaseBtnIcon>
        </template>
    </BaseDialogEdit>
</template>

<script setup lang="ts">
import type { VConfirmEdit } from "vuetify/components";

const project = defineModel<ProjectContract>({ required: true });

useKeybind(["control", "s"], () => save());

const isDialogOpen = ref(false);
const isValid = ref<boolean | null>(null);

const confirmEdit = ref<VConfirmEdit>();

const save = () => {
    confirmEdit.value?.save();
    isDialogOpen.value = false;
};
</script>
