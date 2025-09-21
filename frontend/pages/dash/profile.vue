<!-- frontend/pages/dash/profile.vue -->
<script setup lang="ts">
definePageMeta({
    layout: "dash",
    middleware: ["auth"],
});

const { user, logout } = useAuth();

const profile = reactive({
  first_name: user.value?.name?.split(' ')[0] || '',
  surname: user.value?.name?.split(' ').slice(1).join(' ') || '',
  shortcode: user.value?.shortcode || '',
  current_password: '',
  new_password: '',
  confirm_password: ''
});

const loading = ref(false);
const error = ref('');
const success = ref('');

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

const updateProfile = async () => {
  loading.value = true;
  error.value = '';
  success.value = '';

  try {
    const baseUrl = getApiBase();
    const updateData = {
      first_name: profile.first_name,
      surname: profile.surname,
    };

    const response = await $fetch(`${baseUrl}/api/v1/users/profile`, {
      method: 'PATCH',
      headers: getAuthHeaders(),
      body: updateData
    });

    success.value = 'Profile updated successfully!';
    
    setTimeout(() => {
      success.value = '';
    }, 3000);
    
  } catch (err: any) {
    error.value = err?.data?.message || 'Failed to update profile';
  }
  
  loading.value = false;
};

const updatePassword = async () => {
  if (profile.new_password !== profile.confirm_password) {
    error.value = 'New passwords do not match';
    return;
  }

  if (profile.new_password.length < 8) {
    error.value = 'Password must be at least 8 characters';
    return;
  }

  loading.value = true;
  error.value = '';
  success.value = '';

  try {
    const baseUrl = getApiBase();
    const updateData = {
      current_password: profile.current_password,
      new_password: profile.new_password,
    };

    const response = await $fetch(`${baseUrl}/api/v1/users/password`, {
      method: 'PATCH',
      headers: getAuthHeaders(),
      body: updateData
    });

    success.value = 'Password updated successfully!';
    
    profile.current_password = '';
    profile.new_password = '';
    profile.confirm_password = '';
    
    setTimeout(() => {
      success.value = '';
    }, 3000);
    
  } catch (err: any) {
    error.value = err?.data?.message || 'Failed to update password';
  }
  
  loading.value = false;
};

const deleteAccount = async () => {
  if (!confirm('Are you sure you want to delete your account? This action cannot be undone.')) {
    return;
  }

  const confirmation = prompt('Type "DELETE" to confirm account deletion:');
  if (confirmation !== 'DELETE') {
    return;
  }

  loading.value = true;
  error.value = '';

  try {
    const baseUrl = getApiBase();
    await $fetch(`${baseUrl}/api/v1/users/profile`, {
      method: 'DELETE',
      headers: getAuthHeaders()
    });

    alert('Account deleted successfully.');
    await logout();
    
  } catch (err: any) {
    error.value = err?.data?.message || 'Failed to delete account';
  }
  
  loading.value = false;
};

const getTierName = (tier: number) => {
  switch (tier) {
    case 0: return 'Non-Member';
    case 1: return 'Member';
    case 2: return 'Team Member';
    default: return 'Unknown';
  }
};
</script>

<template>
  <div class="page-container flex-column">
    <h1 class="title">Profile Settings</h1>

    <div class="profile-sections">
      <!-- Current Profile Info -->
      <div class="profile-section">
        <h3 class="section-title">Current Profile</h3>
        <div class="profile-info">
          <div class="info-item">
            <strong>Name:</strong>
            <span>{{ user?.name }}</span>
          </div>
          <div class="info-item">
            <strong>Shortcode:</strong>
            <span>{{ user?.shortcode }}</span>
          </div>
          <div class="info-item">
            <strong>Membership Tier:</strong>
            <span class="tier-badge" :class="`tier-${user?.tier}`">
              {{ getTierName(user?.tier || 0) }}
            </span>
          </div>
          <div class="info-item">
            <strong>Admin:</strong>
            <span>{{ user?.admin ? 'Yes' : 'No' }}</span>
          </div>
        </div>
      </div>

      <!-- Update Profile -->
      <div class="profile-section">
        <h3 class="section-title">Update Profile</h3>
        <form @submit.prevent="updateProfile" class="profile-form">
          <div class="form-row">
            <div class="form-group">
              <label class="field-label">First Name:</label>
              <input 
                v-model="profile.first_name"
                type="text" 
                class="field"
                required
                maxlength="30"
              >
            </div>
            <div class="form-group">
              <label class="field-label">Surname:</label>
              <input 
                v-model="profile.surname"
                type="text" 
                class="field"
                required
                maxlength="30"
              >
            </div>
          </div>
          
          <div class="form-group">
            <label class="field-label">Shortcode (read-only):</label>
            <input 
              :value="profile.shortcode"
              type="text" 
              class="field"
              readonly
              disabled
            >
          </div>

          <button 
            type="submit" 
            class="button primary-button"
            :disabled="loading"
          >
            {{ loading ? 'Updating...' : 'Update Profile' }}
          </button>
        </form>
      </div>

      <!-- Change Password -->
      <div class="profile-section">
        <h3 class="section-title">Change Password</h3>
        <form @submit.prevent="updatePassword" class="profile-form">
          <div class="form-group">
            <label class="field-label">Current Password:</label>
            <input 
              v-model="profile.current_password"
              type="password" 
              class="field"
              required
            >
          </div>

          <div class="form-group">
            <label class="field-label">New Password:</label>
            <input 
              v-model="profile.new_password"
              type="password" 
              class="field"
              required
              minlength="8"
            >
          </div>

          <div class="form-group">
            <label class="field-label">Confirm New Password:</label>
            <input 
              v-model="profile.confirm_password"
              type="password" 
              class="field"
              required
              minlength="8"
            >
          </div>

          <button 
            type="submit" 
            class="button primary-button"
            :disabled="loading"
          >
            {{ loading ? 'Updating...' : 'Change Password' }}
          </button>
        </form>
      </div>

      <!-- Danger Zone -->
      <div class="profile-section danger-section">
        <h3 class="section-title danger-title">Danger Zone</h3>
        <p class="danger-text">
          Permanently delete your account and all associated data. This action cannot be undone.
        </p>
        <button 
          @click="deleteAccount" 
          class="button danger-button"
          :disabled="loading"
        >
          Delete Account
        </button>
      </div>
    </div>

    <!-- Status Messages -->
    <div v-if="error" class="error-message">
      {{ error }}
    </div>
    
    <div v-if="success" class="success-message">
      {{ success }}
    </div>
  </div>
</template>

<style scoped>
@import url("~/assets/css/dash-page.css");
@import url("~/assets/css/profile.css");
</style>