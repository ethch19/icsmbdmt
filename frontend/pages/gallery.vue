<!-- frontend/pages/gallery.vue - Complete gallery implementation -->
<script setup lang="ts">
definePageMeta({
    layout: "default",
    title: 'Gallery - ICSM Badminton'
});

// Gallery data - in a real app this would come from a CMS or API
const galleryCategories = ref([
  {
    id: 'training',
    name: 'Training Sessions',
    description: 'Regular training sessions and skill development',
    images: [
      {
        src: '/img/gallery/training-1.jpg',
        alt: 'Team training session',
        caption: 'Weekly team training at Imperial College'
      },
      {
        src: '/img/gallery/training-2.jpg', 
        alt: 'Coaching session',
        caption: 'Professional coaching for skill development'
      },
      {
        src: '/img/gallery/training-3.jpg',
        alt: 'Beginners session',
        caption: 'Welcoming beginners to the sport'
      },
      {
        src: '/img/gallery/training-4.jpg',
        alt: 'Advanced techniques',
        caption: 'Advanced techniques training'
      }
    ]
  },
  {
    id: 'competitions',
    name: 'Competitions & Tournaments',
    description: 'BUCS competitions and inter-university tournaments',
    images: [
      {
        src: '/img/gallery/comp-1.jpg',
        alt: 'BUCS tournament',
        caption: 'BUCS National Championships 2024'
      },
      {
        src: '/img/gallery/comp-2.jpg',
        alt: 'Victory celebration',
        caption: 'Celebrating our tournament victory'
      },
      {
        src: '/img/gallery/comp-3.jpg',
        alt: 'Team match',
        caption: 'Inter-university team match'
      },
      {
        src: '/img/gallery/comp-4.jpg',
        alt: 'Competition action',
        caption: 'Intense competition action'
      }
    ]
  },
  {
    id: 'social',
    name: 'Social Events',
    description: 'Club socials, Sports Night, and team bonding',
    images: [
      {
        src: '/img/gallery/social-main.jpg',
        alt: 'Sports night',
        caption: 'Wednesday Sports Night at Reynolds'
      },
      {
        src: '/img/gallery/social-left.jpg',
        alt: 'Team dinner',
        caption: 'Annual team dinner celebration'
      },
      {
        src: '/img/gallery/social-right.jpg',
        alt: 'Club social',
        caption: 'Club social event'
      },
      {
        src: '/img/gallery/social-2.jpg',
        alt: 'Welcome event',
        caption: 'Freshers welcome event'
      }
    ]
  },
  {
    id: 'tours',
    name: 'Tours & Travel',
    description: 'Annual tours and away competitions',
    images: [
      {
        src: '/img/gallery/tour-1.jpg',
        alt: 'Annual tour',
        caption: 'Annual club tour 2024'
      },
      {
        src: '/img/gallery/tour-2.jpg',
        alt: 'Team adventure',
        caption: 'Team building activities on tour'
      },
      {
        src: '/img/gallery/tour-3.jpg',
        alt: 'Away match',
        caption: 'Away competition venue'
      }
    ]
  }
]);

const selectedCategory = ref('all');
const selectedImage = ref(null);
const lightboxOpen = ref(false);
const currentImageIndex = ref(0);

// Get all images for lightbox navigation
const allImages = computed(() => {
  if (selectedCategory.value === 'all') {
    return galleryCategories.value.flatMap(category => 
      category.images.map(img => ({
        ...img,
        category: category.name
      }))
    );
  } else {
    const category = galleryCategories.value.find(cat => cat.id === selectedCategory.value);
    return category ? category.images.map(img => ({
      ...img,
      category: category.name
    })) : [];
  }
});

const filteredImages = computed(() => {
  return allImages.value;
});

const openLightbox = (image, index) => {
  selectedImage.value = image;
  currentImageIndex.value = index;
  lightboxOpen.value = true;
  document.body.style.overflow = 'hidden';
};

const closeLightbox = () => {
  lightboxOpen.value = false;
  selectedImage.value = null;
  document.body.style.overflow = 'auto';
};

const nextImage = () => {
  currentImageIndex.value = (currentImageIndex.value + 1) % allImages.value.length;
  selectedImage.value = allImages.value[currentImageIndex.value];
};

const prevImage = () => {
  currentImageIndex.value = currentImageIndex.value === 0 
    ? allImages.value.length - 1 
    : currentImageIndex.value - 1;
  selectedImage.value = allImages.value[currentImageIndex.value];
};

// Handle keyboard navigation
const handleKeydown = (event) => {
  if (!lightboxOpen.value) return;
  
  switch (event.key) {
    case 'Escape':
      closeLightbox();
      break;
    case 'ArrowLeft':
      prevImage();
      break;
    case 'ArrowRight':
      nextImage();
      break;
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown);
  document.body.style.overflow = 'auto';
});
</script>

<template>
    <main class="gallery-page">
        <!-- Hero Section -->
        <section class="gallery-hero">
            <div class="hero-content">
                <h1 class="title">Gallery</h1>
                <p class="hero-description">
                    Explore moments from our training sessions, competitions, social events, and tours. 
                    See what makes ICSM Badminton such a vibrant and welcoming community.
                </p>
            </div>
            <div class="hero-stats">
                <div class="stat-item">
                    <span class="stat-number">{{ allImages.length }}+</span>
                    <span class="stat-label">Photos</span>
                </div>
                <div class="stat-item">
                    <span class="stat-number">{{ galleryCategories.length }}</span>
                    <span class="stat-label">Categories</span>
                </div>
                <div class="stat-item">
                    <span class="stat-number">2024</span>
                    <span class="stat-label">Season</span>
                </div>
            </div>
        </section>

        <!-- Category Filter -->
        <section class="filter-section">
            <div class="filter-container">
                <h3 class="filter-title">Browse by Category</h3>
                <div class="category-filters">
                    <button 
                        @click="selectedCategory = 'all'"
                        class="filter-btn"
                        :class="{ active: selectedCategory === 'all' }"
                    >
                        🏸 All Photos
                    </button>
                    <button 
                        v-for="category in galleryCategories"
                        :key="category.id"
                        @click="selectedCategory = category.id"
                        class="filter-btn"
                        :class="{ active: selectedCategory === category.id }"
                    >
                        {{ category.name }}
                    </button>
                </div>
            </div>
        </section>

        <!-- Category Description -->
        <section v-if="selectedCategory !== 'all'" class="category-description">
            <div class="description-container">
                <h2 class="category-title">
                    {{ galleryCategories.find(cat => cat.id === selectedCategory)?.name }}
                </h2>
                <p class="category-text">
                    {{ galleryCategories.find(cat => cat.id === selectedCategory)?.description }}
                </p>
            </div>
        </section>

        <!-- Gallery Grid -->
        <section class="gallery-section">
            <div class="gallery-container">
                <div v-if="filteredImages.length === 0" class="empty-gallery">
                    <h3>No photos found</h3>
                    <p>There are no photos in this category yet.</p>
                </div>
                
                <div v-else class="gallery-grid">
                    <div 
                        v-for="(image, index) in filteredImages" 
                        :key="`${image.src}-${index}`"
                        class="gallery-item"
                        @click="openLightbox(image, index)"
                    >
                        <div class="image-container">
                            <img 
                                :src="image.src" 
                                :alt="image.alt"
                                class="gallery-image"
                                loading="lazy"
                            >
                            <div class="image-overlay">
                                <div class="overlay-content">
                                    <span class="overlay-icon">🔍</span>
                                    <span class="overlay-text">View Photo</span>
                                </div>
                            </div>
                        </div>
                        <div class="image-info">
                            <p class="image-caption">{{ image.caption }}</p>
                            <span class="image-category">{{ image.category }}</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <!-- Call to Action -->
        <section class="gallery-cta">
            <div class="cta-content">
                <h2>Want to Be Part of Our Story?</h2>
                <p>Join ICSM Badminton and create memories that will last a lifetime. From training sessions to tournaments, social events to tours - there's always something happening!</p>
                <div class="cta-actions">
                    <NuxtLink to="/membership" class="cta-btn primary">
                        Join the Club
                    </NuxtLink>
                    <NuxtLink to="/sessions" class="cta-btn secondary">
                        View Training Sessions
                    </NuxtLink>
                </div>
            </div>
        </section>

        <!-- Lightbox Modal -->
        <Teleport to="body">
            <div v-if="lightboxOpen" class="lightbox-overlay" @click="closeLightbox">
                <div class="lightbox-container" @click.stop>
                    <button class="lightbox-close" @click="closeLightbox">
                        ×
                    </button>
                    
                    <button 
                        v-if="allImages.length > 1"
                        class="lightbox-nav prev" 
                        @click="prevImage"
                    >
                        ‹
                    </button>
                    
                    <button 
                        v-if="allImages.length > 1"
                        class="lightbox-nav next" 
                        @click="nextImage"
                    >
                        ›
                    </button>
                    
                    <div class="lightbox-content">
                        <img 
                            v-if="selectedImage"
                            :src="selectedImage.src" 
                            :alt="selectedImage.alt"
                            class="lightbox-image"
                        >
                        <div v-if="selectedImage" class="lightbox-info">
                            <h3 class="lightbox-caption">{{ selectedImage.caption }}</h3>
                            <p class="lightbox-category">{{ selectedImage.category }}</p>
                            <p class="lightbox-counter">
                                {{ currentImageIndex + 1 }} of {{ allImages.length }}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </Teleport>
    </main>
</template>

<style scoped>
.gallery-page {
    min-height: 100vh;
    background: linear-gradient(135deg, #f8fffe 0%, #f0f8ef 100%);
}

/* Hero Section */
.gallery-hero {
    background: linear-gradient(135deg, var(--accent-colour) 0%, #2d4232 100%);
    color: white;
    padding: 4rem 2rem 3rem;
    text-align: center;
}

.hero-content {
    max-width: 800px;
    margin: 0 auto 3rem;
}

.hero-content .title {
    color: white;
    font-size: 3.5rem;
    margin-bottom: 1.5rem;
    text-transform: uppercase;
}

.hero-description {
    font-size: 1.2rem;
    line-height: 1.6;
    opacity: 0.9;
    max-width: 600px;
    margin: 0 auto;
}

.hero-stats {
    display: flex;
    justify-content: center;
    gap: 4rem;
    margin-top: 2rem;
}

.stat-item {
    text-align: center;
}

.stat-number {
    display: block;
    font-size: 2.5rem;
    font-weight: 700;
    color: #a3d9a5;
    margin-bottom: 0.5rem;
}

.stat-label {
    font-size: 1rem;
    opacity: 0.8;
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

/* Filter Section */
.filter-section {
    padding: 3rem 2rem 2rem;
    background: white;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.filter-container {
    max-width: 1200px;
    margin: 0 auto;
    text-align: center;
}

.filter-title {
    color: var(--dash-txt-colour);
    font-size: 1.3rem;
    margin-bottom: 2rem;
    font-weight: 700;
}

.category-filters {
    display: flex;
    justify-content: center;
    gap: 1rem;
    flex-wrap: wrap;
}

.filter-btn {
    padding: 0.75rem 1.5rem;
    border: 2px solid var(--field-border-colour);
    background: white;
    color: var(--dash-txt-colour);
    border-radius: var(--radius-s);
    cursor: pointer;
    font-weight: 600;
    transition: all 0.2s ease;
    font-size: 0.95rem;
}

.filter-btn:hover {
    border-color: var(--accent-colour);
    background: var(--bg-20-colour);
}

.filter-btn.active {
    background: var(--accent-colour);
    border-color: var(--accent-colour);
    color: white;
}

/* Category Description */
.category-description {
    padding: 2rem;
    background: linear-gradient(135deg, var(--bg-20-colour) 0%, white 100%);
    border-bottom: 1px solid var(--field-border-colour);
}

.description-container {
    max-width: 800px;
    margin: 0 auto;
    text-align: center;
}

.category-title {
    color: var(--accent-colour);
    font-size: 2rem;
    margin-bottom: 1rem;
    font-weight: 700;
}

.category-text {
    color: var(--dash-txt-colour);
    font-size: 1.1rem;
    line-height: 1.6;
    margin: 0;
}

/* Gallery Section */
.gallery-section {
    padding: 3rem 2rem;
}

.gallery-container {
    max-width: 1200px;
    margin: 0 auto;
}

.empty-gallery {
    text-align: center;
    padding: 4rem 2rem;
    color: var(--grey-txt-colour);
}

.gallery-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 2rem;
}

.gallery-item {
    background: white;
    border-radius: var(--radius-m);
    overflow: hidden;
    box-shadow: 0 4px 12px rgba(0,0,0,0.1);
    transition: all 0.3s ease;
    cursor: pointer;
}

.gallery-item:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(0,0,0,0.15);
}

.image-container {
    position: relative;
    padding-bottom: 75%; /* 4:3 Aspect Ratio */
    overflow: hidden;
}

.gallery-image {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.3s ease;
}

.gallery-item:hover .gallery-image {
    transform: scale(1.05);
}

.image-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(52, 78, 65, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 0.3s ease;
}

.gallery-item:hover .image-overlay {
    opacity: 1;
}

.overlay-content {
    text-align: center;
    color: white;
}

.overlay-icon {
    display: block;
    font-size: 2rem;
    margin-bottom: 0.5rem;
}

.overlay-text {
    font-weight: 600;
    font-size: 1.1rem;
}

.image-info {
    padding: 1.5rem;
}

.image-caption {
    margin: 0 0 0.5rem 0;
    color: var(--dash-txt-colour);
    font-weight: 600;
    line-height: 1.4;
}

.image-category {
    color: var(--grey-txt-colour);
    font-size: 0.9rem;
    font-weight: 500;
}

/* CTA Section */
.gallery-cta {
    background: linear-gradient(135deg, var(--bg-20-colour) 0%, #e8f5e8 100%);
    padding: 4rem 2rem;
    text-align: center;
}

.cta-content {
    max-width: 800px;
    margin: 0 auto;
}

.cta-content h2 {
    color: var(--accent-colour);
    font-size: 2.5rem;
    margin-bottom: 1.5rem;
    font-weight: 700;
}

.cta-content p {
    color: var(--dash-txt-colour);
    font-size: 1.1rem;
    line-height: 1.6;
    margin-bottom: 2.5rem;
}

.cta-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
    flex-wrap: wrap;
}

.cta-btn {
    padding: 1rem 2rem;
    text-decoration: none;
    border-radius: var(--radius-s);
    font-weight: 600;
    font-size: 1.1rem;
    transition: all 0.2s ease;
}

.cta-btn.primary {
    background: var(--accent-colour);
    color: white;
}

.cta-btn.primary:hover {
    background: var(--btn-11-colour);
    transform: translateY(-1px);
}

.cta-btn.secondary {
    background: white;
    color: var(--accent-colour);
    border: 2px solid var(--accent-colour);
}

.cta-btn.secondary:hover {
    background: var(--accent-colour);
    color: white;
}

/* Lightbox */
.lightbox-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.9);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 2rem;
}

.lightbox-container {
    position: relative;
    max-width: 90vw;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.lightbox-close {
    position: absolute;
    top: -60px;
    right: 0;
    background: none;
    border: none;
    color: white;
    font-size: 3rem;
    cursor: pointer;
    z-index: 1001;
    width: 50px;
    height: 50px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    transition: background 0.2s ease;
}

.lightbox-close:hover {
    background: rgba(255, 255, 255, 0.1);
}

.lightbox-nav {
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
    background: rgba(255, 255, 255, 0.1);
    border: none;
    color: white;
    font-size: 3rem;
    cursor: pointer;
    width: 60px;
    height: 60px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.2s ease;
    z-index: 1001;
}

.lightbox-nav:hover {
    background: rgba(255, 255, 255, 0.2);
}

.lightbox-nav.prev {
    left: -80px;
}

.lightbox-nav.next {
    right: -80px;
}

.lightbox-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    max-width: 100%;
    max-height: 100%;
}

.lightbox-image {
    max-width: 100%;
    max-height: 70vh;
    object-fit: contain;
    border-radius: var(--radius-s);
}

.lightbox-info {
    background: rgba(255, 255, 255, 0.95);
    padding: 1.5rem;
    border-radius: var(--radius-s);
    margin-top: 1rem;
    text-align: center;
    min-width: 300px;
}

.lightbox-caption {
    margin: 0 0 0.5rem 0;
    color: var(--dash-txt-colour);
    font-size: 1.1rem;
    font-weight: 600;
}

.lightbox-category {
    margin: 0 0 0.5rem 0;
    color: var(--grey-txt-colour);
    font-size: 0.9rem;
}

.lightbox-counter {
    margin: 0;
    color: var(--grey-txt-colour);
    font-size: 0.85rem;
    font-style: italic;
}

/* Responsive Design */
@media (max-width: 768px) {
    .hero-content .title {
        font-size: 2.5rem;
    }
    
    .hero-stats {
        gap: 2rem;
    }
    
    .stat-number {
        font-size: 2rem;
    }
    
    .category-filters {
        gap: 0.5rem;
    }
    
    .filter-btn {
        padding: 0.5rem 1rem;
        font-size: 0.9rem;
    }
    
    .gallery-grid {
        grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
        gap: 1.5rem;
    }
    
    .cta-content h2 {
        font-size: 2rem;
    }
    
    .cta-actions {
        flex-direction: column;
        align-items: center;
    }
    
    .lightbox-nav {
        display: none;
    }
    
    .lightbox-close {
        top: -40px;
        font-size: 2rem;
        width: 40px;
        height: 40px;
    }
    
    .lightbox-info {
        min-width: unset;
        width: 100%;
        margin-top: 0.5rem;
        padding: 1rem;
    }
}

@media (max-width: 480px) {
    .gallery-section {
        padding: 2rem 1rem;
    }
    
    .filter-section {
        padding: 2rem 1rem;
    }
    
    .gallery-grid {
        grid-template-columns: 1fr;
    }
    
    .hero-stats {
        flex-direction: column;
        gap: 1.5rem;
    }
    
    .category-filters {
        flex-direction: column;
        align-items: center;
    }
    
    .filter-btn {
        width: 100%;
        max-width: 250px;
    }
}
</style>