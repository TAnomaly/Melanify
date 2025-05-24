import React, { useEffect, useState } from 'react';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
    ArcElement,
    BarElement,
} from 'chart.js';
import { Line, Pie, Bar } from 'react-chartjs-2';
import api from '../../services/api';
import './StatisticsPage.css';

// Register ChartJS components
ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
    ArcElement,
    BarElement
);

interface StatCardProps {
    title: string;
    value: string | number;
    icon?: React.ReactNode;
}

const StatCard: React.FC<StatCardProps> = ({ title, value, icon }) => (
    <div className="stat-card">
        {icon && <div className="stat-icon">{icon}</div>}
        <div className="stat-content">
            <h3>{title}</h3>
            <p>{value}</p>
        </div>
    </div>
);

const StatisticsPage: React.FC = () => {
    const [statistics, setStatistics] = useState<any>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchStatistics = async () => {
            try {
                const response = await api.get('/statistics/user/current');
                setStatistics(response.data);
                setError(null);
            } catch (err) {
                console.error('Error fetching statistics:', err);
                setError('İstatistikler yüklenirken bir hata oluştu.');
            } finally {
                setLoading(false);
            }
        };

        fetchStatistics();
    }, []);

    if (loading) {
        return <div className="loading">İstatistikler yükleniyor...</div>;
    }

    if (error) {
        return <div className="error">{error}</div>;
    }

    const chartOptions = {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
            legend: {
                position: 'top' as const,
            },
        },
    };

    return (
        <div className="statistics-container">
            <h1>Dinleme İstatistikleri</h1>

            <div className="stats-cards">
                <StatCard
                    title="Toplam Dinleme Süresi"
                    value={`${statistics?.total_listening_time || 0} dakika`}
                />
                <StatCard
                    title="En Çok Dinlenen Tür"
                    value={statistics?.favorite_genres[0]?.genre || 'Veri yok'}
                />
                <StatCard
                    title="En Çok Dinlenen Sanatçı"
                    value={statistics?.favorite_artists[0]?.artist_name || 'Veri yok'}
                />
            </div>

            <div className="stats-charts">
                <div className="chart-container">
                    <h2>Günlük Dinleme Aktivitesi</h2>
                    {statistics?.daily_stats && (
                        <Line
                            options={chartOptions}
                            data={{
                                labels: statistics.daily_stats.map((stat: any) => stat.date),
                                datasets: [{
                                    label: 'Dinleme Süresi (dk)',
                                    data: statistics.daily_stats.map((stat: any) => stat.total_minutes),
                                    borderColor: '#1DB954',
                                    backgroundColor: 'rgba(29, 185, 84, 0.1)',
                                }]
                            }}
                        />
                    )}
                </div>

                <div className="chart-container">
                    <h2>Tür Dağılımı</h2>
                    {statistics?.favorite_genres && (
                        <Pie
                            options={chartOptions}
                            data={{
                                labels: statistics.favorite_genres.map((genre: any) => genre.genre),
                                datasets: [{
                                    data: statistics.favorite_genres.map((genre: any) => genre.percentage),
                                    backgroundColor: [
                                        '#1DB954',
                                        '#1ED760',
                                        '#2EBD59',
                                        '#57B660',
                                        '#7C795D',
                                    ],
                                }]
                            }}
                        />
                    )}
                </div>

                <div className="chart-container">
                    <h2>En Çok Dinlenen Sanatçılar</h2>
                    {statistics?.favorite_artists && (
                        <Bar
                            options={chartOptions}
                            data={{
                                labels: statistics.favorite_artists.map((artist: any) => artist.artist_name),
                                datasets: [{
                                    label: 'Dinlenme Sayısı',
                                    data: statistics.favorite_artists.map((artist: any) => artist.listen_count),
                                    backgroundColor: '#1DB954',
                                }]
                            }}
                        />
                    )}
                </div>
            </div>

            <div className="recent-history">
                <h2>Son Dinlenenler</h2>
                <table>
                    <thead>
                        <tr>
                            <th>Şarkı</th>
                            <th>Sanatçı</th>
                            <th>Tarih</th>
                            <th>Süre</th>
                        </tr>
                    </thead>
                    <tbody>
                        {statistics?.listening_history.map((record: any, index: number) => (
                            <tr key={index}>
                                <td>{record.track_name}</td>
                                <td>{record.artist_name}</td>
                                <td>{new Date(record.listened_at).toLocaleDateString()}</td>
                                <td>{Math.floor(record.duration / 60)}:{record.duration % 60}</td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
};

export default StatisticsPage; 