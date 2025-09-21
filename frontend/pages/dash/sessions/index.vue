<!-- frontend/pages/dash/sessions/index.vue -->
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

// Filtering and search
const searchQuery = ref('');
const selectedTier = ref<number | null>(null);
const selectedAuthor = ref<string | null>(null);
const dateFilter = ref<string>('all'); // 'all', 'upcoming', 'past', 'today', 'this_week'
const sortBy = ref<string>('start_time'); // 'start_time', 'created_at', 'title', 'bookings'
const sortOrder = ref<string>('desc'); // 'asc', 'desc'

const loadSessions = async () => {
  loading.value = true;
  error.value = '';
  
  try {
    const result = await getSessions({ limit: 100, upcoming_only: false });
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

const handleDelete = async (id: string, title: string) => {
  if (!confirm(`Are you sure you want to delete "${title}"?`)) return;
  
  const result = await deleteSession(id, title);
  if (result.success) {
    await loadSessions(); // Refresh list
  }
};

// Computed properties for filtering and sorting
const uniqueAuthors = computed(() => {
  const authors = [...new Set(sessions.value.map(s => s.author_name))];
  return authors.sort();
});

const filteredAndSortedSessions = computed(() => {
  let filtered = sessions.value.filter(session => {
    // Search query filter
    const matchesSearch = !searchQuery.value || 
      session.title.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      session.description.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      session.location.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      session.author_name.toLowerCase().includes(searchQuery.value.toLowerCase());
    
    // Tier filter
    const matchesTier = selectedTier.value === null || session.tier === selectedTier.value;
    
    // Author filter
    const matchesAuthor = !selectedAuthor.value || session.author_name === selectedAuthor.value;
    
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
      case 'past':
        matchesDate = sessionStart < now;
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
    
    return matchesSearch && matchesTier && matchesAuthor && matchesDate;
  });
  
  // Sort the filtered results
  return filtered.sort((a, b) => {
    let aValue, bValue;
    
    switch (sortBy.value) {
      case 'title':
        aValue = a.title.toLowerCase();
        bValue = b.title.toLowerCase();
        break;
      case 'created_at':
        aValue = new Date(a.created_at);
        bValue = new Date(b.created_at);
        break;
      case 'bookings':
        aValue = a.current_bookings;
        bValue = b.current_bookings;
        break;
      case 'start_time':
      default:
        aValue = new Date(a.start_time);
        bValue = new Date(b.start_time);
        break;
    }
    
    if (aValue < bValue) return sortOrder.value === 'asc' ? -1 : 1;
    if (aValue > bValue) return sortOrder.value === 'asc' ? 1 : -1;
    return 0;
  });
});

const sessionStats = computed(() => ({
  total: sessions.value.length,
  upcoming: sessions.value.filter(s => new Date(s.start_time) > new Date()).length,
  past: sessions.value.filter(s => new Date(s.start_time) <= new Date()).length,
  totalBookings: sessions.value.reduce((sum, s) => sum + s.current_bookings, 0),
  mySessions: sessions.value.filter(s => s.author_id === user.value?.id).length
}));

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

const isUpcoming = (dateString: string) => {
  return new Date(dateString) > new Date();
};

const clearFilters = () => {
  searchQuery.value = '';
  selectedTier.value = null;
  selectedAuthor.value = null;
  dateFilter.value = 'all';
  sortBy.value = 'start_time';
  sortOrder.value = 'desc';
};

onMounted(() => {
  loadSessions();
});
</script>

<template>
  <div class="page-container flex-column">
    <div class="header-row flex-row">
      <h1 class="title">Sessions</h1>
      <div class="header-actions flex-row">
        <button @click="clearFilters" class="button tertiary-button">
          Clear Filters
        </button>
        <button @click="loadSessions" class="button secondary-button">
          Refresh
        </button>
        <NuxtLink to="/dash/sessions/create" class="button primary-button">
          Create New Session
        </NuxtLink>
      </div>
    </div>

    <!-- Stats Overview -->
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-number">{{ sessionStats.total }}</div>
        <div class="stat-label">Total Sessions</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">{{ sessionStats.upcoming }}</div>
        <div class="stat-label">Upcoming</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">{{ sessionStats.totalBookings }}</div>
        <div class="stat-label">Total Bookings</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">{{ sessionStats.mySessions }}</div>
        <div class="stat-label">My Sessions</div>
      </div>
    </div>

    <!-- Advanced Filters -->
    <div class="filters-section">
      <div class="filters-row">
        <div class="filter-group">
          <label class="field-label">Search:</label>
          <input 
            v-model="searchQuery"
            type="text" 
            class="field search-field"
            placeholder="Search sessions..."
          >
        </div>
        
        <div class="filter-group">
          <label class="field-label">Date Filter:</label>
          <select v-model="dateFilter" class="field">
            <option value="all">All Sessions</option>
            <option value="upcoming">Upcoming Only</option>
            <option value="past">Past Sessions</option>
            <option value="today">Today</option>
            <option value="this_week">This Week</option>
          </select>
        </div>
        
        <div class="filter-group">
          <label class="field-label">Tier:</label>
          <select v-model="selectedTier" class="field">
            <option :value="null">All Tiers</option>
            <option :value="0">Non-Members</option>
            <option :value="1">Members</option>
            <option :value="2">Team Members</option>
          </select>
        </div>
        
        <div class="filter-group">
          <label class="field-label">Author:</label>
          <select v-model="selectedAuthor" class="field">
            <option :value="null">All Authors</option>
            <option v-for="author in uniqueAuthors" :key="author" :value="author">
              {{ author }}
            </option>
          </select>
        </div>
      </div>
      
      <div class="sort-controls">
        <div class="filter-group">
          <label class="field-label">Sort By:</label>
          <select v-model="sortBy" class="field">
            <option value="start_time">Start Time</option>
            <option value="created_at">Created Date</option>
            <option value="title">Title</option>
            <option value="bookings">Bookings</option>
          </select>
        </div>
        
        <div class="filter-group">
          <label class="field-label">Order:</label>
          <select v-model="sortOrder" class="field">
            <option value="desc">Descending</option>
            <option value="asc">Ascending</option>
          </select>
        </div>
        
        <div class="results-info">
          <span class="results-count">
            {{ filteredAndSortedSessions.length }} of {{ sessions.length }} sessions
          </span>
        </div>
      </div>
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

    <div v-else-if="filteredAndSortedSessions.length === 0" class="empty-container">
      <p class="text">
        {{ sessions.length === 0 ? 'No sessions found.' : 'No sessions match your filters.' }}
      </p>
      <div class="empty-actions">
        <NuxtLink to="/dash/sessions/create" class="button primary-button">
          Create Your First Session
        </NuxtLink>
        <button v-if="sessions.length > 0" @click="clearFilters" class="button secondary-button">
          Clear Filters
        </button>
      </div>
    </div>

    <div v-else class="sessions-grid">
      <div 
        v-for="session in filteredAndSortedSessions" 
        :key="session.id" 
        class="session-card"
        :class="{ 'upcoming-session': isUpcoming(session.start_time), 'past-session': !isUpcoming(session.start_time) }"
      >
        <div class="session-header">
          <h3 class="session-title">{{ session.title }}</h3>
          <div class="session-badges">
            <span class="tier-badge" :class="`tier-${session.tier}`">
              {{ getTierName(session.tier) }}
            </span>
            <span v-if="!isUpcoming(session.start_time)" class="status-badge past">
              Past
            </span>
            <span v-else class="status-badge upcoming">
              Upcoming
            </span>
          </div>
        </div>

        <div class="session-details">
          <p class="session-description">{{ session.description }}</p>
          <div class="session-info">
            <div class="info-item">
              <strong>📍 Location:</strong> {{ session.location }}
            </div>
            <div class="info-item">
              <strong>🕐 Start:</strong> {{ formatDate(session.start_time) }}
            </div>
            <div class="info-item">
              <strong>🕐 End:</strong> {{ formatDate(session.end_time) }}
            </div>
            <div class="info-item">
              <strong>👥 Bookings:</strong> 
              {{ session.current_bookings }}
              <span v-if="session.user_limit">/ {{ session.user_limit }}</span>
              <span v-else>/ ∞</span>
            </div>
            <div class="info-item">
              <strong>👤 Author:</strong> {{ session.author_name }}
            </div>
          </div>
        </div>

        <div class="session-actions">
          <NuxtLink 
            :to="`/dash/sessions/${session.id}`" 
            class="button secondary-button"
          >
            View Details
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
@import url("~/assets/css/enhanced-sessions.css");
</style>