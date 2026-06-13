import { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { api } from "../api/client";
import type { PurchaseOrder } from "../types";
import Table from "../components/Table";
import StatusBadge from "../components/StatusBadge";
import Pagination from "../components/Pagination";
import ConfirmDialog from "../components/ConfirmDialog";

const STATUSES = ["", "pending", "approved", "received", "cancelled"];

export default function PurchaseOrderList() {
  const [pos, setPos] = useState<PurchaseOrder[]>([]);
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
        const res = await api.purchaseOrders.list(params) as PurchaseOrder[] | { data: PurchaseOrder[]; total: number };
        const list = Array.isArray(res) ? res : res.data;
        const count = Array.isArray(res) ? res.length : res.total;
        setPos(list);
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
      await api.purchaseOrders.delete(deleteId);
      setDeleteId(null);
      const params: Record<string, string> = { page: String(page), page_size: String(pageSize) };
      if (status) params.status = status;
      const res = await api.purchaseOrders.list(params) as PurchaseOrder[] | { data: PurchaseOrder[]; total: number };
      const list = Array.isArray(res) ? res : res.data;
      const count = Array.isArray(res) ? res.length : res.total;
      setPos(list);
      setTotal(count);
    } catch {
      // ignore
    }
  };

  const columns = [
    { key: "id", label: "ID" },
    { key: "supplier_id", label: "供應商ID" },
    { key: "order_date", label: "日期" },
    { key: "status", label: "狀態", render: (po: PurchaseOrder) => <StatusBadge status={po.status} /> },
    { key: "total_amount", label: "金額", render: (po: PurchaseOrder) => po.total_amount.toLocaleString() },
    {
      key: "actions",
      label: "操作",
      render: (po: PurchaseOrder) => (
        <div className="space-x-2">
          <Link to={`/purchase-orders/${po.id}`} className="text-blue-600 hover:text-blue-800">檢視</Link>
          <button onClick={() => setDeleteId(po.id)} className="text-red-600 hover:text-red-800">刪除</button>
        </div>
      ),
    },
  ];

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">採購單管理</h1>
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
          <Table columns={columns} data={pos} />
          <Pagination page={page} pageSize={pageSize} total={total} onChange={setPage} />
        </>
      )}

      <ConfirmDialog
        open={deleteId !== null}
        title="確認刪除"
        message="確定要刪除此採購單？此操作不可復原。"
        onConfirm={handleDelete}
        onCancel={() => setDeleteId(null)}
      />
    </div>
  );
}
