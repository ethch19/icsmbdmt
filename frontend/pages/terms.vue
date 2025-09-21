<script setup lang="ts">
definePageMeta({
    layout: "default",
    title: 'Terms of Use - ICSM Badminton'
});
</script>

<template>
    <main class="legal-page">
        <div class="legal-container">
            <h1 class="title">Terms of Use</h1>
            <p class="last-updated">Last updated: December 2024</p>
            
            <div class="legal-content">
                <section class="legal-section">
                    <h2>1. Acceptance of Terms</h2>
                    <p>By accessing and using the ICSM Badminton website and services, you agree to be bound by these Terms of Use and our Privacy Policy. If you do not agree to these terms, please do not use our services.</p>
                </section>

                <section class="legal-section">
                    <h2>2. Membership and Eligibility</h2>
                    <ul>
                        <li>Membership is open to Imperial College School of Medicine students, staff, and alumni</li>
                        <li>You must provide accurate and complete information during registration</li>
                        <li>You are responsible for maintaining the confidentiality of your account</li>
                        <li>Membership tiers are determined by your status with Imperial College Union</li>
                    </ul>
                </section>

                <section class="legal-section">
                    <h2>3. Session Bookings</h2>
                    <ul>
                        <li>Bookings are subject to availability and membership tier requirements</li>
                        <li>You may only book sessions you are eligible to attend</li>
                        <li>Cancellations should be made as early as possible to allow others to book</li>
                        <li>Repeated no-shows may result in booking restrictions</li>
                        <li>The club reserves the right to cancel sessions due to circumstances beyond our control</li>
                    </ul>
                </section>

                <section class="legal-section">
                    <h2>4. Code of Conduct</h2>
                    <p>All members must:</p>
                    <ul>
                        <li>Treat other members, staff, and facilities with respect</li>
                        <li>Follow all safety guidelines and facility rules</li>
                        <li>Use appropriate language and behavior at all times</li>
                        <li>Report any incidents or concerns to the club committee</li>
                        <li>Comply with Imperial College Union policies and procedures</li>
                    </ul>
                </section>

                <section class="legal-section">
                    <h2>5. Health and Safety</h2>
                    <ul>
                        <li>Participation in badminton activities involves inherent risks</li>
                        <li>Members participate at their own risk and should ensure they are physically fit</li>
                        <li>Inform instructors of any medical conditions that may affect participation</li>
                        <li>Follow all safety instructions and wear appropriate equipment</li>
                        <li>The club is not liable for injuries sustained during activities</li>
                    </ul>
                </section>

                <section class="legal-section">
                    <h2>6. Website Usage</h2>
                    <ul>
                        <li>Use the website only for its intended purposes</li>
                        <li>Do not attempt to gain unauthorized access to any systems</li>
                        <li>Do not use the platform to harass, spam, or disturb other users</li>
                        <li>Report any technical issues or security concerns immediately</li>
                    </ul>
                </section>

                <section class="legal-section">
                    <h2>7. Intellectual Property</h2>
                    <ul>
                        <li>The website content, design, and functionality are owned by ICSM Badminton</li>
                        <li>You may not reproduce, distribute, or modify content without permission</li>
                        <li>User-generated content remains your property but you grant us license to use it for club purposes</li>
                    </ul>
                </section>

                <section class="legal-section">
                    <h2>8. Payments and Refunds</h2>
                    <ul>
                        <li>Membership fees are processed through Imperial College Union</li>
                        <li>Refunds are subject to Imperial College Union policies</li>
                        <li>Additional fees for events or equipment may apply</li>
                        <li>Payment terms will be clearly stated for any chargeable services</li>
                    </ul>
                </section>

                <section class="legal-section">
                    <h2>9. Suspension and Termination</h2>
                    <p>We reserve the right to suspend or terminate accounts for:</p>
                    <ul>
                        <li>Violation of these terms or club policies</li>
                        <li>Inappropriate behavior or conduct</li>
                        <li>Fraudulent or unauthorized use of services</li>
                        <li>Non-payment of required fees</li>
                    </ul>
                </section>

                <section class="legal-section">
                    <h2>10. Limitation of Liability</h2>
                    <p>ICSM Badminton and its committee members are not liable for:</p>
                    <ul>
                        <li>Personal injuries sustained during club activities</li>
                        <li>Loss or damage to personal property</li>
                        <li>Technical issues or website downtime</li>
                        <li>Decisions made by Imperial College Union regarding membership</li>
                    </ul>
                </section>

                <section class="legal-section">
                    <h2>11. Changes to Terms</h2>
                    <p>We may update these Terms of Use periodically. Significant changes will be communicated to members via email or website notifications. Continued use after changes indicates acceptance of the new terms.</p>
                </section>

                <section class="legal-section">
                    <h2>12. Governing Law</h2>
                    <p>These terms are governed by English law. Any disputes will be resolved through Imperial College Union procedures where applicable, or through appropriate legal channels.</p>
                </section>

                <section class="legal-section">
                    <h2>13. Contact Information</h2>
                    <p>For questions about these terms, please contact:</p>
                    <ul>
                        <li><strong>Email:</strong> badminton@icsm.org.uk</li>
                        <li><strong>Club Committee:</strong> At club events or through our website</li>
                        <li><strong>Imperial College Union:</strong> For membership or policy issues</li>
                    </ul>
                </section>
            </div>
        </div>
    </main>
</template>

<style scoped>
.legal-page {
    min-height: 100vh;
    background: linear-gradient(135deg, #f8fffe 0%, #f0f8ef 100%);
    padding: 2rem 0;
}

.legal-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 0 2rem;
}

.title {
    color: var(--accent-colour);
    text-align: center;
    margin-bottom: 1rem;
    font-size: 2.5rem;
}

.last-updated {
    text-align: center;
    color: var(--grey-txt-colour);
    font-style: italic;
    margin-bottom: 3rem;
    font-size: 0.9rem;
}

.legal-content {
    background: white;
    padding: 3rem;
    border-radius: var(--radius-m);
    box-shadow: 0 4px 12px rgba(0,0,0,0.1);
    border: 1px solid var(--field-border-colour);
}

.legal-section {
    margin-bottom: 2.5rem;
}

.legal-section:last-child {
    margin-bottom: 0;
}

.legal-section h2 {
    color: var(--accent-colour);
    font-size: 1.3rem;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 2px solid var(--bg-20-colour);
}

.legal-section p {
    color: var(--dash-txt-colour);
    line-height: 1.6;
    margin-bottom: 1rem;
}

.legal-section ul {
    color: var(--dash-txt-colour);
    line-height: 1.6;
    margin-bottom: 1rem;
    padding-left: 1.5rem;
}

.legal-section li {
    margin-bottom: 0.5rem;
}

.legal-section strong {
    color: var(--accent-colour);
    font-weight: 600;
}

@media (max-width: 768px) {
    .legal-container {
        padding: 0 1rem;
    }
    
    .legal-content {
        padding: 2rem 1.5rem;
    }
    
    .title {
        font-size: 2rem;
    }
    
    .legal-section h2 {
        font-size: 1.2rem;
    }
}

@media (max-width: 480px) {
    .legal-content {
        padding: 1.5rem 1rem;
    }
    
    .title {
        font-size: 1.8rem;
    }
    
    .legal-section ul {
        padding-left: 1rem;
    }
}
</style>