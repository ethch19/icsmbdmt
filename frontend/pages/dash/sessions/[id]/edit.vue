<script setup lang="ts">
definePageMeta({
    layout: "dash",
    middleware: ["auth", "admin"],
});

const route = useRoute();
const router = useRouter();
const { getSession, updateSession } = useSessions();
const { user } = useAuth();

const sessionId = route.params.id as string;
const session = ref(null);
const loading = ref(true);
const saving = ref(false);
const error = ref('');

const form = reactive({
  title: '',
  description: '',
  location: '',
  tier: 1,
  start_time: '',
  end_time: '',
  user_limit: null as number | null,
  recurrence: false,
  recurrence_end: ''
});

const loadSession = async () => {
  loading.value = true;
  error.value = '';
  
  try {
    const result = await getSession(sessionId);
    if (result.success) {
      session.value = result.data;
      
      // Check permissions
      if (session.value.author_id !== user.value?.id && !user.value?.admin) {
        throw createError({
          statusCode: 403,
          statusMessage: 'Access Denied'
        });
      }
      
      // Populate form
      form.title = session.value.title;
      form.description = session.value.description;
      form.location = session.value.location;
      form.tier = session.value.tier;
      form.start_time = new Date(session.value.start_time).toISOString().slice(0, 16);
      form.end_time = new Date(session.value.end_time).toISOString().slice(0, 16);
      form.user_limit = session.value.user_limit;
      form.recurrence = !!session.value.recurrence;
      if (session.value.recurrence_end) {
        form.recurrence_end = new Date(session.value.recurrence_end).toISOString().slice(0, 16);
      }
    } else {
      error.value = result.error || 'Session not found';
    }
  } catch (err) {
    error.value = 'Failed to load session';
  }
  
  loading.value = false;
};

const handleSubmit = async () => {
  saving.value = true;
  error.value = '';

  const sessionData = {
    title: form.title,
    description: form.description,
    location: form.location,
    tier: form.tier,
    start_time: new Date(form.start_time).toISOString(),
    end_time: new Date(form.end_time).toISOString(),
    user_limit: form.user_limit || undefined,
  };

  if (form.recurrence && form.recurrence_end) {
    sessionData.recurrence = {
      months: 0,
      days: 7,
      microseconds: 0
    };
    sessionData.recurrence_end = new Date(form.recurrence_end).toISOString();
  }

  const result = await updateSession(sessionId, sessionData);
  
  if (result.success) {
    await router.push(`/dash/sessions/${sessionId}`);
  } else {
    error.value = result.error || 'Failed to update session';
  }
  
  saving.value = false;
};

onMounted(() => {
  loadSession();
});
</script>

<template>
  <div class="page-container flex-column">
    <div class="header-row flex-row">
      <h1 class="title">Edit Session</h1>
      <div class="header-actions flex-row">
        <NuxtLink :to="`/dash/sessions/${sessionId}`" class="button secondary-button">
          Cancel
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

    <form v-else @submit.prevent="handleSubmit" class="session-form">
      <div class="form-section">
        <h3 class="section-title">Basic Information</h3>
        
        <div class="form-group">
          <label class="field-label">Title:</label>
          <input 
            v-model="form.title"
            type="text" 
            class="field"
            placeholder="e.g. Team Training Session"
            required
            maxlength="50"
          >
        </div>

        <div class="form-group">
          <label class="field-label">Description:</label>
          <textarea 
            v-model="form.description"
            class="field textarea-field"
            placeholder="Describe what this session is about..."
            required
            maxlength="500"
            rows="4"
          ></textarea>
        </div>

        <div class="form-group">
          <label class="field-label">Location:</label>
          <input 
            v-model="form.location"
            type="text" 
            class="field"
            placeholder="e.g. Sports Hall, Imperial College"
            required
            maxlength="100"
          >
        </div>

        <div class="form-group">
          <label class="field-label">Required Membership Tier:</label>
          <select v-model="form.tier" class="field">
            <option value="0">Non-Member</option>
            <option value="1">Member</option>
            <option value="2">Team Member</option>
          </select>
        </div>
      </div>

      <div class="form-section">
        <h3 class="section-title">Schedule</h3>
        
        <div class="form-row">
          <div class="form-group">
            <label class="field-label">Start Time:</label>
            <input 
              v-model="form.start_time"
              type="datetime-local" 
              class="field"
              required
            >
          </div>

          <div class="form-group">
            <label class="field-label">End Time:</label>
            <input 
              v-model="form.end_time"
              type="datetime-local" 
              class="field"
              required
            >
          </div>
        </div>

        <div class="form-group">
          <label class="checkbox-label">
            <input 
              v-model="form.recurrence" 
              type="checkbox" 
              class="checkbox"
            >
            <span>Recurring weekly session</span>
          </label>
        </div>

        <div v-if="form.recurrence" class="form-group">
          <label class="field-label">Recurrence End Date:</label>
          <input 
            v-model="form.recurrence_end"
            type="datetime-local" 
            class="field"
            required
          >
        </div>
      </div>

      <div class="form-section">
        <h3 class="section-title">Capacity</h3>
        
        <div class="form-group">
          <label class="field-label">User Limit (optional):</label>
          <input 
            v-model="form.user_limit"
            type="number" 
            class="field"
            placeholder="Leave empty for unlimited"
            min="1"
            max="100"
          >
          <small class="field-hint">Maximum number of people who can book this session</small>
        </div>
      </div>

      <div v-if="error" class="error-message">
        {{ error }}
      </div>

      <div class="form-actions">
        <button 
          type="submit" 
          class="button primary-button"
          :disabled="saving"
        >
          {{ saving ? 'Saving...' : 'Save Changes' }}
        </button>
        <NuxtLink :to="`/dash/sessions/${sessionId}`" class="button secondary-button">
          Cancel
        </NuxtLink>
      </div>
    </form>
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

.session-form {
  max-width: 800px;
  margin: 0 auto;
}

.form-section {
  background-color: var(--dash-bg-box-colour);
  padding: 2rem;
  border-radius: var(--radius-m);
  margin-bottom: 2rem;
}

.section-title {
  margin: 0 0 1.5rem 0;
  color: var(--dash-txt-colour);
  font-size: 1.3rem;
  border-bottom: 2px solid var(--bg-20-colour);
  padding-bottom: 0.5rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.field-label {
  display: block;
  margin-bottom: 0.5rem;
  color: var(--dash-txt-colour);
  font-weight: 600;
}

.field {
  width: 100%;
  box-sizing: border-box;
}

.textarea-field {
  resize: vertical;
  min-height: 100px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  color: var(--dash-txt-colour);
}

.checkbox {
  width: auto;
  height: auto;
}

.field-hint {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.9rem;
  color: var(--grey-txt-colour);
}

.form-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  padding: 2rem 0;
}

.error-message {
  background-color: #f8d7da;
  color: #721c24;
  padding: 1rem;
  border-radius: var(--radius-s);
  margin-bottom: 2rem;
  border: 1px solid #f5c6cb;
}

@media (max-width: 768px) {
  .header-row {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }
  
  .form-row {
    grid-template-columns: 1fr;
  }
  
  .form-actions {
    flex-direction: column;
  }
}
</style>