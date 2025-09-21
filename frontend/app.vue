<!-- frontend/app.vue - Updated with notifications -->
<template>
    <NuxtLayout>
        <NuxtPage /> 
    </NuxtLayout>
    
    <!-- Global Notification System -->
    <NotificationSystem />
</template>

<script setup>
// Global error handler
const { error: notifyError } = useNotifications();

// Handle global errors
onErrorCaptured((error, instance, info) => {
  console.error('Global error captured:', error);
  notifyError('Application Error', error.message || 'An unexpected error occurred');
  return false; // Prevent the error from propagating further
});

// Handle unhandled promise rejections
if (process.client) {
  window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled promise rejection:', event.reason);
    notifyError('Network Error', 'A network request failed. Please check your connection.');
  });
}
</script>

<style>
.page-enter-active, .page-leave-active {
    transition: all 0.4s;
}
.page-enter-from, .page-leave-to {
    transform: translateY(-1rem);
    opacity: 0;
}
.dash-enter-active, .dash-leave-active {
    transition: all 0.4s;
}
.dash-enter-form, .dash-leave-to {
    opacity: 0;
}
.layout-enter-active, .layout-enter-active {
    transition: all 0.4s;
}
.layout-enter-from, .layout-leave-to {
    transform: translateY(-1rem);
    opacity: 0;
}

/* Global notification styles */
.notification-enter-active,
.notification-leave-active {
  transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.notification-enter-from {
  opacity: 0;
  transform: translateX(100%) scale(0.8);
}

.notification-leave-to {
  opacity: 0;
  transform: translateX(100%) scale(0.8);
}

.notification-move {
  transition: transform 0.3s ease;
}
</style>