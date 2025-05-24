import axios from 'axios';

const api = axios.create({
    baseURL: 'http://127.0.0.1:8081',
    headers: {
        'Content-Type': 'application/json',
    },
    withCredentials: true // Enable sending cookies with requests
});

export default api; 