// Shared types between frontend and backend

export interface User {
  id: string;
  email: string;
  firstName?: string;
  lastName?: string;
  companyName?: string;
  taxId?: string;
  createdAt: string;
  updatedAt: string;
}

export interface Client {
  id: string;
  userId: string;
  name: string;
  email?: string;
  company?: string;
  street?: string;
  city?: string;
  postalCode?: string;
  country: string;
  vatNumber?: string;
  createdAt: string;
  updatedAt: string;
}

export interface InvoiceItem {
  id: string;
  invoiceId: string;
  description: string;
  quantity: number;
  unitPrice: number;
  totalPrice: number;
  createdAt: string;
}

export interface Invoice {
  id: string;
  userId: string;
  clientId: string;
  invoiceNumber: string;
  issueDate: string;
  dueDate: string;
  currency: string;
  subtotal: number;
  taxRate: number;
  taxAmount: number;
  totalAmount: number;
  status: 'draft' | 'sent' | 'paid' | 'overdue' | 'cancelled';
  notes?: string;
  pdfUrl?: string;
  sentAt?: string;
  paidAt?: string;
  createdAt: string;
  updatedAt: string;
  items: InvoiceItem[];
}

export interface UserSettings {
  userId: string;
  defaultTaxRate: number;
  currency: string;
  invoicePrefix: string;
  nextInvoiceNumber: number;
  companyLogoUrl?: string;
  paymentTermsDays: number;
  updatedAt: string;
}

// API Request/Response types
export interface CreateUserRequest {
  email: string;
  password: string;
  firstName?: string;
  lastName?: string;
  companyName?: string;
  taxId?: string;
}

export interface CreateClientRequest {
  name: string;
  email?: string;
  company?: string;
  street?: string;
  city?: string;
  postalCode?: string;
  country?: string;
  vatNumber?: string;
}

export interface CreateInvoiceRequest {
  clientId: string;
  issueDate: string;
  dueDate: string;
  currency?: string;
  notes?: string;
  items: Array<{
    description: string;
    quantity: number;
    unitPrice: number;
  }>;
}

export interface AuthResponse {
  user: User;
  token: string;
}