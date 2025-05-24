// HTML Templates for the application

pub const SUCCESS_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>Success!</title>
    <meta charset="UTF-8">
    <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap" rel="stylesheet">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" integrity="sha512-iecdLmaskl7CVkqkXNQ/ZH/XLlvWZOJyj7Yy7tcenmpD1ypASozpmT/E0iPtmFIB46ZmdtAc9eNBvH0H/ZpiBw==" crossorigin="anonymous" referrerpolicy="no-referrer" />
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: 'Roboto', sans-serif;
            background: linear-gradient(135deg, #1DB954 0%, #191414 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
        .container {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            padding: 40px;
            border-radius: 16px;
            text-align: center;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            max-width: 600px;
            width: 90%;
            animation: fadeIn 0.5s ease-out;
        }
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        h1 {
            font-size: 2.5em;
            margin-bottom: 20px;
            color: #1DB954;
        }
        p {
            font-size: 1.2em;
            margin-bottom: 30px;
            line-height: 1.6;
        }
        .button {
            display: inline-block;
            padding: 12px 24px;
            background: #1DB954;
            color: white;
            text-decoration: none;
            border-radius: 50px;
            font-weight: 500;
            margin: 10px;
            transition: all 0.3s ease;
            border: none;
            cursor: pointer;
        }
        .button:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(29, 185, 84, 0.3);
            background: #1ed760;
        }
        .spotify-link {
            background: #1DB954;
            color: white;
            display: flex;
            align-items: center;
            justify-content: center;
            text-decoration: none;
        }
        .spotify-link:hover {
            background: #1ed760;
            transform: translateY(-2px);
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>✨ Success!</h1>
        <p>Your new playlist has been created in your Spotify account.</p>
        <a href="{}" class="button" target="_blank">Open in Spotify</a>
        <a href="/" class="button">Create Another Playlist</a>
    </div>
</body>
</html>"#;

pub const ERROR_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>Error</title>
    <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap" rel="stylesheet">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: 'Roboto', sans-serif;
            background: linear-gradient(135deg, #ff4b4b 0%, #191414 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
        .container {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            padding: 40px;
            border-radius: 16px;
            text-align: center;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            max-width: 600px;
            width: 90%;
            animation: fadeIn 0.5s ease-out;
        }
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        h1 {
            font-size: 2.5em;
            margin-bottom: 20px;
            color: #ff4b4b;
        }
        p {
            font-size: 1.2em;
            margin-bottom: 30px;
            line-height: 1.6;
        }
        .button {
            display: inline-block;
            padding: 12px 24px;
            background: #ff4b4b;
            color: white;
            text-decoration: none;
            border-radius: 50px;
            font-weight: 500;
            transition: all 0.3s ease;
        }
        .button:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(255, 75, 75, 0.3);
            background: #ff6b6b;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>❌ Error</h1>
        <p>{}</p>
        <p>{}</p>
        <a href="/" class="button">Try Again</a>
    </div>
</body>
</html>"#;

pub const INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>AI Music Playlist Generator</title>
    <meta charset="UTF-8">
    <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap" rel="stylesheet">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" integrity="sha512-iecdLmaskl7CVkqkXNQ/ZH/XLlvWZOJyj7Yy7tcenmpD1ypASozpmT/E0iPtmFIB46ZmdtAc9eNBvH0H/ZpiBw==" crossorigin="anonymous" referrerpolicy="no-referrer" />
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: 'Roboto', sans-serif;
            background: linear-gradient(135deg, #1DB954 0%, #191414 100%);
            min-height: 100vh;
            color: white;
            line-height: 1.6;
        }
        .main-container {
            display: flex;
            flex-direction: row;
            max-width: 1200px;
            margin: 0 auto;
            padding: 40px 20px;
        }
        .content-container {
            flex: 2;
            margin-right: 20px;
        }
        .sidebar {
            flex: 1;
            margin-left: 20px;
        }
        .card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            padding: 40px;
            border-radius: 16px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            animation: fadeIn 0.5s ease-out;
            margin-bottom: 20px;
        }
        .sidebar-card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            padding: 20px;
            border-radius: 16px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            animation: fadeIn 0.5s ease-out;
            margin-bottom: 20px;
        }
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        h1 {
            font-size: 2.5em;
            margin-bottom: 20px;
            text-align: center;
        }
        h2 {
            font-size: 1.8em;
            margin-bottom: 15px;
            color: #1DB954;
        }
        p {
            font-size: 1.2em;
            margin-bottom: 30px;
            text-align: center;
        }
        .input-group {
            margin-bottom: 20px;
            position: relative;
        }
        input[type="text"], textarea {
            width: 100%;
            padding: 15px 20px;
            border: 2px solid rgba(255, 255, 255, 0.1);
            background: rgba(255, 255, 255, 0.1);
            border-radius: 50px;
            font-size: 1.1em;
            color: white;
            transition: all 0.3s ease;
        }
        textarea {
            min-height: 100px;
            border-radius: 20px;
            resize: vertical;
            font-family: 'Roboto', sans-serif;
        }
        input[type="text"]:focus, textarea:focus {
            outline: none;
            border-color: #1DB954;
            background: rgba(255, 255, 255, 0.2);
        }
        input[type="text"]::placeholder, textarea::placeholder {
            color: rgba(255, 255, 255, 0.6);
        }
        button {
            width: 100%;
            padding: 15px 30px;
            background: #1DB954;
            color: white;
            border: none;
            border-radius: 50px;
            font-size: 1.1em;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.3s ease;
            margin-bottom: 10px;
        }
        button:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(29, 185, 84, 0.3);
            background: #1ed760;
        }
        button:disabled {
            background: #ccc;
            cursor: not-allowed;
            transform: none;
            box-shadow: none;
        }
        .loading {
            display: none;
            text-align: center;
            margin: 20px 0;
        }
        .loading-spinner {
            display: inline-block;
            width: 30px;
            height: 30px;
            border: 3px solid rgba(255, 255, 255, 0.3);
            border-radius: 50%;
            border-top-color: #fff;
            animation: spin 1s ease-in-out infinite;
        }
        @keyframes spin {
            to { transform: rotate(360deg); }
        }
        #result {
            margin-top: 30px;
        }
        .song {
            background: rgba(255, 255, 255, 0.1);
            padding: 20px;
            border-radius: 10px;
            margin-bottom: 10px;
            transition: all 0.3s ease;
            display: flex;
            align-items: center;
        }
        .song:hover {
            transform: translateX(5px);
            background: rgba(255, 255, 255, 0.2);
        }
        .song-icon {
            margin-right: 15px;
            font-size: 24px;
            color: #1DB954;
        }
        .song-info {
            flex: 1;
        }
        .song-title {
            font-weight: 500;
            font-size: 1.1em;
        }
        .song-artist {
            font-size: 0.9em;
            opacity: 0.8;
        }
        .song a {
            color: #1DB954;
            text-decoration: none;
            font-weight: 500;
        }
        .song a:hover {
            text-decoration: underline;
        }
        .error {
            background: rgba(255, 75, 75, 0.1);
            color: #ff4b4b;
            padding: 15px;
            border-radius: 10px;
            margin-bottom: 20px;
            animation: shake 0.5s ease-in-out;
        }
        @keyframes shake {
            0%, 100% { transform: translateX(0); }
            25% { transform: translateX(-5px); }
            75% { transform: translateX(5px); }
        }
        .spotify-button {
            background: #191414;
            display: none;
        }
        .spotify-button:hover {
            background: #282828;
        }
        .playlist-form {
            display: none;
            margin-top: 20px;
            animation: fadeIn 0.3s ease-out;
            background: rgba(0, 0, 0, 0.2);
            padding: 25px;
            border-radius: 16px;
            border: 1px solid rgba(255, 255, 255, 0.1);
        }
        .playlist-form input, .playlist-form textarea {
            width: 100%;
            padding: 15px 20px;
            border: 2px solid rgba(255, 255, 255, 0.2);
            background: rgba(255, 255, 255, 0.1);
            border-radius: 12px;
            font-size: 1.1em;
            color: white;
            transition: all 0.3s ease;
            margin-bottom: 15px;
        }
        .playlist-form textarea {
            border-radius: 12px;
            resize: vertical;
            min-height: 100px;
        }
        .playlist-form input:focus, .playlist-form textarea:focus {
            outline: none;
            border-color: #1DB954;
            background: rgba(255, 255, 255, 0.15);
        }
        .playlist-form input::placeholder, .playlist-form textarea::placeholder {
            color: rgba(255, 255, 255, 0.6);
        }
        .form-label {
            display: block;
            margin-bottom: 8px;
            color: white;
            font-size: 1.1em;
            text-align: left;
            font-weight: 500;
        }
        .create-playlist-btn {
            background: linear-gradient(45deg, #1DB954, #1ed760);
            display: none;
            padding: 16px 30px;
            font-weight: 600;
            letter-spacing: 0.5px;
            position: relative;
            overflow: hidden;
        }
        .create-playlist-btn:before {
            content: '';
            position: absolute;
            top: -10px;
            left: -10px;
            right: -10px;
            bottom: -10px;
            background: rgba(255, 255, 255, 0.1);
            transform: scale(0);
            border-radius: 50%;
            transition: all 0.4s ease;
        }
        .create-playlist-btn:hover:before {
            transform: scale(1.5);
        }
        .create-playlist-btn span {
            position: relative;
            z-index: 1;
        }
        .recent-track {
            display: flex;
            align-items: center;
            padding: 10px;
            margin-bottom: 10px;
            background: rgba(255, 255, 255, 0.1);
            border-radius: 8px;
            transition: all 0.3s ease;
        }
        .recent-track:hover {
            background: rgba(255, 255, 255, 0.2);
        }
        .track-img {
            width: 50px;
            height: 50px;
            border-radius: 5px;
            margin-right: 15px;
            object-fit: cover;
        }
        .track-info {
            flex: 1;
        }
        .track-name {
            font-weight: 500;
            margin-bottom: 3px;
            font-size: 0.9em;
        }
        .track-artist {
            font-size: 0.8em;
            color: rgba(255, 255, 255, 0.8);
        }
        .track-time {
            font-size: 0.7em;
            color: rgba(255, 255, 255, 0.6);
            margin-top: 3px;
        }
        .ai-input-container {
            background: rgba(0, 0, 0, 0.2);
            padding: 25px;
            border-radius: 16px;
            border: 1px solid rgba(255, 255, 255, 0.1);
            margin-bottom: 25px;
        }
        .ai-heading {
            text-align: center;
            margin-bottom: 20px;
            color: #1DB954;
            font-size: 1.5em;
        }
        .generate-button {
            background: linear-gradient(45deg, #1DB954, #1ed760);
            font-weight: 600;
            letter-spacing: 0.5px;
            box-shadow: 0 4px 15px rgba(29, 185, 84, 0.3);
        }
        #suggested-tracks {
            margin-top: 20px;
        }
        .preview-button {
            background: transparent;
            border: 2px solid #1DB954;
            width: 40px;
            height: 40px;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            margin-left: 15px;
            cursor: pointer;
            color: #1DB954;
            transition: all 0.2s ease;
            padding: 0;
        }
        .preview-button:hover {
            background: rgba(29, 185, 84, 0.2);
        }
        .preview-button.playing {
            background: #1DB954;
            color: white;
        }
        .preview-button.disabled {
            border-color: #888;
            color: #888;
            cursor: not-allowed;
        }
        .preview-button.disabled:hover {
            background: transparent;
        }
        /* Add responsive styling for mobile */
        @media (max-width: 768px) {
            .main-container {
                flex-direction: column;
            }
            .content-container {
                margin-right: 0;
                margin-bottom: 20px;
            }
            .sidebar {
                margin-left: 0;
            }
        }
    </style>
</head>
<body>
    <div class="main-container">
        <div class="content-container">
            <div class="card">
                <h1>🎵 AI Music Playlist Generator</h1>
                
                <div class="ai-input-container">
                    <h2 class="ai-heading">Describe Your Perfect Playlist</h2>
                    <p>Tell our AI what kind of playlist you want, and it will suggest songs that match your vibe</p>
                    <div class="input-group">
                        <textarea id="prompt" placeholder="e.g., A relaxing playlist for a rainy Sunday morning with coffee..." rows="4"></textarea>
                    </div>
                    <button onclick="processPrompt()" class="generate-button">Generate AI Playlist</button>
                </div>
                
                <div class="playlist-form" id="playlistForm">
                    <h2>Customize Your Playlist</h2>
                    <div class="form-label">Playlist Name:</div>
                    <input type="text" id="playlistName" placeholder="Enter a name for your playlist...">
                    <div class="form-label">Description:</div>
                    <textarea id="playlistDescription" placeholder="Add a description for your playlist..."></textarea>
                </div>
                
                <button id="createPlaylist" onclick="createSpotifyPlaylist()" class="create-playlist-btn" style="display: none;">
                    <span>Create Spotify Playlist</span>
                </button>
                
                <div id="loading" class="loading">
                    <div class="loading-spinner"></div>
                    <p id="loading-text">Crafting the perfect playlist for you...</p>
                </div>
                
                <div id="result"></div>
            </div>
        </div>
        
        <div class="sidebar">
            <div class="sidebar-card">
                <h2>Features</h2>
                <p style="font-size: 1em; margin-bottom: 15px;">Create and discover playlists</p>
                <a href="/time-capsules" class="button" style="display:block; margin-bottom: 10px; text-align: center;">
                    <i class="fas fa-clock"></i> Time Capsule Playlists
                </a>
            </div>
            
            <div class="sidebar-card">
                <h2>Spotify History</h2>
                <p style="font-size: 1em; margin-bottom: 15px;">See what you've been listening to recently</p>
                <button id="viewHistoryBtn" onclick="getSpotifyHistory()">View Recent Plays</button>
                <div id="recentlyPlayed" style="margin-top: 20px;">
                    <!-- Recently played tracks will be loaded here -->
                </div>
            </div>
        </div>
    </div>

    <script>
    let currentTracks = [];
    let authWindow = null; // Track the authorization window
    let currentAudio = null; // Track the currently playing audio
    
    document.getElementById('prompt').addEventListener('keypress', function(e) {
        if (e.key === 'Enter' && e.ctrlKey) {
            processPrompt();
        }
    });
    
    async function processPrompt() {
        const promptText = document.getElementById('prompt').value.trim();
        if (!promptText) {
            showError('Please enter a description of the playlist you want to create');
            return;
        }
        
        const loadingDiv = document.getElementById('loading');
        const loadingText = document.getElementById('loading-text');
        loadingText.textContent = 'AI is crafting your playlist...';
        loadingDiv.style.display = 'block';
        document.getElementById('result').innerHTML = '';
        
        try {
            const response = await fetch('/process-prompt', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ prompt: promptText }),
            });
            
            const data = await response.json();
            loadingDiv.style.display = 'none';
            
            if (data.error) {
                showError(data.error);
                return;
            }
            
            console.log("AI response data:", data);
            
            // Validate tracks data
            if (!data.tracks || !Array.isArray(data.tracks) || data.tracks.length === 0) {
                showError('AI bulamadı veya geçersiz şarkı verileri döndürdü.');
                console.error("Invalid tracks data:", data.tracks);
                return;
            }
            
            // Ensure each track has the required fields
            const tracks = data.tracks.map((track, index) => {
                // Check if track has required fields
                if (!track || typeof track !== 'object') {
                    console.error(`Invalid track at index ${index}:`, track);
                    return {
                        name: `Unknown Song ${index + 1}`,
                        artist: 'Unknown Artist',
                        url: "",
                        preview_url: null
                    };
                }
                
                // Ensure all properties exist
                return {
                    name: track.name || track.title || `Unknown Song ${index + 1}`,
                    artist: track.artist || 'Unknown Artist',
                    url: track.url || "",
                    spotify_id: track.spotify_id || null,
                    preview_url: track.preview_url || null
                };
            });
            
            // Pre-fill the playlist form with AI-suggested name and description
            document.getElementById('playlistName').value = data.playlist_name || 'My AI Playlist';
            document.getElementById('playlistDescription').value = data.playlist_description || 'An AI generated playlist';
            
            // Display the tracks in the results area
            displayTracks(tracks);
            
            // Show the playlist creation form
            document.getElementById('playlistForm').style.display = 'block';
            document.getElementById('createPlaylist').style.display = 'block';
            
            // Store the tracks for later use
            currentTracks = tracks;
            
        } catch (error) {
            console.error('Error:', error);
            loadingDiv.style.display = 'none';
            showError('An error occurred while processing your prompt. Please try again.');
        }
    }
    
    // Listen for messages from popup window
    window.addEventListener('message', function(event) {
        if (event.data && event.data.type === 'HISTORY_LOADED') {
            displayRecentlyPlayed(event.data.tracks);
        } else if (event.data && event.data.type === 'HISTORY_ERROR') {
            showHistoryError(event.data.error);
        }
    });
    
    async function getSpotifyHistory() {
        try {
            document.getElementById('recentlyPlayed').innerHTML = `
                <div class="loading" style="display: block;">
                    <div class="loading-spinner"></div>
                    <p>Connecting to Spotify...</p>
                </div>
            `;

            // If there's an existing auth window, close it
            if (authWindow && !authWindow.closed) {
                authWindow.close();
            }
            
            const response = await fetch('/history-auth');
            const data = await response.json();
            
            if (data.auth_url) {
                // Open a popup window for authentication
                const width = 450;
                const height = 730;
                const left = (screen.width / 2) - (width / 2);
                const top = (screen.height / 2) - (height / 2);
                
                authWindow = window.open(
                    data.auth_url, 
                    'spotify-auth-window',
                    `width=${width},height=${height},left=${left},top=${top}`
                );

                // Handle case where popup is blocked
                if (!authWindow || authWindow.closed || typeof authWindow.closed === 'undefined') {
                    document.getElementById('recentlyPlayed').innerHTML = `
                        <div class="error">
                            <p>Popup window was blocked by your browser.</p>
                            <p>Please allow popups for this site or <a href="${data.auth_url}" target="_blank">click here to login</a>.</p>
                        </div>
                    `;
                }

                // Handle authentication window closed without completion
                const checkWindowClosed = setInterval(() => {
                    if (authWindow && authWindow.closed) {
                        clearInterval(checkWindowClosed);
                        // Check if we still have the loading indicator
                        const recentlyPlayed = document.getElementById('recentlyPlayed');
                        if (recentlyPlayed.querySelector('.loading')) {
                            recentlyPlayed.innerHTML = `
                                <div class="error">
                                    <p>Authentication window was closed.</p>
                                    <p>Please try again or <a href="javascript:getSpotifyHistory()">click here to retry</a>.</p>
                                </div>
                            `;
                        }
                    }
                }, 1000);
            } else {
                throw new Error('Failed to get Spotify authentication URL');
            }
        } catch (error) {
            console.error('Error fetching history:', error);
            document.getElementById('recentlyPlayed').innerHTML = `
                <div class="error">
                    <p>Error connecting to Spotify</p>
                    <p>Please try again or <a href="javascript:getSpotifyHistory()">click here to retry</a>.</p>
                </div>
            `;
        }
    }
    
    function showHistoryError(message) {
        document.getElementById('recentlyPlayed').innerHTML = `
            <div class="error">
                <p>${message}</p>
                <p><a href="javascript:getSpotifyHistory()">Click here to try again</a></p>
            </div>
        `;
    }
    
    function displayRecentlyPlayed(tracks) {
        const container = document.getElementById('recentlyPlayed');
        
        if (!tracks || tracks.length === 0) {
            container.innerHTML = `
                <div class="error">
                    <p>No recently played tracks found</p>
                    <p>You might need to play some songs on Spotify first.</p>
                </div>
            `;
            return;
        }
        
        let html = '';
        tracks.forEach((track, index) => {
            const date = new Date(track.played_at);
            const formattedTime = date.toLocaleString();
            
            html += `
                <div class="recent-track" style="animation: fadeIn 0.3s ease-out ${index * 0.1}s both;">
                    <img class="track-img" src="${track.album_image || 'https://via.placeholder.com/50'}" alt="${track.name}">
                    <div class="track-info">
                        <div class="track-name">${track.name}</div>
                        <div class="track-artist">${track.artist}</div>
                        <div class="track-time">${formattedTime}</div>
                    </div>
                </div>
            `;
        });
        
        container.innerHTML = html;
    }

    function showError(message) {
        const resultDiv = document.getElementById('result');
        resultDiv.innerHTML = `<div class="error">${message}</div>`;
    }

    // Play a preview of a track
    function playPreview(previewUrl, buttonElement) {
        // Önce URL'nin geçerli olup olmadığını kontrol edelim
        if (!previewUrl || previewUrl === "null" || previewUrl === "undefined") {
            console.error("Çalınabilir bir önizleme URL'si bulunamadı:", previewUrl);
            alert("Bu şarkı için önizleme kullanılamıyor");
            return;
        }
        
        console.log("Oynatılacak URL:", previewUrl);
        
        // Stop any currently playing audio
        if (currentAudio) {
            currentAudio.pause();
            // Reset all buttons to play state
            document.querySelectorAll('.preview-button').forEach(btn => {
                btn.innerHTML = '<i class="fas fa-play"></i>';
                btn.classList.remove('playing');
            });
        }
        
        // If the button clicked is already playing, just stop it
        if (currentAudio && currentAudio.src === previewUrl && !currentAudio.paused) {
            currentAudio = null;
            return;
        }
        
        // Try to play the preview
        try {
            currentAudio = new Audio(previewUrl);
            
            // Audio yüklenme hatası olursa
            currentAudio.onerror = function(e) {
                console.error("Ses dosyası yüklenemedi:", e);
                buttonElement.innerHTML = '<i class="fas fa-play"></i>';
                buttonElement.classList.remove('playing');
                buttonElement.classList.add('disabled');
                buttonElement.disabled = true;
                alert("Önizleme oynatılamıyor. Spotify'da bu şarkı için önizleme sunulmuyor olabilir.");
                currentAudio = null;
            };
            
            // Oynatmayı başlat
            const playPromise = currentAudio.play();
            
            if (playPromise !== undefined) {
                playPromise.then(_ => {
                    // Oynatma başarılı
                    buttonElement.innerHTML = '<i class="fas fa-pause"></i>';
                    buttonElement.classList.add('playing');
                    
                    // When the preview finishes, reset the button
                    currentAudio.onended = function() {
                        buttonElement.innerHTML = '<i class="fas fa-play"></i>';
                        buttonElement.classList.remove('playing');
                        currentAudio = null;
                    };
                }).catch(error => {
                    // Otomatik oynatma engellendi veya başka bir hata
                    console.error("Oynatma hatası:", error);
                    buttonElement.innerHTML = '<i class="fas fa-play"></i>';
                    buttonElement.classList.remove('playing');
                    alert("Oynatma başlatılamadı. Lütfen sayfayla etkileşime girin ve tekrar deneyin.");
                    currentAudio = null;
                });
            }
        } catch (error) {
            console.error("Önizleme oynatılırken hata oluştu:", error);
            alert("Önizleme oynatılırken bir hata oluştu.");
            currentAudio = null;
        }
    }

    function displayTracks(tracks) {
        const resultDiv = document.getElementById('result');
        let html = '<h2 style="margin: 20px 0; text-align: center;">AI Suggested Songs</h2>';
        
        // Debug bilgisi yazarak sorunu teşhis edelim
        console.log('Tracks to display:', tracks);
        
        html += tracks.map((track, index) => {
            // Her track'in içeriğini konsolda görelim
            console.log(`Track ${index}:`, track);
            
            // preview_url null veya undefined ise butonu devre dışı bırakalım
            const hasPreview = track.preview_url && track.preview_url !== "null";
            const hasSpotifyId = track.spotify_id && track.spotify_id !== "null";
            
            // Spotify ID'yi düzgün formata getir
            let spotifyUrl = null;
            if (hasSpotifyId) {
                // Spotify ID'den tam URL oluşturma
                let idPart = track.spotify_id;
                if (idPart.startsWith('spotify:track:')) {
                    idPart = idPart.replace('spotify:track:', '');
                }
                spotifyUrl = `https://open.spotify.com/track/${idPart}`;
            }
            
            return `
                <div class="song" style="animation: fadeIn 0.3s ease-out ${index * 0.1}s both;">
                    <div class="song-icon">🎵</div>
                    <div class="song-info">
                        <div class="song-title">${track.name || 'Bilinmeyen Şarkı'}</div>
                        <div class="song-artist">${track.artist || 'Bilinmeyen Sanatçı'}</div>
                    </div>
                    ${hasPreview ? 
                        `<button class="preview-button" onclick="playPreview('${track.preview_url}', this)">
                            <i class="fas fa-play"></i>
                        </button>` 
                        : 
                        (spotifyUrl ? 
                            `<a href="${spotifyUrl}" target="_blank" class="preview-button spotify-link" title="Spotify'da aç">
                                <i class="fab fa-spotify"></i>
                            </a>` 
                            : 
                            `<button class="preview-button disabled" disabled>
                                <i class="fas fa-play"></i>
                            </button>`
                        )
                    }
                </div>
            `;
        }).join('');
        
        resultDiv.innerHTML = html;
    }

    async function createSpotifyPlaylist() {
        if (!currentTracks || currentTracks.length === 0) {
            showError('Please generate song suggestions first');
            return;
        }

        const playlistName = document.getElementById('playlistName').value.trim();
        if (!playlistName) {
            showError('Please enter a playlist name');
            return;
        }

        const loadingDiv = document.getElementById('loading');
        const loadingText = document.getElementById('loading-text');
        loadingDiv.style.display = 'block';
        loadingText.textContent = 'Creating your Spotify playlist...';

        try {
            const playlistDescription = document.getElementById('playlistDescription').value.trim();

            console.log('Creating playlist:', {
                name: playlistName,
                description: playlistDescription,
                tracks: currentTracks
            });

            const response = await fetch('/create-spotify-playlist', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    tracks: currentTracks,
                    playlist_name: playlistName,
                    playlist_description: playlistDescription || null
                }),
            });

            const data = await response.json();
            if (data.auth_url) {
                console.log('Redirecting to Spotify auth');
                window.location.href = data.auth_url;
            } else {
                throw new Error('No authorization URL received from server');
            }
        } catch (error) {
            console.error('Error creating playlist:', error);
            showError('Error creating playlist: ' + error.message);
            loadingDiv.style.display = 'none';
        }
    }
    </script>
</body>
</html>
"#;
