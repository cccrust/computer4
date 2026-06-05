import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../AuthContext';

export default function Navbar() {
  const { user, logout } = useAuth();
  const navigate = useNavigate();
  const [searchTerm, setSearchTerm] = useState('');

  const handleSearch = (e) => {
    e.preventDefault();
    if (searchTerm.trim()) {
      navigate(`/?search=${encodeURIComponent(searchTerm)}`);
    }
  };

  const handleLogout = () => {
    logout();
    navigate('/');
  };

  return (
    <nav className="navbar">
      <div className="nav-container">
        <Link to="/" className="logo">
          🛒 Shop4
        </Link>

        <form className="search-form" onSubmit={handleSearch}>
          <input
            type="text"
            placeholder="搜尋商品..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="search-input"
          />
          <button type="submit" className="search-btn">搜尋</button>
        </form>

        <div className="nav-links">
          <Link to="/" className="nav-link">首頁</Link>
          <Link to="/cart" className="nav-link">🛒 購物車</Link>
          {user ? (
            <>
              <Link to="/profile" className="nav-link">👤 {user.username}</Link>
              <button onClick={handleLogout} className="logout-btn">登出</button>
            </>
          ) : (
            <Link to="/login" className="nav-link">登入</Link>
          )}
        </div>
      </div>
    </nav>
  );
}