import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { api } from '../api';
import { useAuth } from '../AuthContext';

export default function Profile() {
  const navigate = useNavigate();
  const { user, logout } = useAuth();
  const [profile, setProfile] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (user) loadProfile();
    else navigate('/login');
  }, [user]);

  const loadProfile = async () => {
    setLoading(true);
    try {
      const res = await api.getUserProfile();
      setProfile(res.data);
    } catch (err) {
      console.error('Failed to load profile:', err);
    }
    setLoading(false);
  };

  const handleLogout = () => {
    logout();
    navigate('/');
  };

  if (loading) return <div className="loading">載入中...</div>;

  return (
    <div className="profile-page">
      <div className="profile-card">
        <div className="profile-header">
          <div className="avatar">👤</div>
          <h2>{profile?.username}</h2>
          <p className="profile-email">{profile?.email}</p>
          <span className="profile-role">
            {profile?.role === 'admin' ? '👑 管理員' : '會員'}
          </span>
        </div>

        <div className="profile-stats">
          <div className="stat">
            <span className="stat-value">會員</span>
            <span className="stat-label">身份</span>
          </div>
          <div className="stat">
            <span className="stat-value">
              {new Date(profile?.created_at || Date.now()).toLocaleDateString()}
            </span>
            <span className="stat-label">加入日期</span>
          </div>
        </div>

        <div className="profile-actions">
          <button onClick={() => navigate('/orders')} className="action-btn">
            📦 我的訂單
          </button>
          <button onClick={() => navigate('/')} className="action-btn">
            🛒 繼續購物
          </button>
          <button onClick={handleLogout} className="logout-action-btn">
            登出
          </button>
        </div>
      </div>
    </div>
  );
}