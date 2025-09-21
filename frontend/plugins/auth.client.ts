// frontend/plugins/auth.client.ts - Updated with better error handling
export default defineNuxtPlugin(async () => {
  console.log('🔌 Auth plugin starting...');
  
  try {
    // Import the composable properly
    const auth = useAuth();
    console.log('✅ Auth composable imported successfully');
    
    const { initAuth, isLoggedIn, user } = auth;
    
    console.log('🔄 Initializing authentication...');
    
    await initAuth();
    
    console.log('✅ Authentication initialized');
    console.log('📊 Auth state:', {
      isLoggedIn: isLoggedIn.value,
      user: user.value?.name || 'none'
    });
  } catch (error) {
    console.error('❌ Auth plugin error:', error);
    
    // Don't let auth errors break the app
    console.log('🔄 Continuing with no authentication');
  }
  
  console.log('🎯 Auth plugin completed');
});