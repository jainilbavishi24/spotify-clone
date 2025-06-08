import React from 'react';
import { useAuth } from '../context/AuthContext';
import { FaUser, FaSignOutAlt } from 'react-icons/fa';
import '../styles/Header.css';

const Header = () => {
  const { user, logout } = useAuth();

  const handleLogout = () => {
    logout();
  };

  return (
    <header className="header">
      <div className="header-content">
        <div className="header-left">
          {/* Navigation buttons could go here */}
        </div>
        
        <div className="header-right">
          <div className="user-menu">
            <div className="user-info">
              <FaUser className="user-icon" />
              <span className="username">{user?.username}</span>
            </div>
            <button className="logout-btn" onClick={handleLogout}>
              <FaSignOutAlt />
              <span>Logout</span>
            </button>
          </div>
        </div>
      </div>
    </header>
  );
};

export default Header;
