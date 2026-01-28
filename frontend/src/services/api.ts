import { User, Client, Invoice, CreateUserRequest, CreateClientRequest, CreateInvoiceRequest } from '../../../shared/src';

// Use environment variable for production, proxy for development
const API_BASE_URL = import.meta.env.PROD 
  ? (import.meta.env.VITE_API_BASE_URL || 'https://minidebet.jamshaidnasar.workers.dev')
  : '';

class ApiService {
  private getToken(): string | null {
    return localStorage.getItem('authToken');
  }

  private getAuthHeaders(includeAuth: boolean = true) {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };
    
    if (includeAuth) {
      const token = this.getToken();
      if (token) {
        headers['Authorization'] = `Bearer ${token}`;
      }
    }
    
    return headers;
  }

  // Auth APIs
  async login(email: string, password: string): Promise<{ user: User; token: string }> {
    const response = await fetch(`${API_BASE_URL}/api/auth/login`, {
      method: 'POST',
      headers: this.getAuthHeaders(false),
      body: JSON.stringify({ email, password }),
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({}));
      throw new Error(error.message || 'Login failed');
    }

    return response.json();
  }

  async register(userData: CreateUserRequest): Promise<{ user: User; token: string }> {
    const response = await fetch(`${API_BASE_URL}/api/auth/register`, {
      method: 'POST',
      headers: this.getAuthHeaders(false),
      body: JSON.stringify(userData),
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({}));
      throw new Error(error.message || 'Registration failed');
    }

    return response.json();
  }

  async getCurrentUser(): Promise<User> {
    const response = await fetch(`${API_BASE_URL}/api/users/me`, {
      method: 'GET',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch user data');
    }

    return response.json();
  }

  async logout(): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/api/auth/logout`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Logout failed');
    }
  }

  // User APIs
  async updateUser(userData: Partial<User>): Promise<User> {
    const response = await fetch(`${API_BASE_URL}/api/users/me`, {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(userData),
    });

    if (!response.ok) {
      throw new Error('Failed to update user');
    }

    return response.json();
  }

  // Client APIs
  async getClients(): Promise<Client[]> {
    const response = await fetch(`${API_BASE_URL}/clients`, {
      method: 'GET',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch clients');
    }

    return response.json();
  }

  async createClient(clientData: CreateClientRequest): Promise<Client> {
    const response = await fetch(`${API_BASE_URL}/clients`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(clientData),
    });

    if (!response.ok) {
      throw new Error('Failed to create client');
    }

    return response.json();
  }

  // Invoice APIs
  async getInvoices(): Promise<Invoice[]> {
    const response = await fetch(`${API_BASE_URL}/invoices`, {
      method: 'GET',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch invoices');
    }

    return response.json();
  }

  async createInvoice(invoiceData: CreateInvoiceRequest): Promise<Invoice> {
    const response = await fetch(`${API_BASE_URL}/invoices`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(invoiceData),
    });

    if (!response.ok) {
      throw new Error('Failed to create invoice');
    }

    return response.json();
  }

  async getInvoice(id: string): Promise<Invoice> {
    const response = await fetch(`${API_BASE_URL}/invoices/${id}`, {
      method: 'GET',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch invoice');
    }

    return response.json();
  }
}

export const apiService = new ApiService();