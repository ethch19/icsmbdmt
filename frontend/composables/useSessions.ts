interface SessionForm {
  id: string;
  author_id: string;
  author_name: string;
  title: string;
  description: string;
  location: string;
  tier: number;
  start_time: string;
  end_time: string;
  recurrence?: any;
  recurrence_end?: string;
  user_limit?: number;
  current_bookings: number;
  created_at: string;
  is_booked: boolean;
}

interface CreateSessionData {
  title: string;
  description: string;
  location: string;
  tier: number;
  start_time: string;
  end_time: string;
  recurrence?: any;
  recurrence_end?: string;
  user_limit?: number;
}

interface BookingUser {
  user_id: string;
  form_id: string;
  user_name: string;
  created_at: string;
}

export const useSessions = () => {
  // Get API base URL - use direct backend URL in development
  const getApiBase = () => {
    if (process.client && window.location.hostname === 'localhost') {
      return 'http://localhost:8000';
    }
    return '';
  };

  // Get auth headers safely
  const getAuthHeaders = (): Record<string, string> => {
    if (process.client) {
      const accessToken = localStorage.getItem('access_token');
      if (accessToken) {
        return {
          'Authorization': `Bearer ${accessToken}`,
          'Content-Type': 'application/json'
        };
      }
    }
    return {
      'Content-Type': 'application/json'
    };
  };

  const createSession = async (sessionData: CreateSessionData) => {
    try {
      const baseUrl = getApiBase();
      const response = await $fetch<SessionForm>(`${baseUrl}/api/v1/sessions`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body: sessionData
      });
      return { success: true, data: response };
    } catch (error: any) {
      console.error('Create session error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Failed to create session' };
    }
  };

  const getSessions = async (query: { limit?: number; offset?: number; upcoming_only?: boolean } = {}) => {
    try {
      const baseUrl = getApiBase();
      const response = await $fetch<SessionForm[]>(`${baseUrl}/api/v1/sessions`, {
        method: 'GET',
        headers: getAuthHeaders(),
        query
      });
      return { success: true, data: response };
    } catch (error: any) {
      console.error('Get sessions error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Failed to load sessions' };
    }
  };

  const getSession = async (id: string) => {
    try {
      const baseUrl = getApiBase();
      const response = await $fetch<SessionForm>(`${baseUrl}/api/v1/sessions/${id}`, {
        method: 'GET',
        headers: getAuthHeaders()
      });
      return { success: true, data: response };
    } catch (error: any) {
      console.error('Get session error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Session not found' };
    }
  };

  const updateSession = async (id: string, sessionData: CreateSessionData) => {
    try {
      const baseUrl = getApiBase();
      const response = await $fetch<SessionForm>(`${baseUrl}/api/v1/sessions/${id}`, {
        method: 'PATCH',
        headers: getAuthHeaders(),
        body: sessionData
      });
      return { success: true, data: response };
    } catch (error: any) {
      console.error('Update session error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Failed to update session' };
    }
  };

  const deleteSession = async (id: string) => {
    try {
      const baseUrl = getApiBase();
      await $fetch(`${baseUrl}/api/v1/sessions/${id}`, {
        method: 'DELETE',
        headers: getAuthHeaders()
      });
      return { success: true };
    } catch (error: any) {
      console.error('Delete session error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Failed to delete session' };
    }
  };

  const bookSession = async (id: string) => {
    try {
      const baseUrl = getApiBase();
      await $fetch(`${baseUrl}/api/v1/bookings/sessions/${id}`, {
        method: 'POST',
        headers: getAuthHeaders()
      });
      return { success: true };
    } catch (error: any) {
      console.error('Book session error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Failed to book session' };
    }
  };

  const unbookSession = async (id: string) => {
    try {
      const baseUrl = getApiBase();
      await $fetch(`${baseUrl}/api/v1/bookings/sessions/${id}`, {
        method: 'DELETE',
        headers: getAuthHeaders()
      });
      return { success: true };
    } catch (error: any) {
      console.error('Unbook session error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Failed to cancel booking' };
    }
  };

  const getSessionBookings = async (id: string) => {
    try {
      const baseUrl = getApiBase();
      const response = await $fetch<BookingUser[]>(`${baseUrl}/api/v1/bookings/sessions/${id}/bookings`, {
        method: 'GET',
        headers: getAuthHeaders()
      });
      return { success: true, data: response };
    } catch (error: any) {
      console.error('Get session bookings error:', error);
      return { success: false, error: error?.data?.message || error?.message || 'Failed to load bookings' };
    }
  };

  return {
    createSession,
    getSessions,
    getSession,
    updateSession,
    deleteSession,
    bookSession,
    unbookSession,
    getSessionBookings
  };
};