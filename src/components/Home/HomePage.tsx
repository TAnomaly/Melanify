import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import api from '../../services/api';
import './HomePage.css';

interface Track {
    name: string;
    artist: string;
    url: string;
    spotify_id?: string | null;
}

interface GeminiTrack {
    title: string;
    artist: string;
}

interface PlaylistData {
    tracks: GeminiTrack[];
    playlist_name: string;
    playlist_description: string;
}

const HomePage: React.FC = () => {
    const [prompt, setPrompt] = useState('');
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [generatedPlaylist, setGeneratedPlaylist] = useState<PlaylistData | null>(null);
    const navigate = useNavigate();
    const location = useLocation();

    useEffect(() => {
        // Check if we're returning from Spotify auth
        const params = new URLSearchParams(location.search);
        const code = params.get('code');
        const error = params.get('error');

        if (error) {
            setError('Spotify yetkilendirmesi başarısız oldu.');
        } else if (code) {
            // Handle successful authorization
            handleSpotifyCallback(code);
        }
    }, [location]);

    const handleSpotifyCallback = async (code: string) => {
        try {
            setLoading(true);
            const response = await api.get(`/callback?code=${code}`);
            if (response.data.success) {
                setError(null);
                // Redirect to success page or show success message
                window.location.href = response.data.playlist_url || '/';
            } else {
                setError('Playlist oluşturulurken bir hata oluştu.');
            }
        } catch (err) {
            console.error('Error handling Spotify callback:', err);
            setError('Spotify callback işlemi başarısız oldu.');
        } finally {
            setLoading(false);
        }
    };

    const handleGeneratePlaylist = async () => {
        if (!prompt.trim()) {
            setError('Lütfen bir açıklama girin.');
            return;
        }

        setLoading(true);
        setError(null);
        setGeneratedPlaylist(null);

        try {
            const response = await api.post('/process-prompt', { prompt });
            const playlist = response.data;

            // Transform tracks to match backend expectations
            const transformedTracks = playlist.tracks.map((track: GeminiTrack): Track => ({
                name: track.title,
                artist: track.artist,
                url: "", // Empty string as required by backend
                spotify_id: null // Add this field
            }));

            setGeneratedPlaylist({
                ...playlist,
                tracks: transformedTracks
            });
        } catch (err) {
            console.error('Error generating playlist:', err);
            setError('Çalma listesi oluşturulurken bir hata oluştu.');
        } finally {
            setLoading(false);
        }
    };

    const handleCreateSpotifyPlaylist = async () => {
        if (!generatedPlaylist) {
            setError('Lütfen önce bir çalma listesi oluşturun.');
            return;
        }

        setLoading(true);
        setError(null);

        try {
            const response = await api.post('/create-spotify-playlist', {
                tracks: generatedPlaylist.tracks,
                playlist_name: generatedPlaylist.playlist_name,
                playlist_description: generatedPlaylist.playlist_description
            }, {
                withCredentials: true,
                headers: {
                    'Content-Type': 'application/json'
                }
            });

            if (response.data.auth_url) {
                // Store session ID in localStorage
                if (response.data.session_id) {
                    localStorage.setItem('spotify_session_id', response.data.session_id);
                }
                console.log('Redirecting to Spotify auth:', response.data.auth_url);
                window.location.href = response.data.auth_url;
            } else {
                throw new Error('Spotify yetkilendirme URL\'i alınamadı.');
            }
        } catch (err) {
            console.error('Error creating playlist:', err);
            setError('Spotify çalma listesi oluşturulurken bir hata oluştu.');
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="home-container">
            <div className="hero-section">
                <h1>AI-Generated Spotify Playlists</h1>
                <p className="subtitle">
                    Yapay zeka ile kişiselleştirilmiş çalma listeleri oluşturun.
                    Sadece ne istediğinizi söyleyin, gerisini bize bırakın!
                </p>
            </div>

            <div className="generate-section">
                <h2>Çalma Listesi Oluştur</h2>
                <div className="prompt-container">
                    <textarea
                        placeholder="Nasıl bir çalma listesi istediğinizi açıklayın... (Örnek: '90'ların en iyi rock şarkılarından oluşan enerjik bir liste' ya da 'Yoga ve meditasyon için sakinleştirici müzikler')"
                        value={prompt}
                        onChange={(e) => setPrompt(e.target.value)}
                        rows={4}
                    />
                    <button
                        className="generate-button"
                        onClick={handleGeneratePlaylist}
                        disabled={loading}
                    >
                        {loading ? 'Oluşturuluyor...' : 'Çalma Listesi Oluştur'}
                    </button>
                    {error && <div className="error-message">{error}</div>}
                </div>

                {generatedPlaylist && (
                    <div className="playlist-preview">
                        <h3>{generatedPlaylist.playlist_name}</h3>
                        <p>{generatedPlaylist.playlist_description}</p>
                        <div className="tracks-list">
                            {generatedPlaylist.tracks.map((track, index) => (
                                <div key={index} className="track-item">
                                    <span className="track-number">{index + 1}</span>
                                    <span className="track-title">{track.title}</span>
                                    <span className="track-artist">{track.artist}</span>
                                </div>
                            ))}
                        </div>
                        <button
                            className="spotify-button"
                            onClick={handleCreateSpotifyPlaylist}
                            disabled={loading}
                        >
                            {loading ? 'Spotify\'a Aktarılıyor...' : 'Spotify\'a Aktar'}
                        </button>
                    </div>
                )}
            </div>

            <div className="features-section">
                <div className="feature-card">
                    <div className="feature-icon">🎵</div>
                    <h3>Akıllı Öneriler</h3>
                    <p>Yapay zeka, müzik zevkinizi analiz ederek size özel çalma listeleri oluşturur.</p>
                </div>
                <div className="feature-card">
                    <div className="feature-icon">🎯</div>
                    <h3>Kolay Kullanım</h3>
                    <p>İstediğiniz türde müziği tanımlayın, AI sizin için mükemmel listeyi hazırlasın.</p>
                </div>
                <div className="feature-card">
                    <div className="feature-icon">📊</div>
                    <h3>Detaylı İstatistikler</h3>
                    <p>Dinleme alışkanlıklarınızı analiz edin ve müzik zevkinizi keşfedin.</p>
                </div>
            </div>

            <div className="how-it-works">
                <h2>Nasıl Çalışır?</h2>
                <div className="steps">
                    <div className="step">
                        <div className="step-number">1</div>
                        <h3>Açıklama Yazın</h3>
                        <p>İstediğiniz çalma listesini doğal bir dille açıklayın.</p>
                    </div>
                    <div className="step">
                        <div className="step-number">2</div>
                        <h3>AI Analiz Eder</h3>
                        <p>Yapay zeka isteğinizi analiz edip en uygun şarkıları seçer.</p>
                    </div>
                    <div className="step">
                        <div className="step-number">3</div>
                        <h3>Liste Oluşturulur</h3>
                        <p>Seçilen şarkılar Spotify'da çalma listesi olarak kaydedilir.</p>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default HomePage; 