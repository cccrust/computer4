import { useState, useEffect } from "react";
import { useParams, Link } from "react-router-dom";
import { api } from "../api/client";
import type { Profile, Interest } from "../types";

export default function ProfilePage() {
  const { id } = useParams<{ id: string }>();
  const [profile, setProfile] = useState<Profile | null>(null);
  const [tags, setTags] = useState<string[]>([]);
  const [user, setUser] = useState<{ id: number; username: string; display_name: string; bio: string | null } | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!id) return;
    const fetch = async () => {
      try {
        const uid = parseInt(id);
        const [userData, profileData] = await Promise.all([
          api.users.get(uid),
          api.profiles.get(uid).catch(() => null),
        ]);
        setUser(userData);
        if (profileData) {
          setProfile(profileData.profile);
          setTags(profileData.tags);
        }
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetch();
  }, [id]);

  if (loading) return <div className="text-center py-20 text-gray-500">載入中...</div>;
  if (!user) return <div className="text-center py-20 text-gray-500">使用者不存在</div>;

  const age = profile?.birthday
    ? new Date().getFullYear() - new Date(profile.birthday).getFullYear()
    : null;

  return (
    <div className="px-4 py-6">
      <div className="flex items-center gap-4 mb-6">
        <div className="w-16 h-16 rounded-full bg-gray-700 flex items-center justify-center text-2xl font-bold shrink-0">
          {user.display_name[0]}
        </div>
        <div>
          <h2 className="text-xl font-bold text-white">{user.display_name}</h2>
          <p className="text-sm text-gray-500">@{user.username}</p>
          {user.bio && <p className="text-sm text-gray-400 mt-1">{user.bio}</p>}
        </div>
      </div>

      {!profile ? (
        <div className="text-center py-10">
          <p className="text-gray-500 mb-4">該使用者尚未填寫交友資料</p>
          <Link
            to="/profile/edit"
            className="text-blue-500 text-sm font-bold"
          >
            前往填寫
          </Link>
        </div>
      ) : (
        <div className="space-y-4">
          <div className="bg-gray-900 rounded-xl p-4 space-y-3">
            <h3 className="font-bold text-white text-sm">基本資料</h3>
            {profile.gender && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">性別</span>
                <span className="text-white">{profile.gender === "male" ? "男" : profile.gender === "female" ? "女" : profile.gender}</span>
              </div>
            )}
            {age && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">年齡</span>
                <span className="text-white">{age} 歲</span>
              </div>
            )}
            {profile.city && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">城市</span>
                <span className="text-white">{profile.city}</span>
              </div>
            )}
            {profile.occupation && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">職業</span>
                <span className="text-white">{profile.occupation}</span>
              </div>
            )}
            {profile.education && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">學歷</span>
                <span className="text-white">{profile.education}</span>
              </div>
            )}
            {profile.height && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">身高</span>
                <span className="text-white">{profile.height} cm</span>
              </div>
            )}
            {profile.looking_for && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">交友目的</span>
                <span className="text-white">
                  {profile.looking_for === "friend" ? "交朋友" : profile.looking_for === "date" ? "約會" : profile.looking_for}
                </span>
              </div>
            )}
          </div>

          {profile.about_me && (
            <div className="bg-gray-900 rounded-xl p-4">
              <h3 className="font-bold text-white text-sm mb-2">關於我</h3>
              <p className="text-sm text-gray-300">{profile.about_me}</p>
            </div>
          )}

          {tags.length > 0 && (
            <div className="bg-gray-900 rounded-xl p-4">
              <h3 className="font-bold text-white text-sm mb-2">興趣</h3>
              <div className="flex flex-wrap gap-2">
                {tags.map((tag) => (
                  <Link
                    key={tag}
                    to={`/search?tags=${tag}`}
                    className="bg-gray-800 text-gray-300 text-xs px-3 py-1 rounded-full hover:bg-gray-700 transition"
                  >
                    {tag}
                  </Link>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
