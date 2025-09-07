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

  const login = async (shortcode: string, password: string, keepLogin: boolean = false) => {
    try {
      const data = await $fetch<AuthTokens>('/api/v1/users/login', {
        method: 'POST',
        body: {
          shortcode,
          password,
          keep_login: keepLogin
        }
      });

      tokens.value = data;
      
      // Decode JWT to get user info
      const payload = JSON.parse(atob(data.access_token.split('.')[1]));
      user.value = {
        id: payload.user_id,
        name: payload.name,
        shortcode: payload.sub,
        tier: payload.tier,
        admin: payload.admin
      };

      // Store tokens in localStorage if keepLogin is true
      if (keepLogin && data.refresh_token) {
        localStorage.setItem('refresh_token', data.refresh_token);
      }
      localStorage.setItem('access_token', data.access_token);

      return { success: true };
    } catch (error: any) {
      console.error('Login error:', error);
      return { success: false, error: error?.data?.message || 'Login failed' };
    }
  };

  const logout = async () => {
    user.value = null;
    tokens.value = null;
    if (process.client) {
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
    }
    await navigateTo('/');
  };

  const refreshToken = async () => {
    if (!process.client) return false;
    
    const refreshToken = localStorage.getItem('refresh_token');
    if (!refreshToken) return false;

    try {
      const data = await $fetch<AuthTokens>('/api/v1/users/refresh', {
        method: 'GET',
        headers: {
          Authorization: `Bearer ${refreshToken}`
        }
      });

      tokens.value = data;
      localStorage.setItem('access_token', data.access_token);
      if (data.refresh_token) {
        localStorage.setItem('refresh_token', data.refresh_token);
      }

      // Update user info from new token
      const payload = JSON.parse(atob(data.access_token.split('.')[1]));
      user.value = {
        id: payload.user_id,
        name: payload.name,
        shortcode: payload.sub,
        tier: payload.tier,
        admin: payload.admin
      };

      return true;
    } catch (error) {
      console.error('Token refresh error:', error);
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
      const response = await $fetch('/api/v1/users/register', {
        method: 'POST',
        body: userData
      });

      return { success: true, data: response };
    } catch (error: any) {
      console.error('Registration error:', error);
      return { success: false, error: error?.data?.message || 'Registration failed' };
    }
  };

  const verifyAccount = async (token: string) => {
    try {
      await $fetch(`/api/v1/users/verify?token=${token}`, {
        method: 'POST'
      });
      return { success: true };
    } catch (error: any) {
      console.error('Verification error:', error);
      return { success: false, error: error?.data?.message || 'Verification failed' };
    }
  };

  // Initialize auth state on app start
  const initAuth = async () => {
    if (!process.client) return;
    
    const accessToken = localStorage.getItem('access_token');
    if (accessToken) {
      try {
        const payload = JSON.parse(atob(accessToken.split('.')[1]));
        const isExpired = payload.exp * 1000 < Date.now();
        
        if (isExpired) {
          const refreshed = await refreshToken();
          if (!refreshed) return;
        } else {
          tokens.value = { access_token: accessToken, token_type: 'Bearer' };
          user.value = {
            id: payload.user_id,
            name: payload.name,
            shortcode: payload.sub,
            tier: payload.tier,
            admin: payload.admin
          };
        }
      } catch (error) {
        console.error('Auth initialization error:', error);
        await logout();
      }
    }
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