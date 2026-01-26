<style>
    body {
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        color: #1f2937;
        line-height: 1.6;
        background-color: #f9fafb;
        margin: 0;
        padding: 0 2rem;
    }
    h1, h2, h3 {
        font-weight: bold;
        margin-bottom: 0.5rem;
    }
    h1 {
        font-size: 3em;
        color: #1f2937;
    }
    h2 {
        color: #4a90e2;
        border-bottom: 2px solid #e5e7eb;
        padding-bottom: 0.3rem;
        margin-top: 2rem;
    }
    h3 {
        color: #10b981;
        margin-top: 1.5rem;
    }
    p {
        font-size: 1.05em;
        margin-bottom: 1rem;
    }
    ul {
        margin-left: 1.5rem;
    }
    blockquote {
        border-left: 5px solid #4a90e2;
        padding-left: 1rem;
        color: #374151;
        font-style: italic;
        background-color: #e0f2fe;
        margin: 1rem 0;
        border-radius: 5px;
    }
    .center {
        text-align: center;
    }
    .card {
        background-color: #fff;
        border-radius: 12px;
        padding: 1rem 1.5rem;
        margin: 1rem 0;
        box-shadow: 0 4px 8px rgba(0,0,0,0.05);
    }
    .highlight {
        background-color: #fef3c7;
        padding: 0.3rem 0.5rem;
        border-radius: 5px;
        font-weight: bold;
    }
    .button {
        display: inline-block;
        background-color: #4a90e2;
        color: #fff;
        padding: 0.5rem 1rem;
        border-radius: 6px;
        text-decoration: none;
        font-weight: bold;
        margin-top: 0.5rem;
    }
    code {
        background-color: #f3f4f6;
        padding: 0.2rem 0.4rem;
        border-radius: 4px;
        font-family: monospace;
    }
    table {
        width: 100%;
        border-collapse: collapse;
        margin: 1rem 0;
    }
    th, td {
        border: 1px solid #e5e7eb;
        padding: 0.5rem 1rem;
        text-align: left;
    }
    th {
        background-color: #f3f4f6;
    }
</style>
</head>
<body>

<div class="center">
    <h1>MiniDebet</h1>
    <p style="font-size:1.2em; color:gray;"><em>The boring, high-performance invoicing SaaS for German freelancers</em></p>
    <p><em>[Frontend: React] · [Backend: Rust] · [DB: PostgreSQL]</em></p>
</div>

<div class="card">
    <h2>🚀 Mission</h2>
    <p>MiniDebet is a utility-first financial tool. We are not trying to reinvent accounting. We are building a specialized, high-performance engine that solves exactly three problems for German freelancers:</p>
    <ul>
        <li><strong>Legally compliant invoices</strong> (GoBD &amp; §19 UStG ready)</li>
        <li><strong>Anxiety-free tax estimation</strong> (VAT vs. Flat tax)</li>
        <li><strong>Clean data exports</strong> for the Steuerberater</li>
    </ul>
    <p><em>Why this exists:</em> To generate predictable, recurring revenue that funds the bigger vision (<code>LedokozOS</code>).</p>
    <blockquote>"Boring technology pays for the future."</blockquote>
</div>

<div class="card">
    <h2>⚙️ Tech Stack</h2>

<h3>🎨 Frontend (Client Side)</h3>
<ul>
        <li>Framework: React (v18+)</li>
        <li>Styling: Tailwind CSS (Clean, minimal, system-first aesthetic)</li>
        <li>State: Zustand / React Query (Server state management)</li>
        <li>Build: Vite</li>
    </ul>

 <h3>⚡ Backend (Server Side)</h3>
    <ul>
        <li>Language: Rust</li>
        <li>Framework: Axum</li>
        <li>Async Runtime: Tokio</li>
        <li>Memory safety + low latency + strict type enforcement</li>
    </ul>

 <h3>💾 Data & Infrastructure</h3>
    <ul>
        <li>Database: PostgreSQL (ACID compliant)</li>
        <li>ORM: Diesel / SeaORM</li>
        <li>Hosting: Hetzner Cloud</li>
        <li>Payments: Stripe (Webhooks handled by Rust)</li>
        <li>Files: S3-compatible object storage for PDF archiving</li>
    </ul>
</div>

<div class="card">
    <h2>🎯 Target Audience</h2>
    <p><strong>✅ Who this is for:</strong></p>
    <ul>
        <li>Selbstständige (Sole proprietors) in Germany</li>
        <li>Freelancers in Tech, Design, Consulting</li>
        <li>Users tracking money in Excel or paper</li>
        <li>Needs: Invoicing + basic expense tracking</li>
    </ul>
    <p><strong>🚫 Who this is NOT for:</strong></p>
    <ul>
        <li>GmbHs / AGs</li>
        <li>Merchants (Handelsregistereintragung)</li>
        <li>Companies needing payroll (Lohnabrechnung) or complex depreciation</li>
    </ul>
</div>

<div class="card">
    <h2>💼 MVP Feature Scope</h2>
    <ul>
        <li><span class="highlight">Authentication:</span> Email/Password (JWT tokens, secure sessions)</li>
        <li><span class="highlight">Dashboard:</span> Revenue overview, open invoices, quick "Create New Invoice"</li>
        <li><span class="highlight">Invoice Builder:</span> Customers, line items, auto-totals, PDF generation, download/email</li>
        <li><span class="highlight">Settings:</span> Kleinunternehmer toggle, company data, tax number input</li>
        <li><span class="highlight">Exports:</span> CSV for DATEV import</li>
    </ul>
</div>

<div class="card center">
    <h2>💳 Pricing</h2>
    <p>Simple flat pricing:</p>
    <ul>
        <li>Free Tier: 3 invoices per month</li>
        <li>Pro (€15/mo): Unlimited invoices + PDF branding + Exports</li>
    </ul>
    <p>Accepted: Credit Card & SEPA via Stripe</p>
    <a href="#" class="button">Subscribe Now</a>
</div>

</body>
</html>
