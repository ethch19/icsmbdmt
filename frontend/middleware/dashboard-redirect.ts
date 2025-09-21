// frontend/middleware/dashboard-redirect.ts - Simple dashboard routing without loops
export default defineNuxtRouteMiddleware((to) => {
  console.log('🎯 Dashboard redirect middleware for:', to.path);
  
  // Only run for exact dashboard paths to prevent infinite loops
  if (to.path !== '/dashboard' && to.path !== '/dash') {
    console.log('⏭️ Not a dashboard route, skipping');
    return;
  }
  
  // Prevent infinite redirects by tracking if we're already redirecting
  if (process.client && sessionStorage.getItem('dashboard-redirecting') === 'true') {
    console.log('🔄 Already redirecting, stopping to prevent loop');
    sessionStorage.removeItem('dashboard-redirecting');
    return;
  }
  
  // Server-side check using cookies
  if (process.server) {
    const accessTokenCookie = useCookie('access_token');
    
    if (!accessTokenCookie.value) {
      console.log('❌ No server token for dashboard routing');
      return;
    }
    
    try {
      const tokenParts = accessTokenCookie.value.split('.');
      if (tokenParts.length !== 3) {
        throw new Error('Invalid token format');
      }
      
      const payload = JSON.parse(atob(tokenParts[1]));
      const isExpired = payload.exp * 1000 < Date.now();
      
      if (isExpired) {
        console.log('❌ Server token expired for dashboard routing');
        return;
      }
      
      const isAdminOrTeam = payload.admin || payload.tier >= 2;
      
      console.log('🔍 Server-side dashboard routing:', {
        user: payload.sub,
        admin: payload.admin,
        tier: payload.tier,
        isAdminOrTeam,
        targetPath: to.path
      });
      
      // Route to appropriate dashboard - only redirect if going to wrong one
      if (to.path === '/dashboard' && isAdminOrTeam) {
        console.log('🔄 Server: Redirecting admin/team to /dash');
        return navigateTo('/dash');
      }
      
      if (to.path === '/dash' && !isAdminOrTeam) {
        console.log('🔄 Server: Redirecting regular user to /dashboard');
        return navigateTo('/dashboard');
      }
      
      console.log('✅ Server: Correct dashboard for user type');
      return;
      
    } catch (error) {
      console.log('❌ Server token parsing error:', error.message);
      return;
    }
  }

  // Client-side check
  const { user, isLoggedIn } = useAuth();
  
  if (!isLoggedIn.value) {
    console.log('❌ Not logged in for dashboard routing');
    return;
  }
  
  // Check tokens from client storage
  const accessTokenCookie = useCookie('access_token');
  const localStorageToken = process.client ? localStorage.getItem('access_token') : null;
  const accessToken = accessTokenCookie.value || localStorageToken;
  
  if (!accessToken) {
    console.log('❌ No client token for dashboard routing');
    return;
  }
  
  try {
    const tokenParts = accessToken.split('.');
    if (tokenParts.length === 3) {
      const payload = JSON.parse(atob(tokenParts[1]));
      const isExpired = payload.exp * 1000 < Date.now();
      
      if (isExpired) {
        console.log('❌ Client token expired for dashboard routing');
        return;
      }
      
      const isAdminOrTeam = payload.admin || payload.tier >= 2;
      
      console.log('🔍 Client-side dashboard routing:', {
        user: payload.sub,
        admin: payload.admin,
        tier: payload.tier,
        isAdminOrTeam,
        targetPath: to.path
      });
      
      // Set redirecting flag to prevent loops
      if (process.client) {
        sessionStorage.setItem('dashboard-redirecting', 'true');
      }
      
      // Route to appropriate dashboard - only redirect if going to wrong one
      if (to.path === '/dashboard' && isAdminOrTeam) {
        console.log('🔄 Client: Redirecting admin/team to /dash');
        return navigateTo('/dash');
      }
      
      if (to.path === '/dash' && !isAdminOrTeam) {
        console.log('🔄 Client: Redirecting regular user to /dashboard');
        return navigateTo('/dashboard');
      }
      
      // Clear redirecting flag if we're not redirecting
      if (process.client) {
        sessionStorage.removeItem('dashboard-redirecting');
      }
      
      console.log('✅ Client: Correct dashboard for user type');
      return;
    }
  } catch (e) {
    console.log('❌ Client token parsing error:', e.message);
    return;
  }
  
  console.log('✅ Dashboard routing completed');
});