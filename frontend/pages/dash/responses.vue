<script setup lang="ts">
definePageMeta({
    layout: "dash",
    middleware: ["auth", "admin", "dashboard-redirect"],
    pageTransition: {
        name: "dash",
    },
});

const { user } = useAuth();
</script>

<template>
    <div class="page-container flex-column">
        <h1 class="title">Session Responses</h1>
        
        <div class="welcome-message">
            <p class="text">Welcome to the responses management page, {{ user?.name }}!</p>
        </div>
        
        <div class="content-section">
            <div style="background: #f8f9fa; padding: 2rem; border-radius: 8px; text-align: center;">
                <h3>📋 Coming Soon</h3>
                <p>This page will show responses to session registrations and forms.</p>
                <p style="color: #6c757d; font-style: italic;">
                    Features to be implemented:
                </p>
                <ul style="text-align: left; max-width: 400px; margin: 1rem auto;">
                    <li>View all session registrations</li>
                    <li>Export attendance lists</li>
                    <li>Manage waitlists</li>
                    <li>Send notifications to participants</li>
                </ul>
            </div>
        </div>

        <div class="quick-actions flex-column">
            <h2 class="subtitle">Meanwhile, you can:</h2>
            <div class="actions-grid flex-row">
                <NuxtLink to="/dash/sessions" class="button primary-button">
                    View All Sessions
                </NuxtLink>
                <NuxtLink to="/dash/sessions/create" class="button secondary-button">
                    Create New Session
                </NuxtLink>
            </div>
        </div>
    </div>
</template>

<style scoped>
@import url("~/assets/css/dash-page.css");

.welcome-message {
    margin-bottom: 2rem;
}

.welcome-message .text {
    font-size: 1.2rem;
    color: var(--dash-txt-colour);
    margin: 0;
    padding: 1rem;
    background-color: var(--dash-bg-box-colour);
    border-radius: var(--radius-s);
}

.content-section {
    margin: 2rem 0;
}

.quick-actions {
    margin-top: 2rem;
    gap: 1rem;
}

.actions-grid {
    gap: 1rem;
    flex-wrap: wrap;
}

.actions-grid .button {
    padding: 1rem 2rem;
    font-size: 1.1rem;
}

@media (max-width: 480px) {
  .actions-grid {
    flex-direction: column;
  }
  
  .actions-grid .button {
    width: 100%;
  }
}
</style>