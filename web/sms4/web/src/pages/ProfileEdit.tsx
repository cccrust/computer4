import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";

export default function ProfileEdit() {
  const navigate = useNavigate();
  const [userId, setUserId] = useState<number | null>(null);
  const [users, setUsers] = useState<{ id: number; username: string; display_name: string }[]>([]);
  const [birthday, setBirthday] = useState("");
  const [gender, setGender] = useState("");
  const [city, setCity] = useState("");
  const [occupation, setOccupation] = useState("");
  const [education, setEducation] = useState("");
  const [height, setHeight] = useState("");
  const [lookingFor, setLookingFor] = useState("friend");
  const [aboutMe, setAboutMe] = useState("");
  const [newTag, setNewTag] = useState("");
  const [tags, setTags] = useState<string[]>([]);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    api.users.list().then((list) => {
      setUsers(list);
      if (list.length > 0) {
        const uid = list[0].id;
        setUserId(uid);
        api.profiles.get(uid).then((data) => {
          const p = data.profile;
          setBirthday(p.birthday || "");
          setGender(p.gender || "");
          setCity(p.city || "");
          setOccupation(p.occupation || "");
          setEducation(p.education || "");
          setHeight(p.height?.toString() || "");
          setLookingFor(p.looking_for || "friend");
          setAboutMe(p.about_me || "");
          setTags(data.tags);
        }).catch(() => {});
      }
    });
  }, []);

  const handleSave = async () => {
    if (!userId) return;
    setSaving(true);
    try {
      await api.profiles.update(userId, {
        birthday: birthday || null,
        gender: gender || null,
        city: city || null,
        occupation: occupation || null,
        education: education || null,
        height: height ? parseInt(height) : null,
        looking_for: lookingFor,
        about_me: aboutMe || null,
      });
      navigate(`/profile/${userId}`);
    } catch (e: any) {
      alert(e.message);
    } finally {
      setSaving(false);
    }
  };

  const addTag = async () => {
    if (!userId || !newTag.trim()) return;
    await api.interests.add(userId, newTag.trim());
    setTags([...tags, newTag.trim()]);
    setNewTag("");
  };

  const removeTag = async (tag: string) => {
    if (!userId) return;
    await api.interests.remove(userId, tag);
    setTags(tags.filter((t) => t !== tag));
  };

  return (
    <div className="px-4 py-6">
      <h2 className="font-bold text-white text-lg mb-4">編輯交友資料</h2>

      <div className="mb-4 flex gap-2 overflow-x-auto">
        {users.map((u) => (
          <button
            key={u.id}
            onClick={() => setUserId(u.id)}
            className={`shrink-0 text-xs px-3 py-1 rounded-full border transition ${
              userId === u.id
                ? "bg-white text-black border-white"
                : "bg-transparent text-gray-400 border-gray-700"
            }`}
          >
            @{u.username}
          </button>
        ))}
      </div>

      <div className="space-y-4">
        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="text-xs text-gray-500 block mb-1">生日</label>
            <input type="date" value={birthday} onChange={(e) => setBirthday(e.target.value)}
              className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600" />
          </div>
          <div>
            <label className="text-xs text-gray-500 block mb-1">性別</label>
            <select value={gender} onChange={(e) => setGender(e.target.value)}
              className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600">
              <option value="">選擇性別</option>
              <option value="male">男</option>
              <option value="female">女</option>
              <option value="other">其他</option>
            </select>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="text-xs text-gray-500 block mb-1">城市</label>
            <input type="text" value={city} onChange={(e) => setCity(e.target.value)} placeholder="例如：台北"
              className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600" />
          </div>
          <div>
            <label className="text-xs text-gray-500 block mb-1">身高 (cm)</label>
            <input type="number" value={height} onChange={(e) => setHeight(e.target.value)} placeholder="170"
              className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600" />
          </div>
        </div>

        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="text-xs text-gray-500 block mb-1">職業</label>
            <input type="text" value={occupation} onChange={(e) => setOccupation(e.target.value)} placeholder="工程師"
              className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600" />
          </div>
          <div>
            <label className="text-xs text-gray-500 block mb-1">學歷</label>
            <select value={education} onChange={(e) => setEducation(e.target.value)}
              className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600">
              <option value="">選擇學歷</option>
              <option value="高中">高中</option>
              <option value="大學">大學</option>
              <option value="碩士">碩士</option>
              <option value="博士">博士</option>
            </select>
          </div>
        </div>

        <div>
          <label className="text-xs text-gray-500 block mb-1">交友目的</label>
          <select value={lookingFor} onChange={(e) => setLookingFor(e.target.value)}
            className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600">
            <option value="friend">交朋友</option>
            <option value="date">約會</option>
            <option value="any">不拘</option>
          </select>
        </div>

        <div>
          <label className="text-xs text-gray-500 block mb-1">關於我</label>
          <textarea value={aboutMe} onChange={(e) => setAboutMe(e.target.value)} rows={3}
            placeholder="簡單介紹自己..."
            className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600 resize-none" />
        </div>

        <div>
          <label className="text-xs text-gray-500 block mb-1">興趣標籤</label>
          <div className="flex gap-2 mb-2">
            <input type="text" value={newTag} onChange={(e) => setNewTag(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && addTag()}
              placeholder="新增興趣..."
              className="flex-1 bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600" />
            <button onClick={addTag}
              className="bg-gray-700 text-white px-3 py-2 rounded-lg text-sm hover:bg-gray-600 transition">
              新增
            </button>
          </div>
          <div className="flex flex-wrap gap-2">
            {tags.map((tag) => (
              <span key={tag} className="bg-gray-800 text-gray-300 text-xs px-3 py-1 rounded-full flex items-center gap-1">
                {tag}
                <button onClick={() => removeTag(tag)} className="text-red-400 hover:text-red-300">&times;</button>
              </span>
            ))}
          </div>
        </div>

        <button
          onClick={handleSave}
          disabled={saving}
          className="w-full bg-blue-500 text-white rounded-lg py-3 font-bold hover:bg-blue-600 disabled:opacity-50 transition"
        >
          {saving ? "儲存中..." : "儲存"}
        </button>
      </div>
    </div>
  );
}
