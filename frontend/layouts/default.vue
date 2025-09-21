<!-- frontend/layouts/default.vue - Fixed with proper error handling -->
<template>
    <div class="landing-page">
        <header class="landing-header">
            <div class="header-container">
                <NuxtLink to="/">
                    <img class="m-logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
                </NuxtLink>
                <span v-if="!mobile" class="link-container">
                    <NuxtLink to="/">Home</NuxtLink>
                    <NuxtLink to="/about">About</NuxtLink>
                    <NuxtLink to="/sessions">Sessions</NuxtLink>
                    <NuxtLink to="/membership">Membership</NuxtLink>
                    <NuxtLink to="/gallery">Gallery</NuxtLink>
                    <NuxtLink v-if="!authLoaded || !isLoggedIn" to="/login" class="auth-link">Login</NuxtLink>
                    <div v-if="authLoaded && isLoggedIn" class="user-menu">
                        <button @click="toggleUserMenu" class="user-menu-btn">
                            <span class="user-avatar">{{ user?.name?.charAt(0) || 'U' }}</span>
                            <span class="user-name">{{ user?.name?.split(' ')[0] || 'User' }}</span>
                            <span class="dropdown-arrow">▾</span>
                        </button>
                        <div v-if="userMenuOpen" class="user-dropdown" @click.stop>
                            <div class="user-info">
                                <span class="user-full-name">{{ user?.name }}</span>
                                <span class="user-tier">{{ getTierName(user?.tier || 0) }}</span>
                            </div>
                            <div class="dropdown-divider"></div>
                            <NuxtLink 
                                :to="user?.admin || user?.tier >= 2 ? '/dash' : '/dashboard'" 
                                class="dropdown-item"
                                @click="closeUserMenu"
                            >
                                <span class="dropdown-icon">📊</span>
                                {{ user?.admin || user?.tier >= 2 ? 'Admin Dashboard' : 'My Dashboard' }}
                            </NuxtLink>
                            <NuxtLink to="/dash/profile" class="dropdown-item" @click="closeUserMenu">
                                <span class="dropdown-icon">👤</span>
                                Profile Settings
                            </NuxtLink>
                            <div class="dropdown-divider"></div>
                            <button @click="handleLogout" class="dropdown-item logout-item">
                                <span class="dropdown-icon">🚪</span>
                                Logout
                            </button>
                        </div>
                    </div>
                </span>
                <button v-else class="menu-btn button" @click="menuPressed">
                    <img class="menu-svg" src="/img/menu.svg"/>
                </button>
            </div>
        </header>
        <Transition name="menu">
            <div v-if="menuToggle" class="mobile-menu flex-column">
                <div class="menu-links flex-column">
                    <NuxtLink @click="menuPressed" to="/">Home</NuxtLink>
                    <div class="sep-line"></div>
                    <NuxtLink @click="menuPressed" to="/about">About</NuxtLink>
                    <div class="sep-line"></div>
                    <NuxtLink @click="menuPressed" to="/sessions">Sessions</NuxtLink>
                    <div class="sep-line"></div>
                    <NuxtLink @click="menuPressed" to="/membership">Membership</NuxtLink>
                    <div class="sep-line"></div>
                    <NuxtLink @click="menuPressed" to="/gallery">Gallery</NuxtLink>
                    <div class="sep-line"></div>
                    <NuxtLink v-if="!authLoaded || !isLoggedIn" @click="menuPressed" to="/login" class="mobile-auth-link">Login</NuxtLink>
                    <div v-if="authLoaded && isLoggedIn" class="mobile-user-section">
                        <NuxtLink 
                            @click="menuPressed" 
                            :to="user?.admin || user?.tier >= 2 ? '/dash' : '/dashboard'" 
                            class="mobile-auth-link"
                        >
                            {{ user?.admin || user?.tier >= 2 ? 'Admin Dashboard' : 'My Dashboard' }}
                        </NuxtLink>
                        <div class="sep-line"></div>
                        <NuxtLink @click="menuPressed" to="/dash/profile" class="mobile-auth-link">Profile</NuxtLink>
                        <div class="sep-line"></div>
                        <button @click="handleLogout" class="mobile-logout-btn">Logout</button>
                    </div>
                </div>
                <div class="mobile-footer">
                    <div v-if="authLoaded && isLoggedIn" class="mobile-user-info">
                        <div class="mobile-user-avatar">{{ user?.name?.charAt(0) || 'U' }}</div>
                        <div class="mobile-user-details">
                            <span class="mobile-user-name">{{ user?.name }}</span>
                            <span class="mobile-user-tier">{{ getTierName(user?.tier || 0) }}</span>
                        </div>
                    </div>
                    <p class="credit">Copyright © 2024 ICSM Badminton by <a class="author">Ethan Chang</a> | All Rights Reserved.</p>
                </div>
            </div>
        </Transition>
        <div class="page-container flex-column">
            <div>
                <slot />
            </div>
            <footer class="landing-footer flex-column">
                <div class="landing-footer-container flex-row">
                    <img v-if="!mobile" class="m-logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
                    <div class="landing-footer-columns flex-row">
                        <div v-if="!mobile" class="flex-column">
                            <h3 class="landing-footer-h3 landing-footer-link-cap">Site Map</h3>
                            <ul class="ul-no-dec">
                                <li><NuxtLink class="linkbutton" to="/">Home</NuxtLink></li>
                                <li><NuxtLink class="linkbutton" to="/about">About</NuxtLink></li>
                                <li><NuxtLink class="linkbutton" to="/sessions">Sessions</NuxtLink></li>
                                <li><NuxtLink class="linkbutton" to="/membership">Membership</NuxtLink></li>
                                <li><NuxtLink class="linkbutton" to="/gallery">Gallery</NuxtLink></li>
                                <li v-if="!authLoaded || !isLoggedIn"><NuxtLink class="linkbutton" to="/login">Login</NuxtLink></li>
                                <li v-if="authLoaded && isLoggedIn">
                                    <NuxtLink 
                                        class="linkbutton" 
                                        :to="user?.admin || user?.tier >= 2 ? '/dash' : '/dashboard'"
                                    >
                                        Dashboard
                                    </NuxtLink>
                                </li>
                            </ul>
                        </div>
                        <div class="footer-2-column flex-column">
                            <NuxtLink class="linkbutton landing-footer-link-cap" to="/terms">Terms of Use</NuxtLink>
                            <NuxtLink class="linkbutton landing-footer-link-cap" to="/privacy">Privacy Policy</NuxtLink>
                        </div>
                    </div>
                </div>
                <p class="credit">Copyright © 2024 ICSM Badminton by <a class="author">Ethan Chang</a> | All Rights Reserved.</p>
            </footer>
        </div>
    </div>
</template>

<script setup lang="ts">
const route = useRoute();

// Initialize auth state with error handling
const authLoaded = ref(false);
const authError = ref(false);
let isLoggedIn = ref(false);
let user = ref(null);
let logout = () => Promise.resolve();

// Try to initialize auth
try {
    console.log('🔧 Layout: Attempting to initialize auth...');
    const auth = useAuth();
    isLoggedIn = auth.isLoggedIn;
    user = auth.user;
    logout = auth.logout;
    authLoaded.value = true;
    console.log('✅ Layout: Auth initialized successfully');
} catch (error) {
    console.error('❌ Layout: Auth initialization failed:', error);
    authError.value = true;
    authLoaded.value = true; // Still set to loaded to show the page
}

const menuToggle = ref(false);
const userMenuOpen = ref(false);

function menuPressed() {
    menuToggle.value = !menuToggle.value;
}

function toggleUserMenu() {
    userMenuOpen.value = !userMenuOpen.value;
}

function closeUserMenu() {
    userMenuOpen.value = false;
}

const handleLogout = async () => {
    if (confirm('Are you sure you want to logout?')) {
        try {
            await logout();
            closeUserMenu();
            menuToggle.value = false;
        } catch (error) {
            console.error('Logout error:', error);
            // Force navigation even if logout fails
            await navigateTo('/');
        }
    }
};

const getTierName = (tier: number) => {
    switch (tier) {
        case 0: return 'Non-Member';
        case 1: return 'Member';
        case 2: return 'Team Member';
        default: return 'User';
    }
};

const vw = ref(0);
const mobile = ref(false);

function widthResized() {
    vw.value = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);
    if (vw.value <= 992) {
        mobile.value = true;
    } else {
        mobile.value = false;
        menuToggle.value = false;
        userMenuOpen.value = false;
    }
}

// Close user menu when clicking outside
const handleOutsideClick = (event: Event) => {
    if (userMenuOpen.value && !event.target?.closest('.user-menu')) {
        closeUserMenu();
    }
};

onMounted(() => {
    console.log('🎯 Layout mounted');
    widthResized();
    let doit: NodeJS.Timeout;
    window.onresize = () => {
        clearTimeout(doit);
        doit = setTimeout(widthResized, 100);
    };
    
    document.addEventListener('click', handleOutsideClick);
});

onUnmounted(() => {
    if (typeof window !== 'undefined') {
        window.onresize = null;
        document.removeEventListener('click', handleOutsideClick);
    }
});
</script>

<style scoped>
@import url("~/assets/css/headerfooter.css");
@import url("~/assets/css/nav-auth.css");

/* User Menu Styles */
.user-menu {
    position: relative;
}

.user-menu-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: none;
    border: 2px solid transparent;
    border-radius: var(--radius-s);
    padding: 0.5rem 1rem;
    cursor: pointer;
    transition: all 0.2s ease;
    color: var(--header-footer-bold-colour);
    font-weight: 600;
}

.user-menu-btn:hover {
    border-color: var(--accent-colour);
    background: var(--bg-20-colour);
}

.user-avatar {
    width: 32px;
    height: 32px;
    background: var(--accent-colour);
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 0.9rem;
}

.user-name {
    font-size: 0.95rem;
}

.dropdown-arrow {
    font-size: 0.8rem;
    transition: transform 0.2s ease;
}

.user-menu-btn:hover .dropdown-arrow {
    transform: rotate(180deg);
}

.user-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    background: white;
    border: 1px solid var(--field-border-colour);
    border-radius: var(--radius-s);
    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
    min-width: 200px;
    z-index: 1000;
    overflow: hidden;
    margin-top: 0.5rem;
}

.user-info {
    padding: 1rem;
    background: var(--bg-20-colour);
    text-align: center;
}

.user-full-name {
    display: block;
    font-weight: 600;
    color: var(--dash-txt-colour);
    margin-bottom: 0.25rem;
}

.user-tier {
    font-size: 0.85rem;
    color: var(--grey-txt-colour);
}

.dropdown-divider {
    height: 1px;
    background: var(--field-border-colour);
}

.dropdown-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    text-decoration: none;
    color: var(--dash-txt-colour);
    transition: background 0.2s ease;
    border: none;
    background: none;
    width: 100%;
    text-align: left;
    cursor: pointer;
    font-size: 0.95rem;
}

.dropdown-item:hover {
    background: var(--bg-20-colour);
}

.dropdown-icon {
    width: 16px;
    text-align: center;
}

.logout-item {
    color: #dc3545;
}

.logout-item:hover {
    background: #fee;
}

/* Mobile User Section */
.mobile-user-section {
    display: flex;
    flex-direction: column;
}

.mobile-logout-btn {
    display: block;
    text-align: center;
    color: #dc3545;
    background: none;
    border: none;
    text-decoration: none;
    padding: 0;
    font: var(--a);
    font-size: 1.5rem;
    line-height: 5rem;
    cursor: pointer;
    transition: opacity 0.2s ease;
}

.mobile-logout-btn:hover {
    opacity: 0.7;
}

.mobile-user-info {
    display: flex;
    align-items: center;
    gap: 1rem;
    background-color: var(--bg-20-colour);
    color: var(--accent-colour);
    padding: 1rem;
    border-radius: var(--radius-s);
    margin-bottom: 1rem;
}

.mobile-user-avatar {
    width: 48px;
    height: 48px;
    background: var(--accent-colour);
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 1.2rem;
}

.mobile-user-details {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.mobile-user-name {
    font-weight: 600;
    font-size: 1rem;
}

.mobile-user-tier {
    font-size: 0.85rem;
    opacity: 0.8;
}

@media (max-width: 992px) {
    .mobile-footer {
        margin-top: 1rem;
        padding-top: 1rem;
        border-top: 2px solid var(--bg-20-colour);
    }
}
</style>