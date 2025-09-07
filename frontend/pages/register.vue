<script setup lang="ts">
definePageMeta({
    layout: false,
    title: 'Register - ICSM Badminton'
});

const { register } = useAuth();

const form = reactive({
    first_name: '',
    surname: '',
    shortcode: '',
    cid: '',
    password: ''
});

const error = ref('');
const loading = ref(false);
const submitted = ref(false);
const verificationToken = ref('');

const handleRegister = async () => {
    loading.value = true;
    error.value = '';

    const result = await register(form);
    
    if (result.success) {
        submitted.value = true;
        if (typeof result.data === 'string') {
            verificationToken.value = result.data;
        }
    } else {
        error.value = result.error || 'Registration failed';
    }
    
    loading.value = false;
};

const mobile = ref(false);

function checkMobile() {
    mobile.value = window.innerWidth <= 480;
}

onMounted(() => {
    checkMobile();
    window.addEventListener('resize', checkMobile);
});

onUnmounted(() => {
    window.removeEventListener('resize', checkMobile);
});
</script>

<template>
    <NuxtLayout name="form">
        <main class="form-container flex-column">
            <div class="form-heading-container flex-column">
                <div class="form-heading-row flex-row">
                    <img v-if="!mobile" class="form-logo" src="/img/document.svg" alt="Register Icon">
                    <h1 class="title form-heading">Register</h1>
                </div>
                <h2 class="form-subheading subtitle">Join ICSM Badminton</h2>
            </div>

            <div v-if="!submitted" class="info-container flex-column">
                <p class="text info-text">
                    <strong>Registration Requirements:</strong><br/><br/>
                    • You must be a member of ICSM Badminton to register<br/><br/>
                    • Team members will be automatically verified<br/><br/>
                    • Standard members will need email verification<br/><br/>
                    Please ensure your details match your Imperial College records.
                </p>
                <div class="info-buttons flex-row">
                    <NuxtLink to="/login" class="button secondary-button">Already have an account?</NuxtLink>
                    <NuxtLink to="/membership" class="button secondary-button">Membership Info</NuxtLink>
                </div>
            </div>
            
            <form v-if="!submitted" @submit.prevent="handleRegister" class="field-container">
                <div class="field-row">
                    <div class="field-input flex-column">
                        <label class="field-label">First Name:</label>
                        <input 
                            v-model="form.first_name" 
                            class="field" 
                            type="text" 
                            name="firstName" 
                            placeholder="John"
                            required
                        >
                    </div>
                    
                    <div class="field-input flex-column">
                        <label class="field-label">Surname:</label>
                        <input 
                            v-model="form.surname" 
                            class="field" 
                            type="text" 
                            name="surname" 
                            placeholder="Doe"
                            required
                        >
                    </div>
                </div>

                <div class="field-row">
                    <div class="field-input flex-column">
                        <label class="field-label">Shortcode:</label>
                        <input 
                            v-model="form.shortcode" 
                            class="field" 
                            type="text" 
                            name="shortcode" 
                            placeholder="jd123"
                            required
                        >
                    </div>
                    
                    <div class="field-input flex-column">
                        <label class="field-label">CID:</label>
                        <input 
                            v-model="form.cid" 
                            class="field" 
                            type="text" 
                            name="cid" 
                            placeholder="01234567"
                            maxlength="8"
                            pattern="[0-9]{8}"
                            required
                        >
                    </div>
                </div>

                <div class="field-input flex-column password-field">
                    <label class="field-label">Password:</label>
                    <input 
                        v-model="form.password" 
                        class="field" 
                        type="password" 
                        name="password" 
                        placeholder="At least 8 characters with numbers"
                        minlength="8"
                        required
                    >
                    <small class="field-hint">Must contain at least 8 characters including numbers</small>
                </div>

                <div v-if="error" class="error-message">
                    {{ error }}
                </div>

                <button 
                    type="submit" 
                    class="button primary-button submit-button"
                    :disabled="loading"
                >
                    {{ loading ? 'Registering...' : 'Register' }}
                </button>
            </form>

            <div v-else class="success-container flex-column">
                <div class="success-icon">✓</div>
                <h2 class="success-title">Registration Successful!</h2>
                
                <div v-if="verificationToken" class="verification-container flex-column">
                    <p class="text">
                        Your account requires verification. Please check your email for verification instructions.
                    </p>
                    <div class="token-container">
                        <label class="field-label">Verification Token:</label>
                        <code class="verification-token">{{ verificationToken }}</code>
                    </div>
                    <NuxtLink to="/login" class="button secondary-button">Go to Login</NuxtLink>
                </div>
                
                <div v-else class="auto-verified-container flex-column">
                    <p class="text">
                        Welcome! Your account has been automatically verified as a team member.
                    </p>
                    <NuxtLink to="/login" class="button primary-button">Login Now</NuxtLink>
                </div>
            </div>
        </main>
    </NuxtLayout>
</template>

<style scoped>
@import url("~/assets/css/form.css");
@import url("~/assets/css/register.css");
</style>