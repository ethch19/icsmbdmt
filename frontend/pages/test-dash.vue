<template>
    <div class="page-container flex-column">
        <h1 class="title">🧪 Test Dashboard - No Middleware</h1>
        
        <!-- Auth State Display -->
        <div style="background: #e7f3ff; padding: 1.5rem; border-radius: 8px; margin-bottom: 2rem; font-family: monospace;">
            <h3>🔍 Current Auth State:</h3>
            <div style="margin: 1rem 0;">
                <strong>✅ isLoggedIn:</strong> {{ isLoggedIn }} <br>
                <strong>👤 user.value:</strong> {{ user }} <br>
                <strong>📧 user?.name:</strong> {{ user?.name }} <br>
                <strong>🆔 user?.id:</strong> {{ user?.id }} <br>
                <strong>🎫 user?.tier:</strong> {{ user?.tier }} <br>
                <strong>🛡️ user?.admin:</strong> {{ user?.admin }}
            </div>
        </div>

        <!-- Success Message -->
        <div style="background: #d4edda; color: #155724; padding: 1rem; border-radius: 8px; margin-bottom: 2rem;">
            <h2>🎉 Success!</h2>
            <p>If you can see this page, it means:</p>
            <ul>
                <li>✅ Your auth state is working</li>
                <li>✅ The dashboard layout is loading</li>
                <li>❌ The issue is with the middleware logic</li>
            </ul>
        </div>

        <!-- Test Navigation -->
        <div class="actions-grid flex-row" style="gap: 1rem; margin: 2rem 0;">
            <button @click="testAuthState" class="button primary-button">
                🔍 Test Auth State
            </button>
            <button @click="checkLocalStorage" class="button secondary-button">
                💾 Check localStorage
            </button>
            <NuxtLink to="/login" class="button tertiary-button">
                ← Back to Login
            </NuxtLink>
        </div>

        <div id="test-results" style="background: #f8f9fa; padding: 1rem; border-radius: 8px; min-height: 200px;">
            <h4>Test Results:</h4>
            <p>Click the buttons above to run tests...</p>
        </div>
    </div>
</template>

<script setup lang="ts">
// Completely remove middleware for testing
definePageMeta({
    layout: "dash",
    pageTransition: {
        name: "dash",
    },
});

const { user, isLoggedIn } = useAuth();

console.log('🏠 Test dashboard page loading...');
console.log('👤 User state:', user.value);
console.log('🔐 Is logged in:', isLoggedIn.value);

const testAuthState = () => {
    const { user, isLoggedIn, tokens } = useAuth();
    const results = document.getElementById('test-results');
    
    if (results) {
        results.innerHTML = `
            <h4>🧪 Auth State Test Results:</h4>
            <pre style="background: white; padding: 1rem; border-radius: 4px; margin: 1rem 0;">
isLoggedIn: ${isLoggedIn.value}
user: ${JSON.stringify(user.value, null, 2)}
tokens: ${JSON.stringify(tokens.value, null, 2)}
            </pre>
        `;
    }
};

const checkLocalStorage = () => {
    const results = document.getElementById('test-results');
    
    if (process.client && results) {
        const accessToken = localStorage.getItem('access_token');
        const refreshToken = localStorage.getItem('refresh_token');
        
        let tokenInfo = 'No token found';
        if (accessToken) {
            try {
                const parts = accessToken.split('.');
                const payload = JSON.parse(atob(parts[1]));
                const expired = payload.exp * 1000 < Date.now();
                
                tokenInfo = `
Token exists: Yes
Token expired: ${expired}
Expires at: ${new Date(payload.exp * 1000)}
Payload: ${JSON.stringify(payload, null, 2)}
                `;
            } catch (e) {
                tokenInfo = `Token parsing error: ${e.message}`;
            }
        }
        
        results.innerHTML = `
            <h4>💾 localStorage Test Results:</h4>
            <pre style="background: white; padding: 1rem; border-radius: 4px; margin: 1rem 0;">
Access Token: ${accessToken ? 'EXISTS' : 'NOT FOUND'}
Refresh Token: ${refreshToken ? 'EXISTS' : 'NOT FOUND'}

${tokenInfo}
            </pre>
        `;
    } else if (results) {
        results.innerHTML = '<p>❌ Not running on client side</p>';
    }
};

onMounted(() => {
  console.log('🎯 Test dashboard mounted successfully!');
  console.log('🔍 Final check - isLoggedIn:', isLoggedIn.value);
  console.log('🔍 Final check - user:', user.value);
});
</script>

<style scoped>
@import url("~/assets/css/dash-page.css");

.actions-grid {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
}

.actions-grid .button {
    padding: 0.75rem 1.5rem;
}

pre {
    white-space: pre-wrap;
    word-wrap: break-word;
}
</style>