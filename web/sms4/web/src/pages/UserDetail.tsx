import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { UserDetail as UserDetailType, PostWithUser, UserBrief } from "../types";
import PostCard from "../components/PostCard";

export default function UserDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [user, setUser] = useState<UserDetailType | null>(null);
  const [posts, setPosts] = useState<PostWithUser[]>([]);
  const [followers, setFollowers] = useState<UserBrief[]>([]);
  const [following, setFollowing] = useState<UserBrief[]>([]);
  const [loading, setLoading] = useState(true);
  const [showFollowers, setShowFollowers] = useState(false);
  const [showFollowing, setShowFollowing] = useState(false);
  const [likedPosts, setLikedPosts] = useState<Set<number>>(new Set());
  const [currentUserId, setCurrentUserId] = useState<number | null>(null);
  const [isFollowing, setIsFollowing] = useState(false);

  useEffect(() => {
    const fetchData = async () => {
      if (!id) return;
      setLoading(true);
      try {
        const uid = parseInt(id);
        const [userData, usersList] = await Promise.all([
          api.users.get(uid),
          api.users.list(),
        ]);
        setUser(userData);
        if (usersList.length > 0) {
          setCurrentUserId(usersList[0].id);
        }

        const [postsData, followersData, followingData] = await Promise.all([
          api.posts.list({ user_id: id }),
          api.users.followers(uid),
          api.users.following(uid),
        ]);
        setPosts(postsData);
        setFollowers(followersData);
        setFollowing(followingData);

        if (usersList.length > 0) {
          const me = usersList[0].id;
          setIsFollowing(followersData.some((f) => f.id === me));
        }
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, [id]);

  const handleFollow = async () => {
    if (currentUserId == null || !user) return;
    if (isFollowing) {
      await api.follow.remove(currentUserId, user.id);
      setIsFollowing(false);
      setUser({ ...user, followers_count: user.followers_count - 1 });
    } else {
      await api.follow.add(currentUserId, user.id);
      setIsFollowing(true);
      setUser({ ...user, followers_count: user.followers_count + 1 });
    }
  };

  const handleLike = async (postId: number) => {
    if (currentUserId == null) return;
    if (likedPosts.has(postId)) {
      await api.likes.remove(currentUserId, postId);
      setLikedPosts((prev) => { const n = new Set(prev); n.delete(postId); return n; });
      setPosts((prev) => prev.map((p) => p.id === postId ? { ...p, likes_count: p.likes_count - 1 } : p));
    } else {
      await api.likes.add(currentUserId, postId);
      setLikedPosts((prev) => { const n = new Set(prev); n.add(postId); return n; });
      setPosts((prev) => prev.map((p) => p.id === postId ? { ...p, likes_count: p.likes_count + 1 } : p));
    }
  };

  if (loading || !user) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-4 border-b border-gray-800">
        <button
          onClick={() => navigate(-1)}
          className="text-white mb-3 inline-flex items-center gap-1 text-sm"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
          返回
        </button>

        <div className="flex items-center gap-4">
          <div className="w-16 h-16 rounded-full bg-gray-700 flex items-center justify-center text-2xl font-bold shrink-0">
            {user.display_name[0]}
          </div>
          <div className="flex-1 min-w-0">
            <h2 className="text-xl font-bold text-white">{user.display_name}</h2>
            <p className="text-sm text-gray-500">@{user.username}</p>
          </div>
          {currentUserId && currentUserId !== user.id && (
            <div className="flex items-center gap-2">
              <button
                onClick={() => navigate(`/messages/${user.id}?uid=${currentUserId}`)}
                className="shrink-0 px-4 py-1.5 rounded-full text-sm font-bold bg-blue-500 text-white hover:bg-blue-600 transition"
              >
                傳送訊息
              </button>
              <button
                onClick={handleFollow}
                className={`shrink-0 px-4 py-1.5 rounded-full text-sm font-bold transition ${
                  isFollowing
                    ? "bg-transparent text-white border border-gray-600 hover:border-red-500 hover:text-red-500"
                    : "bg-white text-black hover:bg-gray-200"
                }`}
              >
                {isFollowing ? "追蹤中" : "追蹤"}
              </button>
            </div>
          )}
        </div>

        {user.bio && (
          <p className="mt-3 text-[15px] text-white">{user.bio}</p>
        )}

        <div className="flex gap-4 mt-3 text-sm">
          <button
            onClick={() => setShowFollowing(true)}
            className="text-gray-500 hover:text-white transition"
          >
            <span className="font-bold text-white">{user.following_count}</span> 追蹤中
          </button>
          <button
            onClick={() => setShowFollowers(true)}
            className="text-gray-500 hover:text-white transition"
          >
            <span className="font-bold text-white">{user.followers_count}</span> 粉絲
          </button>
        </div>
      </div>

      <div className="text-sm text-gray-500 px-4 py-2 border-b border-gray-800">
        貼文 ({posts.length})
      </div>

      {posts.length === 0 ? (
        <div className="text-center py-20 text-gray-500">尚無貼文</div>
      ) : (
        posts.map((post) => (
          <PostCard
            key={post.id}
            post={post}
            showThread
            onLike={() => handleLike(post.id)}
            onReply={() => navigate(`/posts/${post.id}`)}
            liked={likedPosts.has(post.id)}
          />
        ))
      )}

      {showFollowers && (
        <Overlay title="粉絲" users={followers} onClose={() => setShowFollowers(false)} />
      )}
      {showFollowing && (
        <Overlay title="追蹤中" users={following} onClose={() => setShowFollowing(false)} />
      )}
    </div>
  );
}

function Overlay({ title, users, onClose }: { title: string; users: UserBrief[]; onClose: () => void }) {
  const navigate = useNavigate();
  return (
    <div className="fixed inset-0 z-50 bg-black/80 flex items-end" onClick={onClose}>
      <div
        className="w-full max-w-lg mx-auto bg-gray-900 rounded-t-2xl max-h-[60vh] flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between px-4 py-3 border-b border-gray-800">
          <h3 className="font-bold text-white">{title}</h3>
          <button onClick={onClose} className="text-gray-400">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <div className="flex-1 overflow-y-auto">
          {users.length === 0 ? (
            <div className="text-center py-8 text-gray-500">無資料</div>
          ) : (
            users.map((u) => (
              <div
                key={u.id}
                className="flex items-center gap-3 px-4 py-3 hover:bg-gray-800 cursor-pointer"
                onClick={() => { onClose(); navigate(`/users/${u.id}`); }}
              >
                <div className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center text-sm font-bold shrink-0">
                  {u.display_name[0]}
                </div>
                <div>
                  <div className="font-bold text-white text-sm">{u.display_name}</div>
                  <div className="text-xs text-gray-500">@{u.username}</div>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
