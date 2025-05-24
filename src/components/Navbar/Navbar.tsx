import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import './Navbar.css';

const Navbar: React.FC = () => {
    const location = useLocation();

    return (
        <nav className="navbar">
            <div className="navbar-brand">
                <Link to="/">
                    <span className="brand-text">AI Playlist</span>
                </Link>
            </div>
            <div className="navbar-links">
                <Link
                    to="/"
                    className={`nav-link ${location.pathname === '/' ? 'active' : ''}`}
                >
                    Ana Sayfa
                </Link>
                <Link
                    to="/statistics"
                    className={`nav-link ${location.pathname === '/statistics' ? 'active' : ''}`}
                >
                    İstatistikler
                </Link>
            </div>
        </nav>
    );
};

export default Navbar; 