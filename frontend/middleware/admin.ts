export default defineNuxtRouteMiddleware((to) => {
  const { user, isLoggedIn } = useAuth();
  
  if (!isLoggedIn.value) {
    return navigateTo('/login');
  }
  
  if (!user.value?.admin && user.value?.tier < 2) {
    throw createError({
      statusCode: 403,
      statusMessage: 'Access Denied'
    });
  }
});