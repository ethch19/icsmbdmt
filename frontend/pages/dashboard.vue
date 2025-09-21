<!-- frontend/pages/dashboard.vue - User dashboard for regular users -->
<script setup lang="ts">
definePageMeta({
    middleware: ["auth", "dashboard-redirect"],
    title: 'My Dashboard - ICSM Badminton'
});

const { user, logout } = useAuth();
const { getSessions } = useSessions();
const { success, error: notifyError } = useNotifications();

// Redirect admins to admin dashboard
onMounted(() => {
  if (user.value?.admin || user.value?.tier >= 2) {
    navigateTo('/dash');
  }
});

const userBookings = ref([]);
const upcomingSessions = ref([]);
const loading = ref(true);
const stats = reactive({
  totalBookings: 0,
  upcomingBookings: 0,
  completedSessions: 0,
  memberSince: ''
});

// Get API base URL
const getApiBase = () => {
  if (process.client && window.location.hostname === 'localhost') {
    return 'http://localhost:8000';
  }
  return '';
};

const getAuthHeaders = (): Record<string, string> => {
  if (process.client) {
    const accessToken = localStorage.getItem('access_token');
    if (accessToken) {
      return {
        'Authorization': `Bearer ${accessToken}`,
        'Content-Type': 'application/json'
      };
    }
  }
  return { 'Content-Type': 'application/json' };
};

const loadUserData = async () => {
  loading.value = true;
  
  try {
    const baseUrl = getApiBase();
    
    // Load user bookings
    const bookingsResponse = await $fetch(`${baseUrl}/api/v1/bookings`, {
      headers: getAuthHeaders()
    });
    userBookings.value = bookingsResponse;
    
    // Load upcoming sessions for discovery
    const sessionsResult = await getSessions({ limit: 5, upcoming_only: true });
    if (sessionsResult.success) {
      upcomingSessions.value = sessionsResult.data.filter(s => !s.is_booked);
    }
    
    // Calculate stats
    const now = new Date();
    stats.totalBookings = userBookings.value.length;
    stats.upcomingBookings = userBookings.value.filter(b => 
      new Date(b.session_start_time) > now
    ).length;
    stats.completedSessions = userBookings.value.filter(b => 
      new Date(b.session_start_time) <= now
    ).length;
    
    // Calculate member since date (mock for now)
    stats.memberSince = new Date().toLocaleDateString('en-GB', { 
      year: 'numeric', 
      month: 'long' 
    });
    
  } catch (error) {
    console.error('Failed to load user data:', error);
    notifyError('Error', 'Failed to load dashboard data');
  }
  
  loading.value = false;
};

const cancelBooking = async (bookingId: string, sessionTitle: string) => {
  if (!confirm(`Cancel booking for "${sessionTitle}"?`)) return;
  
  try {
    const baseUrl = getApiBase();
    await $fetch(`${baseUrl}/api/v1/bookings/sessions/${bookingId}`, {
      method: 'DELETE',
      headers: getAuthHeaders()
    });
    
    success('Booking Cancelled', `Your booking for "${sessionTitle}" has been cancelled`);
    await loadUserData(); // Refresh data
  } catch (error) {
    notifyError('Error', 'Failed to cancel booking');
  }
};

const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return {
    date: date.toLocaleDateString('en-GB', { 
      weekday: 'short', 
      day: 'numeric',
      month: 'short',
      year: 'numeric'
    }),
    time: date.toLocaleTimeString('en-GB', { 
      hour: '2-digit', 
      minute: '2-digit' 
    })
  };
};

const getTierName = (tier: number) => {
  switch (tier) {
    case 0: return 'Non-Member';
    case 1: return 'Member';
    case 2: return 'Team Member';
    default: return 'Unknown';
  }
};

const handleLogout = async () => {
  if (confirm('Are you sure you want to logout?')) {
    await logout();
  }
};

onMounted(() => {
  loadUserData();
});
</script>

<template>
  <div class="dashboard-container">
    <!-- Header -->
    <header class="dashboard-header">
      <div class="header-content">
        <div class="user-info">
          <div class="avatar">{{ user?.name?.charAt(0) || 'U' }}</div>
          <div class="user-details">
            <h1 class="user-name">Welcome, {{ user?.name }}!</h1>
            <p class="user-meta">
              {{ getTierName(user?.tier || 0) }} • Member since {{ stats.memberSince }}
            </p>
          </div>
        </div>
        
        <div class="header-actions">
          <NuxtLink to="/sessions" class="action-btn browse-btn">
            🏸 Browse Sessions
          </NuxtLink>
          <NuxtLink to="/dash/profile" class="action-btn profile-btn">
            👤 Profile
          </NuxtLink>
          <button @click="handleLogout" class="action-btn logout-btn">
            🚪 Logout
          </button>
        </div>
      </div>
    </header>

    <main class="dashboard-main">
      <!-- Loading State -->
      <div v-if="loading" class="loading-section">
        <div class="loading-spinner"></div>
        <p>Loading your dashboard...</p>
      </div>

      <div v-else class="dashboard-content">
        <!-- Stats Overview -->
        <section class="stats-section">
          <h2 class="section-title">📊 Your Activity</h2>
          <div class="stats-grid">
            <div class="stat-card">
              <div class="stat-icon">🎯</div>
              <div class="stat-content">
                <div class="stat-number">{{ stats.totalBookings }}</div>
                <div class="stat-label">Total Bookings</div>
              </div>
            </div>
            <div class="stat-card">
              <div class="stat-icon">⏰</div>
              <div class="stat-content">
                <div class="stat-number">{{ stats.upcomingBookings }}</div>
                <div class="stat-label">Upcoming Sessions</div>
              </div>
            </div>
            <div class="stat-card">
              <div class="stat-icon">✅</div>
              <div class="stat-content">
                <div class="stat-number">{{ stats.completedSessions }}</div>
                <div class="stat-label">Completed Sessions</div>
              </div>
            </div>
            <div class="stat-card tier-card">
              <div class="stat-icon">🏆</div>
              <div class="stat-content">
                <div class="stat-number">Tier {{ user?.tier || 0 }}</div>
                <div class="stat-label">{{ getTierName(user?.tier || 0) }}</div>
              </div>
            </div>
          </div>
        </section>

        <!-- My Bookings -->
        <section class="bookings-section">
          <div class="section-header">
            <h2 class="section-title">📅 My Bookings</h2>
            <button @click="loadUserData" class="refresh-btn">
              🔄 Refresh
            </button>
          </div>
          
          <div v-if="userBookings.length === 0" class="empty-bookings">
            <div class="empty-icon">📋</div>
            <h3>No bookings yet</h3>
            <p>You haven't booked any training sessions yet. Browse available sessions to get started!</p>
            <NuxtLink to="/sessions" class="cta-btn">
              Browse Training Sessions
            </NuxtLink>
          </div>
          
          <div v-else class="bookings-grid">
            <div 
              v-for="booking in userBookings" 
              :key="booking.session_id"
              class="booking-card"
              :class="{ 
                'upcoming-booking': new Date(booking.session_start_time) > new Date(),
                'past-booking': new Date(booking.session_start_time) <= new Date()
              }"
            >
              <div class="booking-header">
                <h3 class="booking-title">{{ booking.session_title }}</h3>
                <div class="booking-status">
                  <span v-if="new Date(booking.session_start_time) > new Date()" 
                        class="status-badge upcoming">
                    Upcoming
                  </span>
                  <span v-else class="status-badge completed">
                    Completed
                  </span>
                </div>
              </div>
              
              <div class="booking-details">
                <div class="detail-row">
                  <span class="detail-icon">📅</span>
                  <span>{{ formatDate(booking.session_start_time).date }}</span>
                </div>
                <div class="detail-row">
                  <span class="detail-icon">🕐</span>
                  <span>{{ formatDate(booking.session_start_time).time }}</span>
                </div>
                <div class="detail-row">
                  <span class="detail-icon">🎫</span>
                  <span>Booked on {{ formatDate(booking.created_at).date }}</span>
                </div>
              </div>
              
              <div v-if="new Date(booking.session_start_time) > new Date()" 
                   class="booking-actions">
                <NuxtLink :to="`/sessions/${booking.session_id}`" class="view-btn">
                  View Details
                </NuxtLink>
                <button 
                  @click="cancelBooking(booking.session_id, booking.session_title)"
                  class="cancel-btn"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </section>

        <!-- Discover More Sessions -->
        <section v-if="upcomingSessions.length > 0" class="discover-section">
          <div class="section-header">
            <h2 class="section-title">🔍 Discover More Sessions</h2>
            <NuxtLink to="/sessions" class="view-all-link">
              View All →
            </NuxtLink>
          </div>
          
          <div class="discover-grid">
            <div 
              v-for="session in upcomingSessions.slice(0, 3)" 
              :key="session.id"
              class="discover-card"
            >
              <h4 class="discover-title">{{ session.title }}</h4>
              <p class="discover-description">{{ session.description }}</p>
              <div class="discover-meta">
                <span class="discover-date">{{ formatDate(session.start_time).date }}</span>
                <span class="discover-time">{{ formatDate(session.start_time).time }}</span>
              </div>
              <NuxtLink :to="`/sessions#session-${session.id}`" class="discover-btn">
                Learn More
              </NuxtLink>
            </div>
          </div>
        </section>

        <!-- Membership Info -->
        <section class="membership-section">
          <div class="membership-card">
            <div class="membership-content">
              <h3>🎯 Unlock More Benefits</h3>
              <p>
                {{ user?.tier === 0 ? 'Upgrade to a membership to access more training sessions and exclusive events.' :
                   user?.tier === 1 ? 'Consider team membership for access to competitive training and tournaments.' :
                   'You have full access to all club activities!' }}
              </p>
              <div class="membership-actions">
                <NuxtLink to="/membership" class="membership-btn">
                  {{ user?.tier === 0 ? 'View Membership Options' : 
                     user?.tier === 1 ? 'Upgrade to Team Member' : 
                     'Membership Details' }}
                </NuxtLink>
                <NuxtLink to="/about" class="about-btn">
                  About the Club
                </NuxtLink>
              </div>
            </div>
            <div class="membership-graphic">
              <div class="graphic-icon">🏸</div>
            </div>
          </div>
        </section>
      </div>
    </main>
  </div>
</template>

<style scoped>
.dashboard-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #f8fffe 0%, #f0f8ef 100%);
}

/* Header */
.dashboard-header {
  background: linear-gradient(135deg, var(--accent-colour) 0%, #2d4232 100%);
  color: white;
  padding: 2rem;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 2rem;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.avatar {
  width: 60px;
  height: 60px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  font-weight: 700;
  color: white;
  border: 2px solid rgba(255, 255, 255, 0.3);
}

.user-name {
  margin: 0;
  font-size: 1.8rem;
  font-weight: 700;
}

.user-meta {
  margin: 0;
  opacity: 0.8;
  font-size: 0.9rem;
}

.header-actions {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.action-btn {
  padding: 0.75rem 1.5rem;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: var(--radius-s);
  color: white;
  text-decoration: none;
  font-weight: 600;
  background: rgba(255, 255, 255, 0.1);
  transition: all 0.2s ease;
  cursor: pointer;
  font-size: 0.9rem;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: translateY(-1px);
}

.browse-btn:hover {
  background: #4caf50;
  border-color: #4caf50;
}

.profile-btn:hover {
  background: #2196f3;
  border-color: #2196f3;
}

.logout-btn {
  background: rgba(244, 67, 54, 0.2);
  border-color: #f44336;
}

.logout-btn:hover {
  background: #f44336;
  border-color: #f44336;
}

/* Main Content */
.dashboard-main {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

.loading-section {
  text-align: center;
  padding: 4rem 2rem;
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

.dashboard-content {
  display: flex;
  flex-direction: column;
  gap: 3rem;
}

/* Section Styling */
.section-title {
  font-size: 1.5rem;
  color: var(--dash-txt-colour);
  margin: 0;
  font-weight: 700;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.refresh-btn, .view-all-link {
  padding: 0.5rem 1rem;
  background: var(--accent-colour);
  color: white;
  border: none;
  border-radius: var(--radius-s);
  text-decoration: none;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.9rem;
}

.refresh-btn:hover, .view-all-link:hover {
  background: var(--btn-11-colour);
}

/* Stats Section */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
}

.stat-card {
  background: white;
  padding: 2rem;
  border-radius: var(--radius-m);
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  border: 1px solid var(--field-border-colour);
  display: flex;
  align-items: center;
  gap: 1rem;
  transition: all 0.2s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
}

.tier-card {
  background: linear-gradient(135deg, #fff3e0 0%, white 100%);
  border-color: #ff9800;
}

.stat-icon {
  font-size: 2rem;
  width: 60px;
  height: 60px;
  background: var(--bg-20-colour);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.stat-content {
  flex: 1;
}

.stat-number {
  font-size: 2rem;
  font-weight: 700;
  color: var(--dash-txt-colour);
  line-height: 1;
  margin-bottom: 0.25rem;
}

.stat-label {
  color: var(--grey-txt-colour);
  font-size: 0.9rem;
  font-weight: 500;
}

/* Bookings Section */
.empty-bookings {
  text-align: center;
  padding: 4rem 2rem;
  background: white;
  border-radius: var(--radius-m);
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.empty-bookings h3 {
  color: var(--dash-txt-colour);
  margin-bottom: 1rem;
}

.empty-bookings p {
  color: var(--grey-txt-colour);
  margin-bottom: 2rem;
  max-width: 400px;
  margin-left: auto;
  margin-right: auto;
}

.cta-btn {
  background: var(--accent-colour);
  color: white;
  padding: 1rem 2rem;
  border-radius: var(--radius-s);
  text-decoration: none;
  font-weight: 600;
  transition: all 0.2s ease;
}

.cta-btn:hover {
  background: var(--btn-11-colour);
  transform: translateY(-1px);
}

.bookings-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 1.5rem;
}

.booking-card {
  background: white;
  padding: 1.5rem;
  border-radius: var(--radius-m);
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  border: 1px solid var(--field-border-colour);
  transition: all 0.2s ease;
}

.booking-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
}

.upcoming-booking {
  border-left: 4px solid #4caf50;
}

.past-booking {
  opacity: 0.8;
  background: #f9f9f9;
}

.booking-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  margin-bottom: 1rem;
  gap: 1rem;
}

.booking-title {
  margin: 0;
  color: var(--dash-txt-colour);
  font-size: 1.1rem;
  line-height: 1.3;
}

.status-badge {
  padding: 0.25rem 0.75rem;
  border-radius: var(--radius-xs);
  font-size: 0.8rem;
  font-weight: 600;
  white-space: nowrap;
}

.status-badge.upcoming {
  background: #4caf50;
  color: white;
}

.status-badge.completed {
  background: #9e9e9e;
  color: white;
}

.booking-details {
  margin-bottom: 1rem;
}

.detail-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
  color: var(--dash-txt-colour);
}

.detail-icon {
  width: 16px;
  text-align: center;
}

.booking-actions {
  display: flex;
  gap: 0.75rem;
}

.view-btn, .cancel-btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: var(--radius-s);
  font-weight: 600;
  cursor: pointer;
  text-decoration: none;
  font-size: 0.9rem;
  transition: all 0.2s ease;
}

.view-btn {
  background: var(--accent-colour);
  color: white;
  flex: 1;
  text-align: center;
}

.view-btn:hover {
  background: var(--btn-11-colour);
}

.cancel-btn {
  background: #f44336;
  color: white;
}

.cancel-btn:hover {
  background: #d32f2f;
}

/* Discover Section */
.discover-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
}

.discover-card {
  background: white;
  padding: 1.5rem;
  border-radius: var(--radius-m);
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  border: 1px solid var(--field-border-colour);
  transition: all 0.2s ease;
}

.discover-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
}

.discover-title {
  margin: 0 0 0.5rem 0;
  color: var(--dash-txt-colour);
  font-size: 1.1rem;
}

.discover-description {
  color: var(--grey-txt-colour);
  font-size: 0.9rem;
  line-height: 1.4;
  margin-bottom: 1rem;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.discover-meta {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
  font-size: 0.9rem;
  color: var(--dash-txt-colour);
}

.discover-btn {
  background: var(--bg-20-colour);
  color: var(--accent-colour);
  padding: 0.5rem 1rem;
  border-radius: var(--radius-s);
  text-decoration: none;
  font-weight: 600;
  transition: all 0.2s ease;
  display: inline-block;
}

.discover-btn:hover {
  background: var(--accent-colour);
  color: white;
}

/* Membership Section */
.membership-card {
  background: linear-gradient(135deg, #e8f5e8 0%, white 100%);
  border: 2px solid var(--accent-colour);
  border-radius: var(--radius-m);
  padding: 2rem;
  display: flex;
  align-items: center;
  gap: 2rem;
}

.membership-content {
  flex: 1;
}

.membership-content h3 {
  color: var(--accent-colour);
  margin: 0 0 1rem 0;
  font-size: 1.3rem;
}

.membership-content p {
  color: var(--dash-txt-colour);
  margin-bottom: 1.5rem;
  line-height: 1.5;
}

.membership-actions {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.membership-btn, .about-btn {
  padding: 0.75rem 1.5rem;
  border-radius: var(--radius-s);
  text-decoration: none;
  font-weight: 600;
  transition: all 0.2s ease;
}

.membership-btn {
  background: var(--accent-colour);
  color: white;
}

.membership-btn:hover {
  background: var(--btn-11-colour);
}

.about-btn {
  background: white;
  color: var(--accent-colour);
  border: 2px solid var(--accent-colour);
}

.about-btn:hover {
  background: var(--accent-colour);
  color: white;
}

.membership-graphic {
  flex-shrink: 0;
}

.graphic-icon {
  font-size: 4rem;
  width: 100px;
  height: 100px;
  background: var(--accent-colour);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Responsive Design */
@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    text-align: center;
    gap: 1.5rem;
  }
  
  .header-actions {
    width: 100%;
    justify-content: center;
  }
  
  .dashboard-main {
    padding: 1rem;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .bookings-grid, .discover-grid {
    grid-template-columns: 1fr;
  }
  
  .membership-card {
    flex-direction: column;
    text-align: center;
  }
  
  .section-header {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }
}

@media (max-width: 480px) {
  .dashboard-header {
    padding: 1rem;
  }
  
  .user-name {
    font-size: 1.4rem;
  }
  
  .action-btn {
    padding: 0.5rem 1rem;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .stat-card {
    padding: 1.5rem;
  }
  
  .booking-card, .discover-card {
    padding: 1rem;
  }
  
  .membership-actions {
    flex-direction: column;
  }
}
</style>