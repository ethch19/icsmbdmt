export interface NotificationOptions {
  type?: 'success' | 'error' | 'warning' | 'info';
  duration?: number;
  persistent?: boolean;
}

export interface Notification extends NotificationOptions {
  id: string;
  title: string;
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
}

export const useNotifications = () => {
  const notifications = useState<Notification[]>('notifications', () => []);

  const addNotification = (title: string, message: string, options: NotificationOptions = {}) => {
    const id = Math.random().toString(36).substr(2, 9);
    const notification: Notification = {
      id,
      title,
      message,
      type: options.type || 'info',
      duration: options.duration || 5000,
      persistent: options.persistent || false,
      ...options
    };

    notifications.value.push(notification);

    // Auto remove if not persistent
    if (!notification.persistent && notification.duration) {
      setTimeout(() => {
        removeNotification(id);
      }, notification.duration);
    }

    return id;
  };

  const removeNotification = (id: string) => {
    const index = notifications.value.findIndex(n => n.id === id);
    if (index > -1) {
      notifications.value.splice(index, 1);
    }
  };

  const clearAll = () => {
    notifications.value = [];
  };

  // Convenience methods
  const success = (title: string, message: string, options?: Omit<NotificationOptions, 'type'>) => {
    return addNotification(title, message, { ...options, type: 'success' });
  };

  const error = (title: string, message: string, options?: Omit<NotificationOptions, 'type'>) => {
    return addNotification(title, message, { ...options, type: 'error', persistent: true });
  };

  const warning = (title: string, message: string, options?: Omit<NotificationOptions, 'type'>) => {
    return addNotification(title, message, { ...options, type: 'warning' });
  };

  const info = (title: string, message: string, options?: Omit<NotificationOptions, 'type'>) => {
    return addNotification(title, message, { ...options, type: 'info' });
  };

  return {
    notifications: readonly(notifications),
    addNotification,
    removeNotification,
    clearAll,
    success,
    error,
    warning,
    info
  };
};