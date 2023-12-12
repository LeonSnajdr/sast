<template>
    <v-dialog v-model="dialog" width="auto" activator="parent">
        <v-card>
            <v-card-text>
                {{ message }}
            </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn @click="emit('confirmAction')">{{ $t("confirmationDialog.confirm") }}</v-btn>
                <v-btn @click="cancel">{{ $t("confirmationDialog.cancel") }}</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
defineProps<{
    message: string;
}>();

const emit = defineEmits<{
    (e: "confirmAction"): void;
    (e: "cancelAction"): void;
}>();

const dialog = ref(false);

const cancel = () => {
    dialog.value = false;
    emit("cancelAction");
};
</script>
