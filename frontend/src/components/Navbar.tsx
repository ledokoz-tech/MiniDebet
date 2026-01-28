import React from 'react';
import { useAuth } from '../contexts/AuthContext';

const Navbar: React.FC = () => {
  const { user, logout } = useAuth();

  return (
    <nav className="bg-white shadow-sm border-b border-secondary-200">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between h-16">
          <div className="flex">
            <div className="flex-shrink-0 flex items-center">
              <div className="flex items-center space-x-2">
                <div className="h-8 w-8 rounded-lg bg-primary-600 flex items-center justify-center">
                  <svg className="h-5 w-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                </div>
                <h1 className="text-xl font-bold text-secondary-900">MiniDebet</h1>
              </div>
            </div>
            <div className="hidden sm:ml-10 sm:flex sm:space-x-8">
              <a
                href="#"
                className="border-primary-500 text-secondary-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium transition-colors duration-200"
              >
                Dashboard
              </a>
              <a
                href="#"
                className="border-transparent text-secondary-500 hover:border-secondary-300 hover:text-secondary-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium transition-colors duration-200"
              >
                Clients
              </a>
              <a
                href="#"
                className="border-transparent text-secondary-500 hover:border-secondary-300 hover:text-secondary-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium transition-colors duration-200"
              >
                Invoices
              </a>
              <a
                href="#"
                className="border-transparent text-secondary-500 hover:border-secondary-300 hover:text-secondary-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium transition-colors duration-200"
              >
                Settings
              </a>
            </div>
          </div>
          
          <div className="flex items-center">
            <div className="flex-shrink-0 flex items-center space-x-4">
              <div className="hidden md:block">
                <span className="text-sm text-secondary-600 mr-2">
                  Welcome,
                </span>
                <span className="text-sm font-medium text-secondary-900">
                  {user?.firstName} {user?.lastName}
                </span>
              </div>
              <button
                onClick={logout}
                className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-lg text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 transition duration-200 transform hover:scale-[1.02] active:scale-[0.98]"
              >
                <svg className="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                </svg>
                Logout
              </button>
            </div>
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;