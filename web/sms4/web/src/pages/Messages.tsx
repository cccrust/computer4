import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { Conversation } from "../types";

export default function Messages() {
  const navigate = useNavigate();
  const [convs, setConvs] = useState<Conversation[]>([]);
  const [currentUserId, setCurrentUserId] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [unread, setUnread] = useState(0);

  useEffect(() => {
    const init = async () => {
      try {
        const userList = await api.users.list();
        if (userList.length > 0) {
          const uid = userList[0].id;
          setCurrentUserId(uid);
          const [convsData, unreadData] = await Promise.all([
            api.messages.conversations(uid),
            api.messages.unread(uid),
          ]);
          setConvs(convsData);
          setUnread(unreadData.unread);
        }
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    init();
  }, []);

  const startConversation = (otherId: number) => {
    navigate(`/messages/${otherId}?uid=${currentUserId}`);
  };

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center justify-between">
        <h2 className="font-bold text-white">私訊</h2>
        {unread > 0 && (
          <span className="text-xs text-red-500">{unread} 未讀</span>
        )}
      </div>

      {convs.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">尚無對話</p>
          <p className="text-sm mt-1">前往「使用者」頁面搜尋並傳送訊息</p>
        </div>
      ) : (
        convs.map((c) => (
          <div
            key={c.other_user_id}
            onClick={() => startConversation(c.other_user_id)}
            className="border-b border-gray-800 px-4 py-3 hover:bg-gray-900/50 transition cursor-pointer"
          >
            <div className="flex items-center gap-3">
              <div className="w-12 h-12 rounded-full bg-gray-700 flex items-center justify-center text-lg font-bold shrink-0">
                {c.other_display_name[0]}
              </div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <span className="font-bold text-white text-sm">{c.other_display_name}</span>
                  {c.unread_count > 0 && (
                    <span className="bg-blue-500 text-white text-xs rounded-full px-1.5 py-0.5">
                      {c.unread_count}
                    </span>
                  )}
                </div>
                <div className="text-sm text-gray-500 truncate">{c.last_message}</div>
              </div>
              <span className="text-xs text-gray-600 shrink-0">
                {new Date(c.last_message_at).toLocaleDateString("zh-TW", { month: "short", day: "numeric" })}
              </span>
            </div>
          </div>
        ))
      )}
    </div>
  );
}
