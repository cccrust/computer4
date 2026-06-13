import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";

export default function SupplierForm() {
  const { id } = useParams();
  const navigate = useNavigate();
  const isEdit = Boolean(id);
  const [loading, setLoading] = useState(isEdit);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState("");
  const [form, setForm] = useState({
    name: "",
    contact_person: "",
    email: "",
    phone: "",
    address: "",
  });

  useEffect(() => {
    if (!id) return;
    const fetchData = async () => {
      try {
        const s = await api.suppliers.get(Number(id));
        setForm({
          name: s.name,
          contact_person: s.contact_person ?? "",
          email: s.email ?? "",
          phone: s.phone ?? "",
          address: s.address ?? "",
        });
      } catch {
        setError("載入供應商資料失敗");
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
      const data: Record<string, unknown> = { name: form.name };
      if (form.contact_person) data.contact_person = form.contact_person;
      if (form.email) data.email = form.email;
      if (form.phone) data.phone = form.phone;
      if (form.address) data.address = form.address;

      if (isEdit) {
        await api.suppliers.update(Number(id), data);
      } else {
        await api.suppliers.create(data);
      }
      navigate("/suppliers");
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
        {isEdit ? "編輯供應商" : "新增供應商"}
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
          <label className="block text-sm font-medium mb-1">聯絡人</label>
          <input
            type="text"
            name="contact_person"
            value={form.contact_person}
            onChange={handleChange}
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Email</label>
          <input
            type="email"
            name="email"
            value={form.email}
            onChange={handleChange}
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">電話</label>
          <input
            type="text"
            name="phone"
            value={form.phone}
            onChange={handleChange}
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">地址</label>
          <textarea
            name="address"
            value={form.address}
            onChange={handleChange}
            rows={3}
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div className="flex justify-end gap-3">
          <button
            type="button"
            onClick={() => navigate("/suppliers")}
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
