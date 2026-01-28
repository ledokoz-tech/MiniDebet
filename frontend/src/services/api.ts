import { User, Client, Invoice, CreateUserRequest, CreateClientRequest, CreateInvoiceRequest } from '../../../shared/src';

// Use environment variable for production, proxy for development
const API_BASE_URL = import.meta.env.PROD 
  ? (import.meta.env.VITE_API_BASE_URL || 'https://minidebet.jamshaidnasar.workers.dev/api')
  : '/api';

class ApiService {
  private getToken(): string | null {
    return localStorage.getItem('authToken');
  }

  private getAuthHeaders() {
    const token = this.getToken();
    return {
      'Content-Type': 'application/json',
      ...(token && { 'Authorization': `Bearer ${token}` }),
    };
  }

  // User APIs
  async createUser(userData: CreateUserRequest): Promise<User> {
    const response = await fetch(`${API_BASE_URL}/users`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(userData),
    });

    if (!response.ok) {
      throw new Error('Failed to create user');
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