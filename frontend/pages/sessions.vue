<!-- frontend/pages/sessions.vue - Public session browsing -->
<script setup lang="ts">
definePageMeta({
    title: 'Training Sessions - ICSM Badminton'
});

const { getSessions, bookSession, unbookSession } = useSessions();
const { isLoggedIn, user } = useAuth();
const { success, error: notifyError } = useNotifications();

const sessions = ref([]);
const loading = ref(true);
const error = ref('');

// Filtering
const searchQuery = ref('');
const selectedTier = ref<number | null>(null);
const dateFilter = ref<string>('upcoming'); // 'all', 'upcoming', 'today', 'this_week'

const loadSessions = async () => {
  loading.value = true;
  error.value = '';
  
  try {
    const result = await getSessions({ limit: 50, upcoming_only: false });
    if (result.success) {
      sessions.value = result.data;
    } else {
      error.value = result.error || 'Failed to load sessions';
    }
  } catch (err) {
    error.value = 'Failed to load sessions';
  }
  
  loading.value = false;
};

const handleBookToggle = async (session: any) => {
  if (!isLoggedIn.value) {
    notifyError('Login Required', 'Please login to book sessions');
    await navigateTo('/login?redirect=/sessions');
    return;
  }

  const action = session.is_booked ? unbookSession : bookSession;
  const result = await action(session.id, session.title);
  
  if (result.success) {
    await loadSessions(); // Refresh sessions
  }
};

// Computed properties for filtering
const filteredSessions = computed(() => {
  return sessions.value.filter(session => {
    // Search filter
    const matchesSearch = !searchQuery.value || 
      session.title.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      session.description.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      session.location.toLowerCase().includes(searchQuery.value.toLowerCase());
    
    // Tier filter (show sessions user can access)
    const userTier = user.value?.tier || 0;
    const matchesTier = selectedTier.value === null || session.tier === selectedTier.value;
    const canAccess = session.tier <= userTier;
    
    // Date filter
    const now = new Date();
    const sessionStart = new Date(session.start_time);
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const weekStart = new Date(today);
    weekStart.setDate(today.getDate() - today.getDay());
    const weekEnd = new Date(weekStart);
    weekEnd.setDate(weekStart.getDate() + 6);
    
    let matchesDate = true;
    switch (dateFilter.value) {
      case 'upcoming':
        matchesDate = sessionStart > now;
        break;
      case 'today':
        matchesDate = sessionStart >= today && sessionStart < new Date(today.getTime() + 24 * 60 * 60 * 1000);
        break;
      case 'this_week':
        matchesDate = sessionStart >= weekStart && sessionStart <= weekEnd;
        break;
      default:
        matchesDate = true;
    }
    
    return matchesSearch && matchesTier && canAccess && matchesDate;
  }).sort((a, b) => new Date(a.start_time) - new Date(b.start_time));
});

const upcomingSessions = computed(() => {
  return sessions.value.filter(s => new Date(s.start_time) > new Date()).length;
});

const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return {
    date: date.toLocaleDateString('en-GB', { 
      weekday: 'long', 
      year: 'numeric', 
      month: 'long', 
      day: 'numeric' 
    }),
    time: date.toLocaleTimeString('en-GB', { 
      hour: '2-digit', 
      minute: '2-digit' 
    })
  };
};

const getTierName = (tier: number) => {
  switch (tier) {
    case 0: return 'Open to All';
    case 1: return 'Members Only';
    case 2: return 'Team Members Only';
    default: return 'Unknown';
  }
};

const getTierBadgeClass = (tier: number) => {
  switch (tier) {
    case 0: return 'tier-open';
    case 1: return 'tier-members';
    case 2: return 'tier-team';
    default: return 'tier-unknown';
  }
};

const canBook = (session: any) => {
  if (!isLoggedIn.value) return false;
  if (session.is_booked) return true; // Can unbook
  
  const userTier = user.value?.tier || 0;
  if (session.tier > userTier) return false;
  
  if (session.user_limit && session.current_bookings >= session.user_limit) return false;
  if (new Date(session.start_time) < new Date()) return false;
  
  return true;
};

const getBookingStatus = (session: any) => {
  if (!isLoggedIn.value) return 'login';
  if (session.is_booked) return 'booked';
  
  const userTier = user.value?.tier || 0;
  if (session.tier > userTier) return 'restricted';
  
  if (session.user_limit && session.current_bookings >= session.user_limit) return 'full';
  if (new Date(session.start_time) < new Date()) return 'past';
  
  return 'available';
};

onMounted(() => {
  loadSessions();
});
</script>

<template>
  <main class="page-container flex-column">
    <div class="hero-section">
      <div class="hero-content">
        <h1 class="title">Training Sessions</h1>
        <p class="hero-subtitle">Join our badminton training sessions and improve your game!</p>
        <div class="hero-stats">
          <div class="stat-item">
            <span class="stat-number">{{ upcomingSessions }}</span>
            <span class="stat-label">Upcoming Sessions</span>
          </div>
          <div class="stat-item">
            <span class="stat-number">{{ sessions.length }}</span>
            <span class="stat-label">Total Sessions</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Filters Section -->
    <div class="filters-container">
      <div class="filters-row">
        <div class="search-container">
          <input 
            v-model="searchQuery"
            type="text" 
            class="search-input"
            placeholder="🔍 Search sessions..."
          >
        </div>
        
        <div class="filter-group">
          <label class="filter-label">Date:</label>
          <select v-model="dateFilter" class="filter-select">
            <option value="upcoming">Upcoming</option>
            <option value="all">All Sessions</option>
            <option value="today">Today</option>
            <option value="this_week">This Week</option>
          </select>
        </div>
        
        <div class="filter-group">
          <label class="filter-label">Access Level:</label>
          <select v-model="selectedTier" class="filter-select">
            <option :value="null">All Levels</option>
            <option :value="0">Open to All</option>
            <option :value="1">Members Only</option>
            <option :value="2">Team Members Only</option>
          </select>
        </div>
        
        <button @click="loadSessions" class="refresh-btn">
          🔄 Refresh
        </button>
      </div>
    </div>

    <!-- Auth Banner for Non-logged in Users -->
    <div v-if="!isLoggedIn" class="auth-banner">
      <div class="auth-content">
        <h3>Join ICSM Badminton</h3>
        <p>Login or create an account to book training sessions</p>
        <div class="auth-actions">
          <NuxtLink to="/login" class="auth-btn primary">Login</NuxtLink>
          <NuxtLink to="/register" class="auth-btn secondary">Register</NuxtLink>
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="loading-container">
      <div class="loading-spinner"></div>
      <p>Loading sessions...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="error-container">
      <h3>Oops! Something went wrong</h3>
      <p>{{ error }}</p>
      <button @click="loadSessions" class="retry-btn">Try Again</button>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredSessions.length === 0" class="empty-container">
      <h3>No sessions found</h3>
      <p v-if="searchQuery || selectedTier !== null">
        Try adjusting your filters to see more sessions.
      </p>
      <p v-else>
        No training sessions are currently available.
      </p>
      <div class="empty-actions">
        <button @click="searchQuery = ''; selectedTier = null; dateFilter = 'upcoming'" class="clear-filters-btn">
          Clear Filters
        </button>
        <NuxtLink to="/membership" class="membership-btn">
          Learn About Membership
        </NuxtLink>
      </div>
    </div>

    <!-- Sessions Grid -->
    <div v-else class="sessions-grid">
      <div 
        v-for="session in filteredSessions" 
        :key="session.id" 
        class="session-card"
        :class="{ 
          'past-session': new Date(session.start_time) < new Date(),
          'user-booked': session.is_booked 
        }"
      >
        <!-- Session Header -->
        <div class="session-header">
          <h3 class="session-title">{{ session.title }}</h3>
          <div class="session-badges">
            <span class="tier-badge" :class="getTierBadgeClass(session.tier)">
              {{ getTierName(session.tier) }}
            </span>
            <span v-if="session.is_booked" class="booked-badge">
              ✓ Booked
            </span>
          </div>
        </div>

        <!-- Session Description -->
        <p class="session-description">{{ session.description }}</p>

        <!-- Session Details -->
        <div class="session-details">
          <div class="detail-item">
            <span class="detail-icon">📍</span>
            <span class="detail-text">{{ session.location }}</span>
          </div>
          
          <div class="detail-item">
            <span class="detail-icon">📅</span>
            <span class="detail-text">{{ formatDate(session.start_time).date }}</span>
          </div>
          
          <div class="detail-item">
            <span class="detail-icon">🕐</span>
            <span class="detail-text">
              {{ formatDate(session.start_time).time }} - {{ formatDate(session.end_time).time }}
            </span>
          </div>
          
          <div class="detail-item">
            <span class="detail-icon">👥</span>
            <span class="detail-text">
              {{ session.current_bookings }}
              <span v-if="session.user_limit">/ {{ session.user_limit }}</span>
              <span v-else>participants</span>
            </span>
          </div>
        </div>

        <!-- Session Actions -->
        <div class="session-actions">
          <template v-if="getBookingStatus(session) === 'login'">
            <NuxtLink to="/login" class="action-btn login-btn">
              Login to Book
            </NuxtLink>
          </template>
          
          <template v-else-if="getBookingStatus(session) === 'booked'">
            <button 
              @click="handleBookToggle(session)"
              class="action-btn cancel-btn"
            >
              Cancel Booking
            </button>
          </template>
          
          <template v-else-if="getBookingStatus(session) === 'available'">
            <button 
              @click="handleBookToggle(session)"
              class="action-btn book-btn"
            >
              Book Session
            </button>
          </template>
          
          <template v-else-if="getBookingStatus(session) === 'full'">
            <button class="action-btn full-btn" disabled>
              Session Full
            </button>
          </template>
          
          <template v-else-if="getBookingStatus(session) === 'restricted'">
            <div class="restricted-info">
              <span class="restricted-text">
                Requires {{ getTierName(session.tier) }} Access
              </span>
              <NuxtLink to="/membership" class="membership-link">
                Upgrade Membership
              </NuxtLink>
            </div>
          </template>
          
          <template v-else-if="getBookingStatus(session) === 'past'">
            <span class="past-text">Session Completed</span>
          </template>
        </div>
      </div>
    </div>

    <!-- Call to Action -->
    <div class="cta-section">
      <div class="cta-content">
        <h2>Ready to Join?</h2>
        <p>Become a member and access all our training sessions, tournaments, and social events.</p>
        <div class="cta-actions">
          <NuxtLink to="/membership" class="cta-btn primary">
            View Membership Options
          </NuxtLink>
          <NuxtLink to="/about" class="cta-btn secondary">
            Learn More About Us
          </NuxtLink>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.page-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #f8fffe 0%, #f0f8ef 100%);
}

/* Hero Section */
.hero-section {
  background: linear-gradient(135deg, var(--accent-colour) 0%, #2d4232 100%);
  color: white;
  padding: 4rem 2rem 3rem;
  margin-bottom: 3rem;
}

.hero-content {
  max-width: 1200px;
  margin: 0 auto;
  text-align: center;
}

.hero-content .title {
  color: white;
  font-size: 3rem;
  margin-bottom: 1rem;
}

.hero-subtitle {
  font-size: 1.2rem;
  opacity: 0.9;
  margin-bottom: 2rem;
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
}

.hero-stats {
  display: flex;
  justify-content: center;
  gap: 3rem;
  margin-top: 2rem;
}

.stat-item {
  text-align: center;
}

.stat-number {
  display: block;
  font-size: 2.5rem;
  font-weight: 700;
  color: #a3d9a5;
}

.stat-label {
  font-size: 0.9rem;
  opacity: 0.8;
}

/* Filters */
.filters-container {
  max-width: 1200px;
  margin: 0 auto 2rem;
  padding: 0 2rem;
}

.filters-row {
  display: flex;
  gap: 1rem;
  align-items: end;
  flex-wrap: wrap;
  background: white;
  padding: 1.5rem;
  border-radius: var(--radius-m);
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.search-container {
  flex: 1;
  min-width: 250px;
}

.search-input {
  width: 100%;
  padding: 0.75rem 1rem;
  border: 2px solid var(--field-border-colour);
  border-radius: var(--radius-s);
  font-size: 1rem;
  background: var(--bg-field-colour);
}

.search-input:focus {
  outline: none;
  border-color: var(--accent-colour);
  background: white;
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  min-width: 150px;
}

.filter-label {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--dash-txt-colour);
}

.filter-select {
  padding: 0.75rem;
  border: 2px solid var(--field-border-colour);
  border-radius: var(--radius-s);
  background: var(--bg-field-colour);
  font-size: 0.9rem;
}

.refresh-btn {
  padding: 0.75rem 1.5rem;
  background: var(--accent-colour);
  color: white;
  border: none;
  border-radius: var(--radius-s);
  cursor: pointer;
  font-weight: 600;
  transition: all 0.2s ease;
}

.refresh-btn:hover {
  background: var(--btn-11-colour);
  transform: translateY(-1px);
}

/* Auth Banner */
.auth-banner {
  background: linear-gradient(135deg, #e3f2fd 0%, #f3e5f5 100%);
  border: 2px solid #2196f3;
  border-radius: var(--radius-m);
  padding: 2rem;
  margin: 0 auto 2rem;
  max-width: 1200px;
  margin-left: auto;
  margin-right: auto;
}

.auth-content {
  text-align: center;
}

.auth-content h3 {
  color: #1976d2;
  margin: 0 0 0.5rem 0;
  font-size: 1.3rem;
}

.auth-content p {
  color: #424242;
  margin: 0 0 1.5rem 0;
}

.auth-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.auth-btn {
  padding: 0.75rem 2rem;
  text-decoration: none;
  border-radius: var(--radius-s);
  font-weight: 600;
  transition: all 0.2s ease;
}

.auth-btn.primary {
  background: #1976d2;
  color: white;
}

.auth-btn.primary:hover {
  background: #1565c0;
}

.auth-btn.secondary {
  background: white;
  color: #1976d2;
  border: 2px solid #1976d2;
}

.auth-btn.secondary:hover {
  background: #1976d2;
  color: white;
}

/* Loading, Error, Empty States */
.loading-container, .error-container, .empty-container {
  text-align: center;
  padding: 4rem 2rem;
  max-width: 600px;
  margin: 0 auto;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid var(--accent-colour);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 1rem;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-container h3 {
  color: #d32f2f;
  margin-bottom: 1rem;
}

.retry-btn, .clear-filters-btn, .membership-btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: var(--radius-s);
  cursor: pointer;
  font-weight: 600;
  text-decoration: none;
  display: inline-block;
  margin: 0.5rem;
  transition: all 0.2s ease;
}

.retry-btn {
  background: var(--accent-colour);
  color: white;
}

.clear-filters-btn {
  background: #f5f5f5;
  color: #666;
}

.membership-btn {
  background: #1976d2;
  color: white;
}

/* Sessions Grid */
.sessions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
  gap: 2rem;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
}

.session-card {
  background: white;
  border-radius: var(--radius-m);
  padding: 2rem;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  border: 1px solid var(--field-border-colour);
  transition: all 0.3s ease;
  position: relative;
}

.session-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0,0,0,0.15);
}

.user-booked {
  border-left: 4px solid #4caf50;
  background: linear-gradient(135deg, #f1f8e9 0%, white 100%);
}

.past-session {
  opacity: 0.7;
  background: #f9f9f9;
}

.session-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  margin-bottom: 1rem;
  gap: 1rem;
}

.session-title {
  margin: 0;
  color: var(--dash-txt-colour);
  font-size: 1.3rem;
  line-height: 1.3;
  flex: 1;
}

.session-badges {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  flex-shrink: 0;
}

.tier-badge, .booked-badge {
  padding: 0.25rem 0.75rem;
  border-radius: var(--radius-xs);
  font-size: 0.8rem;
  font-weight: 600;
  text-align: center;
  white-space: nowrap;
}

.tier-open { background: #4caf50; color: white; }
.tier-members { background: #ff9800; color: white; }
.tier-team { background: #f44336; color: white; }

.booked-badge {
  background: #4caf50;
  color: white;
}

.session-description {
  color: var(--dash-txt-colour);
  line-height: 1.5;
  margin-bottom: 1.5rem;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.session-details {
  margin-bottom: 1.5rem;
}

.detail-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.75rem;
  font-size: 0.95rem;
}

.detail-icon {
  font-size: 1.1rem;
  width: 20px;
  text-align: center;
}

.detail-text {
  color: var(--dash-txt-colour);
  font-weight: 500;
}

.session-actions {
  margin-top: auto;
}

.action-btn {
  width: 100%;
  padding: 0.75rem 1rem;
  border: none;
  border-radius: var(--radius-s);
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  text-decoration: none;
  display: inline-block;
  text-align: center;
}

.book-btn {
  background: var(--accent-colour);
  color: white;
}

.book-btn:hover {
  background: var(--btn-11-colour);
}

.cancel-btn {
  background: #f44336;
  color: white;
}

.cancel-btn:hover {
  background: #d32f2f;
}

.login-btn {
  background: #1976d2;
  color: white;
}

.login-btn:hover {
  background: #1565c0;
}

.full-btn {
  background: #9e9e9e;
  color: white;
  cursor: not-allowed;
}

.restricted-info {
  text-align: center;
}

.restricted-text {
  display: block;
  color: #ff9800;
  font-weight: 600;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
}

.membership-link {
  display: inline-block;
  color: #1976d2;
  text-decoration: underline;
  font-size: 0.9rem;
}

.past-text {
  color: #666;
  font-style: italic;
  text-align: center;
  display: block;
}

/* CTA Section */
.cta-section {
  background: linear-gradient(135deg, var(--bg-20-colour) 0%, #f0f8ef 100%);
  padding: 4rem 2rem;
  margin-top: 4rem;
}

.cta-content {
  max-width: 800px;
  margin: 0 auto;
  text-align: center;
}

.cta-content h2 {
  color: var(--accent-colour);
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.cta-content p {
  font-size: 1.1rem;
  color: var(--dash-txt-colour);
  margin-bottom: 2rem;
  line-height: 1.6;
}

.cta-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
}

.cta-btn {
  padding: 1rem 2rem;
  text-decoration: none;
  border-radius: var(--radius-s);
  font-weight: 600;
  font-size: 1.1rem;
  transition: all 0.2s ease;
}

.cta-btn.primary {
  background: var(--accent-colour);
  color: white;
}

.cta-btn.primary:hover {
  background: var(--btn-11-colour);
  transform: translateY(-2px);
}

.cta-btn.secondary {
  background: white;
  color: var(--accent-colour);
  border: 2px solid var(--accent-colour);
}

.cta-btn.secondary:hover {
  background: var(--accent-colour);
  color: white;
}

/* Responsive Design */
@media (max-width: 768px) {
  .hero-content .title {
    font-size: 2rem;
  }
  
  .hero-stats {
    gap: 2rem;
  }
  
  .stat-number {
    font-size: 2rem;
  }
  
  .filters-row {
    flex-direction: column;
    align-items: stretch;
  }
  
  .sessions-grid {
    grid-template-columns: 1fr;
    padding: 0 1rem;
  }
  
  .session-header {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
  }
  
  .session-badges {
    flex-direction: row;
  }
  
  .auth-actions {
    flex-direction: column;
    align-items: center;
  }
  
  .cta-actions {
    flex-direction: column;
    align-items: center;
  }
  
  .cta-content h2 {
    font-size: 2rem;
  }
}

@media (max-width: 480px) {
  .hero-section {
    padding: 2rem 1rem;
  }
  
  .session-card {
    padding: 1.5rem;
  }
  
  .filters-container {
    padding: 0 1rem;
  }
}
</style>