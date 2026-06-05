import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../AuthContext';

export default function Auth() {
  const navigate = useNavigate();
  const { login, register } = useAuth();
  const [isLogin, setIsLogin] = useState(true);
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      if (isLogin) {
        await login(email, password);
      } else {
        if (!username.trim()) {
          setError('請輸入用戶名');
          setLoading(false);
          return;
        }
        await register(username, email, password);
      }
      navigate('/');
    } catch (err) {
      setError(err.message);
    }
    setLoading(false);
  };

  const switchMode = () => {
    setIsLogin(!isLogin);
    setError('');
  };

  return (
    <div className="auth-page">
      <div className="auth-card">
        <h1>{isLogin ? '登入' : '註冊'}</h1>

        {error && <div className="auth-error">{error}</div>}

        <form onSubmit={handleSubmit} className="auth-form">
          {!isLogin && (
            <div className="form-group">
              <label>用戶名</label>
              <input
                type="text"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                placeholder="請輸入用戶名"
              />
            </div>
          )}

          <div className="form-group">
            <label>電子郵件</label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="請輸入電子郵件"
            />
          </div>

          <div className="form-group">
            <label>密碼</label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="請輸入密碼"
            />
          </div>

          <button type="submit" disabled={loading} className="auth-submit-btn">
            {loading ? '處理中...' : isLogin ? '登入' : '註冊'}
          </button>
        </form>

        <p className="auth-switch">
          {isLogin ? '還沒有帳號？' : '已經有帳號？'}
          <button onClick={switchMode} className="switch-btn">
            {isLogin ? '註冊' : '登入'}
          </button>
        </p>

        <div className="auth-demo">
          <p>測試帳號: wangxiaoming@shop4.com / password123</p>
        </div>
      </div>
    </div>
  );
}