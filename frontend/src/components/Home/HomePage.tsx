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

type GenerationMode = 'spotify' | 'ai-music';

interface AiSong {
    title: string;
    file_id: string;
    file_path: string;
    prompt: string;
}

const HomePage: React.FC = () => {
    const [prompt, setPrompt] = useState('');
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [generatedPlaylist, setGeneratedPlaylist] = useState<PlaylistData | null>(null);
    const [generationMode, setGenerationMode] = useState<GenerationMode>('spotify');
    const [aiSongs, setAiSongs] = useState<AiSong[] | null>(null);
    const [selectedImage, setSelectedImage] = useState<string | null>(null);
    const [imageCaption, setImageCaption] = useState<string | null>(null);
    const [duration, setDuration] = useState<number>(10);
    const [withVocals, setWithVocals] = useState<boolean>(false);
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

    const handleImageUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        if (!file) return;

        // Preview image
        const reader = new FileReader();
        reader.onloadend = () => {
            setSelectedImage(reader.result as string);
        };
        reader.readAsDataURL(file);

        // Analyze image
        setLoading(true);
        setError(null);
        setImageCaption(null);

        try {
            const base64Image = await new Promise<string>((resolve) => {
                const r = new FileReader();
                r.onloadend = () => resolve(r.result as string);
                r.readAsDataURL(file);
            });

            const response = await api.post('http://localhost:5000/analyze-image', {
                image: base64Image
            });

            console.log('Image analysis response:', response.data);

            if (response.data.success) {
                const caption = response.data.caption;
                const suggestedPrompt = response.data.suggested_prompt;

                console.log('Caption:', caption);
                console.log('Suggested prompt:', suggestedPrompt);

                setImageCaption(caption);
                setPrompt(suggestedPrompt);

                console.log('State updated - caption and prompt set');
            } else {
                setError('Görüntü analizi başarısız oldu.');
            }
        } catch (err) {
            console.error('Error analyzing image:', err);
            setError('Görüntü analiz edilirken bir hata oluştu.');
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
        setAiSongs(null);

        try {
            if (generationMode === 'spotify') {
                // Generate Spotify playlist with real songs
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
            } else {
                // Generate AI music
                const response = await api.post('/generate-ai-music-batch', {
                    prompts: [
                        { title: 'Generated Track', prompt: prompt },
                    ],
                    duration: duration,
                    with_vocals: withVocals
                });

                if (response.data.success) {
                    setAiSongs(response.data.songs);
                } else {
                    throw new Error('AI müzik oluşturma başarısız oldu.');
                }
            }
        } catch (err) {
            console.error('Error generating playlist:', err);
            if (generationMode === 'ai-music') {
                setError('AI müzik oluşturulurken bir hata oluştu. Python servisinin çalıştığından emin olun.');
            } else {
                setError('Çalma listesi oluşturulurken bir hata oluştu.');
            }
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

                {/* Mode Selector */}
                <div className="mode-selector">
                    <button
                        className={`mode-button ${generationMode === 'spotify' ? 'active' : ''}`}
                        onClick={() => {
                            setGenerationMode('spotify');
                            setGeneratedPlaylist(null);
                            setAiSongs(null);
                            setError(null);
                        }}
                    >
                        <span className="mode-icon">🎵</span>
                        <div className="mode-text">
                            <strong>Spotify Şarkıları</strong>
                            <small>Gerçek şarkılarla playlist</small>
                        </div>
                    </button>
                    <button
                        className={`mode-button ${generationMode === 'ai-music' ? 'active' : ''}`}
                        onClick={() => {
                            setGenerationMode('ai-music');
                            setGeneratedPlaylist(null);
                            setAiSongs(null);
                            setError(null);
                        }}
                    >
                        <span className="mode-icon">🤖</span>
                        <div className="mode-text">
                            <strong>AI Müzik Üret</strong>
                            <small>Yapay zeka ile özgün müzik</small>
                        </div>
                    </button>
                </div>

                {/* Image Upload Section - Only for AI Music mode */}
                {generationMode === 'ai-music' && (
                    <div className="image-upload-section">
                        <h3>📸 Fotoğraftan Müzik Üret</h3>
                        <p>Fotoğraf yükleyin, AI duygu durumunu analiz edip ona göre müzik üretsin!</p>
                        <input
                            type="file"
                            accept="image/*"
                            onChange={handleImageUpload}
                            style={{marginBottom: '20px'}}
                        />
                        {selectedImage && (
                            <div className="image-preview">
                                <img src={selectedImage} alt="Selected" style={{maxWidth: '300px', marginBottom: '10px'}} />
                                {imageCaption && (
                                    <div className="caption-box">
                                        <strong>🔍 Görüntü Analizi:</strong> {imageCaption}
                                    </div>
                                )}
                            </div>
                        )}
                        <div className="separator">— VEYA —</div>
                    </div>
                )}

                {/* AI Music Options - Only for AI Music mode */}
                {generationMode === 'ai-music' && (
                    <div className="ai-options">
                        <div className="option-group">
                            <label htmlFor="duration">⏱️ Müzik Süresi:</label>
                            <select
                                id="duration"
                                value={duration}
                                onChange={(e) => setDuration(Number(e.target.value))}
                                className="duration-select"
                            >
                                <option value={5}>5 saniye</option>
                                <option value={10}>10 saniye</option>
                                <option value={15}>15 saniye</option>
                                <option value={20}>20 saniye</option>
                                <option value={30}>30 saniye</option>
                            </select>
                        </div>
                        <div className="option-group">
                            <label className="checkbox-label">
                                <input
                                    type="checkbox"
                                    checked={withVocals}
                                    onChange={(e) => setWithVocals(e.target.checked)}
                                />
                                <span>🎤 Sözlü Müzik (Vokal içersin)</span>
                            </label>
                        </div>
                    </div>
                )}

                <div className="prompt-container">
                    <textarea
                        placeholder={
                            generationMode === 'spotify'
                                ? "Nasıl bir çalma listesi istediğinizi açıklayın... (Örnek: '90'ların en iyi rock şarkılarından oluşan enerjik bir liste')"
                                : "Nasıl bir müzik üretmek istediğinizi açıklayın... (Örnek: 'Upbeat electronic dance music' ya da 'Sakinleştirici piyano melodisi')"
                        }
                        value={prompt}
                        onChange={(e) => setPrompt(e.target.value)}
                        rows={4}
                    />
                    <button
                        className="generate-button"
                        onClick={handleGeneratePlaylist}
                        disabled={loading}
                    >
                        {loading ? 'Oluşturuluyor...' : (generationMode === 'spotify' ? 'Çalma Listesi Oluştur' : 'AI Müzik Üret')}
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

                {aiSongs && aiSongs.length > 0 && (
                    <div className="ai-songs-preview">
                        <h3>🎼 Üretilen AI Müzikler</h3>
                        <p>Aşağıdaki müzikler yapay zeka tarafından özel olarak sizin için üretildi!</p>
                        <div className="ai-songs-list">
                            {aiSongs.map((song, index) => (
                                <div key={song.file_id} className="ai-song-item">
                                    <span className="song-number">{index + 1}</span>
                                    <div className="song-info">
                                        <strong>{song.title}</strong>
                                        <small>{song.prompt}</small>
                                    </div>
                                    <audio controls src={`http://localhost:5000${song.file_path}`} />
                                    <a
                                        href={`http://localhost:5000${song.file_path}`}
                                        download={`${song.title}.wav`}
                                        className="download-button"
                                    >
                                        İndir
                                    </a>
                                </div>
                            ))}
                        </div>
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