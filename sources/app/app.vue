<template>
    <VApp>
        <WindowBar />
        <NuxtLayout>
            <VMain class="h-screen">
                <PtySessionPool />
                <NotificationView />
                <NuxtPage />
            </VMain>
        </NuxtLayout>
    </VApp>
</template>
