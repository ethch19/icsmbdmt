<script setup lang="ts">
definePageMeta({
    title: 'Login - ICSM Badminton'
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

// Redirect if already logged in
onMounted(() => {
    if (isLoggedIn.value) {
        navigateTo('/dash');
    }
});

const handleLogin = async () => {
    loading.value = true;
    error.value = '';

    console.log('🚀 Login form submitted');
    console.log('📝 Form data:', { shortcode: form.shortcode, keep_login: form.keep_login });

    const result = await login(form);
    
    console.log('📋 Login result:', result);
    
    if (result.success) {
        console.log('✅ Login successful!');
        
        // Wait longer for state to update properly
        console.log('⏳ Waiting for auth state to update...');
        await new Promise(resolve => setTimeout(resolve, 500));
        
        // Check if we're actually logged in now
        console.log('🔍 Checking auth state after login...');
        console.log('📊 isLoggedIn:', isLoggedIn.value);
        console.log('👤 user:', user.value);
        
        if (isLoggedIn.value && user.value) {
            // Get redirect path from query params or default to dashboard
            const redirectTo = route.query.redirect as string || '/dash';
            console.log('🎯 Redirecting to:', redirectTo);
            
            try {
                // Force a hard navigation to ensure middleware runs fresh
                console.log('🔄 Using window.location for reliable redirect');
                window.location.href = redirectTo;
                
            } catch (navError) {
                console.error('❌ Navigation error:', navError);
                error.value = 'Navigation failed. Please try refreshing the page.';
            }
        } else {
            console.error('❌ Login succeeded but auth state not properly set');
            console.log('🔍 Debug - isLoggedIn:', isLoggedIn.value, 'user:', user.value);
            
            // Try to initialize auth manually
            console.log('🔄 Attempting manual auth initialization...');
            const { initAuth } = useAuth();
            await initAuth();
            
            await new Promise(resolve => setTimeout(resolve, 200));
            
            if (isLoggedIn.value) {
                console.log('✅ Auth state recovered, redirecting');
                window.location.href = route.query.redirect as string || '/dash';
            } else {
                error.value = 'Login succeeded but authentication state not updated. Please refresh and try again.';
            }
        }
    } else {
        console.error('❌ Login failed:', result.error);
        error.value = result.error || 'Login failed';
    }
    
    loading.value = false;
};

const manualRedirect = () => {
    console.log('🔧 Manual redirect button clicked');
    const redirectTo = route.query.redirect as string || '/dash';
    console.log('🎯 Manually redirecting to:', redirectTo);
    window.location.href = redirectTo;
};

// Watch for login state changes
watch(isLoggedIn, (newValue) => {
    if (newValue) {
        console.log('User is now logged in, redirecting...');
        const redirectTo = route.query.redirect as string || '/dash';
        router.push(redirectTo);
    }
});
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
                        
                        <!-- Debug button - remove this later -->
                        <button 
                            v-if="isLoggedIn" 
                            @click="manualRedirect" 
                            type="button" 
                            class="primary-btn"
                            style="background-color: #28a745;"
                        >
                            Go to Dashboard (Debug)
                        </button>
                        
                        <div class="auth-links">
                            <span class="auth-text">Don't have an account?</span>
                            <NuxtLink to="/register" class="auth-link">Create one here</NuxtLink>
                        </div>
                        
                        <!-- Debug info -->
                        <div v-if="isLoggedIn" style="margin-top: 1rem; padding: 1rem; background: #e7f3ff; border-radius: 4px; font-size: 0.9rem;">
                            <strong>Debug Info:</strong><br>
                            Logged in: {{ isLoggedIn }}<br>
                            User: {{ user?.name }}<br>
                            Tier: {{ user?.tier }}
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