import { useState, useEffect } from "react";
import { api } from "../api/client";
import type { Invoice } from "../types";
import StatusBadge from "../components/StatusBadge";

export default function AgingReport() {
  const [data, setData] = useState<Invoice[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api.reports
      .aging()
      .then(setData)
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <div className="text-gray-500">載入中...</div>;

  const total = data.reduce((sum, inv) => sum + inv.amount, 0);

  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">帳齡報表</h1>
      <div className="bg-white rounded-lg shadow overflow-hidden">
        <table className="min-w-full">
          <thead>
            <tr className="bg-gray-50 border-b">
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">ID</th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">發票號碼</th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">客戶ID</th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">到期日</th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">狀態</th>
              <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">金額</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {data.map((inv) => (
              <tr key={inv.id} className="hover:bg-gray-50">
                <td className="px-4 py-3 text-sm">{inv.id}</td>
                <td className="px-4 py-3 text-sm">{inv.invoice_number}</td>
                <td className="px-4 py-3 text-sm">{inv.customer_id}</td>
                <td className="px-4 py-3 text-sm">{inv.due_date}</td>
                <td className="px-4 py-3 text-sm">
                  <StatusBadge status={inv.status} />
                </td>
                <td className="px-4 py-3 text-sm text-right">{inv.amount.toLocaleString()}</td>
              </tr>
            ))}
          </tbody>
          <tfoot className="bg-gray-50 border-t font-semibold">
            <tr>
              <td colSpan={5} className="px-4 py-3 text-sm">合計</td>
              <td className="px-4 py-3 text-sm text-right">{total.toLocaleString()}</td>
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
