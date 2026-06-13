interface StatusBadgeProps {
  status: string;
}

const colorMap: Record<string, string> = {
  pending: "bg-yellow-100 text-yellow-800",
  overdue: "bg-yellow-100 text-yellow-800",
  confirmed: "bg-green-100 text-green-800",
  approved: "bg-green-100 text-green-800",
  paid: "bg-green-100 text-green-800",
  shipped: "bg-blue-100 text-blue-800",
  delivered: "bg-blue-100 text-blue-800",
  cancelled: "bg-red-100 text-red-800",
  unpaid: "bg-orange-100 text-orange-800",
};

export default function StatusBadge({ status }: StatusBadgeProps) {
  const colorClass = colorMap[status] ?? "bg-gray-100 text-gray-800";

  return (
    <span
      className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${colorClass}`}
    >
      {status}
    </span>
  );
}
