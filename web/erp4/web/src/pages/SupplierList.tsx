import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { Supplier } from "../types";
import Table from "../components/Table";
import Pagination from "../components/Pagination";
import ConfirmDialog from "../components/ConfirmDialog";

export default function SupplierList() {
  const navigate = useNavigate();
  const [suppliers, setSuppliers] = useState<Supplier[]>([]);
  const [loading, setLoading] = useState(false);
  const [search, setSearch] = useState("");
  const [page, setPage] = useState(1);
  const [total, setTotal] = useState(0);
  const [deleteId, setDeleteId] = useState<number | null>(null);
  const pageSize = 20;

  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      try {
        const params: Record<string, string> = { page: String(page), page_size: String(pageSize) };
        if (search) params.search = search;
        const res = await api.suppliers.list(params) as Supplier[] | { data: Supplier[]; total: number };
        const list = Array.isArray(res) ? res : res.data;
        const count = Array.isArray(res) ? res.length : res.total;
        setSuppliers(list);
        setTotal(count);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, [page, search]);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    setPage(1);
  };

  const handleDelete = async () => {
    if (!deleteId) return;
    try {
      await api.suppliers.delete(deleteId);
      setDeleteId(null);
      const params: Record<string, string> = { page: String(page), page_size: String(pageSize) };
      if (search) params.search = search;
      const res = await api.suppliers.list(params) as Supplier[] | { data: Supplier[]; total: number };
      const list = Array.isArray(res) ? res : res.data;
      const count = Array.isArray(res) ? res.length : res.total;
      setSuppliers(list);
      setTotal(count);
    } catch {
      // ignore
    }
  };

  const columns = [
    { key: "id", label: "ID" },
    { key: "name", label: "名稱" },
    { key: "contact_person", label: "聯絡人" },
    { key: "email", label: "Email" },
    { key: "phone", label: "電話" },
  ];

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">供應商管理</h1>
        <button
          onClick={() => navigate("/suppliers/new")}
          className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 text-sm"
        >
          新增供應商
        </button>
      </div>

      <form onSubmit={handleSearch} className="mb-4 flex gap-2">
        <input
          type="text"
          placeholder="搜尋供應商..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="border rounded px-3 py-2 w-64 text-sm"
        />
        <button
          type="submit"
          className="px-4 py-2 text-sm border rounded hover:bg-gray-100"
        >
          搜尋
        </button>
      </form>

      {loading ? (
        <div className="text-gray-500">載入中...</div>
      ) : (
        <>
          <Table
            columns={columns}
            data={suppliers}
            onEdit={(item) => navigate(`/suppliers/${item.id}/edit`)}
            onDelete={(item) => setDeleteId(item.id)}
          />
          <Pagination page={page} pageSize={pageSize} total={total} onChange={setPage} />
        </>
      )}

      <ConfirmDialog
        open={deleteId !== null}
        title="確認刪除"
        message="確定要刪除此供應商？此操作不可復原。"
        onConfirm={handleDelete}
        onCancel={() => setDeleteId(null)}
      />
    </div>
  );
}
