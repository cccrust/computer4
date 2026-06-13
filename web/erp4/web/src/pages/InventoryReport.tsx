import { useState, useEffect } from "react";
import { api } from "../api/client";
import type { Product } from "../types";

export default function InventoryReport() {
  const [data, setData] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api.reports
      .inventory()
      .then(setData)
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <div className="text-gray-500">載入中...</div>;

  const grandTotal = data.reduce(
    (sum, p) => sum + p.price * p.stock,
    0,
  );

  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">庫存報表</h1>
      <div className="bg-white rounded-lg shadow overflow-hidden">
        <table className="min-w-full">
          <thead>
            <tr className="bg-gray-50 border-b">
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">ID</th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">名稱</th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">SKU</th>
              <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">價格</th>
              <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">庫存</th>
              <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">價值</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {data.map((p) => (
              <tr key={p.id} className="hover:bg-gray-50">
                <td className="px-4 py-3 text-sm">{p.id}</td>
                <td className="px-4 py-3 text-sm">{p.name}</td>
                <td className="px-4 py-3 text-sm">{p.sku}</td>
                <td className="px-4 py-3 text-sm text-right">{p.price.toLocaleString()}</td>
                <td className="px-4 py-3 text-sm text-right">{p.stock}</td>
                <td className="px-4 py-3 text-sm text-right">
                  {(p.price * p.stock).toLocaleString()}
                </td>
              </tr>
            ))}
          </tbody>
          <tfoot className="bg-gray-50 border-t font-semibold">
            <tr>
              <td colSpan={5} className="px-4 py-3 text-sm">總價值</td>
              <td className="px-4 py-3 text-sm text-right">{grandTotal.toLocaleString()}</td>
            </tr>
          </tfoot>
        </table>
        {data.length === 0 && (
          <div className="text-center text-gray-500 py-8">無資料</div>
        )}
      </div>
    </div>
  );
}
