import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { api } from '../api';
import { useAuth } from '../AuthContext';

export default function Cart() {
  const navigate = useNavigate();
  const { user } = useAuth();
  const [cartItems, setCartItems] = useState([]);
  const [loading, setLoading] = useState(true);
  const [updating, setUpdating] = useState(null);
  const [address, setAddress] = useState('');
  const [showCheckout, setShowCheckout] = useState(false);
  const [ordering, setOrdering] = useState(false);

  useEffect(() => {
    if (user) loadCart();
    else navigate('/login');
  }, [user]);

  const loadCart = async () => {
    setLoading(true);
    try {
      const res = await api.getCart();
      setCartItems(res.data || []);
    } catch (err) {
      console.error('Failed to load cart:', err);
    }
    setLoading(false);
  };

  const handleUpdateQuantity = async (id, quantity) => {
    if (quantity < 1) return;
    setUpdating(id);
    try {
      await api.updateCartItem(id, quantity);
      loadCart();
    } catch (err) {
      alert(err.message);
    }
    setUpdating(null);
  };

  const handleRemove = async (id) => {
    setUpdating(id);
    try {
      await api.removeFromCart(id);
      loadCart();
    } catch (err) {
      alert(err.message);
    }
    setUpdating(null);
  };

  const handleCheckout = async (e) => {
    e.preventDefault();
    if (!address.trim()) {
      alert('請填寫寄送地址');
      return;
    }
    setOrdering(true);
    try {
      await api.createOrder(address);
      alert('訂單已成立！');
      navigate('/orders');
    } catch (err) {
      alert(err.message);
    }
    setOrdering(false);
  };

  const total = cartItems.reduce(
    (sum, item) => sum + item.product.price * item.quantity,
    0
  );

  if (loading) return <div className="loading">載入中...</div>;

  return (
    <div className="cart-page">
      <h1>🛒 購物車</h1>

      {cartItems.length === 0 ? (
        <div className="empty-cart">
          <p>購物車是空的</p>
          <button onClick={() => navigate('/')} className="continue-shopping-btn">
            繼續購物
          </button>
        </div>
      ) : (
        <>
          <div className="cart-items">
            {cartItems.map((item) => (
              <div key={item.id} className="cart-item">
                <img
                  src={item.product.image_url || 'https://via.placeholder.com/100'}
                  alt={item.product.name}
                  className="cart-item-image"
                />
                <div className="cart-item-info">
                  <h3>{item.product.name}</h3>
                  <p className="cart-item-price">
                    ${item.product.price.toLocaleString()}
                  </p>
                </div>
                <div className="cart-item-actions">
                  <div className="quantity-controls">
                    <button
                      onClick={() =>
                        handleUpdateQuantity(item.id, item.quantity - 1)
                      }
                      disabled={updating === item.id}
                    >
                      -
                    </button>
                    <span>{item.quantity}</span>
                    <button
                      onClick={() =>
                        handleUpdateQuantity(item.id, item.quantity + 1)
                      }
                      disabled={updating === item.id}
                    >
                      +
                    </button>
                  </div>
                  <p className="cart-item-subtotal">
                    小計: ${item.subtotal.toLocaleString()}
                  </p>
                  <button
                    onClick={() => handleRemove(item.id)}
                    disabled={updating === item.id}
                    className="remove-btn"
                  >
                    移除
                  </button>
                </div>
              </div>
            ))}
          </div>

          <div className="cart-summary">
            <h3>訂單摘要</h3>
            <div className="summary-row">
              <span>商品總數</span>
              <span>{cartItems.length} 件</span>
            </div>
            <div className="summary-row total">
              <span>總計</span>
              <span>${total.toLocaleString()}</span>
            </div>

            {!showCheckout ? (
              <button
                onClick={() => setShowCheckout(true)}
                className="checkout-btn"
              >
                結帳
              </button>
            ) : (
              <form onSubmit={handleCheckout} className="checkout-form">
                <input
                  type="text"
                  placeholder="請輸入寄送地址"
                  value={address}
                  onChange={(e) => setAddress(e.target.value)}
                  className="address-input"
                />
                <div className="checkout-actions">
                  <button
                    type="button"
                    onClick={() => setShowCheckout(false)}
                    className="cancel-btn"
                  >
                    取消
                  </button>
                  <button
                    type="submit"
                    disabled={ordering}
                    className="confirm-btn"
                  >
                    {ordering ? '處理中...' : '確認訂單'}
                  </button>
                </div>
              </form>
            )}
          </div>
        </>
      )}
    </div>
  );
}