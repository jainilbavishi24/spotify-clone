import React, { useState, useEffect } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { FaHome, FaSearch, FaMusic, FaPlus } from 'react-icons/fa';
import { musicService } from '../services/musicService';
import '../styles/Sidebar.css';

const Sidebar = () => {
  const location = useLocation();
  const [playlists, setPlaylists] = useState([]);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [newPlaylistName, setNewPlaylistName] = useState('');
  const [newPlaylistDescription, setNewPlaylistDescription] = useState('');

  useEffect(() => {
    fetchPlaylists();
  }, []);

  const fetchPlaylists = async () => {
    try {
      const userPlaylists = await musicService.getUserPlaylists();
      setPlaylists(userPlaylists);
    } catch (error) {
      console.error('Failed to fetch playlists:', error);
    }
  };

  const handleCreatePlaylist = async (e) => {
    e.preventDefault();
    if (!newPlaylistName.trim()) return;

    try {
      await musicService.createPlaylist(newPlaylistName, newPlaylistDescription);
      setNewPlaylistName('');
      setNewPlaylistDescription('');
      setShowCreateModal(false);
      fetchPlaylists();
    } catch (error) {
      console.error('Failed to create playlist:', error);
    }
  };

  const isActive = (path) => location.pathname === path;

  return (
    <div className="sidebar">
      <div className="sidebar-logo">
        <h1>Spotify Clone</h1>
      </div>

      <nav className="sidebar-nav">
        <Link 
          to="/" 
          className={`sidebar-link ${isActive('/') ? 'active' : ''}`}
        >
          <FaHome />
          <span>Home</span>
        </Link>
        
        <Link 
          to="/search" 
          className={`sidebar-link ${isActive('/search') ? 'active' : ''}`}
        >
          <FaSearch />
          <span>Search</span>
        </Link>
        
        <Link 
          to="/library" 
          className={`sidebar-link ${isActive('/library') ? 'active' : ''}`}
        >
          <FaMusic />
          <span>Your Library</span>
        </Link>
      </nav>

      <div className="sidebar-playlists">
        <div className="sidebar-section-header">
          <h3>Playlists</h3>
          <button 
            className="create-playlist-btn"
            onClick={() => setShowCreateModal(true)}
          >
            <FaPlus />
          </button>
        </div>

        <div className="playlist-list">
          {playlists.map((playlist) => (
            <Link
              key={playlist.id}
              to={`/playlist/${playlist.id}`}
              className={`playlist-item ${isActive(`/playlist/${playlist.id}`) ? 'active' : ''}`}
            >
              <div className="playlist-cover">
                {playlist.cover_image ? (
                  <img src={playlist.cover_image} alt={playlist.name} />
                ) : (
                  <div className="playlist-cover-placeholder">
                    <FaMusic />
                  </div>
                )}
              </div>
              <div className="playlist-info">
                <span className="playlist-name">{playlist.name}</span>
                {playlist.description && (
                  <span className="playlist-description">{playlist.description}</span>
                )}
              </div>
            </Link>
          ))}
        </div>
      </div>

      {showCreateModal && (
        <div className="modal-overlay" onClick={() => setShowCreateModal(false)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <h2>Create Playlist</h2>
            <form onSubmit={handleCreatePlaylist}>
              <input
                type="text"
                placeholder="Playlist name"
                value={newPlaylistName}
                onChange={(e) => setNewPlaylistName(e.target.value)}
                required
              />
              <textarea
                placeholder="Description (optional)"
                value={newPlaylistDescription}
                onChange={(e) => setNewPlaylistDescription(e.target.value)}
                rows="3"
              />
              <div className="modal-actions">
                <button type="button" onClick={() => setShowCreateModal(false)}>
                  Cancel
                </button>
                <button type="submit">Create</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default Sidebar;
