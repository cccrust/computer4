import { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { api } from "../api/client";
import type { Order } from "../types";
import Table from "../components/Table";
import StatusBadge from "../components/StatusBadge";
import Pagination from "../components/Pagination";
import ConfirmDialog from "../components/ConfirmDialog";

const STATUSES = ["", "pending", "confirmed", "shipped", "delivered", "cancelled"];

export default function OrderList() {
  const [orders, setOrders] = useState<Order[]>([]);
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
        const res = await api.orders.list(params) as Order[] | { data: Order[]; total: number };
        const list = Array.isArray(res) ? res : res.data;
        const count = Array.isArray(res) ? res.length : res.total;
        setOrders(list);
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
      await api.orders.delete(deleteId);
      setDeleteId(null);
      const params: Record<string, string> = { page: String(page), page_size: String(pageSize) };
      if (status) params.status = status;
      const res = await api.orders.list(params) as Order[] | { data: Order[]; total: number };
      const list = Array.isArray(res) ? res : res.data;
      const count = Array.isArray(res) ? res.length : res.total;
      setOrders(list);
      setTotal(count);
    } catch {
      // ignore
    }
  };

  const columns = [
    { key: "id", label: "ID" },
    { key: "customer_id", label: "客戶ID" },
    { key: "order_date", label: "日期" },
    { key: "status", label: "狀態", render: (o: Order) => <StatusBadge status={o.status} /> },
    { key: "total_amount", label: "金額", render: (o: Order) => o.total_amount.toLocaleString() },
    {
      key: "actions",
      label: "操作",
      render: (o: Order) => (
        <div className="space-x-2">
          <Link to={`/orders/${o.id}`} className="text-blue-600 hover:text-blue-800">檢視</Link>
          <button onClick={() => setDeleteId(o.id)} className="text-red-600 hover:text-red-800">刪除</button>
        </div>
      ),
    },
  ];

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">訂單管理</h1>
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
          <Table columns={columns} data={orders} />
          <Pagination page={page} pageSize={pageSize} total={total} onChange={setPage} />
        </>
      )}

      <ConfirmDialog
        open={deleteId !== null}
        title="確認刪除"
        message="確定要刪除此訂單？此操作不可復原。"
        onConfirm={handleDelete}
        onCancel={() => setDeleteId(null)}
      />
    </div>
  );
}
