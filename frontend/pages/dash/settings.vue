<script setup lang="ts">
definePageMeta({
    layout: "dash",
    middleware: ["auth", "dashboard-redirect"],
    pageTransition: {
        name: "dash",
    },
});

const { user, logout } = useAuth();

const handleLogout = async () => {
    if (confirm('Are you sure you want to logout?')) {
        await logout();
    }
};
</script>

<template>
    <div class="page-container flex-column">
        <h1 class="title">Settings</h1>
        
        <!-- User Profile Section -->
        <div class="settings-section">
            <h2 class="section-title">👤 User Profile</h2>
            <div class="profile-card">
                <div class="profile-info">
                    <div class="info-item">
                        <strong>Name:</strong> {{ user?.name }}
                    </div>
                    <div class="info-item">
                        <strong>Shortcode:</strong> {{ user?.shortcode }}
                    </div>
                    <div class="info-item">
                        <strong>User ID:</strong> {{ user?.id }}
                    </div>
                    <div class="info-item">
                        <strong>Access Level:</strong> 
                        <span class="access-badge" :class="`tier-${user?.tier}`">
                            {{ user?.admin ? 'Administrator' : 
                               user?.tier === 2 ? 'Team Member' : 
                               user?.tier === 1 ? 'Member' : 'Non-Member' }}
                            (Tier {{ user?.tier }})
                        </span>
                    </div>
                </div>
            </div>
        </div>

        <!-- Account Actions -->
        <div class="settings-section">
            <h2 class="section-title">🔧 Account Actions</h2>
            <div class="actions-card">
                <div class="action-item">
                    <div class="action-info">
                        <h3>Change Password</h3>
                        <p>Update your account password</p>
                    </div>
                    <button class="button secondary-button" disabled>
                        Coming Soon
                    </button>
                </div>
                
                <div class="action-item">
                    <div class="action-info">
                        <h3>Update Profile</h3>
                        <p>Edit your name and contact information</p>
                    </div>
                    <button class="button secondary-button" disabled>
                        Coming Soon
                    </button>
                </div>

                <div class="action-item">
                    <div class="action-info">
                        <h3>Logout</h3>
                        <p>Sign out of your account</p>
                    </div>
                    <button @click="handleLogout" class="button danger-button">
                        Logout
                    </button>
                </div>
            </div>
        </div>

        <!-- System Info (Admin Only) -->
        <div v-if="user?.admin" class="settings-section">
            <h2 class="section-title">⚙️ System Settings</h2>
            <div class="system-card">
                <div class="action-item">
                    <div class="action-info">
                        <h3>User Management</h3>
                        <p>Manage user accounts and permissions</p>
                    </div>
                    <button class="button secondary-button" disabled>
                        Coming Soon
                    </button>
                </div>
                
                <div class="action-item">
                    <div class="action-info">
                        <h3>System Logs</h3>
                        <p>View application logs and activity</p>
                    </div>
                    <button class="button secondary-button" disabled>
                        Coming Soon
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
@import url("~/assets/css/dash-page.css");

.settings-section {
    margin-bottom: 3rem;
}

.section-title {
    color: var(--dash-txt-colour);
    font-size: 1.3rem;
    margin: 0 0 1rem 0;
    border-bottom: 2px solid var(--bg-20-colour);
    padding-bottom: 0.5rem;
}

.profile-card,
.actions-card,
.system-card {
    background-color: var(--dash-bg-box-colour);
    border-radius: var(--radius-m);
    padding: 2rem;
    border: 1px solid var(--field-border-colour);
}

.profile-info {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.info-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem 0;
}

.info-item strong {
    min-width: 120px;
    color: var(--dash-txt-colour);
}

.access-badge {
    padding: 0.25rem 0.75rem;
    border-radius: var(--radius-xs);
    font-size: 0.9rem;
    font-weight: 600;
    color: white;
}

.tier-0 { background-color: #6c757d; }
.tier-1 { background-color: #28a745; }
.tier-2 { background-color: #007bff; }

.action-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 0;
    border-bottom: 1px solid var(--field-border-colour);
}

.action-item:last-child {
    border-bottom: none;
}

.action-info h3 {
    margin: 0 0 0.5rem 0;
    color: var(--dash-txt-colour);
    font-size: 1.1rem;
}

.action-info p {
    margin: 0;
    color: var(--grey-txt-colour);
    font-size: 0.9rem;
}

.danger-button {
    background-color: #dc3545;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: var(--radius-xs);
    cursor: pointer;
    font: var(--btn);
}

.danger-button:hover {
    background-color: #c82333;
}

.button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

@media (max-width: 768px) {
    .action-item {
        flex-direction: column;
        align-items: flex-start;
        gap: 1rem;
    }
    
    .info-item {
        flex-direction: column;
        align-items: flex-start;
        gap: 0.25rem;
    }
    
    .info-item strong {
        min-width: auto;
    }
}
</style>