import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";

export default function ProductForm() {
  const { id } = useParams();
  const navigate = useNavigate();
  const isEdit = Boolean(id);
  const [loading, setLoading] = useState(isEdit);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState("");
  const [form, setForm] = useState({
    name: "",
    sku: "",
    price: "",
    stock: "",
    description: "",
  });

  useEffect(() => {
    if (!id) return;
    const fetchData = async () => {
      try {
        const product = await api.products.get(Number(id));
        setForm({
          name: product.name,
          sku: product.sku,
          price: String(product.price),
          stock: String(product.stock),
          description: product.description ?? "",
        });
      } catch {
        setError("載入產品資料失敗");
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, [id]);

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ) => {
    setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setSubmitting(true);
    try {
      const data: Record<string, unknown> = {
        name: form.name,
        sku: form.sku,
        price: Number(form.price),
        stock: Number(form.stock),
      };
      if (form.description) data.description = form.description;

      if (isEdit) {
        await api.products.update(Number(id), data);
      } else {
        await api.products.create(data);
      }
      navigate("/products");
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : "送出失敗");
    } finally {
      setSubmitting(false);
    }
  };

  if (loading) {
    return <div className="text-gray-500">載入中...</div>;
  }

  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">
        {isEdit ? "編輯產品" : "新增產品"}
      </h1>
      {error && (
        <div className="bg-red-100 text-red-700 p-3 rounded mb-4 text-sm">{error}</div>
      )}
      <form onSubmit={handleSubmit} className="bg-white rounded-lg shadow p-6 space-y-4">
        <div>
          <label className="block text-sm font-medium mb-1">
            名稱 <span className="text-red-500">*</span>
          </label>
          <input
            type="text"
            name="name"
            value={form.name}
            onChange={handleChange}
            required
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">
            SKU <span className="text-red-500">*</span>
          </label>
          <input
            type="text"
            name="sku"
            value={form.sku}
            onChange={handleChange}
            required
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">
            價格 <span className="text-red-500">*</span>
          </label>
          <input
            type="number"
            step="0.01"
            name="price"
            value={form.price}
            onChange={handleChange}
            required
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">
            庫存 <span className="text-red-500">*</span>
          </label>
          <input
            type="number"
            name="stock"
            value={form.stock}
            onChange={handleChange}
            required
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">說明</label>
          <textarea
            name="description"
            value={form.description}
            onChange={handleChange}
            rows={3}
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div className="flex justify-end gap-3">
          <button
            type="button"
            onClick={() => navigate("/products")}
            className="px-4 py-2 text-sm border rounded hover:bg-gray-100"
          >
            取消
          </button>
          <button
            type="submit"
            disabled={submitting}
            className="px-4 py-2 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
          >
            {submitting ? "送出中..." : isEdit ? "更新" : "建立"}
          </button>
        </div>
      </form>
    </div>
  );
}
