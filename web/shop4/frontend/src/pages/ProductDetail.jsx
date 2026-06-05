import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { api } from '../api';
import { useAuth } from '../AuthContext';

export default function ProductDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const { user } = useAuth();
  const [product, setProduct] = useState(null);
  const [reviews, setReviews] = useState([]);
  const [quantity, setQuantity] = useState(1);
  const [loading, setLoading] = useState(true);
  const [adding, setAdding] = useState(false);
  const [reviewRating, setReviewRating] = useState(5);
  const [reviewComment, setReviewComment] = useState('');
  const [showReviewForm, setShowReviewForm] = useState(false);

  useEffect(() => {
    loadProduct();
  }, [id]);

  const loadProduct = async () => {
    setLoading(true);
    try {
      const [productRes, reviewsRes] = await Promise.all([
        api.getProduct(id),
        api.getProductReviews(id),
      ]);
      setProduct(productRes.data);
      setReviews(reviewsRes.data || []);
    } catch (err) {
      console.error('Failed to load product:', err);
    }
    setLoading(false);
  };

  const handleAddToCart = async () => {
    if (!user) {
      navigate('/login');
      return;
    }
    setAdding(true);
    try {
      await api.addToCart(id, quantity);
      alert('已加入購物車！');
    } catch (err) {
      alert(err.message);
    }
    setAdding(false);
  };

  const handleSubmitReview = async (e) => {
    e.preventDefault();
    try {
      await api.createReview(id, reviewRating, reviewComment);
      alert('評論已發表！');
      setShowReviewForm(false);
      setReviewComment('');
      loadProduct();
    } catch (err) {
      alert(err.message);
    }
  };

  if (loading) return <div className="loading">載入中...</div>;
  if (!product) return <div className="error">商品不存在</div>;

  return (
    <div className="product-detail">
      <div className="product-detail-card">
        <img
          src={product.image_url || 'https://via.placeholder.com/600'}
          alt={product.name}
          className="product-detail-image"
        />
        <div className="product-detail-info">
          <h1>{product.name}</h1>
          <p className="product-detail-desc">{product.description}</p>
          <div className="product-detail-meta">
            <span className="rating">⭐ {product.rating.toFixed(1)}</span>
            <span>已售 {product.sold_count}</span>
          </div>
          <p className="product-detail-price">${product.price.toLocaleString()}</p>
          <div className="product-detail-stock">庫存: {product.stock}</div>

          <div className="add-to-cart">
            <input
              type="number"
              min="1"
              max={product.stock}
              value={quantity}
              onChange={(e) => setQuantity(Math.max(1, parseInt(e.target.value) || 1))}
              className="quantity-input"
            />
            <button
              onClick={handleAddToCart}
              disabled={adding || product.stock === 0}
              className="add-cart-btn"
            >
              {adding ? '加入中...' : '加入購物車'}
            </button>
          </div>
        </div>
      </div>

      <div className="reviews-section">
        <div className="reviews-header">
          <h2>商品評論</h2>
          {user && (
            <button
              onClick={() => setShowReviewForm(!showReviewForm)}
              className="review-toggle-btn"
            >
              {showReviewForm ? '取消' : '撰寫評論'}
            </button>
          )}
        </div>

        {showReviewForm && (
          <form onSubmit={handleSubmitReview} className="review-form">
            <div className="rating-select">
              <label>評分: </label>
              {[1, 2, 3, 4, 5].map((r) => (
                <button
                  key={r}
                  type="button"
                  onClick={() => setReviewRating(r)}
                  className={reviewRating === r ? 'selected' : ''}
                >
                  {'⭐'.repeat(r)}
                </button>
              ))}
            </div>
            <textarea
              placeholder="分享您的使用心得..."
              value={reviewComment}
              onChange={(e) => setReviewComment(e.target.value)}
              className="review-textarea"
            />
            <button type="submit" className="submit-review-btn">
              發表評論
            </button>
          </form>
        )}

        {reviews.length === 0 ? (
          <p className="no-reviews">尚無評論</p>
        ) : (
          <div className="reviews-list">
            {reviews.map((review) => (
              <div key={review.id} className="review-item">
                <div className="review-header">
                  <span className="review-user">{review.username}</span>
                  <span className="review-rating">{'⭐'.repeat(review.rating)}</span>
                </div>
                <p className="review-comment">{review.comment}</p>
                <span className="review-date">
                  {new Date(review.created_at).toLocaleDateString()}
                </span>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}