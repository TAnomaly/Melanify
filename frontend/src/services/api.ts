import axios from 'axios';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://127.0.0.1:8081';

const api = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Content-Type': 'application/json',
    },
    withCredentials: true,
});

export default api;
