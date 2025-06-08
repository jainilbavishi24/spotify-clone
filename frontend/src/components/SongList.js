import React from 'react';
import { FaPlay, FaPause, FaMusic, FaPlus } from 'react-icons/fa';
import { usePlayer } from '../context/PlayerContext';
import '../styles/SongList.css';

const SongList = ({ songs, showAddToPlaylist = false, onAddToPlaylist }) => {
  const { currentSong, isPlaying, playSong, pauseSong } = usePlayer();

  const handlePlayClick = (song) => {
    if (currentSong?.id === song.id && isPlaying) {
      pauseSong();
    } else {
      playSong(song, songs);
    }
  };

  const formatDuration = (seconds) => {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
  };

  return (
    <div className="song-list">
      <div className="song-list-header">
        <span className="header-number">#</span>
        <span className="header-title">Title</span>
        <span className="header-album">Album</span>
        <span className="header-duration">Duration</span>
        {showAddToPlaylist && <span className="header-actions">Actions</span>}
      </div>

      <div className="song-list-content">
        {songs.map((song, index) => {
          const isCurrentSong = currentSong?.id === song.id;
          const isCurrentlyPlaying = isCurrentSong && isPlaying;

          return (
            <div 
              key={song.id} 
              className={`song-item ${isCurrentSong ? 'active' : ''}`}
            >
              <div className="song-number">
                <span className="number">{index + 1}</span>
                <button 
                  className="play-button"
                  onClick={() => handlePlayClick(song)}
                >
                  {isCurrentlyPlaying ? <FaPause /> : <FaPlay />}
                </button>
              </div>

              <div className="song-info">
                <div className="song-cover">
                  {song.cover_art ? (
                    <img src={song.cover_art} alt={song.title} />
                  ) : (
                    <div className="cover-placeholder">
                      <FaMusic />
                    </div>
                  )}
                </div>
                <div className="song-details">
                  <span className="song-title">{song.title}</span>
                  <span className="song-artist">{song.artist}</span>
                </div>
              </div>

              <div className="song-album">
                <span>{song.album}</span>
              </div>

              <div className="song-duration">
                <span>{formatDuration(song.duration)}</span>
              </div>

              {showAddToPlaylist && (
                <div className="song-actions">
                  <button 
                    className="add-to-playlist-btn"
                    onClick={() => onAddToPlaylist && onAddToPlaylist(song)}
                    title="Add to playlist"
                  >
                    <FaPlus />
                  </button>
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default SongList;
