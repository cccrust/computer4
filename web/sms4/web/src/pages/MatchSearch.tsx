import { useState, useEffect } from "react";
import { Link, useSearchParams } from "react-router-dom";
import { api } from "../api/client";
import type { ProfileWithUser } from "../types";

export default function MatchSearch() {
  const [searchParams, setSearchParams] = useSearchParams();
  const [results, setResults] = useState<ProfileWithUser[]>([]);
  const [gender, setGender] = useState(searchParams.get("gender") || "");
  const [city, setCity] = useState(searchParams.get("city") || "");
  const [tags, setTags] = useState(searchParams.get("tags") || "");
  const [q, setQ] = useState(searchParams.get("q") || "");
  const [searched, setSearched] = useState(false);

  const doSearch = () => {
    const params: Record<string, string> = {};
    if (gender) params.gender = gender;
    if (city) params.city = city;
    if (tags) params.tags = tags;
    if (q) params.q = q;
    setSearchParams(params);
    api.profiles.search(params).then((data) => {
      setResults(data.results);
      setSearched(true);
    }).catch(() => {});
  };

  useEffect(() => {
    if (searchParams.toString()) doSearch();
  }, []);

  return (
    <div className="px-4 py-6">
      <h2 className="font-bold text-white text-lg mb-4">探索配對</h2>

      <div className="bg-gray-900 rounded-xl p-4 mb-6 space-y-3">
        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="text-xs text-gray-500 block mb-1">性別</label>
            <select value={gender} onChange={(e) => setGender(e.target.value)}
              className="w-full bg-gray-800 text-white rounded-lg px-3 py-2 text-sm outline-none">
              <option value="">不限</option>
              <option value="male">男</option>
              <option value="female">女</option>
            </select>
          </div>
          <div>
            <label className="text-xs text-gray-500 block mb-1">城市</label>
            <input type="text" value={city} onChange={(e) => setCity(e.target.value)} placeholder="例如：台北"
              className="w-full bg-gray-800 text-white rounded-lg px-3 py-2 text-sm outline-none" />
          </div>
        </div>
        <div>
          <label className="text-xs text-gray-500 block mb-1">興趣標籤</label>
          <input type="text" value={tags} onChange={(e) => setTags(e.target.value)} placeholder="以逗號分隔，例如：爬山,攝影"
            className="w-full bg-gray-800 text-white rounded-lg px-3 py-2 text-sm outline-none" />
        </div>
        <div>
          <label className="text-xs text-gray-500 block mb-1">關鍵字</label>
          <input type="text" value={q} onChange={(e) => setQ(e.target.value)} placeholder="搜尋關於我、職業..."
            className="w-full bg-gray-800 text-white rounded-lg px-3 py-2 text-sm outline-none"
            onKeyDown={(e) => e.key === "Enter" && doSearch()} />
        </div>
        <button onClick={doSearch}
          className="w-full bg-blue-500 text-white rounded-lg py-2 font-bold text-sm hover:bg-blue-600 transition">
          搜尋
        </button>
      </div>

      {searched && results.length === 0 && (
        <div className="text-center py-10 text-gray-500">沒有符合條件的使用者</div>
      )}

      <div className="space-y-3">
        {results.map((r) => (
          <Link
            key={r.user_id}
            to={`/profile/${r.user_id}`}
            className="block bg-gray-900 rounded-xl p-4 hover:bg-gray-800 transition"
          >
            <div className="flex items-center gap-3 mb-2">
              <div className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center text-sm font-bold shrink-0">
                {r.display_name[0]}
              </div>
              <div>
                <div className="font-bold text-white text-sm">{r.display_name}</div>
                <div className="text-xs text-gray-500">
                  @{r.username}
                  {r.age && <> · {r.age} 歲</>}
                  {r.city && <> · {r.city}</>}
                </div>
              </div>
            </div>
            {r.tags.length > 0 && (
              <div className="flex flex-wrap gap-1 mb-1">
                {r.tags.map((tag) => (
                  <span key={tag} className="bg-gray-800 text-gray-400 text-xs px-2 py-0.5 rounded-full">{tag}</span>
                ))}
              </div>
            )}
            {r.about_me && (
              <p className="text-xs text-gray-500 line-clamp-2">{r.about_me}</p>
            )}
          </Link>
        ))}
      </div>
    </div>
  );
}
