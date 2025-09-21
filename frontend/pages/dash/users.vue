<!-- frontend/pages/dash/users.vue -->
<script setup lang="ts">
definePageMeta({
    layout: "dash",
    middleware: ["auth", "admin"],
});

interface User {
  id: string;
  first_name: string;
  surname: string;
  shortcode: string;
  cid: string;
  admin: boolean;
  tier: number;
  created_at: string;
  last_login?: string;
}

const users = ref<User[]>([]);
const loading = ref(true);
const error = ref('');
const searchQuery = ref('');
const selectedTier = ref<number | null>(null);

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

const loadUsers = async () => {
  loading.value = true;
  error.value = '';
  
  try {
    const baseUrl = getApiBase();
    const response = await $fetch<User[]>(`${baseUrl}/api/v1/admin/users`, {
      method: 'GET',
      headers: getAuthHeaders()
    });
    
    users.value = response.sort((a, b) => 
      new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
    );
  } catch (err: any) {
    error.value = err?.data?.message || 'Failed to load users';
  }
  
  loading.value = false;
};

const updateUserTier = async (userId: string, newTier: number) => {
  try {
    const baseUrl = getApiBase();
    await $fetch(`${baseUrl}/api/v1/admin/users/${userId}/tier`, {
      method: 'PATCH',
      headers: getAuthHeaders(),
      body: { tier: newTier }
    });
    
    // Update local state
    const user = users.value.find(u => u.id === userId);
    if (user) {
      user.tier = newTier;
    }
  } catch (err: any) {
    alert(`Failed to update user tier: ${err?.data?.message || 'Unknown error'}`);
  }
};

const toggleAdminStatus = async (userId: string, currentAdmin: boolean) => {
  if (!confirm(`Are you sure you want to ${currentAdmin ? 'remove admin' : 'grant admin'} privileges?`)) {
    return;
  }

  try {
    const baseUrl = getApiBase();
    await $fetch(`${baseUrl}/api/v1/admin/users/${userId}/admin`, {
      method: 'PATCH',
      headers: getAuthHeaders(),
      body: { admin: !currentAdmin }
    });
    
    // Update local state
    const user = users.value.find(u => u.id === userId);
    if (user) {
      user.admin = !currentAdmin;
    }
  } catch (err: any) {
    alert(`Failed to update admin status: ${err?.data?.message || 'Unknown error'}`);
  }
};

const deleteUser = async (userId: string, userName: string) => {
  if (!confirm(`Are you sure you want to delete user "${userName}"? This action cannot be undone.`)) {
    return;
  }

  const confirmation = prompt(`Type "DELETE" to confirm deletion of ${userName}:`);
  if (confirmation !== 'DELETE') {
    return;
  }

  try {
    const baseUrl = getApiBase();
    await $fetch(`${baseUrl}/api/v1/admin/users/${userId}`, {
      method: 'DELETE',
      headers: getAuthHeaders()
    });
    
    // Remove from local state
    users.value = users.value.filter(u => u.id !== userId);
  } catch (err: any) {
    alert(`Failed to delete user: ${err?.data?.message || 'Unknown error'}`);
  }
};

const filteredUsers = computed(() => {
  return users.value.filter(user => {
    const matchesSearch = !searchQuery.value || 
      user.first_name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      user.surname.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      user.shortcode.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      user.cid.includes(searchQuery.value);
    
    const matchesTier = selectedTier.value === null || user.tier === selectedTier.value;
    
    return matchesSearch && matchesTier;
  });
});

const formatDate = (dateString?: string) => {
  if (!dateString) return 'Never';
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

const stats = computed(() => ({
  total: users.value.length,
  admins: users.value.filter(u => u.admin).length,
  teamMembers: users.value.filter(u => u.tier === 2).length,
  members: users.value.filter(u => u.tier === 1).length,
  nonMembers: users.value.filter(u => u.tier === 0).length
}));

onMounted(() => {
  loadUsers();
});
</script>

<template>
  <div class="page-container flex-column">
    <div class="header-row flex-row">
      <h1 class="title">User Management</h1>
      <button @click="loadUsers" class="button secondary-button">
        Refresh
      </button>
    </div>

    <!-- Stats Overview -->
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-number">{{ stats.total }}</div>
        <div class="stat-label">Total Users</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">{{ stats.admins }}</div>
        <div class="stat-label">Admins</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">{{ stats.teamMembers }}</div>
        <div class="stat-label">Team Members</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">{{ stats.members }}</div>
        <div class="stat-label">Members</div>
      </div>
    </div>

    <!-- Filters -->
    <div class="filters-section">
      <div class="filter-group">
        <label class="field-label">Search:</label>
        <input 
          v-model="searchQuery"
          type="text" 
          class="field search-field"
          placeholder="Search by name, shortcode, or CID..."
        >
      </div>
      
      <div class="filter-group">
        <label class="field-label">Filter by Tier:</label>
        <select v-model="selectedTier" class="field">
          <option :value="null">All Tiers</option>
          <option :value="0">Non-Members</option>
          <option :value="1">Members</option>
          <option :value="2">Team Members</option>
        </select>
      </div>
    </div>

    <div v-if="loading" class="loading-container">
      <p class="text">Loading users...</p>
    </div>

    <div v-else-if="error" class="error-container">
      <p class="error-text">{{ error }}</p>
      <button @click="loadUsers" class="button secondary-button">
        Try Again
      </button>
    </div>

    <div v-else-if="filteredUsers.length === 0" class="empty-container">
      <p class="text">No users found.</p>
    </div>

    <div v-else class="users-table-container">
      <div class="table-info">
        <p class="text">Showing {{ filteredUsers.length }} of {{ users.length }} users</p>
      </div>
      
      <div class="users-table">
        <div class="table-header">
          <div class="header-cell">User</div>
          <div class="header-cell">Credentials</div>
          <div class="header-cell">Status</div>
          <div class="header-cell">Dates</div>
          <div class="header-cell">Actions</div>
        </div>

        <div 
          v-for="user in filteredUsers" 
          :key="user.id" 
          class="table-row"
        >
          <div class="cell user-info">
            <div class="user-name">{{ user.first_name }} {{ user.surname }}</div>
            <div class="user-shortcode">{{ user.shortcode }}</div>
          </div>

          <div class="cell credentials">
            <div class="cid">CID: {{ user.cid }}</div>
          </div>

          <div class="cell status">
            <div class="status-badges">
              <span class="tier-badge" :class="`tier-${user.tier}`">
                {{ getTierName(user.tier) }}
              </span>
              <span v-if="user.admin" class="admin-badge">
                Admin
              </span>
            </div>
          </div>

          <div class="cell dates">
            <div class="date-info">
              <small>Created: {{ formatDate(user.created_at) }}</small>
              <small>Last Login: {{ formatDate(user.last_login) }}</small>
            </div>
          </div>

          <div class="cell actions">
            <div class="action-buttons">
              <select 
                :value="user.tier"
                @change="updateUserTier(user.id, parseInt($event.target.value))"
                class="tier-select"
              >
                <option value="0">Non-Member</option>
                <option value="1">Member</option>
                <option value="2">Team Member</option>
              </select>
              
              <button 
                @click="toggleAdminStatus(user.id, user.admin)"
                class="button"
                :class="user.admin ? 'danger-button' : 'tertiary-button'"
              >
                {{ user.admin ? 'Remove Admin' : 'Make Admin' }}
              </button>
              
              <button 
                @click="deleteUser(user.id, `${user.first_name} ${user.surname}`)"
                class="button danger-button"
              >
                Delete
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@import url("~/assets/css/dash-page.css");
@import url("~/assets/css/users-management.css");
</style>