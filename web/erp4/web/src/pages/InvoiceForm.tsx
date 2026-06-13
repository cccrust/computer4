import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";

export default function InvoiceForm() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [form, setForm] = useState({
    invoice_number: "",
    order_id: "",
    customer_id: "",
    due_date: "",
    amount: "",
    notes: "",
  });

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ) => {
    setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);
    try {
      const data: Record<string, unknown> = {
        customer_id: Number(form.customer_id),
        due_date: form.due_date,
        amount: Number(form.amount),
      };
      if (form.invoice_number) data.invoice_number = form.invoice_number;
      if (form.order_id) data.order_id = Number(form.order_id);
      if (form.notes) data.notes = form.notes;
      await api.invoices.create(data);
      navigate("/invoices");
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : "建立失敗");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">新增發票</h1>
      {error && (
        <div className="bg-red-100 text-red-700 p-3 rounded mb-4 text-sm">{error}</div>
      )}
      <form onSubmit={handleSubmit} className="bg-white rounded-lg shadow p-6 space-y-4">
        <div>
          <label className="block text-sm font-medium mb-1">發票號碼</label>
          <input
            type="text"
            name="invoice_number"
            value={form.invoice_number}
            onChange={handleChange}
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">訂單 ID</label>
          <input
            type="number"
            name="order_id"
            value={form.order_id}
            onChange={handleChange}
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">
            客戶 ID <span className="text-red-500">*</span>
          </label>
          <input
            type="number"
            name="customer_id"
            value={form.customer_id}
            onChange={handleChange}
            required
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">
            到期日 <span className="text-red-500">*</span>
          </label>
          <input
            type="date"
            name="due_date"
            value={form.due_date}
            onChange={handleChange}
            required
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">
            金額 <span className="text-red-500">*</span>
          </label>
          <input
            type="number"
            step="0.01"
            name="amount"
            value={form.amount}
            onChange={handleChange}
            required
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">備註</label>
          <textarea
            name="notes"
            value={form.notes}
            onChange={handleChange}
            rows={3}
            className="w-full border rounded px-3 py-2"
          />
        </div>
        <div className="flex justify-end gap-3">
          <button
            type="button"
            onClick={() => navigate("/invoices")}
            className="px-4 py-2 text-sm border rounded hover:bg-gray-100"
          >
            取消
          </button>
          <button
            type="submit"
            disabled={loading}
            className="px-4 py-2 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
          >
            {loading ? "送出中..." : "建立"}
          </button>
        </div>
      </form>
    </div>
  );
}
