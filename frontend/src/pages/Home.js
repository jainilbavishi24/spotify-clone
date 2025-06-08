import React, { useState, useEffect } from 'react';
import { musicService } from '../services/musicService';
import SongList from '../components/SongList';
import '../styles/Home.css';

const Home = () => {
  const [songs, setSongs] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    fetchSongs();
  }, []);

  const fetchSongs = async () => {
    try {
      setLoading(true);
      const allSongs = await musicService.getAllSongs();
      setSongs(allSongs);
    } catch (error) {
      setError('Failed to load songs');
      console.error('Error fetching songs:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="page-loading">
        <div className="loading-spinner"></div>
        <p>Loading songs...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="page-error">
        <p>{error}</p>
        <button onClick={fetchSongs}>Try Again</button>
      </div>
    );
  }

  return (
    <div className="home-page">
      <div className="page-header">
        <h1>Good evening</h1>
        <p>Discover new music and enjoy your favorites</p>
      </div>

      <section className="recently-played">
        <h2>Recently Added</h2>
        {songs.length > 0 ? (
          <SongList songs={songs.slice(0, 10)} />
        ) : (
          <div className="empty-state">
            <p>No songs available. Upload some music to get started!</p>
          </div>
        )}
      </section>

      <section className="all-songs">
        <h2>All Songs</h2>
        {songs.length > 0 ? (
          <SongList songs={songs} />
        ) : (
          <div className="empty-state">
            <p>No songs in your library yet.</p>
          </div>
        )}
      </section>
    </div>
  );
};

export default Home;
