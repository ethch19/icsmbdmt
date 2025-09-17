export default defineNuxtPlugin(async () => {
  const { initAuth, isLoggedIn, user } = useAuth();
  
  console.log('Auth plugin: Initializing authentication...');
  
  try {
    await initAuth();
    console.log('Auth plugin: Authentication initialized');
    console.log('Auth plugin: isLoggedIn:', isLoggedIn.value);
    console.log('Auth plugin: user:', user.value);
  } catch (error) {
    console.error('Auth plugin: Failed to initialize authentication:', error);
  }
});