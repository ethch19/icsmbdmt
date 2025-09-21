<!-- frontend/pages/dash/analytics.vue -->
<script setup lang="ts">
definePageMeta({
    layout: "dash",
    middleware: ["auth", "admin"],
});

const { getSessions } = useSessions();
const { user } = useAuth();

const loading = ref(true);
const sessions = ref([]);
const users = ref([]);

const analytics = reactive({
  sessions: {
    total: 0,
    upcoming: 0,
    past: 0,
    thisMonth: 0,
    lastMonth: 0,
    byTier: { 0: 0, 1: 0, 2: 0 },
    byAuthor: {},
    avgBookings: 0,
    mostPopular: null
  },
  users: {
    total: 0,
    admins: 0,
    teamMembers: 0,
    members: 0,
    nonMembers: 0,
    newThisMonth: 0,
    activeThisMonth: 0
  },
  bookings: {
    total: 0,
    thisMonth: 0,
    lastMonth: 0,
    avgPerSession: 0,
    topUsers: []
  },
  trends: {
    sessionsOverTime: [],
    bookingsOverTime: [],
    userGrowth: []
  }
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

const loadAnalyticsData = async () => {
  loading.value = true;
  
  try {
    const baseUrl = getApiBase();
    
    // Load sessions data
    const sessionsResult = await getSessions({ limit: 1000 });
    if (sessionsResult.success) {
      sessions.value = sessionsResult.data;
    }
    
    // Load users data (admin only)
    if (user.value?.admin) {
      try {
        const usersResponse = await $fetch(`${baseUrl}/api/v1/admin/users`, {
          headers: getAuthHeaders()
        });
        users.value = usersResponse;
      } catch (err) {
        console.error('Failed to load users:', err);
      }
    }
    
    // Calculate analytics
    calculateAnalytics();
    
  } catch (error) {
    console.error('Failed to load analytics data:', error);
  }
  
  loading.value = false;
};

const calculateAnalytics = () => {
  const now = new Date();
  const thisMonth = new Date(now.getFullYear(), now.getMonth(), 1);
  const lastMonth = new Date(now.getFullYear(), now.getMonth() - 1, 1);
  
  // Session analytics
  analytics.sessions.total = sessions.value.length;
  analytics.sessions.upcoming = sessions.value.filter(s => new Date(s.start_time) > now).length;
  analytics.sessions.past = sessions.value.filter(s => new Date(s.start_time) <= now).length;
  
  analytics.sessions.thisMonth = sessions.value.filter(s => 
    new Date(s.created_at) >= thisMonth
  ).length;
  
  analytics.sessions.lastMonth = sessions.value.filter(s => {
    const createdAt = new Date(s.created_at);
    return createdAt >= lastMonth && createdAt < thisMonth;
  }).length;
  
  // Sessions by tier
  sessions.value.forEach(session => {
    analytics.sessions.byTier[session.tier] = (analytics.sessions.byTier[session.tier] || 0) + 1;
  });
  
  // Sessions by author
  sessions.value.forEach(session => {
    const author = session.author_name;
    analytics.sessions.byAuthor[author] = (analytics.sessions.byAuthor[author] || 0) + 1;
  });
  
  // Average bookings per session
  const totalBookings = sessions.value.reduce((sum, s) => sum + (s.current_bookings || 0), 0);
  analytics.sessions.avgBookings = sessions.value.length ? 
    Math.round((totalBookings / sessions.value.length) * 10) / 10 : 0;
  
  // Most popular session
  if (sessions.value.length > 0) {
    analytics.sessions.mostPopular = sessions.value.reduce((max, session) => 
      (session.current_bookings || 0) > (max.current_bookings || 0) ? session : max
    );
  }
  
  // User analytics
  if (users.value.length > 0) {
    analytics.users.total = users.value.length;
    analytics.users.admins = users.value.filter(u => u.admin).length;
    analytics.users.teamMembers = users.value.filter(u => u.tier === 2).length;
    analytics.users.members = users.value.filter(u => u.tier === 1).length;
    analytics.users.nonMembers = users.value.filter(u => u.tier === 0).length;
    
    analytics.users.newThisMonth = users.value.filter(u => 
      new Date(u.created_at) >= thisMonth
    ).length;
    
    analytics.users.activeThisMonth = users.value.filter(u => 
      u.last_login && new Date(u.last_login) >= thisMonth
    ).length;
  }
  
  // Booking analytics
  analytics.bookings.total = totalBookings;
  analytics.bookings.avgPerSession = analytics.sessions.avgBookings;
  
  // Calculate trends
  calculateTrends();
};

const calculateTrends = () => {
  // Generate last 6 months of data
  const months = [];
  for (let i = 5; i >= 0; i--) {
    const date = new Date();
    date.setMonth(date.getMonth() - i);
    months.push({
      month: date.toLocaleDateString('en-US', { month: 'short', year: 'numeric' }),
      date: new Date(date.getFullYear(), date.getMonth(), 1)
    });
  }
  
  // Sessions over time
  analytics.trends.sessionsOverTime = months.map(({ month, date }) => {
    const nextMonth = new Date(date.getFullYear(), date.getMonth() + 1, 1);
    const count = sessions.value.filter(s => {
      const createdAt = new Date(s.created_at);
      return createdAt >= date && createdAt < nextMonth;
    }).length;
    
    return { month, sessions: count };
  });
  
  // Bookings over time
  analytics.trends.bookingsOverTime = months.map(({ month, date }) => {
    const nextMonth = new Date(date.getFullYear(), date.getMonth() + 1, 1);
    const sessionsInMonth = sessions.value.filter(s => {
      const createdAt = new Date(s.created_at);
      return createdAt >= date && createdAt < nextMonth;
    });
    
    const bookings = sessionsInMonth.reduce((sum, s) => sum + (s.current_bookings || 0), 0);
    return { month, bookings };
  });
  
  // User growth
  if (users.value.length > 0) {
    analytics.trends.userGrowth = months.map(({ month, date }) => {
      const count = users.value.filter(u => new Date(u.created_at) <= date).length;
      return { month, users: count };
    });
  }
};

const formatPercentChange = (current: number, previous: number) => {
  if (previous === 0) return current > 0 ? '+∞%' : '0%';
  const change = ((current - previous) / previous) * 100;
  return `${change >= 0 ? '+' : ''}${Math.round(change)}%`;
};

const getTopAuthors = computed(() => {
  return Object.entries(analytics.sessions.byAuthor)
    .sort(([,a], [,b]) => b - a)
    .slice(0, 5)
    .map(([name, count]) => ({ name, count }));
});

onMounted(() => {
  loadAnalyticsData();
});
</script>

<template>
  <div class="page-container flex-column">
    <div class="header-row flex-row">
      <h1 class="title">Analytics Dashboard</h1>
      <button @click="loadAnalyticsData" class="button secondary-button">
        Refresh Data
      </button>
    </div>

    <div v-if="loading" class="loading-container">
      <p class="text">Loading analytics...</p>
    </div>

    <div v-else class="analytics-dashboard">
      <!-- Key Metrics -->
      <div class="metrics-section">
        <h2 class="section-title">📊 Key Metrics</h2>
        <div class="metrics-grid">
          <div class="metric-card">
            <div class="metric-header">
              <span class="metric-title">Total Sessions</span>
              <span class="metric-trend" :class="{ 
                positive: analytics.sessions.thisMonth > analytics.sessions.lastMonth,
                negative: analytics.sessions.thisMonth < analytics.sessions.lastMonth
              }">
                {{ formatPercentChange(analytics.sessions.thisMonth, analytics.sessions.lastMonth) }}
              </span>
            </div>
            <div class="metric-value">{{ analytics.sessions.total }}</div>
            <div class="metric-subtitle">
              {{ analytics.sessions.thisMonth }} this month
            </div>
          </div>

          <div class="metric-card">
            <div class="metric-header">
              <span class="metric-title">Total Users</span>
              <span class="metric-trend positive">
                +{{ analytics.users.newThisMonth }}
              </span>
            </div>
            <div class="metric-value">{{ analytics.users.total }}</div>
            <div class="metric-subtitle">
              {{ analytics.users.activeThisMonth }} active this month
            </div>
          </div>

          <div class="metric-card">
            <div class="metric-header">
              <span class="metric-title">Total Bookings</span>
              <span class="metric-trend">{{ analytics.sessions.avgBookings }} avg/session</span>
            </div>
            <div class="metric-value">{{ analytics.bookings.total }}</div>
            <div class="metric-subtitle">
              Across {{ analytics.sessions.total }} sessions
            </div>
          </div>

          <div class="metric-card">
            <div class="metric-header">
              <span class="metric-title">Upcoming Sessions</span>
              <span class="metric-trend">{{ Math.round((analytics.sessions.upcoming / analytics.sessions.total) * 100) || 0 }}% of total</span>
            </div>
            <div class="metric-value">{{ analytics.sessions.upcoming }}</div>
            <div class="metric-subtitle">
              {{ analytics.sessions.past }} completed
            </div>
          </div>
        </div>
      </div>

      <!-- Session Breakdown -->
      <div class="breakdown-section">
        <h2 class="section-title">🏸 Session Breakdown</h2>
        <div class="breakdown-grid">
          <div class="breakdown-card">
            <h3 class="breakdown-title">By Membership Tier</h3>
            <div class="tier-stats">
              <div class="tier-stat">
                <span class="tier-label">Non-Members</span>
                <span class="tier-value">{{ analytics.sessions.byTier[0] || 0 }}</span>
                <div class="tier-bar">
                  <div class="tier-fill tier-0" :style="{ width: `${(analytics.sessions.byTier[0] / analytics.sessions.total) * 100}%` }"></div>
                </div>
              </div>
              <div class="tier-stat">
                <span class="tier-label">Members</span>
                <span class="tier-value">{{ analytics.sessions.byTier[1] || 0 }}</span>
                <div class="tier-bar">
                  <div class="tier-fill tier-1" :style="{ width: `${(analytics.sessions.byTier[1] / analytics.sessions.total) * 100}%` }"></div>
                </div>
              </div>
              <div class="tier-stat">
                <span class="tier-label">Team Members</span>
                <span class="tier-value">{{ analytics.sessions.byTier[2] || 0 }}</span>
                <div class="tier-bar">
                  <div class="tier-fill tier-2" :style="{ width: `${(analytics.sessions.byTier[2] / analytics.sessions.total) * 100}%` }"></div>
                </div>
              </div>
            </div>
          </div>

          <div class="breakdown-card">
            <h3 class="breakdown-title">Top Session Creators</h3>
            <div class="author-stats">
              <div v-for="author in getTopAuthors" :key="author.name" class="author-stat">
                <span class="author-name">{{ author.name }}</span>
                <span class="author-count">{{ author.count }} sessions</span>
              </div>
              <div v-if="getTopAuthors.length === 0" class="no-data">
                No data available
              </div>
            </div>
          </div>

          <div v-if="analytics.sessions.mostPopular" class="breakdown-card highlight-card">
            <h3 class="breakdown-title">🏆 Most Popular Session</h3>
            <div class="popular-session">
              <div class="popular-title">{{ analytics.sessions.mostPopular.title }}</div>
              <div class="popular-stats">
                <span>{{ analytics.sessions.mostPopular.current_bookings }} bookings</span>
                <span>•</span>
                <span>{{ analytics.sessions.mostPopular.author_name }}</span>
              </div>
              <div class="popular-date">
                {{ new Date(analytics.sessions.mostPopular.start_time).toLocaleDateString() }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- User Statistics -->
      <div v-if="user?.admin && users.length > 0" class="user-stats-section">
        <h2 class="section-title">👥 User Statistics</h2>
        <div class="user-stats-grid">
          <div class="user-stat-card">
            <div class="user-stat-icon">👑</div>
            <div class="user-stat-value">{{ analytics.users.admins }}</div>
            <div class="user-stat-label">Admins</div>
          </div>
          <div class="user-stat-card">
            <div class="user-stat-icon">🏸</div>
            <div class="user-stat-value">{{ analytics.users.teamMembers }}</div>
            <div class="user-stat-label">Team Members</div>
          </div>
          <div class="user-stat-card">
            <div class="user-stat-icon">🎯</div>
            <div class="user-stat-value">{{ analytics.users.members }}</div>
            <div class="user-stat-label">Members</div>
          </div>
          <div class="user-stat-card">
            <div class="user-stat-icon">👤</div>
            <div class="user-stat-value">{{ analytics.users.nonMembers }}</div>
            <div class="user-stat-label">Non-Members</div>
          </div>
        </div>
      </div>

      <!-- Trends -->
      <div class="trends-section">
        <h2 class="section-title">📈 Trends (Last 6 Months)</h2>
        <div class="trends-grid">
          <div class="trend-card">
            <h3 class="trend-title">Sessions Created</h3>
            <div class="trend-chart">
              <div class="chart-bars">
                <div 
                  v-for="(data, index) in analytics.trends.sessionsOverTime" 
                  :key="index"
                  class="chart-bar"
                >
                  <div 
                    class="bar-fill sessions-bar"
                    :style="{ 
                      height: `${Math.max(5, (data.sessions / Math.max(...analytics.trends.sessionsOverTime.map(d => d.sessions))) * 100)}%` 
                    }"
                  ></div>
                  <div class="bar-label">{{ data.month.split(' ')[0] }}</div>
                  <div class="bar-value">{{ data.sessions }}</div>
                </div>
              </div>
            </div>
          </div>

          <div class="trend-card">
            <h3 class="trend-title">Total Bookings</h3>
            <div class="trend-chart">
              <div class="chart-bars">
                <div 
                  v-for="(data, index) in analytics.trends.bookingsOverTime" 
                  :key="index"
                  class="chart-bar"
                >
                  <div 
                    class="bar-fill bookings-bar"
                    :style="{ 
                      height: `${Math.max(5, (data.bookings / Math.max(...analytics.trends.bookingsOverTime.map(d => d.bookings))) * 100)}%` 
                    }"
                  ></div>
                  <div class="bar-label">{{ data.month.split(' ')[0] }}</div>
                  <div class="bar-value">{{ data.bookings }}</div>
                </div>
              </div>
            </div>
          </div>

          <div v-if="analytics.trends.userGrowth.length > 0" class="trend-card">
            <h3 class="trend-title">User Growth</h3>
            <div class="trend-chart">
              <div class="chart-bars">
                <div 
                  v-for="(data, index) in analytics.trends.userGrowth" 
                  :key="index"
                  class="chart-bar"
                >
                  <div 
                    class="bar-fill users-bar"
                    :style="{ 
                      height: `${Math.max(5, (data.users / Math.max(...analytics.trends.userGrowth.map(d => d.users))) * 100)}%` 
                    }"
                  ></div>
                  <div class="bar-label">{{ data.month.split(' ')[0] }}</div>
                  <div class="bar-value">{{ data.users }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Quick Actions -->
      <div class="actions-section">
        <h2 class="section-title">⚡ Quick Actions</h2>
        <div class="actions-grid">
          <NuxtLink to="/dash/sessions/create" class="action-card">
            <div class="action-icon">➕</div>
            <div class="action-title">Create Session</div>
            <div class="action-subtitle">Add a new training session</div>
          </NuxtLink>
          
          <NuxtLink to="/dash/users" class="action-card" v-if="user?.admin">
            <div class="action-icon">👥</div>
            <div class="action-title">Manage Users</div>
            <div class="action-subtitle">View and edit user accounts</div>
          </NuxtLink>
          
          <NuxtLink to="/dash/sessions" class="action-card">
            <div class="action-icon">📅</div>
            <div class="action-title">View Sessions</div>
            <div class="action-subtitle">Browse all sessions</div>
          </NuxtLink>
          
          <button @click="loadAnalyticsData" class="action-card">
            <div class="action-icon">🔄</div>
            <div class="action-title">Refresh Data</div>
            <div class="action-subtitle">Update analytics</div>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@import url("~/assets/css/dash-page.css");
@import url("~/assets/css/analytics.css");
</style>