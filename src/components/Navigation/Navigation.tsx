import React from 'react';
import { Link } from 'react-router-dom';
import './Navigation.css';

const Navigation: React.FC = () => {
    return (
        <nav className="navigation">
            <div className="nav-container">
                <Link to="/" className="nav-logo">
                    Spotify AI Playlist
                </Link>
                <div className="nav-links">
                    <Link to="/" className="nav-link">
                        Ana Sayfa
                    </Link>
                    <Link to="/playlists" className="nav-link">
                        Çalma Listeleri
                    </Link>
                    <Link to="/statistics" className="nav-link">
                        İstatistikler
                    </Link>
                    <Link to="/profile" className="nav-link">
                        Profil
                    </Link>
                </div>
            </div>
        </nav>
    );
};

export default Navigation; 