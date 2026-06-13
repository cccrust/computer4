const BASE = "/api";

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...(options?.headers as Record<string, string>),
  };
  const res = await fetch(`${BASE}${path}`, { ...options, headers });
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(body.error || "請求失敗");
  }
  return res.json();
}

export const api = {
  users: {
    list: (params?: Record<string, string>) =>
      request<import("../types").User[]>(`/users?${new URLSearchParams(params)}`),
    get: (id: number) => request<import("../types").UserDetail>(`/users/${id}`),
    create: (data: { username: string; display_name: string; bio?: string }) =>
      request<import("../types").User>("/users", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    update: (id: number, data: { display_name?: string; bio?: string }) =>
      request<import("../types").User>(`/users/${id}`, {
        method: "PUT",
        body: JSON.stringify(data),
      }),
    delete: (id: number) => request<{ deleted: boolean }>(`/users/${id}`, { method: "DELETE" }),
    timeline: (id: number) => request<import("../types").PostWithUser[]>(`/users/${id}/timeline`),
    followers: (id: number) => request<import("../types").UserBrief[]>(`/users/${id}/followers`),
    following: (id: number) => request<import("../types").UserBrief[]>(`/users/${id}/following`),
  },
  posts: {
    list: (params?: Record<string, string>) =>
      request<import("../types").PostWithUser[]>(`/posts?${new URLSearchParams(params)}`),
    get: (id: number) => request<import("../types").PostDetail>(`/posts/${id}`),
    create: (data: { user_id: number; content: string }) =>
      request<import("../types").PostWithUser>("/posts", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    reply: (id: number, data: { user_id: number; content: string }) =>
      request<import("../types").PostWithUser>(`/posts/${id}/reply`, {
        method: "POST",
        body: JSON.stringify(data),
      }),
    delete: (id: number) => request<{ deleted: boolean }>(`/posts/${id}`, { method: "DELETE" }),
  },
  follow: {
    add: (follower_id: number, followee_id: number) =>
      request<{ message: string }>("/follow", {
        method: "POST",
        body: JSON.stringify({ follower_id, followee_id }),
      }),
    remove: (follower_id: number, followee_id: number) =>
      request<{ message: string }>("/follow", {
        method: "DELETE",
        body: JSON.stringify({ follower_id, followee_id }),
      }),
  },
  messages: {
    send: (sender_id: number, receiver_id: number, content: string) =>
      request<{ id: number; message: string }>("/messages/send", {
        method: "POST",
        body: JSON.stringify({ sender_id, receiver_id, content }),
      }),
    conversations: (userId: number) =>
      request<import("../types").Conversation[]>(`/messages/${userId}/conversations`),
    messages: (userId: number, otherId: number) =>
      request<import("../types").MessageWithUser[]>(`/messages/${userId}/${otherId}`),
    unread: (userId: number) =>
      request<{ unread: number }>(`/messages/${userId}/unread`),
  },
  profiles: {
    get: (userId: number) =>
      request<{ profile: import("../types").Profile; tags: string[] }>(`/profiles/${userId}`),
    update: (userId: number, data: Record<string, string | number | null>) =>
      request<{ message: string }>(`/profiles/${userId}`, {
        method: "PUT",
        body: JSON.stringify(data),
      }),
    search: (params: Record<string, string>) =>
      request<{ results: import("../types").ProfileWithUser[]; count: number }>(
        `/profiles/search?${new URLSearchParams(params)}`
      ),
  },
  interests: {
    add: (userId: number, tag: string) =>
      request<{ id: number }>("/interests", {
        method: "POST",
        body: JSON.stringify({ user_id: userId, tag }),
      }),
    remove: (userId: number, tag: string) =>
      request<{ message: string }>("/interests", {
        method: "DELETE",
        body: JSON.stringify({ user_id: userId, tag }),
      }),
    list: (userId: number) =>
      request<{ interests: import("../types").Interest[] }>(`/interests/${userId}`),
  },
  likes: {
    add: (user_id: number, post_id: number) =>
      request<{ message: string }>("/likes", {
        method: "POST",
        body: JSON.stringify({ user_id, post_id }),
      }),
    remove: (user_id: number, post_id: number) =>
      request<{ message: string }>("/likes", {
        method: "DELETE",
        body: JSON.stringify({ user_id, post_id }),
      }),
  },
};
