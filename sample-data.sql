-- Sample data for Spotify Clone
-- Run this after setting up the database

-- Insert sample songs (you'll need to have actual audio files in the uploads/songs directory)
INSERT INTO songs (id, title, artist, album, duration, file_path, created_at) VALUES
    (gen_random_uuid(), 'Sample Song 1', 'Artist One', 'Album One', 180, 'songs/sample1.mp3', NOW()),
    (gen_random_uuid(), 'Sample Song 2', 'Artist Two', 'Album Two', 240, 'songs/sample2.mp3', NOW()),
    (gen_random_uuid(), 'Sample Song 3', 'Artist One', 'Album Three', 200, 'songs/sample3.mp3', NOW()),
    (gen_random_uuid(), 'Sample Song 4', 'Artist Three', 'Album Four', 220, 'songs/sample4.mp3', NOW()),
    (gen_random_uuid(), 'Sample Song 5', 'Artist Two', 'Album Five', 190, 'songs/sample5.mp3', NOW())
ON CONFLICT DO NOTHING;

-- Note: To add real songs, you'll need to:
-- 1. Upload audio files to the backend/uploads/songs/ directory
-- 2. Use the upload API endpoint or manually insert records with correct file paths
