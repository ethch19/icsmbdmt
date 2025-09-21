<!-- frontend/pages/login.vue - Fixed to prevent infinite redirects -->
<script setup lang="ts">
definePageMeta({
    title: 'Login - ICSM Badminton',
    middleware: [] // Remove all middleware from login page
});

const { login, isLoggedIn, user } = useAuth();
const route = useRoute();
const router = useRouter();

const form = reactive({
    shortcode: '',
    password: '',
    keep_login: false
});

const error = ref('');
const loading = ref(false);

// Don't redirect if already on login page - let user choose where to go
// onMounted(() => {
//     if (isLoggedIn.value) {
//         navigateTo('/dash');
//     }
// });

const handleLogin = async () => {
    loading.value = true;
    error.value = '';

    console.log('🚀 Login form submitted');

    const result = await login(form);
    
    console.log('📋 Login result:', result);
    
    if (result.success) {
        console.log('✅ Login successful!');
        
        // Wait for state to update
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Get redirect path from query params
        const redirectTo = route.query.redirect as string;
        
        console.log('🎯 Redirect query param:', redirectTo);
        
        if (redirectTo && redirectTo !== '/login') {
            // If there's a specific redirect, go there
            console.log('🔄 Redirecting to requested path:', redirectTo);
            try {
                await router.push(redirectTo);
            } catch (navError) {
                console.error('❌ Navigation error:', navError);
                // Fallback to manual redirect
                window.location.href = redirectTo;
            }
        } else {
            // No specific redirect, determine based on user type
            const userType = user.value?.admin || user.value?.tier >= 2 ? 'admin' : 'user';
            const defaultPath = userType === 'admin' ? '/dash' : '/dashboard';
            
            console.log('🎯 No redirect param, going to default:', defaultPath);
            try {
                await router.push(defaultPath);
            } catch (navError) {
                console.error('❌ Navigation error:', navError);
                // Fallback to manual redirect
                window.location.href = defaultPath;
            }
        }
    } else {
        console.error('❌ Login failed:', result.error);
        error.value = result.error || 'Login failed';
    }
    
    loading.value = false;
};

// Show login success message if redirected after verification
const showVerifiedMessage = computed(() => {
    return route.query.verified === 'true';
});

onMounted(() => {
    if (showVerifiedMessage.value) {
        setTimeout(() => {
            // Clear the query param
            router.replace({ query: {} });
        }, 3000);
    }
});
</script>

<template>
    <main class="page-container flex-column">
        <div class="page-header">
            <h1 class="title text-center">Welcome Back</h1>
            <p class="page-subtitle text-center">Sign in to your ICSM Badminton account</p>
        </div>
        
        <!-- Verification Success Message -->
        <div v-if="showVerifiedMessage" class="success-banner">
            <h3>✅ Account Verified Successfully!</h3>
            <p>Your account has been verified. You can now login below.</p>
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

.success-banner {
    background: linear-gradient(135deg, #d4edda 0%, #c3e6cb 100%);
    border: 2px solid #28a745;
    border-radius: var(--radius-m);
    padding: 1.5rem;
    margin: 0 auto 2rem;
    max-width: 600px;
    text-align: center;
}

.success-banner h3 {
    color: #155724;
    margin: 0 0 0.5rem 0;
    font-size: 1.2rem;
}

.success-banner p {
    color: #155724;
    margin: 0;
    opacity: 0.9;
}
</style>