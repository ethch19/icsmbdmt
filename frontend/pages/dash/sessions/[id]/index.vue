<script setup lang="ts">
definePageMeta({
    layout: "dash",
    middleware: ["auth", "admin"],
});

const route = useRoute();
const { getSession, getSessionBookings, bookSession, unbookSession } = useSessions();
const { user } = useAuth();

const sessionId = route.params.id as string;
const session = ref(null);
const bookings = ref([]);
const loading = ref(true);
const bookingsLoading = ref(false);
const error = ref('');

const loadSession = async () => {
  loading.value = true;
  error.value = '';
  
  try {
    const result = await getSession(sessionId);
    if (result.success) {
      session.value = result.data;
      await loadBookings();
    } else {
      error.value = result.error || 'Session not found';
    }
  } catch (err) {
    error.value = 'Failed to load session';
  }
  
  loading.value = false;
};

const loadBookings = async () => {
  if (!session.value || (!canViewBookings.value)) return;
  
  bookingsLoading.value = true;
  try {
    const result = await getSessionBookings(sessionId);
    if (result.success) {
      bookings.value = result.data;
    }
  } catch (err) {
    console.error('Failed to load bookings:', err);
  }
  bookingsLoading.value = false;
};

const handleBookToggle = async () => {
  if (!session.value) return;
  
  const action = session.value.is_booked ? unbookSession : bookSession;
  const result = await action(sessionId);
  
  if (result.success) {
    await loadSession(); // Refresh session data
  } else {
    alert(`Failed to ${session.value.is_booked ? 'unbook' : 'book'}: ${result.error}`);
  }
};

const canEdit = computed(() => {
  return session.value && (
    session.value.author_id === user.value?.id || 
    user.value?.admin
  );
});

const canViewBookings = computed(() => {
  return session.value && (
    session.value.author_id === user.value?.id || 
    user.value?.admin
  );
});

const canBook = computed(() => {
  if (!session.value) return false;
  
  // Check if session is full
  if (session.value.user_limit && session.value.current_bookings >= session.value.user_limit) {
    return false;
  }
  
  // Check if session has already started
  if (new Date(session.value.start_time) < new Date()) {
    return false;
  }
  
  return true;
});

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
  loadSession();
});
</script>

<template>
  <div class="page-container flex-column">
    <div class="header-row flex-row">
      <h1 class="title">Session Details</h1>
      <div class="header-actions flex-row">
        <NuxtLink 
          v-if="canEdit"
          :to="`/dash/sessions/${sessionId}/edit`" 
          class="button tertiary-button"
        >
          Edit
        </NuxtLink>
        <NuxtLink to="/dash/sessions" class="button secondary-button">
          Back to Sessions
        </NuxtLink>
      </div>
    </div>

    <div v-if="loading" class="loading-container">
      <p class="text">Loading session...</p>
    </div>

    <div v-else-if="error" class="error-container">
      <p class="error-text">{{ error }}</p>
      <NuxtLink to="/dash/sessions" class="button secondary-button">
        Back to Sessions
      </NuxtLink>
    </div>

    <div v-else class="session-details">
      <div class="session-info-card">
        <div class="session-header">
          <h2 class="session-title">{{ session.title }}</h2>
          <div class="session-badges">
            <span class="tier-badge" :class="`tier-${session.tier}`">
              {{ getTierName(session.tier) }}
            </span>
            <span v-if="session.is_booked" class="status-badge booked">
              Booked
            </span>
          </div>
        </div>

        <div class="session-content">
          <div class="session-description">
            <h3>Description</h3>
            <p>{{ session.description }}</p>
          </div>

          <div class="session-meta">
            <div class="meta-item">
              <strong>Location:</strong>
              <span>{{ session.location }}</span>
            </div>
            <div class="meta-item">
              <strong>Start Time:</strong>
              <span>{{ formatDate(session.start_time) }}</span>
            </div>
            <div class="meta-item">
              <strong>End Time:</strong>
              <span>{{ formatDate(session.end_time) }}</span>
            </div>
            <div class="meta-item">
              <strong>Author:</strong>
              <span>{{ session.author_name }}</span>
            </div>
            <div class="meta-item">
              <strong>Capacity:</strong>
              <span>
                {{ session.current_bookings }}
                <span v-if="session.user_limit">/ {{ session.user_limit }}</span>
                <span v-else>/ Unlimited</span>
              </span>
            </div>
            <div v-if="session.recurrence" class="meta-item">
              <strong>Recurring:</strong>
              <span>Weekly until {{ formatDate(session.recurrence_end) }}</span>
            </div>
          </div>

          <div class="session-actions">
            <button 
              v-if="user?.tier >= session.tier && !canEdit"
              @click="handleBookToggle"
              class="button"
              :class="session.is_booked ? 'danger-button' : 'primary-button'"
              :disabled="!session.is_booked && !canBook"
            >
              {{ session.is_booked ? 'Cancel Booking' : 'Book Session' }}
            </button>
            <span v-if="!canBook && !session.is_booked && user?.tier >= session.tier" class="booking-disabled">
              {{ session.user_limit && session.current_bookings >= session.user_limit ? 'Session is full' : 'Session has started' }}
            </span>
          </div>
        </div>
      </div>

      <div v-if="canViewBookings" class="bookings-section">
        <div class="section-header">
          <h3>Bookings ({{ session.current_bookings }})</h3>
          <button @click="loadBookings" class="button secondary-button">
            Refresh
          </button>
        </div>

        <div v-if="bookingsLoading" class="loading-text">
          Loading bookings...
        </div>

        <div v-else-if="bookings.length === 0" class="no-bookings">
          <p>No bookings yet.</p>
        </div>

        <div v-else class="bookings-list">
          <div v-for="booking in bookings" :key="booking.user_id" class="booking-item">
            <div class="booking-info">
              <span class="booking-name">{{ booking.user_name }}</span>
              <span class="booking-time">Booked {{ formatDate(booking.created_at) }}</span>
            </div>
          </div>
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

.header-actions {
  gap: 1rem;
}

.loading-container,
.error-container {
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

.session-details {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.session-info-card {
  background-color: var(--dash-bg-box-colour);
  border-radius: var(--radius-m);
  padding: 2rem;
  border: 1px solid var(--field-border-colour);
}

.session-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 2rem;
}

.session-title {
  margin: 0;
  color: var(--dash-txt-colour);
  font-size: 1.8rem;
}

.session-badges {
  display: flex;
  gap: 0.5rem;
}

.tier-badge,
.status-badge {
  padding: 0.25rem 0.75rem;
  border-radius: var(--radius-xs);
  font-size: 0.8rem;
  font-weight: 600;
  color: white;
}

.tier-0 { background-color: #6c757d; }
.tier-1 { background-color: #28a745; }
.tier-2 { background-color: #007bff; }

.status-badge.booked {
  background-color: #28a745;
}

.session-content {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.session-description h3 {
  margin: 0 0 1rem 0;
  color: var(--dash-txt-colour);
}

.session-description p {
  margin: 0;
  line-height: 1.6;
  color: var(--dash-txt-colour);
}

.session-meta {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.meta-item strong {
  color: var(--dash-txt-colour);
  font-size: 0.9rem;
}

.meta-item span {
  color: var(--dash-txt-colour);
  font-weight: 500;
}

.session-actions {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.booking-disabled {
  color: var(--grey-txt-colour);
  font-style: italic;
}

.bookings-section {
  background-color: var(--dash-bg-box-colour);
  border-radius: var(--radius-m);
  padding: 2rem;
  border: 1px solid var(--field-border-colour);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.section-header h3 {
  margin: 0;
  color: var(--dash-txt-colour);
}

.loading-text,
.no-bookings {
  text-align: center;
  color: var(--grey-txt-colour);
  padding: 2rem;
}

.bookings-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.booking-item {
  padding: 1rem;
  background-color: var(--bg-20-colour);
  border-radius: var(--radius-s);
  border: 1px solid var(--field-border-colour);
}

.booking-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.booking-name {
  font-weight: 600;
  color: var(--dash-txt-colour);
}

.booking-time {
  font-size: 0.9rem;
  color: var(--grey-txt-colour);
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

@media (max-width: 768px) {
  .header-row {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }
  
  .header-actions {
    justify-content: stretch;
  }
  
  .session-header {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }
  
  .session-meta {
    grid-template-columns: 1fr;
  }
  
  .booking-info {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
}
</style>