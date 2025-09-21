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
  shortcode: string;
  tier: number;
  created_at: string;
}

export const useSessions = () => {
  const { success, error: notifyError, info } = useNotifications();

  // Get API base URL
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

      success('Session Created', `"${sessionData.title}" has been created successfully!`);
      return { success: true, data: response };
    } catch (error: any) {
      console.error('Create session error:', error);
      const errorMessage = error?.data?.message || error?.message || 'Failed to create session';
      notifyError('Failed to Create Session', errorMessage);
      return { success: false, error: errorMessage };
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
      const errorMessage = error?.data?.message || error?.message || 'Failed to load sessions';
      notifyError('Failed to Load Sessions', errorMessage);
      return { success: false, error: errorMessage };
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
      const errorMessage = error?.data?.message || error?.message || 'Session not found';
      notifyError('Failed to Load Session', errorMessage);
      return { success: false, error: errorMessage };
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

      success('Session Updated', `"${sessionData.title}" has been updated successfully!`);
      return { success: true, data: response };
    } catch (error: any) {
      console.error('Update session error:', error);
      const errorMessage = error?.data?.message || error?.message || 'Failed to update session';
      notifyError('Failed to Update Session', errorMessage);
      return { success: false, error: errorMessage };
    }
  };

  const deleteSession = async (id: string, title?: string) => {
    try {
      const baseUrl = getApiBase();
      await $fetch(`${baseUrl}/api/v1/sessions/${id}`, {
        method: 'DELETE',
        headers: getAuthHeaders()
      });

      success('Session Deleted', title ? `"${title}" has been deleted.` : 'Session has been deleted.');
      return { success: true };
    } catch (error: any) {
      console.error('Delete session error:', error);
      const errorMessage = error?.data?.message || error?.message || 'Failed to delete session';
      notifyError('Failed to Delete Session', errorMessage);
      return { success: false, error: errorMessage };
    }
  };

  const bookSession = async (id: string, title?: string) => {
    try {
      const baseUrl = getApiBase();
      await $fetch(`${baseUrl}/api/v1/bookings/sessions/${id}`, {
        method: 'POST',
        headers: getAuthHeaders()
      });

      success('Session Booked', title ? `You've successfully booked "${title}"!` : 'Session booked successfully!');
      return { success: true };
    } catch (error: any) {
      console.error('Book session error:', error);
      const errorMessage = error?.data?.message || error?.message || 'Failed to book session';
      notifyError('Failed to Book Session', errorMessage);
      return { success: false, error: errorMessage };
    }
  };

  const unbookSession = async (id: string, title?: string) => {
    try {
      const baseUrl = getApiBase();
      await $fetch(`${baseUrl}/api/v1/bookings/sessions/${id}`, {
        method: 'DELETE',
        headers: getAuthHeaders()
      });

      info('Booking Cancelled', title ? `Your booking for "${title}" has been cancelled.` : 'Your booking has been cancelled.');
      return { success: true };
    } catch (error: any) {
      console.error('Unbook session error:', error);
      const errorMessage = error?.data?.message || error?.message || 'Failed to cancel booking';
      notifyError('Failed to Cancel Booking', errorMessage);
      return { success: false, error: errorMessage };
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
      const errorMessage = error?.data?.message || error?.message || 'Failed to load bookings';
      notifyError('Failed to Load Bookings', errorMessage);
      return { success: false, error: errorMessage };
    }
  };

  // Real-time session updates using polling (could be replaced with WebSocket)
  const useSessionUpdates = (sessionId: string, intervalMs: number = 30000) => {
    const session = ref<SessionForm | null>(null);
    const loading = ref(false);
    let intervalId: NodeJS.Timeout | null = null;

    const fetchSession = async () => {
      if (loading.value) return;
      loading.value = true;

      const result = await getSession(sessionId);
      if (result.success) {
        const oldBookings = session.value?.current_bookings || 0;
        const newBookings = result.data.current_bookings;
        
        // Notify of new bookings
        if (session.value && newBookings > oldBookings) {
          info('New Booking', `Someone just booked "${result.data.title}"!`);
        }
        
        session.value = result.data;
      }
      loading.value = false;
    };

    const startPolling = () => {
      fetchSession(); // Initial fetch
      intervalId = setInterval(fetchSession, intervalMs);
    };

    const stopPolling = () => {
      if (intervalId) {
        clearInterval(intervalId);
        intervalId = null;
      }
    };

    onMounted(startPolling);
    onUnmounted(stopPolling);

    return {
      session: readonly(session),
      loading: readonly(loading),
      refresh: fetchSession
    };
  };

  // Batch operations
  const bulkDeleteSessions = async (sessionIds: string[]) => {
    const results = await Promise.allSettled(
      sessionIds.map(id => deleteSession(id))
    );

    const successful = results.filter(r => r.status === 'fulfilled' && r.value.success).length;
    const failed = results.length - successful;

    if (successful > 0) {
      success('Bulk Delete Complete', `${successful} session(s) deleted successfully.`);
    }
    if (failed > 0) {
      notifyError('Some Deletions Failed', `${failed} session(s) could not be deleted.`);
    }

    return { successful, failed };
  };

  return {
    createSession,
    getSessions,
    getSession,
    updateSession,
    deleteSession,
    bookSession,
    unbookSession,
    getSessionBookings,
    useSessionUpdates,
    bulkDeleteSessions
  };
};