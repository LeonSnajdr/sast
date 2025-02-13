<template>
    <VChip @click:close.prevent="props.destroy()" class="mx-1" prependIcon="mdi-account" size="small" closable>
        {{ item.placeholderName }} | {{ item.placeholderId }}
        <VMenu activator="parent">
            <VCard>
                <VCardText>
                    <p>Some more information</p>
                </VCardText>
            </VCard>
        </VMenu>
    </VChip>
</template>

<script setup lang="ts">
import type { InsertProps } from "vue-text-insert";

const props = defineProps<InsertProps<PlaceholderInsertTileContract>>();

onMounted(() => {});

onBeforeUnmount(() => {
    console.log("unmounted");
});
</script>
