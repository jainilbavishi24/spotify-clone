import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { FaMusic, FaUpload, FaPlus } from 'react-icons/fa';
import { musicService } from '../services/musicService';
import '../styles/Library.css';

const Library = () => {
  const [playlists, setPlaylists] = useState([]);
  const [loading, setLoading] = useState(true);
  const [showUploadModal, setShowUploadModal] = useState(false);
  const [uploadData, setUploadData] = useState({
    title: '',
    artist: '',
    album: '',
    duration: '',
    audio: null
  });
  const [uploading, setUploading] = useState(false);

  useEffect(() => {
    fetchPlaylists();
  }, []);

  const fetchPlaylists = async () => {
    try {
      const userPlaylists = await musicService.getUserPlaylists();
      setPlaylists(userPlaylists);
    } catch (error) {
      console.error('Failed to fetch playlists:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleFileChange = (e) => {
    const file = e.target.files[0];
    if (file) {
      setUploadData(prev => ({ ...prev, audio: file }));
    }
  };

  const handleUpload = async (e) => {
    e.preventDefault();
    if (!uploadData.audio || !uploadData.title || !uploadData.artist) {
      alert('Please fill in all required fields and select an audio file');
      return;
    }

    const formData = new FormData();
    formData.append('title', uploadData.title);
    formData.append('artist', uploadData.artist);
    formData.append('album', uploadData.album || 'Unknown Album');
    formData.append('duration', uploadData.duration || '0');
    formData.append('audio', uploadData.audio);

    try {
      setUploading(true);
      await musicService.uploadSong(formData);
      setShowUploadModal(false);
      setUploadData({
        title: '',
        artist: '',
        album: '',
        duration: '',
        audio: null
      });
      alert('Song uploaded successfully!');
    } catch (error) {
      console.error('Upload failed:', error);
      alert('Upload failed. Please try again.');
    } finally {
      setUploading(false);
    }
  };

  if (loading) {
    return (
      <div className="page-loading">
        <div className="loading-spinner"></div>
        <p>Loading your library...</p>
      </div>
    );
  }

  return (
    <div className="library-page">
      <div className="library-header">
        <h1>Your Library</h1>
        <button 
          className="upload-btn"
          onClick={() => setShowUploadModal(true)}
        >
          <FaUpload />
          Upload Song
        </button>
      </div>

      <section className="playlists-section">
        <h2>Your Playlists</h2>
        <div className="playlists-grid">
          {playlists.map((playlist) => (
            <Link
              key={playlist.id}
              to={`/playlist/${playlist.id}`}
              className="playlist-card"
            >
              <div className="playlist-cover">
                {playlist.cover_image ? (
                  <img src={playlist.cover_image} alt={playlist.name} />
                ) : (
                  <div className="cover-placeholder">
                    <FaMusic />
                  </div>
                )}
              </div>
              <div className="playlist-info">
                <h3>{playlist.name}</h3>
                {playlist.description && <p>{playlist.description}</p>}
              </div>
            </Link>
          ))}
          
          <div className="create-playlist-card">
            <div className="create-playlist-content">
              <FaPlus />
              <span>Create Playlist</span>
            </div>
          </div>
        </div>
      </section>

      {showUploadModal && (
        <div className="modal-overlay" onClick={() => setShowUploadModal(false)}>
          <div className="modal upload-modal" onClick={(e) => e.stopPropagation()}>
            <h2>Upload Song</h2>
            <form onSubmit={handleUpload}>
              <div className="form-group">
                <label>Title *</label>
                <input
                  type="text"
                  value={uploadData.title}
                  onChange={(e) => setUploadData(prev => ({ ...prev, title: e.target.value }))}
                  required
                />
              </div>
              
              <div className="form-group">
                <label>Artist *</label>
                <input
                  type="text"
                  value={uploadData.artist}
                  onChange={(e) => setUploadData(prev => ({ ...prev, artist: e.target.value }))}
                  required
                />
              </div>
              
              <div className="form-group">
                <label>Album</label>
                <input
                  type="text"
                  value={uploadData.album}
                  onChange={(e) => setUploadData(prev => ({ ...prev, album: e.target.value }))}
                />
              </div>
              
              <div className="form-group">
                <label>Duration (seconds)</label>
                <input
                  type="number"
                  value={uploadData.duration}
                  onChange={(e) => setUploadData(prev => ({ ...prev, duration: e.target.value }))}
                />
              </div>
              
              <div className="form-group">
                <label>Audio File *</label>
                <input
                  type="file"
                  accept="audio/*"
                  onChange={handleFileChange}
                  required
                />
              </div>
              
              <div className="modal-actions">
                <button type="button" onClick={() => setShowUploadModal(false)}>
                  Cancel
                </button>
                <button type="submit" disabled={uploading}>
                  {uploading ? 'Uploading...' : 'Upload'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default Library;
