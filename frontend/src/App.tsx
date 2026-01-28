import { useState } from 'react';
import { AuthProvider, useAuth } from './contexts/AuthContext';
import LoginForm from './components/LoginForm';
import RegisterForm from './components/RegisterForm';
import Dashboard from './pages/Dashboard';

function AppContent() {
  const { isAuthenticated, isLoading } = useAuth();
  const [showRegister, setShowRegister] = useState(false);

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 to-primary-100">
        <div className="animate-pulse-slow">
          <div className="h-16 w-16 rounded-full bg-primary-600 flex items-center justify-center mb-4 shadow-lg">
            <svg className="h-8 w-8 text-white animate-spin" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          <div className="text-center">
            <p className="text-secondary-600 font-medium">Loading your account...</p>
          </div>
        </div>
      </div>
    );
  }

  if (!isAuthenticated) {
    return showRegister ? (
      <RegisterForm onLoginClick={() => setShowRegister(false)} />
    ) : (
      <LoginForm onRegisterClick={() => setShowRegister(true)} />
    );
  }

  return <Dashboard />;
}

function App() {
  return (
    <AuthProvider>
      <AppContent />
    </AuthProvider>
  );
}

export default App;