import { BrowserRouter, Routes, Route } from "react-router-dom";
import Layout from "./components/Layout";
import Timeline from "./pages/Timeline";
import Messages from "./pages/Messages";
import Conversation from "./pages/Conversation";
import ProfilePage from "./pages/ProfilePage";
import ProfileEdit from "./pages/ProfileEdit";
import MatchSearch from "./pages/MatchSearch";
import UserList from "./pages/UserList";
import UserDetail from "./pages/UserDetail";
import PostDetail from "./pages/PostDetail";

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route index element={<Timeline />} />
          <Route path="messages" element={<Messages />} />
          <Route path="messages/:otherId" element={<Conversation />} />
          <Route path="profile/:id" element={<ProfilePage />} />
          <Route path="profile/edit" element={<ProfileEdit />} />
          <Route path="search" element={<MatchSearch />} />
          <Route path="users" element={<UserList />} />
          <Route path="users/:id" element={<UserDetail />} />
          <Route path="posts/:id" element={<PostDetail />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
