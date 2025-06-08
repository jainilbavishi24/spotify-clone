import React from 'react';
import { 
  FaPlay, 
  FaPause, 
  FaStepBackward, 
  FaStepForward, 
  FaVolumeUp,
  FaMusic 
} from 'react-icons/fa';
import { usePlayer } from '../context/PlayerContext';
import '../styles/Player.css';

const Player = () => {
  const {
    currentSong,
    isPlaying,
    currentTime,
    duration,
    volume,
    togglePlayPause,
    playNext,
    playPrevious,
    seekTo,
    setVolume,
    formatTime
  } = usePlayer();

  const handleProgressClick = (e) => {
    const progressBar = e.currentTarget;
    const rect = progressBar.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    const newTime = percent * duration;
    seekTo(newTime);
  };

  const handleVolumeChange = (e) => {
    setVolume(parseFloat(e.target.value));
  };

  if (!currentSong) {
    return (
      <div className="player">
        <div className="player-content">
          <div className="player-info">
            <div className="song-placeholder">
              <FaMusic />
              <span>No song selected</span>
            </div>
          </div>
        </div>
      </div>
    );
  }

  const progressPercent = duration ? (currentTime / duration) * 100 : 0;

  return (
    <div className="player">
      <div className="player-content">
        <div className="player-info">
          <div className="song-cover">
            {currentSong.cover_art ? (
              <img src={currentSong.cover_art} alt={currentSong.title} />
            ) : (
              <div className="cover-placeholder">
                <FaMusic />
              </div>
            )}
          </div>
          <div className="song-details">
            <span className="song-title">{currentSong.title}</span>
            <span className="song-artist">{currentSong.artist}</span>
          </div>
        </div>

        <div className="player-controls">
          <div className="control-buttons">
            <button className="control-btn" onClick={playPrevious}>
              <FaStepBackward />
            </button>
            
            <button className="control-btn play-btn" onClick={togglePlayPause}>
              {isPlaying ? <FaPause /> : <FaPlay />}
            </button>
            
            <button className="control-btn" onClick={playNext}>
              <FaStepForward />
            </button>
          </div>

          <div className="progress-section">
            <span className="time-display">{formatTime(currentTime)}</span>
            <div className="progress-bar" onClick={handleProgressClick}>
              <div className="progress-track">
                <div 
                  className="progress-fill" 
                  style={{ width: `${progressPercent}%` }}
                />
              </div>
            </div>
            <span className="time-display">{formatTime(duration)}</span>
          </div>
        </div>

        <div className="player-volume">
          <FaVolumeUp className="volume-icon" />
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={volume}
            onChange={handleVolumeChange}
            className="volume-slider"
          />
        </div>
      </div>
    </div>
  );
};

export default Player;
