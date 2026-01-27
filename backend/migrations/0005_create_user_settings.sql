-- Create user_settings table
CREATE TABLE IF NOT EXISTS user_settings (
    user_id TEXT PRIMARY KEY,
    default_tax_rate REAL DEFAULT 19.0,
    currency TEXT DEFAULT 'EUR',
    invoice_prefix TEXT DEFAULT 'INV',
    next_invoice_number INTEGER DEFAULT 1,
    company_logo_url TEXT,
    payment_terms_days INTEGER DEFAULT 14,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Insert default settings for existing users
INSERT INTO user_settings (user_id, default_tax_rate, currency, invoice_prefix, next_invoice_number)
SELECT id, 19.0, 'EUR', 'INV', 1 
FROM users 
WHERE id NOT IN (SELECT user_id FROM user_settings);