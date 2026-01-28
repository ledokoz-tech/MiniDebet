import { useState, useEffect } from 'react';
import { useAuth } from '../contexts/AuthContext';
import { apiService } from '../services/api';
import { Invoice, Client } from '../../../shared/src';
import Navbar from '../components/Navbar';

const Dashboard: React.FC = () => {
  const { user } = useAuth();
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [clients, setClients] = useState<Client[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [invoicesData, clientsData] = await Promise.all([
          apiService.getInvoices(),
          apiService.getClients(),
        ]);
        setInvoices(invoicesData);
        setClients(clientsData);
      } catch (error) {
        console.error('Failed to fetch data:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'paid':
        return 'bg-emerald-100 text-emerald-800 border border-emerald-200';
      case 'sent':
        return 'bg-blue-100 text-blue-800 border border-blue-200';
      case 'overdue':
        return 'bg-rose-100 text-rose-800 border border-rose-200';
      default:
        return 'bg-gray-100 text-gray-800 border border-gray-200';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'paid':
        return (
          <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
          </svg>
        );
      case 'sent':
        return (
          <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
          </svg>
        );
      case 'overdue':
        return (
          <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
          </svg>
        );
      default:
        return (
          <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
          </svg>
        );
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-primary-50 to-secondary-50">
        <Navbar />
        <div className="flex items-center justify-center h-96">
          <div className="text-center">
            <div className="h-16 w-16 rounded-full bg-primary-600 flex items-center justify-center mx-auto mb-4 shadow-lg animate-pulse-slow">
              <svg className="h-8 w-8 text-white animate-spin" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <p className="text-secondary-600 font-medium">Loading dashboard...</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-primary-50 to-secondary-50">
      <Navbar />
      
      <div className="max-w-7xl mx-auto py-8 sm:px-6 lg:px-8">
        {/* Welcome Section */}
        <div className="px-4 sm:px-0">
          <div className="mb-10">
            <div className="flex items-center mb-2">
              <div className="h-10 w-10 rounded-full bg-gradient-to-r from-primary-500 to-secondary-500 flex items-center justify-center mr-3 shadow-md">
                <span className="text-white font-bold text-lg">{user?.firstName?.charAt(0)}</span>
              </div>
              <div>
                <h1 className="text-3xl font-bold text-gray-900">
                  Welcome back, {user?.firstName}!
                </h1>
                <p className="text-secondary-600 mt-1">
                  Here's what's happening with your invoices today.
                </p>
              </div>
            </div>
          </div>

          {/* Stats Cards */}
          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4 mb-10">
            <div className="bg-white rounded-xl shadow-sm hover:shadow-md transition-all duration-300 border border-gray-100 overflow-hidden transform hover:-translate-y-1">
              <div className="px-5 py-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0 h-12 w-12 rounded-lg bg-gradient-to-r from-primary-500 to-primary-600 flex items-center justify-center shadow-md">
                    <svg className="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                  </div>
                  <div className="ml-4 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Total Invoices
                      </dt>
                      <dd className="flex items-baseline">
                        <div className="text-2xl font-bold text-gray-900">
                          {invoices.length}
                        </div>
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-xl shadow-sm hover:shadow-md transition-all duration-300 border border-gray-100 overflow-hidden transform hover:-translate-y-1">
              <div className="px-5 py-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0 h-12 w-12 rounded-lg bg-gradient-to-r from-emerald-500 to-emerald-600 flex items-center justify-center shadow-md">
                    <svg className="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                  </div>
                  <div className="ml-4 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Paid Invoices
                      </dt>
                      <dd className="flex items-baseline">
                        <div className="text-2xl font-bold text-gray-900">
                          {invoices.filter(inv => inv.status === 'paid').length}
                        </div>
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-xl shadow-sm hover:shadow-md transition-all duration-300 border border-gray-100 overflow-hidden transform hover:-translate-y-1">
              <div className="px-5 py-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0 h-12 w-12 rounded-lg bg-gradient-to-r from-blue-500 to-blue-600 flex items-center justify-center shadow-md">
                    <svg className="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                  </div>
                  <div className="ml-4 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Pending
                      </dt>
                      <dd className="flex items-baseline">
                        <div className="text-2xl font-bold text-gray-900">
                          {invoices.filter(inv => inv.status === 'sent').length}
                        </div>
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-xl shadow-sm hover:shadow-md transition-all duration-300 border border-gray-100 overflow-hidden transform hover:-translate-y-1">
              <div className="px-5 py-6">
                <div className="flex items-center">
                  <div className="flex-shrink-0 h-12 w-12 rounded-lg bg-gradient-to-r from-amber-500 to-amber-600 flex items-center justify-center shadow-md">
                    <svg className="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                    </svg>
                  </div>
                  <div className="ml-4 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">
                        Clients
                      </dt>
                      <dd className="flex items-baseline">
                        <div className="text-2xl font-bold text-gray-900">
                          {clients.length}
                        </div>
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Recent Invoices */}
          <div className="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden transition-all duration-300 hover:shadow-md">
            <div className="px-6 py-5 border-b border-gray-100">
              <div className="flex items-center justify-between">
                <h3 className="text-lg font-bold text-gray-900 flex items-center">
                  <svg className="h-5 w-5 text-primary-500 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                  Recent Invoices
                </h3>
                <span className="text-sm text-secondary-500 bg-secondary-50 px-3 py-1 rounded-full">
                  Showing {Math.min(5, invoices.length)} of {invoices.length}
                </span>
              </div>
            </div>
            <ul className="divide-y divide-gray-100">
              {invoices.slice(0, 5).map((invoice) => (
                <li key={invoice.id} className="transition-colors duration-200 hover:bg-gray-50">
                  <div className="px-6 py-4">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center">
                        <div className="h-10 w-10 rounded-lg bg-gradient-to-r from-primary-100 to-secondary-100 flex items-center justify-center mr-4">
                          <span className="text-primary-600 font-bold">{invoice.invoiceNumber.charAt(0)}</span>
                        </div>
                        <div>
                          <p className="text-sm font-bold text-primary-600">
                            {invoice.invoiceNumber}
                          </p>
                          <p className="text-xs text-secondary-500 mt-1">
                            Created {new Date(invoice.createdAt).toLocaleDateString()}
                          </p>
                        </div>
                      </div>
                      <div className="flex items-center space-x-4">
                        <div className="text-right">
                          <p className="text-lg font-bold text-gray-900">
                            €{invoice.totalAmount.toFixed(2)}
                          </p>
                          <p className="text-xs text-secondary-500">
                            Due {new Date(invoice.dueDate).toLocaleDateString()}
                          </p>
                        </div>
                        <div className="flex items-center">
                          <span className={`px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full ${getStatusColor(invoice.status)} flex items-center`}>
                            <span className="mr-1.5">{getStatusIcon(invoice.status)}</span>
                            {invoice.status.charAt(0).toUpperCase() + invoice.status.slice(1)}
                          </span>
                        </div>
                      </div>
                    </div>
                  </div>
                </li>
              ))}
              {invoices.length === 0 && (
                <li className="px-6 py-12 text-center">
                  <div className="mx-auto h-16 w-16 rounded-full bg-gray-100 flex items-center justify-center mb-4">
                    <svg className="h-8 w-8 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                  </div>
                  <h3 className="text-lg font-medium text-gray-900 mb-1">No invoices yet</h3>
                  <p className="text-secondary-500">Create your first invoice to get started!</p>
                </li>
              )}
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;