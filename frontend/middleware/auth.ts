export default defineNuxtRouteMiddleware((to) => {
  console.log('🛡️ Secure auth middleware running for:', to.path);
  
  // For server-side rendering, check cookies
  if (process.server) {
    const accessTokenCookie = useCookie('access_token');
    console.log('🌐 Server-side auth check, cookie exists:', !!accessTokenCookie.value);
    
    if (!accessTokenCookie.value) {
      console.log('❌ No server-side token, redirecting to login');
      return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
    }
    
    try {
      // Validate token server-side
      const tokenParts = accessTokenCookie.value.split('.');
      if (tokenParts.length !== 3) {
        throw new Error('Invalid token format');
      }
      
      const payload = JSON.parse(atob(tokenParts[1]));
      const isExpired = payload.exp * 1000 < Date.now();
      
      console.log('🔍 Server-side token check:', {
        expires: new Date(payload.exp * 1000),
        isExpired,
        user: payload.sub
      });
      
      if (isExpired) {
        console.log('❌ Server-side token expired, redirecting to login');
        return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
      }
      
      console.log('✅ Server-side token valid, allowing access');
      return;
    } catch (error) {
      console.log('❌ Server-side token validation failed:', error.message);
      return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
    }
  }

  // Client-side check
  const { isLoggedIn } = useAuth();
  
  console.log('💻 Client-side auth check');
  console.log('  - isLoggedIn state:', isLoggedIn.value);
  
  // Check both cookie and localStorage as fallbacks
  const accessTokenCookie = useCookie('access_token');
  const localStorageToken = process.client ? localStorage.getItem('access_token') : null;
  
  const accessToken = accessTokenCookie.value || localStorageToken;
  
  console.log('  - Cookie token exists:', !!accessTokenCookie.value);
  console.log('  - localStorage token exists:', !!localStorageToken);
  
  if (!accessToken) {
    console.log('❌ No client-side token found, redirecting to login');
    return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
  }
  
  try {
    const tokenParts = accessToken.split('.');
    if (tokenParts.length === 3) {
      const payload = JSON.parse(atob(tokenParts[1]));
      const isExpired = payload.exp * 1000 < Date.now();
      
      console.log('🔍 Client-side token check:', {
        expires: new Date(payload.exp * 1000),
        isExpired,
        user: payload.sub
      });
      
      if (isExpired) {
        console.log('❌ Client-side token expired, redirecting to login');
        return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
      }
      
      console.log('✅ Client-side token valid, allowing access');
      return;
    }
  } catch (e) {
    console.log('❌ Client-side token parsing error:', e.message);
    return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
  }
  
  // Final fallback - should not reach here with valid tokens
  if (!isLoggedIn.value) {
    console.log('❌ Auth state check failed, redirecting to login');
    return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`);
  }
  
  console.log('✅ All auth checks passed, allowing access to:', to.path);
});