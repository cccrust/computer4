import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { api } from '../api';
import { useAuth } from '../AuthContext';

export default function Orders() {
  const navigate = useNavigate();
  const { user } = useAuth();
  const [orders, setOrders] = useState([]);
  const [loading, setLoading] = useState(true);
  const [expandedOrder, setExpandedOrder] = useState(null);

  useEffect(() => {
    if (user) loadOrders();
    else navigate('/login');
  }, [user]);

  const loadOrders = async () => {
    setLoading(true);
    try {
      const res = await api.getOrders();
      setOrders(res.data || []);
    } catch (err) {
      console.error('Failed to load orders:', err);
    }
    setLoading(false);
  };

  const toggleOrder = (orderId) => {
    setExpandedOrder(expandedOrder === orderId ? null : orderId);
  };

  const getStatusText = (status) => {
    const statusMap = {
      pending: '待處理',
      processing: '處理中',
      shipped: '已出貨',
      delivered: '已送達',
      cancelled: '已取消',
    };
    return statusMap[status] || status;
  };

  const getStatusClass = (status) => {
    const classMap = {
      pending: 'status-pending',
      processing: 'status-processing',
      shipped: 'status-shipped',
      delivered: 'status-delivered',
      cancelled: 'status-cancelled',
    };
    return classMap[status] || '';
  };

  if (loading) return <div className="loading">載入中...</div>;

  return (
    <div className="orders-page">
      <h1>📦 我的訂單</h1>

      {orders.length === 0 ? (
        <div className="empty-orders">
          <p>還沒有訂單</p>
          <button onClick={() => navigate('/')} className="continue-shopping-btn">
            開始購物
          </button>
        </div>
      ) : (
        <div className="orders-list">
          {orders.map((order) => (
            <div key={order.id} className="order-card">
              <div
                className="order-header"
                onClick={() => toggleOrder(order.id)}
              >
                <div className="order-info">
                  <span className="order-id">訂單 #{order.id.slice(0, 8)}</span>
                  <span className="order-date">
                    {new Date(order.created_at).toLocaleDateString()}
                  </span>
                </div>
                <div className="order-summary">
                  <span className={`order-status ${getStatusClass(order.status)}`}>
                    {getStatusText(order.status)}
                  </span>
                  <span className="order-total">
                    ${order.total_amount.toLocaleString()}
                  </span>
                  <span className="order-toggle">
                    {expandedOrder === order.id ? '▲' : '▼'}
                  </span>
                </div>
              </div>

              {expandedOrder === order.id && (
                <div className="order-details">
                  <div className="shipping-address">
                    <strong>寄送地址:</strong> {order.shipping_address}
                  </div>
                  <div className="order-items">
                    <h4>商品明細</h4>
                    {order.items.map((item, idx) => (
                      <div key={idx} className="order-item">
                        <img
                          src={item.product.image_url || 'https://via.placeholder.com/50'}
                          alt={item.product.name}
                          className="order-item-image"
                        />
                        <div className="order-item-info">
                          <span className="order-item-name">
                            {item.product.name}
                          </span>
                          <span className="order-item-qty">
                            x{item.quantity}
                          </span>
                          <span className="order-item-price">
                            ${item.price.toLocaleString()}
                          </span>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}