import { useState, useEffect } from 'react';
import { Link, useSearchParams } from 'react-router-dom';
import { api } from '../api';

export default function Home() {
  const [products, setProducts] = useState([]);
  const [categories, setCategories] = useState([]);
  const [loading, setLoading] = useState(true);
  const [searchParams] = useSearchParams();
  const search = searchParams.get('search') || '';
  const categoryId = searchParams.get('category') || '';

  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      try {
        const [productsRes, categoriesRes] = await Promise.all([
          api.getProducts({ search, category_id: categoryId }),
          api.getCategories(),
        ]);
        setProducts(productsRes.data || []);
        setCategories(categoriesRes.data || []);
      } catch (err) {
        console.error('Failed to load data:', err);
      }
      setLoading(false);
    };
    loadData();
  }, [search, categoryId]);

  const getCategoryName = (parentId) => {
    if (!parentId) return null;
    const parent = categories.find((c) => c.id === parentId);
    return parent?.name;
  };

  const getCategoryPath = (cat) => {
    const parts = [];
    if (cat.parent_id) {
      const parent = categories.find((c) => c.id === cat.parent_id);
      if (parent) parts.push(parent.name);
    }
    parts.push(cat.name);
    return parts.join(' > ');
  };

  return (
    <div className="home">
      <aside className="sidebar">
        <h3>商品分類</h3>
        <ul className="category-list">
          <li>
            <Link to="/" className={!categoryId ? 'active' : ''}>
              全部商品
            </Link>
          </li>
          {categories.filter((c) => !c.parent_id).map((cat) => (
            <li key={cat.id}>
              <Link
                to={`/?category=${cat.id}`}
                className={categoryId === cat.id ? 'active' : ''}
              >
                {getCategoryPath(cat)}
              </Link>
              {categories
                .filter((c) => c.parent_id === cat.id)
                .map((subCat) => (
                  <ul key={subCat.id} className="subcategory-list">
                    <li>
                      <Link
                        to={`/?category=${subCat.id}`}
                        className={categoryId === subCat.id ? 'active' : ''}
                      >
                        {subCat.name}
                      </Link>
                    </li>
                  </ul>
                ))}
            </li>
          ))}
        </ul>
      </aside>

      <main className="main-content">
        <div className="products-header">
          <h2>{search ? `搜尋: ${search}` : categoryId ? getCategoryName(categoryId) || '商品' : '全部商品'}</h2>
          <span className="product-count">{products.length} 件商品</span>
        </div>

        {loading ? (
          <div className="loading">載入中...</div>
        ) : products.length === 0 ? (
          <div className="empty">找不到商品</div>
        ) : (
          <div className="product-grid" key={categoryId || 'all'}>
            {products.map((product) => (
              <Link
                to={`/product/${product.id}`}
                key={product.id}
                className="product-card"
              >
                <img
                  src={product.image_url || 'https://via.placeholder.com/400'}
                  alt={product.name}
                  className="product-image"
                />
                <div className="product-info">
                  <h3 className="product-name">{product.name}</h3>
                  <p className="product-desc">{product.description}</p>
                  <div className="product-meta">
                    <span className="product-price">${product.price.toLocaleString()}</span>
                    <span className="product-rating">⭐ {product.rating.toFixed(1)}</span>
                  </div>
                  <div className="product-stats">
                    已售 {product.sold_count} | 庫存 {product.stock}
                  </div>
                </div>
              </Link>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}