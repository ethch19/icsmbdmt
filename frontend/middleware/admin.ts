export default defineNuxtRouteMiddleware((to) => {
  console.log('🛡️ Secure admin middleware running for:', to.path);

  // Server-side check using cookies
  if (process.server) {
    const accessTokenCookie = useCookie('access_token');
    console.log('🌐 Server-side admin check, cookie exists:', !!accessTokenCookie.value);
    
    if (!accessTokenCookie.value) {
      console.log('❌ No server-side token, redirecting to login');
      return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
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
        return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
      }
      
      // Check admin/tier permissions from token
      const hasAdminAccess = payload.admin || payload.tier >= 2;
      
      console.log('🔍 Server-side admin check:', {
        user: payload.sub,
        admin: payload.admin,
        tier: payload.tier,
        hasAccess: hasAdminAccess
      });
      
      if (!hasAdminAccess) {
        console.log('❌ Server-side insufficient permissions');
        throw createError({
          statusCode: 403,
          statusMessage: 'Access Denied - You need admin privileges or team member status'
        });
      }
      
      console.log('✅ Server-side admin access granted');
      return;
    } catch (e) {
      if (e.statusCode) throw e; // Re-throw createError
      console.log('❌ Server-side token parsing error:', e.message);
      return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
    }
  }

  // Client-side check
  const { user, isLoggedIn } = useAuth();
  
  console.log('💻 Client-side admin check');
  
  // Check both cookie and localStorage for token
  const accessTokenCookie = useCookie('access_token');
  const localStorageToken = process.client ? localStorage.getItem('access_token') : null;
  const accessToken = accessTokenCookie.value || localStorageToken;
  
  if (!accessToken) {
    console.log('❌ No client-side token found, redirecting to login');
    return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
  }
  
  try {
    const tokenParts = accessToken.split('.');
    if (tokenParts.length === 3) {
      const payload = JSON.parse(atob(tokenParts[1]));
      const isExpired = payload.exp * 1000 < Date.now();
      
      if (isExpired) {
        console.log('❌ Client-side token expired, redirecting to login');
        return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
      }
      
      // Check admin/tier permissions from token
      const hasAdminAccess = payload.admin || payload.tier >= 2;
      
      console.log('🔍 Client-side admin check:', {
        user: payload.sub,
        admin: payload.admin,
        tier: payload.tier,
        hasAccess: hasAdminAccess
      });
      
      if (!hasAdminAccess) {
        console.log('❌ Client-side insufficient permissions');
        throw createError({
          statusCode: 403,
          statusMessage: 'Access Denied - You need admin privileges or team member status'
        });
      }
      
      console.log('✅ Client-side admin access granted');
      return;
    }
  } catch (e) {
    if (e.statusCode) throw e; // Re-throw createError
    console.log('❌ Client-side token parsing error:', e.message);
    return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
  }
  
  // Fallback to auth state (should not be needed with valid tokens)
  if (!isLoggedIn.value) {
    console.log('❌ Client-side user not logged in, redirecting to login');
    return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
  }
  
  if (!user.value?.admin && user.value?.tier < 2) {
    console.log('❌ Client-side user does not have admin access or sufficient tier');
    throw createError({
      statusCode: 403,
      statusMessage: 'Access Denied - You need admin privileges or team member status'
    });
  }
  
  console.log('✅ All admin checks passed, allowing access');
});