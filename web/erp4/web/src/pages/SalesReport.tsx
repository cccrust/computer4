import { useState, useEffect } from "react";
import { api } from "../api/client";
import type { SalesRow } from "../types";
import Table from "../components/Table";

export default function SalesReport() {
  const [data, setData] = useState<SalesRow[]>([]);
  const [loading, setLoading] = useState(false);
  const [from, setFrom] = useState("");
  const [to, setTo] = useState("");

  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      try {
        const params: Record<string, string> = {};
        if (from) params.from = from;
        if (to) params.to = to;
        const result = await api.reports.sales(params);
        setData(result);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, [from, to]);

  const totalQty = data.reduce((sum, r) => sum + r.total_qty, 0);
  const totalAmount = data.reduce((sum, r) => sum + r.total_amount, 0);

  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">銷售報表</h1>
      <div className="flex gap-4 mb-4">
        <div>
          <label className="block text-sm font-medium mb-1">起始日期</label>
          <input
            type="date"
            value={from}
            onChange={(e) => setFrom(e.target.value)}
            className="border rounded px-3 py-2"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">結束日期</label>
          <input
            type="date"
            value={to}
            onChange={(e) => setTo(e.target.value)}
            className="border rounded px-3 py-2"
          />
        </div>
      </div>
      {loading ? (
        <div className="text-gray-500">載入中...</div>
      ) : (
        <div className="bg-white rounded-lg shadow overflow-hidden">
          <table className="min-w-full">
            <thead>
              <tr className="bg-gray-50 border-b">
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">產品ID</th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">產品名稱</th>
                <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">銷售量</th>
                <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">金額</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200">
              {data.map((row, idx) => (
                <tr key={idx} className="hover:bg-gray-50">
                  <td className="px-4 py-3 text-sm">{row.product_id}</td>
                  <td className="px-4 py-3 text-sm">{row.product_name}</td>
                  <td className="px-4 py-3 text-sm text-right">{row.total_qty}</td>
                  <td className="px-4 py-3 text-sm text-right">{row.total_amount.toLocaleString()}</td>
                </tr>
              ))}
            </tbody>
            <tfoot className="bg-gray-50 border-t font-semibold">
              <tr>
                <td colSpan={2} className="px-4 py-3 text-sm">合計</td>
                <td className="px-4 py-3 text-sm text-right">{totalQty}</td>
                <td className="px-4 py-3 text-sm text-right">{totalAmount.toLocaleString()}</td>
              </tr>
            </tfoot>
          </table>
          {data.length === 0 && (
            <div className="text-center text-gray-500 py-8">無資料</div>
          )}
        </div>
      )}
    </div>
  );
}
