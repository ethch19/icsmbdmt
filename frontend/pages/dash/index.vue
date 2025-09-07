<script setup lang="ts">
definePageMeta({
    layout: "dash",
    middleware: ["auth", "admin"],
    pageTransition: {
        name: "dash",
    },
});

const { getSessions } = useSessions();
const { user } = useAuth();

const sessions = ref([]);
const stats = reactive({
  totalSessions: 0,
  totalBookings: 0,
  recentSessions: 0
});
const loading = ref(true);

const loadDashboardData = async () => {
  loading.value = true;
  
  try {
    const result = await getSessions({ limit: 50, upcoming_only: false });
    if (result.success) {
      sessions.value = result.data;
      
      // Calculate stats
      stats.totalSessions = result.data.length;
      stats.totalBookings = result.data.reduce((sum, session) => sum + session.current_bookings, 0);
      
      // Sessions created in last 7 days
      const weekAgo = new Date();
      weekAgo.setDate(weekAgo.getDate() - 7);
      stats.recentSessions = result.data.filter(session => 
        new Date(session.created_at) >= weekAgo
      ).length;
    }
  } catch (error) {
    console.error('Failed to load dashboard data:', error);
  }
  
  loading.value = false;
};

onMounted(() => {
  loadDashboardData();
});
</script>

<template>
    <div class="page-container flex-column">
        <h1 class="title">Dashboard</h1>
        <div class="welcome-message">
            <p class="text">Welcome back, {{ user?.name }}!</p>
        </div>
        
        <div v-if="loading" class="loading-container">
            <p class="text">Loading dashboard...</p>
        </div>
        
        <div v-else class="home-container flex-row">
            <SummaryBlock 
              line1="Total Sessions" 
              line2="All Time" 
              :num="stats.totalSessions" 
            />
            <SummaryBlock 
              line1="Total Bookings" 
              line2="All Sessions" 
              :num="stats.totalBookings" 
            />
            <SummaryBlock 
              line1="New Sessions" 
              line2="Past 7 Days" 
              :num="stats.recentSessions" 
            />
        </div>

        <div class="quick-actions flex-column">
            <h2 class="subtitle">Quick Actions</h2>
            <div class="actions-grid flex-row">
                <NuxtLink to="/dash/sessions/create" class="button primary-button">
                    Create New Session
                </NuxtLink>
                <NuxtLink to="/dash/sessions" class="button secondary-button">
                    Manage Sessions
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

.loading-container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 200px;
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