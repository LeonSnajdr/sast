<template>
    <v-dialog v-model="dialog" activator="parent" width="auto">
        <v-card>
            <v-card-text> Hai </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="primary" @click="dialog = false">TODO Close</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import { ref } from "vue";

const dialog = ref(false);
</script>
