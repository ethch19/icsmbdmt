<script setup lang="ts">
definePageMeta({
    title: 'Login - ICSM Badminton'
});

const { login } = useAuth();

const form = reactive({
    shortcode: '',
    password: '',
    keep_login: false
});

const error = ref('');
const loading = ref(false);

const handleLogin = async () => {
    loading.value = true;
    error.value = '';

    const result = await login(form);
    
    if (result.success) {
        await navigateTo('/dash');
    } else {
        error.value = result.error || 'Login failed';
    }
    
    loading.value = false;
};
</script>

<template>
    <main class="page-container flex-column">
        <div class="page-header">
            <h1 class="title text-center">Welcome Back</h1>
            <p class="page-subtitle text-center">Sign in to your ICSM Badminton account</p>
        </div>
        
        <div class="login-wrapper">
            <div class="login-card">
                <form @submit.prevent="handleLogin" class="login-form">
                    <div class="field-group">
                        <label class="field-label">Shortcode</label>
                        <input 
                            v-model="form.shortcode" 
                            class="field-input" 
                            type="text" 
                            placeholder="jd123"
                            required
                        >
                    </div>

                    <div class="field-group">
                        <label class="field-label">Password</label>
                        <input 
                            v-model="form.password" 
                            class="field-input" 
                            type="password" 
                            placeholder="Enter your password"
                            required
                        >
                    </div>

                    <div class="checkbox-group">
                        <input 
                            v-model="form.keep_login" 
                            type="checkbox" 
                            id="keep_login"
                            class="checkbox-input"
                        >
                        <label for="keep_login" class="checkbox-label">Keep me logged in</label>
                    </div>

                    <div v-if="error" class="error-card">
                        {{ error }}
                    </div>

                    <div class="form-actions">
                        <button 
                            type="submit" 
                            class="primary-btn"
                            :disabled="loading"
                        >
                            {{ loading ? 'Signing in...' : 'Sign In' }}
                        </button>
                        
                        <div class="auth-links">
                            <span class="auth-text">Don't have an account?</span>
                            <NuxtLink to="/register" class="auth-link">Create one here</NuxtLink>
                        </div>
                    </div>
                </form>
            </div>
            
            <!-- Side Info -->
            <div class="info-section">
                <div class="info-card">
                    <h3 class="info-title">New to ICSM Badminton?</h3>
                    <p class="info-text">Join our vibrant community of badminton enthusiasts at Imperial College School of Medicine.</p>
                    
                    <div class="info-features">
                        <div class="feature-item">
                            <span class="feature-icon">🏸</span>
                            <span>Regular training sessions</span>
                        </div>
                        <div class="feature-item">
                            <span class="feature-icon">🏆</span>
                            <span>Competitive tournaments</span>
                        </div>
                        <div class="feature-item">
                            <span class="feature-icon">🎉</span>
                            <span>Social events & networking</span>
                        </div>
                    </div>
                    
                    <NuxtLink to="/register" class="info-btn">Get Started</NuxtLink>
                </div>
            </div>
        </div>
    </main>
</template>

<style scoped>
@import url("~/assets/css/login.css");
</style>