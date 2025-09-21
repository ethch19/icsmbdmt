// frontend/middleware/dashboard.ts - Route users to appropriate dashboard
export default defineNuxtRouteMiddleware((to) => {
  console.log('🎯 Dashboard middleware running for:', to.path);
  
  // Only run for dashboard-related routes
  if (!to.path.startsWith('/dash') && to.path !== '/dashboard') {
    return;
  }
  
  // Server-side check using cookies
  if (process.server) {
    const accessTokenCookie = useCookie('access_token');
    
    if (!accessTokenCookie.value) {
      console.log('❌ No server-side token, redirecting to login');
      return navigateTo('/login');
    }
    
    try {
      const tokenParts = accessTokenCookie.value.split('.');
      if (tokenParts.length !== 3) {
        throw new Error('Invalid token format');
      }
      
      const payload = JSON.parse(atob(tokenParts[1]));
      const isExpired = payload.exp * 1000 < Date.now();
      
      if (isExpired) {
        console.log('❌ Server-side token expired, redirecting to login');
        return navigateTo('/login');
      }
      
      const isAdminOrTeam = payload.admin || payload.tier >= 2;
      
      console.log('🔍 Server-side dashboard routing:', {
        user: payload.sub,
        admin: payload.admin,
        tier: payload.tier,
        isAdminOrTeam,
        targetPath: to.path
      });
      
      // Route to appropriate dashboard
      if (to.path === '/dashboard') {
        if (isAdminOrTeam) {
          console.log('🔄 Redirecting admin/team to /dash');
          return navigateTo('/dash');
        }
        // Regular user accessing /dashboard - allow
        return;
      }
      
      if (to.path.startsWith('/dash')) {
        if (!isAdminOrTeam && to.path !== '/dash/profile') {
          console.log('🔄 Redirecting regular user to /dashboard');
          return navigateTo('/dashboard');
        }
        // Admin/team accessing /dash/* - allow
        return;
      }
      
    } catch (error) {
      console.log('❌ Server-side token parsing error:', error.message);
      return navigateTo('/login');
    }
  }

  // Client-side check
  const { user, isLoggedIn } = useAuth();
  
  if (!isLoggedIn.value) {
    console.log('❌ Not logged in, redirecting to login');
    return navigateTo('/login');
  }
  
  // Check tokens from client storage
  const accessTokenCookie = useCookie('access_token');
  const localStorageToken = process.client ? localStorage.getItem('access_token') : null;
  const accessToken = accessTokenCookie.value || localStorageToken;
  
  if (!accessToken) {
    console.log('❌ No client-side token found');
    return navigateTo('/login');
  }
  
  try {
    const tokenParts = accessToken.split('.');
    if (tokenParts.length === 3) {
      const payload = JSON.parse(atob(tokenParts[1]));
      const isExpired = payload.exp * 1000 < Date.now();
      
      if (isExpired) {
        console.log('❌ Client-side token expired');
        return navigateTo('/login');
      }
      
      const isAdminOrTeam = payload.admin || payload.tier >= 2;
      
      console.log('🔍 Client-side dashboard routing:', {
        user: payload.sub,
        admin: payload.admin,
        tier: payload.tier,
        isAdminOrTeam,
        targetPath: to.path
      });
      
      // Route to appropriate dashboard
      if (to.path === '/dashboard') {
        if (isAdminOrTeam) {
          console.log('🔄 Client-side redirecting admin/team to /dash');
          return navigateTo('/dash');
        }
        // Regular user accessing /dashboard - allow
        return;
      }
      
      if (to.path.startsWith('/dash')) {
        if (!isAdminOrTeam && to.path !== '/dash/profile') {
          console.log('🔄 Client-side redirecting regular user to /dashboard');
          return navigateTo('/dashboard');
        }
        // Admin/team accessing /dash/* - allow
        return;
      }
    }
  } catch (e) {
    console.log('❌ Client-side token parsing error:', e.message);
    return navigateTo('/login');
  }
  
  console.log('✅ Dashboard routing completed successfully');
});