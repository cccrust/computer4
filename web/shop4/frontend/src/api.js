const API_BASE = 'http://localhost:8080';

function getAuthHeaders() {
  const token = localStorage.getItem('token');
  return token ? { Authorization: `Bearer ${token}` } : {};
}

async function request(method, path, body = null) {
  const headers = { 'Content-Type': 'application/json', ...getAuthHeaders() };
  const config = { method, headers };
  if (body) config.body = JSON.stringify(body);

  const res = await fetch(`${API_BASE}${path}`, config);
  const data = await res.json();

  if (!res.ok) {
    throw new Error(data.error || 'Request failed');
  }
  return data;
}

export const api = {
  register: (username, email, password) =>
    request('POST', '/api/auth/register', { username, email, password }),
  login: (email, password) =>
    request('POST', '/api/auth/login', { email, password }),
  getCategories: () => request('GET', '/api/categories'),
  getProducts: (params = {}) => {
    const cleanParams = Object.entries(params)
      .filter(([_, v]) => v !== '' && v !== null && v !== undefined)
      .reduce((acc, [k, v]) => ({ ...acc, [k]: v }), {});
    const query = new URLSearchParams(cleanParams).toString();
    return request('GET', `/api/products${query ? '?' + query : ''}`);
  },
  getProduct: (id) => request('GET', `/api/products/${id}`),
  createProduct: (product) => request('POST', '/api/products', product),
  updateProduct: (id, product) => request('PUT', `/api/products/${id}`, product),
  deleteProduct: (id) => request('DELETE', `/api/products/${id}`),
  getCart: () => request('GET', '/api/cart'),
  addToCart: (productId, quantity) =>
    request('POST', '/api/cart', { product_id: productId, quantity }),
  updateCartItem: (id, quantity) =>
    request('PUT', `/api/cart/${id}`, { quantity }),
  removeFromCart: (id) => request('DELETE', `/api/cart/${id}`),
  getOrders: () => request('GET', '/api/orders'),
  createOrder: (shippingAddress) =>
    request('POST', '/api/orders', { shipping_address: shippingAddress }),
  getOrder: (id) => request('GET', `/api/orders/${id}`),
  getUserProfile: () => request('GET', '/api/user/profile'),
  createReview: (productId, rating, comment) =>
    request('POST', `/api/products/${productId}/reviews`, { rating, comment }),
  getProductReviews: (productId) => request('GET', `/api/products/${productId}/reviews`),
};