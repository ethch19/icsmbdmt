<template>
    <div class="landing-page">
        <header class="landing-header">
            <div class="header-container">
                <img class="m-logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
                <span v-if="!mobile" class="link-container">
                    <NuxtLink to="/">Home</NuxtLink>
                    <NuxtLink to="/about">About</NuxtLink>
                    <NuxtLink to="/membership">Membership</NuxtLink>
                    <NuxtLink to="/gallery">Gallery</NuxtLink>
                    <NuxtLink v-if="!isLoggedIn" to="/login" class="auth-link">Login</NuxtLink>
                    <NuxtLink v-if="isLoggedIn" to="/dash" class="auth-link dashboard-link">Dashboard</NuxtLink>
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
                    <NuxtLink @click="menuPressed" to="/membership">Membership</NuxtLink>
                    <div class="sep-line"></div>
                    <NuxtLink @click="menuPressed" to="/gallery">Gallery</NuxtLink>
                    <div class="sep-line"></div>
                    <NuxtLink v-if="!isLoggedIn" @click="menuPressed" to="/login" class="mobile-auth-link">Login</NuxtLink>
                    <NuxtLink v-if="isLoggedIn" @click="menuPressed" to="/dash" class="mobile-auth-link">Dashboard</NuxtLink>
                </div>
                <div class="mobile-footer">
                    <div v-if="isLoggedIn" class="mobile-user-info">
                        Welcome back, {{ user?.name }}!
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
                                <li><NuxtLink class="linkbutton" to="/membership">Membership</NuxtLink></li>
                                <li><NuxtLink class="linkbutton" to="/gallery">Gallery</NuxtLink></li>
                                <li v-if="!isLoggedIn"><NuxtLink class="linkbutton" to="/login">Login</NuxtLink></li>
                                <li v-if="isLoggedIn"><NuxtLink class="linkbutton" to="/dash">Dashboard</NuxtLink></li>
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
const { isLoggedIn, user } = useAuth();

const menuToggle = ref(false);

function menuPressed() {
    menuToggle.value = !menuToggle.value;
}

const vw = ref(0);
const mobile = ref(false);

function widthResized() {
    vw.value = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);
    if (vw.value <= 992) {
        mobile.value = true;
    } else {
        mobile.value = false;
        menuToggle.value = false;
    }
}

onMounted(() => {
    widthResized();
    let doit: NodeJS.Timeout;
    window.onresize = () => {
        clearTimeout(doit);
        doit = setTimeout(widthResized, 100);
    };
});

onUnmounted(() => {
    if (typeof window !== 'undefined') {
        window.onresize = null;
    }
});
</script>

<style scoped>
@import url("~/assets/css/headerfooter.css");
@import url("~/assets/css/nav-auth.css");
</style>