interface User {
  id: string;
  name: string;
  shortcode: string;
  tier: number;
  admin: boolean;
}

interface AuthTokens {
  access_token: string;
  refresh_token?: string;
  token_type: string;
}

export const useAuth = () => {
  const user = useState<User | null>('auth.user', () => null);
  const tokens = useState<AuthTokens | null>('auth.tokens', () => null);
  const isLoggedIn = computed(() => !!user.value);
  
  // Use cookies for server-side access
  const accessTokenCookie = useCookie('access_token', {
    maxAge: 60 * 60, // 1 hour
    httpOnly: false, // Need to access from client-side
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'strict',
    default: () => null
  });
  
  const refreshTokenCookie = useCookie('refresh_token', {
    maxAge: 60 * 60 * 24 * 84, // 12 weeks
    httpOnly: false,
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'strict',
    default: () => null
  });

  const getApiBase = () => {
    if (process.client && window.location.hostname === 'localhost') {
      return 'http://localhost:8000';
    }
    return '';
  };

  const login = async (form: {shortcode: string, password: string, keep_login?: boolean}) => {
    try {
      console.log('🔑 Starting cookie-based login process...');
      
      const baseUrl = getApiBase();
      const data = await $fetch<AuthTokens>(`${baseUrl}/api/v1/users/login`, {
        method: 'POST',
        body: form
      });

      console.log('✅ Login request successful');

      // Store tokens in cookies (primary storage)
      accessTokenCookie.value = data.access_token;
      if (form.keep_login && data.refresh_token) {
        refreshTokenCookie.value = data.refresh_token;
      }
      
      console.log('🍪 Tokens stored in cookies');
      
      // Also store in localStorage for compatibility with existing code
      if (process.client) {
        localStorage.setItem('access_token', data.access_token);
        if (form.keep_login && data.refresh_token) {
          localStorage.setItem('refresh_token', data.refresh_token);
        }
        console.log('💾 Tokens also stored in localStorage for compatibility');
      }

      // Set tokens state
      tokens.value = data;
      
      // Decode JWT to get user info
      const tokenParts = data.access_token.split('.');
      if (tokenParts.length !== 3) {
        throw new Error('Invalid token format');
      }
      
      const payload = JSON.parse(atob(tokenParts[1]));
      const userData = {
        id: payload.user_id,
        name: payload.name,
        shortcode: payload.sub,
        tier: payload.tier,
        admin: payload.admin
      };
      
      user.value = userData;
      console.log('👤 User data set:', userData);

      await nextTick();
      console.log('🎉 Cookie-based login completed successfully');

      return { success: true };
    } catch (error: any) {
      console.error('❌ Login error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Login failed' };
    }
  };

  const logout = async () => {
    console.log('👋 Logging out...');
    
    // Clear cookies
    accessTokenCookie.value = null;
    refreshTokenCookie.value = null;
    console.log('🗑️ Cookies cleared');
    
    // Clear localStorage for compatibility
    if (process.client) {
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
      console.log('🗑️ localStorage cleared');
    }
    
    // Clear state
    user.value = null;
    tokens.value = null;
    console.log('🗑️ Auth state cleared');
    
    await navigateTo('/');
  };

  const refreshToken = async () => {
    console.log('🔄 Attempting token refresh...');
    
    // Try cookie first, then localStorage
    const refreshToken = refreshTokenCookie.value || 
      (process.client ? localStorage.getItem('refresh_token') : null);
      
    if (!refreshToken) {
      console.log('❌ No refresh token available');
      return false;
    }

    try {
      const baseUrl = getApiBase();
      const data = await $fetch<AuthTokens>(`${baseUrl}/api/v1/users/refresh`, {
        method: 'GET',
        headers: {
          Authorization: `Bearer ${refreshToken}`
        }
      });

      console.log('✅ Token refresh successful');

      // Update cookies
      accessTokenCookie.value = data.access_token;
      if (data.refresh_token) {
        refreshTokenCookie.value = data.refresh_token;
      }
      
      // Update localStorage for compatibility
      if (process.client) {
        localStorage.setItem('access_token', data.access_token);
        if (data.refresh_token) {
          localStorage.setItem('refresh_token', data.refresh_token);
        }
      }

      tokens.value = data;

      // Update user info from new token
      const tokenParts = data.access_token.split('.');
      if (tokenParts.length !== 3) {
        throw new Error('Invalid token format');
      }
      const payload = JSON.parse(atob(tokenParts[1]));
      user.value = {
        id: payload.user_id,
        name: payload.name,
        shortcode: payload.sub,
        tier: payload.tier,
        admin: payload.admin
      };

      return true;
    } catch (error) {
      console.error('❌ Token refresh error:', error);
      await logout();
      return false;
    }
  };

  const register = async (userData: {
    first_name: string;
    surname: string;
    shortcode: string;
    cid: string;
    password: string;
  }) => {
    try {
      const baseUrl = getApiBase();
      const response = await $fetch(`${baseUrl}/api/v1/users/register`, {
        method: 'POST',
        body: userData
      });

      return { success: true, data: response };
    } catch (error: any) {
      console.error('Registration error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Registration failed' };
    }
  };

  const verifyAccount = async (token: string) => {
    try {
      const baseUrl = getApiBase();
      await $fetch(`${baseUrl}/api/v1/users/verify?token=${token}`, {
        method: 'POST'
      });
      return { success: true };
    } catch (error: any) {
      console.error('Verification error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Verification failed' };
    }
  };

  // Initialize auth state from cookies (works server-side) or localStorage (fallback)
  const initAuth = async () => {
    console.log('🔄 Initializing cookie-based auth state...');
    
    // Try cookies first (works on both server and client)
    let accessToken = accessTokenCookie.value;
    
    // Fallback to localStorage (client-side only)
    if (!accessToken && process.client) {
      accessToken = localStorage.getItem('access_token');
      console.log('🔄 Fallback to localStorage token');
    }
    
    console.log('💾 Access token found:', !!accessToken);
    
    if (accessToken) {
      try {
        const tokenParts = accessToken.split('.');
        if (tokenParts.length !== 3) {
          throw new Error('Invalid token format');
        }
        
        const payload = JSON.parse(atob(tokenParts[1]));
        const isExpired = payload.exp * 1000 < Date.now();
        
        console.log('🔓 Token payload:', payload);
        console.log('⏰ Token expires at:', new Date(payload.exp * 1000));
        console.log('❓ Token is expired:', isExpired);
        
        if (isExpired) {
          console.log('🔄 Token expired, attempting refresh...');
          const refreshed = await refreshToken();
          if (!refreshed) {
            console.log('❌ Token refresh failed');
            return;
          }
          console.log('✅ Token refreshed successfully');
        } else {
          console.log('✅ Token is valid, setting auth state');
          
          // Sync cookie with localStorage if needed
          if (process.client && accessTokenCookie.value && accessToken !== accessTokenCookie.value) {
            localStorage.setItem('access_token', accessTokenCookie.value);
          }
          
          tokens.value = { access_token: accessToken, token_type: 'Bearer' };
          const userData = {
            id: payload.user_id,
            name: payload.name,
            shortcode: payload.sub,
            tier: payload.tier,
            admin: payload.admin
          };
          user.value = userData;
          console.log('👤 User state set from cookie/token:', userData);
        }
      } catch (error) {
        console.error('❌ Auth initialization error:', error);
        console.log('🧹 Clearing invalid auth data');
        await logout();
      }
    } else {
      console.log('ℹ️ No access token found in cookies or localStorage');
    }
    
    console.log('🎯 Cookie-based auth initialization complete');
    console.log('📊 Final state - isLoggedIn:', isLoggedIn.value, 'user:', user.value?.name);
  };

  return {
    user: readonly(user),
    tokens: readonly(tokens),
    isLoggedIn,
    login,
    logout,
    register,
    verifyAccount,
    refreshToken,
    initAuth
  };
};