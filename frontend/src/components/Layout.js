import React from 'react';
import { Outlet } from 'react-router-dom';
import Sidebar from './Sidebar';
import Header from './Header';
import Player from './Player';
import '../styles/Layout.css';

const Layout = () => {
  return (
    <div className="layout">
      <div className="layout-sidebar">
        <Sidebar />
      </div>
      
      <div className="layout-main">
        <Header />
        <main className="layout-content">
          <Outlet />
        </main>
      </div>
      
      <div className="layout-player">
        <Player />
      </div>
    </div>
  );
};

export default Layout;
