import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { FaMusic, FaPlay, FaPause } from 'react-icons/fa';
import { musicService } from '../services/musicService';
import { usePlayer } from '../context/PlayerContext';
import SongList from '../components/SongList';
import '../styles/Playlist.css';

const Playlist = () => {
  const { id } = useParams();
  const [playlist, setPlaylist] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  
  const { currentSong, isPlaying, playSong, pauseSong } = usePlayer();

  useEffect(() => {
    fetchPlaylist();
  }, [id]);

  const fetchPlaylist = async () => {
    try {
      setLoading(true);
      const playlistData = await musicService.getPlaylist(id);
      setPlaylist(playlistData);
    } catch (error) {
      setError('Failed to load playlist');
      console.error('Error fetching playlist:', error);
    } finally {
      setLoading(false);
    }
  };

  const handlePlayAll = () => {
    if (!playlist?.songs?.length) return;

    const firstSong = playlist.songs[0];
    const isCurrentPlaylist = currentSong && playlist.songs.some(song => song.id === currentSong.id);
    
    if (isCurrentPlaylist && isPlaying) {
      pauseSong();
    } else {
      playSong(firstSong, playlist.songs);
    }
  };

  const formatDate = (dateString) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
  };

  if (loading) {
    return (
      <div className="page-loading">
        <div className="loading-spinner"></div>
        <p>Loading playlist...</p>
      </div>
    );
  }

  if (error || !playlist) {
    return (
      <div className="page-error">
        <p>{error || 'Playlist not found'}</p>
        <button onClick={fetchPlaylist}>Try Again</button>
      </div>
    );
  }

  const isCurrentPlaylist = currentSong && playlist.songs.some(song => song.id === currentSong.id);
  const showPlayButton = !isCurrentPlaylist || !isPlaying;

  return (
    <div className="playlist-page">
      <div className="playlist-header">
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
          <span className="playlist-type">Playlist</span>
          <h1 className="playlist-title">{playlist.name}</h1>
          {playlist.description && (
            <p className="playlist-description">{playlist.description}</p>
          )}
          <div className="playlist-meta">
            <span>{playlist.songs?.length || 0} songs</span>
            <span>•</span>
            <span>Created {formatDate(playlist.created_at)}</span>
          </div>
        </div>
      </div>

      <div className="playlist-controls">
        <button 
          className="play-all-btn"
          onClick={handlePlayAll}
          disabled={!playlist.songs?.length}
        >
          {showPlayButton ? <FaPlay /> : <FaPause />}
          <span>{showPlayButton ? 'Play' : 'Pause'}</span>
        </button>
      </div>

      <div className="playlist-content">
        {playlist.songs && playlist.songs.length > 0 ? (
          <SongList songs={playlist.songs} />
        ) : (
          <div className="empty-playlist">
            <FaMusic />
            <h3>This playlist is empty</h3>
            <p>Add some songs to get started</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default Playlist;
