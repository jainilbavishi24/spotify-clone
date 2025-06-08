import React, { useState, useEffect } from 'react';
import { FaSearch } from 'react-icons/fa';
import { musicService } from '../services/musicService';
import SongList from '../components/SongList';
import '../styles/Search.css';

const Search = () => {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [hasSearched, setHasSearched] = useState(false);

  useEffect(() => {
    const timeoutId = setTimeout(() => {
      if (query.trim()) {
        handleSearch();
      } else {
        setResults([]);
        setHasSearched(false);
      }
    }, 500);

    return () => clearTimeout(timeoutId);
  }, [query]);

  const handleSearch = async () => {
    if (!query.trim()) return;

    try {
      setLoading(true);
      setError('');
      const searchResults = await musicService.searchSongs(query);
      setResults(searchResults);
      setHasSearched(true);
    } catch (error) {
      setError('Search failed. Please try again.');
      console.error('Search error:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="search-page">
      <div className="search-header">
        <h1>Search</h1>
        <div className="search-bar">
          <FaSearch className="search-icon" />
          <input
            type="text"
            placeholder="What do you want to listen to?"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="search-input"
          />
        </div>
      </div>

      <div className="search-content">
        {loading && (
          <div className="search-loading">
            <div className="loading-spinner"></div>
            <p>Searching...</p>
          </div>
        )}

        {error && (
          <div className="search-error">
            <p>{error}</p>
          </div>
        )}

        {!loading && !error && hasSearched && (
          <div className="search-results">
            {results.length > 0 ? (
              <>
                <h2>Search Results ({results.length})</h2>
                <SongList songs={results} />
              </>
            ) : (
              <div className="no-results">
                <p>No results found for "{query}"</p>
                <p>Try searching for something else.</p>
              </div>
            )}
          </div>
        )}

        {!hasSearched && !loading && (
          <div className="search-placeholder">
            <h2>Start typing to search for songs</h2>
            <p>Find your favorite tracks by title, artist, or album</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default Search;
