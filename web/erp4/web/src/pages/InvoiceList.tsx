import { useState, useEffect } from "react";
import { api } from "../api/client";
import type { Invoice } from "../types";
import Table from "../components/Table";
import StatusBadge from "../components/StatusBadge";
import Pagination from "../components/Pagination";
import ConfirmDialog from "../components/ConfirmDialog";

const STATUSES = ["", "unpaid", "paid", "overdue", "cancelled"];

export default function InvoiceList() {
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState("");
  const [page, setPage] = useState(1);
  const [total, setTotal] = useState(0);
  const [deleteId, setDeleteId] = useState<number | null>(null);
  const pageSize = 20;

  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      try {
        const params: Record<string, string> = { page: String(page), page_size: String(pageSize) };
        if (status) params.status = status;
        const res = await api.invoices.list(params) as Invoice[] | { data: Invoice[]; total: number };
        const list = Array.isArray(res) ? res : res.data;
        const count = Array.isArray(res) ? res.length : res.total;
        setInvoices(list);
        setTotal(count);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, [page, status]);

  const handleDelete = async () => {
    if (!deleteId) return;
    try {
      await api.invoices.delete(deleteId);
      setDeleteId(null);
      const params: Record<string, string> = { page: String(page), page_size: String(pageSize) };
      if (status) params.status = status;
      const res = await api.invoices.list(params) as Invoice[] | { data: Invoice[]; total: number };
      const list = Array.isArray(res) ? res : res.data;
      const count = Array.isArray(res) ? res.length : res.total;
      setInvoices(list);
      setTotal(count);
    } catch {
      // ignore
    }
  };

  const columns = [
    { key: "id", label: "ID" },
    { key: "invoice_number", label: "發票號碼" },
    { key: "customer_id", label: "客戶ID" },
    { key: "due_date", label: "到期日" },
    { key: "status", label: "狀態", render: (inv: Invoice) => <StatusBadge status={inv.status} /> },
    { key: "amount", label: "金額", render: (inv: Invoice) => inv.amount.toLocaleString() },
    {
      key: "actions",
      label: "操作",
      render: (inv: Invoice) => (
        <button onClick={() => setDeleteId(inv.id)} className="text-red-600 hover:text-red-800">
          刪除
        </button>
      ),
    },
  ];

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">發票管理</h1>
      </div>

      <div className="mb-4">
        <select
          value={status}
          onChange={(e) => { setStatus(e.target.value); setPage(1); }}
          className="border rounded px-3 py-2 text-sm"
        >
          <option value="">全部狀態</option>
          {STATUSES.filter(Boolean).map((s) => (
            <option key={s} value={s}>{s}</option>
          ))}
        </select>
      </div>

      {loading ? (
        <div className="text-gray-500">載入中...</div>
      ) : (
        <>
          <Table columns={columns} data={invoices} />
          <Pagination page={page} pageSize={pageSize} total={total} onChange={setPage} />
        </>
      )}

      <ConfirmDialog
        open={deleteId !== null}
        title="確認刪除"
        message="確定要刪除此發票？此操作不可復原。"
        onConfirm={handleDelete}
        onCancel={() => setDeleteId(null)}
      />
    </div>
  );
}
