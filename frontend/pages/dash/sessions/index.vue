<script setup lang="ts">
definePageMeta({
    layout: "dash",
    middleware: ["auth", "admin"],
});

const { getSessions, deleteSession } = useSessions();
const { user } = useAuth();

const sessions = ref([]);
const loading = ref(true);
const error = ref('');

const loadSessions = async () => {
  loading.value = true;
  error.value = '';
  
  try {
    const result = await getSessions({ limit: 100, upcoming_only: false });
    if (result.success) {
      sessions.value = result.data.sort((a, b) => 
        new Date(b.start_time) - new Date(a.start_time)
      );
    } else {
      error.value = result.error || 'Failed to load sessions';
    }
  } catch (err) {
    error.value = 'Failed to load sessions';
  }
  
  loading.value = false;
};

const handleDelete = async (id: string, title: string) => {
  if (!confirm(`Are you sure you want to delete "${title}"?`)) return;
  
  const result = await deleteSession(id);
  if (result.success) {
    await loadSessions(); // Refresh list
  } else {
    alert(`Failed to delete session: ${result.error}`);
  }
};

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString();
};

const getTierName = (tier: number) => {
  switch (tier) {
    case 0: return 'Non-Member';
    case 1: return 'Member';
    case 2: return 'Team Member';
    default: return 'Unknown';
  }
};

onMounted(() => {
  loadSessions();
});
</script>

<template>
  <div class="page-container flex-column">
    <div class="header-row flex-row">
      <h1 class="title">Sessions</h1>
      <NuxtLink to="/dash/sessions/create" class="button primary-button">
        Create New Session
      </NuxtLink>
    </div>

    <div v-if="loading" class="loading-container">
      <p class="text">Loading sessions...</p>
    </div>

    <div v-else-if="error" class="error-container">
      <p class="error-text">{{ error }}</p>
      <button @click="loadSessions" class="button secondary-button">
        Try Again
      </button>
    </div>

    <div v-else-if="sessions.length === 0" class="empty-container">
      <p class="text">No sessions found.</p>
      <NuxtLink to="/dash/sessions/create" class="button primary-button">
        Create Your First Session
      </NuxtLink>
    </div>

    <div v-else class="sessions-grid">
      <div 
        v-for="session in sessions" 
        :key="session.id" 
        class="session-card"
      >
        <div class="session-header">
          <h3 class="session-title">{{ session.title }}</h3>
          <div class="session-meta">
            <span class="tier-badge" :class="`tier-${session.tier}`">
              {{ getTierName(session.tier) }}
            </span>
          </div>
        </div>

        <div class="session-details">
          <p class="session-description">{{ session.description }}</p>
          <div class="session-info">
            <p><strong>Location:</strong> {{ session.location }}</p>
            <p><strong>Start:</strong> {{ formatDate(session.start_time) }}</p>
            <p><strong>End:</strong> {{ formatDate(session.end_time) }}</p>
            <p><strong>Bookings:</strong> 
              {{ session.current_bookings }}
              <span v-if="session.user_limit">/ {{ session.user_limit }}</span>
            </p>
            <p><strong>Author:</strong> {{ session.author_name }}</p>
          </div>
        </div>

        <div class="session-actions">
          <NuxtLink 
            :to="`/dash/sessions/${session.id}`" 
            class="button secondary-button"
          >
            View
          </NuxtLink>
          <NuxtLink 
            :to="`/dash/sessions/${session.id}/edit`" 
            class="button tertiary-button"
            v-if="session.author_id === user?.id || user?.admin"
          >
            Edit
          </NuxtLink>
          <button 
            @click="handleDelete(session.id, session.title)"
            class="button danger-button"
            v-if="session.author_id === user?.id || user?.admin"
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@import url("~/assets/css/dash-page.css");

.header-row {
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.loading-container,
.error-container,
.empty-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  padding: 3rem;
  text-align: center;
}

.error-text {
  color: #dc3545;
  margin: 0;
}

.sessions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
  gap: 2rem;
}

.session-card {
  background-color: var(--dash-bg-box-colour);
  border-radius: var(--radius-m);
  padding: 1.5rem;
  border: 1px solid var(--field-border-colour);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.session-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.session-title {
  margin: 0;
  color: var(--dash-txt-colour);
  font-size: 1.2rem;
}

.tier-badge {
  padding: 0.25rem 0.5rem;
  border-radius: var(--radius-xs);
  font-size: 0.8rem;
  font-weight: 600;
  color: white;
}

.tier-0 { background-color: #6c757d; }
.tier-1 { background-color: #28a745; }
.tier-2 { background-color: #007bff; }

.session-description {
  margin: 0;
  color: var(--dash-txt-colour);
  line-height: 1.4;
}

.session-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.session-info p {
  margin: 0;
  font-size: 0.9rem;
  color: var(--dash-txt-colour);
}

.session-actions {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
  margin-top: auto;
}

.danger-button {
  background-color: #dc3545;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: var(--radius-xs);
  cursor: pointer;
  font: var(--btn);
}

.danger-button:hover {
  background-color: #c82333;
}

@media (max-width: 768px) {
  .sessions-grid {
    grid-template-columns: 1fr;
  }
  
  .header-row {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }
}
</style>