import { useState, useEffect } from "react";
import { useParams, Link } from "react-router-dom";
import { api } from "../api/client";
import type { PurchaseOrder, POItem } from "../types";
import StatusBadge from "../components/StatusBadge";

const STATUSES = ["pending", "approved", "received", "cancelled"];

export default function PurchaseOrderDetail() {
  const { id } = useParams();
  const [po, setPo] = useState<PurchaseOrder | null>(null);
  const [items, setItems] = useState<POItem[]>([]);
  const [error, setError] = useState("");
  const [newStatus, setNewStatus] = useState("");
  const [showForm, setShowForm] = useState(false);
  const [productId, setProductId] = useState("");
  const [quantity, setQuantity] = useState("");
  const [unitPrice, setUnitPrice] = useState("");

  useEffect(() => {
    const poid = Number(id);
    (async () => {
      try {
        const [p, it] = await Promise.all([
          api.purchaseOrders.get(poid),
          api.purchaseOrders.listItems(poid),
        ]);
        setPo(p);
        setItems(it);
        setNewStatus(p.status);
      } catch (err: unknown) {
        setError(err instanceof Error ? err.message : "載入失敗");
      }
    })();
  }, [id]);

  const handleStatusUpdate = async () => {
    if (!po || newStatus === po.status) return;
    try {
      const updated = await api.purchaseOrders.updateStatus(po.id, newStatus);
      setPo(updated);
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : "更新狀態失敗");
    }
  };

  const handleAddItem = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const newItem = await api.purchaseOrders.addItem(Number(id), {
        product_id: Number(productId),
        quantity: Number(quantity),
        unit_price: Number(unitPrice),
      });
      setItems([...items, newItem]);
      setProductId("");
      setQuantity("");
      setUnitPrice("");
      setShowForm(false);
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : "新增明細失敗");
    }
  };

  if (!po) return <div className="text-gray-500 py-4">載入中...</div>;

  return (
    <div>
      <Link to="/purchase-orders" className="text-blue-600 hover:text-blue-800 mb-4 inline-block">
        &larr; 返回採購單列表
      </Link>

      {error && <div className="bg-red-100 text-red-700 p-3 rounded mb-4 text-sm">{error}</div>}

      <h1 className="text-2xl font-bold mb-4">採購單 #{po.id}</h1>

      <div className="bg-white p-6 rounded-lg shadow mb-6">
        <h2 className="text-lg font-semibold mb-3">採購單資訊</h2>
        <div className="grid grid-cols-2 gap-4 text-sm">
          <div><span className="text-gray-500">供應商ID：</span>{po.supplier_id}</div>
          <div><span className="text-gray-500">日期：</span>{po.order_date}</div>
          <div>
            <span className="text-gray-500">狀態：</span>
            <StatusBadge status={po.status} />
          </div>
          <div><span className="text-gray-500">金額：</span>{Number(po.total_amount).toLocaleString()}</div>
          <div className="col-span-2">
            <span className="text-gray-500">備註：</span>{po.notes || "-"}
          </div>
        </div>
      </div>

      <div className="bg-white p-6 rounded-lg shadow mb-6">
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-lg font-semibold">採購明細</h2>
          <button
            onClick={() => setShowForm(!showForm)}
            className="bg-blue-600 text-white px-3 py-1.5 rounded text-sm hover:bg-blue-700"
          >
            新增明細
          </button>
        </div>

        {showForm && (
          <form onSubmit={handleAddItem} className="mb-4 p-4 border rounded bg-gray-50 space-y-3">
            <div className="grid grid-cols-3 gap-3">
              <div>
                <label className="block text-xs font-medium mb-1">產品ID</label>
                <input
                  type="number"
                  value={productId}
                  onChange={(e) => setProductId(e.target.value)}
                  required
                  className="w-full border rounded px-2 py-1.5 text-sm"
                />
              </div>
              <div>
                <label className="block text-xs font-medium mb-1">數量</label>
                <input
                  type="number"
                  value={quantity}
                  onChange={(e) => setQuantity(e.target.value)}
                  required
                  className="w-full border rounded px-2 py-1.5 text-sm"
                />
              </div>
              <div>
                <label className="block text-xs font-medium mb-1">單價</label>
                <input
                  type="number"
                  step="0.01"
                  value={unitPrice}
                  onChange={(e) => setUnitPrice(e.target.value)}
                  required
                  className="w-full border rounded px-2 py-1.5 text-sm"
                />
              </div>
            </div>
            <div className="flex gap-2">
              <button
                type="submit"
                className="bg-green-600 text-white px-3 py-1.5 rounded text-sm hover:bg-green-700"
              >
                確定新增
              </button>
              <button
                type="button"
                onClick={() => setShowForm(false)}
                className="border px-3 py-1.5 rounded text-sm hover:bg-gray-100"
              >
                取消
              </button>
            </div>
          </form>
        )}

        <table className="min-w-full text-sm">
          <thead>
            <tr className="bg-gray-50 border-b">
              <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">產品ID</th>
              <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">數量</th>
              <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">單價</th>
              <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">小計</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {items.map((item) => (
              <tr key={item.id} className="hover:bg-gray-50">
                <td className="px-4 py-2">{item.product_id}</td>
                <td className="px-4 py-2">{item.quantity}</td>
                <td className="px-4 py-2">{Number(item.unit_price).toLocaleString()}</td>
                <td className="px-4 py-2">{(item.quantity * item.unit_price).toLocaleString()}</td>
              </tr>
            ))}
            {items.length === 0 && (
              <tr>
                <td colSpan={4} className="px-4 py-4 text-center text-gray-400">暫無明細</td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-lg font-semibold mb-3">更新狀態</h2>
        <div className="flex gap-3 items-center">
          <select
            value={newStatus}
            onChange={(e) => setNewStatus(e.target.value)}
            className="border rounded px-3 py-2"
          >
            {STATUSES.map((s) => (
              <option key={s} value={s}>{s}</option>
            ))}
          </select>
          <button
            onClick={handleStatusUpdate}
            disabled={newStatus === po.status}
            className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 disabled:opacity-50"
          >
            確認更新
          </button>
        </div>
      </div>
    </div>
  );
}
