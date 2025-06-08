import api from './authService';

export const musicService = {
  async getAllSongs() {
    try {
      const response = await api.get('/songs');
      return response.data;
    } catch (error) {
      throw new Error(error.response?.data?.error || 'Failed to fetch songs');
    }
  },

  async getSong(id) {
    try {
      const response = await api.get(`/songs/${id}`);
      return response.data;
    } catch (error) {
      throw new Error(error.response?.data?.error || 'Failed to fetch song');
    }
  },

  async searchSongs(query) {
    try {
      const response = await api.get(`/songs/search?q=${encodeURIComponent(query)}`);
      return response.data;
    } catch (error) {
      throw new Error(error.response?.data?.error || 'Search failed');
    }
  },

  async uploadSong(formData) {
    try {
      const response = await api.post('/songs/upload', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      });
      return response.data;
    } catch (error) {
      throw new Error(error.response?.data?.error || 'Upload failed');
    }
  },

  async getUserPlaylists() {
    try {
      const response = await api.get('/playlists');
      return response.data;
    } catch (error) {
      throw new Error(error.response?.data?.error || 'Failed to fetch playlists');
    }
  },

  async getPlaylist(id) {
    try {
      const response = await api.get(`/playlists/${id}`);
      return response.data;
    } catch (error) {
      throw new Error(error.response?.data?.error || 'Failed to fetch playlist');
    }
  },

  async createPlaylist(name, description) {
    try {
      const response = await api.post('/playlists', { name, description });
      return response.data;
    } catch (error) {
      throw new Error(error.response?.data?.error || 'Failed to create playlist');
    }
  },

  async addSongToPlaylist(playlistId, songId) {
    try {
      const response = await api.post(`/playlists/${playlistId}/songs`, { song_id: songId });
      return response.data;
    } catch (error) {
      throw new Error(error.response?.data?.error || 'Failed to add song to playlist');
    }
  }
};
