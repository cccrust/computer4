import { useEffect, useState } from "react";
import { api } from "../api/client";
import type { DashboardData } from "../types";
import StatusBadge from "../components/StatusBadge";

export default function Dashboard() {
  const [data, setData] = useState<DashboardData | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const result = await api.dashboard();
        setData(result);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, []);

  if (loading) {
    return <div className="text-gray-500">載入中...</div>;
  }

  const cards = [
    { label: "客戶數", value: data?.customer_count },
    { label: "產品數", value: data?.product_count },
    { label: "低庫存品項", value: data?.low_stock_count },
    { label: "待處理訂單", value: data?.pending_orders },
    { label: "待處理採購單", value: data?.pending_pos },
    { label: "逾期發票總額", value: data?.overdue_invoices_total },
  ];

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">儀表板</h1>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 mb-8">
        {cards.map((card) => (
          <div key={card.label} className="bg-white rounded-lg shadow p-6">
            <p className="text-sm text-gray-500">{card.label}</p>
            <p className="text-3xl font-bold mt-1">
              {typeof card.value === "number"
                ? card.value.toLocaleString()
                : card.value}
            </p>
          </div>
        ))}
      </div>
      <h2 className="text-xl font-semibold mb-4">近期訂單</h2>
      <div className="bg-white rounded-lg shadow overflow-hidden">
        <table className="min-w-full">
          <thead>
            <tr className="bg-gray-50 border-b">
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">ID</th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">客戶</th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">日期</th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">狀態</th>
              <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">金額</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {data?.recent_orders.map((o) => (
              <tr key={o.id} className="hover:bg-gray-50">
                <td className="px-4 py-3 text-sm">{o.id}</td>
                <td className="px-4 py-3 text-sm">{o.customer_id}</td>
                <td className="px-4 py-3 text-sm">{o.order_date}</td>
                <td className="px-4 py-3 text-sm">
                  <StatusBadge status={o.status} />
                </td>
                <td className="px-4 py-3 text-sm text-right">
                  {o.total_amount.toLocaleString()}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {(!data || data.recent_orders.length === 0) && (
          <div className="text-center text-gray-500 py-8">暫無訂單</div>
        )}
      </div>
    </div>
  );
}
