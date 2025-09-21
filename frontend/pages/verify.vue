<!-- frontend/pages/verify.vue - Account verification page -->
<script setup lang="ts">
definePageMeta({
    layout: false,
    title: 'Verify Account - ICSM Badminton'
});

const { verifyAccount } = useAuth();
const route = useRoute();
const router = useRouter();

const verificationToken = ref('');
const loading = ref(false);
const error = ref('');
const success = ref(false);
const autoVerifying = ref(false);

// Check if token is provided in URL
onMounted(() => {
  const urlToken = route.query.token as string;
  if (urlToken) {
    verificationToken.value = urlToken;
    autoVerifyAccount();
  }
});

const autoVerifyAccount = async () => {
  if (!verificationToken.value) return;
  
  autoVerifying.value = true;
  error.value = '';
  
  const result = await verifyAccount(verificationToken.value);
  
  if (result.success) {
    success.value = true;
    setTimeout(() => {
      router.push('/login?verified=true');
    }, 3000);
  } else {
    error.value = result.error || 'Verification failed';
  }
  
  autoVerifying.value = false;
};

const handleManualVerification = async () => {
  if (!verificationToken.value.trim()) {
    error.value = 'Please enter a verification token';
    return;
  }
  
  loading.value = true;
  error.value = '';
  
  const result = await verifyAccount(verificationToken.value.trim());
  
  if (result.success) {
    success.value = true;
    setTimeout(() => {
      router.push('/login?verified=true');
    }, 3000);
  } else {
    error.value = result.error || 'Verification failed';
  }
  
  loading.value = false;
};

const resendVerification = async () => {
  // This would typically call an API endpoint to resend verification email
  // For now, just show a message
  alert('Verification email resend functionality coming soon. Please contact an administrator for help.');
};
</script>

<template>
    <NuxtLayout name="form">
        <main class="form-container flex-column">
            <div class="form-heading-container flex-column">
                <div class="form-heading-row flex-row">
                    <img class="form-logo" src="/img/mail.svg" alt="Verification Icon">
                    <h1 class="title form-heading">Verify Account</h1>
                </div>
                <h2 class="form-subheading subtitle">Complete Your Registration</h2>
            </div>

            <!-- Auto-verification in progress -->
            <div v-if="autoVerifying" class="verification-container auto-verify">
                <div class="verification-spinner"></div>
                <h3>Verifying your account...</h3>
                <p>Please wait while we verify your account with the provided token.</p>
            </div>

            <!-- Success State -->
            <div v-else-if="success" class="verification-container success-state">
                <div class="success-icon">✅</div>
                <h3>Account Verified Successfully!</h3>
                <p>Your account has been verified and is now active. You can now login and start booking training sessions.</p>
                <div class="success-actions">
                    <NuxtLink to="/login" class="button primary-button">
                        Go to Login
                    </NuxtLink>
                    <NuxtLink to="/sessions" class="button secondary-button">
                        Browse Sessions
                    </NuxtLink>
                </div>
                <p class="redirect-notice">You will be automatically redirected to login in a few seconds...</p>
            </div>

            <!-- Manual Verification Form -->
            <div v-else class="verification-form-container">
                <div class="info-container flex-column">
                    <p class="text info-text">
                        <strong>Account Verification Required</strong><br/><br/>
                        Please enter the verification token that was provided during registration or sent to your email address.
                        <br/><br/>
                        If you didn't receive a verification token, you may need to contact an administrator.
                    </p>
                </div>

                <form @submit.prevent="handleManualVerification" class="verification-form">
                    <div class="field-group">
                        <label class="field-label">Verification Token:</label>
                        <input 
                            v-model="verificationToken"
                            type="text" 
                            class="field verification-input"
                            placeholder="Enter your verification token"
                            required
                        >
                        <small class="field-hint">
                            This is the UUID token provided during registration
                        </small>
                    </div>

                    <div v-if="error" class="error-message">
                        {{ error }}
                    </div>

                    <button 
                        type="submit" 
                        class="button primary-button verify-button"
                        :disabled="loading"
                    >
                        {{ loading ? 'Verifying...' : 'Verify Account' }}
                    </button>
                </form>

                <div class="help-section">
                    <div class="help-content">
                        <h4>Need Help?</h4>
                        <div class="help-options">
                            <button @click="resendVerification" class="help-btn">
                                📧 Resend Verification Email
                            </button>
                            <NuxtLink to="/register" class="help-btn">
                                🔄 Register Again
                            </NuxtLink>
                            <a href="mailto:badminton@icsm.org.uk" class="help-btn">
                                💬 Contact Support
                            </a>
                        </div>
                    </div>
                </div>

                <div class="navigation-links">
                    <NuxtLink to="/login" class="nav-link">
                        ← Back to Login
                    </NuxtLink>
                    <NuxtLink to="/" class="nav-link">
                        🏠 Home
                    </NuxtLink>
                </div>
            </div>
        </main>
    </NuxtLayout>
</template>

<style scoped>
@import url("~/assets/css/form.css");

.verification-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;
    padding: 3rem 2rem;
    text-align: center;
}

.auto-verify {
    background: linear-gradient(135deg, #e3f2fd 0%, #f3e5f5 100%);
    border-radius: var(--radius-m);
    border: 2px solid #2196f3;
}

.verification-spinner {
    width: 50px;
    height: 50px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #2196f3;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}

.success-state {
    background: linear-gradient(135deg, #e8f5e8 0%, #f0f8ef 100%);
    border-radius: var(--radius-m);
    border: 2px solid #4caf50;
}

.success-icon {
    font-size: 4rem;
    color: #4caf50;
}

.success-state h3 {
    color: #2e7d32;
    margin: 0;
    font-size: 1.8rem;
}

.success-state p {
    color: #388e3c;
    margin: 0;
    max-width: 500px;
    line-height: 1.6;
}

.success-actions {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
    flex-wrap: wrap;
    justify-content: center;
}

.redirect-notice {
    font-size: 0.9rem;
    color: #666;
    font-style: italic;
    margin-top: 1rem;
}

.verification-form-container {
    display: flex;
    flex-direction: column;
    gap: 2rem;
}

.verification-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 500px;
    margin: 0 auto;
    width: 100%;
}

.field-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.field-label {
    color: var(--dash-txt-colour);
    font-weight: 600;
    font-size: 1rem;
}

.verification-input {
    font-family: 'Courier New', monospace;
    letter-spacing: 0.5px;
    padding: 1rem;
    font-size: 0.9rem;
}

.field-hint {
    color: var(--grey-txt-colour);
    font-size: 0.85rem;
    line-height: 1.4;
}

.verify-button {
    padding: 1rem 2rem;
    font-size: 1.1rem;
    margin-top: 0.5rem;
}

.verify-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.error-message {
    background-color: #f8d7da;
    color: #721c24;
    padding: 1rem;
    border-radius: var(--radius-s);
    border: 1px solid #f5c6cb;
    text-align: center;
}

.help-section {
    background: var(--bg-20-colour);
    padding: 2rem;
    border-radius: var(--radius-m);
    text-align: center;
}

.help-content h4 {
    color: var(--dash-txt-colour);
    margin: 0 0 1.5rem 0;
    font-size: 1.2rem;
}

.help-options {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    align-items: center;
}

.help-btn {
    padding: 0.75rem 1.5rem;
    background: white;
    color: var(--dash-txt-colour);
    border: 2px solid var(--field-border-colour);
    border-radius: var(--radius-s);
    text-decoration: none;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    min-width: 200px;
}

.help-btn:hover {
    border-color: var(--accent-colour);
    background: var(--accent-colour);
    color: white;
}

.navigation-links {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    margin-top: 1rem;
    flex-wrap: wrap;
}

.nav-link {
    color: var(--accent-colour);
    text-decoration: none;
    font-weight: 600;
    padding: 0.5rem 1rem;
    border-radius: var(--radius-s);
    transition: all 0.2s ease;
}

.nav-link:hover {
    background: var(--bg-20-colour);
    text-decoration: underline;
}

/* Responsive Design */
@media (max-width: 768px) {
    .verification-container {
        padding: 2rem 1rem;
    }
    
    .success-actions {
        flex-direction: column;
        align-items: center;
    }
    
    .success-actions .button {
        width: 100%;
        max-width: 250px;
    }
    
    .help-options {
        gap: 0.75rem;
    }
    
    .help-btn {
        min-width: unset;
        width: 100%;
        max-width: 250px;
    }
    
    .navigation-links {
        justify-content: center;
        flex-direction: column;
        align-items: center;
    }
}

@media (max-width: 480px) {
    .verification-container {
        padding: 1.5rem 0.5rem;
    }
    
    .verification-form {
        padding: 0 1rem;
    }
    
    .success-state h3 {
        font-size: 1.5rem;
    }
    
    .success-icon {
        font-size: 3rem;
    }
    
    .help-section {
        padding: 1.5rem;
    }
}
</style>