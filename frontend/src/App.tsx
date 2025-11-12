import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import HomePage from './components/Home/HomePage';
import Navbar from './components/Navbar/Navbar';
import './App.css';

const App: React.FC = () => {
    return (
        <Router>
            <div className="app">
                <Navbar />
                <main>
                    <Routes>
                        <Route path="/" element={<HomePage />} />
                    </Routes>
                </main>
            </div>
        </Router>
    );
};

export default App; 