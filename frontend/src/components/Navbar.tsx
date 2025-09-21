import { useAuth } from "../contexts/AuthContext"

export default function Navbar() {
  const { user, loading, logout } = useAuth()

  const handleLogin = () => {
    window.location.href = "http://localhost:8080/api/auth/login"
  }

  const handleLogout = async () => {
    await logout()
  }

  return (
    <nav>
      <div className="flex justify-between bg-gray-800 items-center">
        <ul className="flex text-white gap-5 p-5">
          <li>Home</li>
          <li>Team</li>
          <li>Leaderboards</li>
          <li>Players</li>
        </ul>

        <div className="p-3">
          {loading ? null : user ? (
            <div className="flex items-center gap-3 text-white">
              {user.avatar_url ? (
                <img src={user.avatar_url} alt="avatar" className="w-8 h-8 rounded-full" />
              ) : (
                <div className="w-8 h-8 rounded-full bg-gray-600" />
              )}
              <span>{user.username}</span>
              <button className="ml-4 px-3 py-1 bg-red-500 rounded" onClick={handleLogout}>
                Logout
              </button>
            </div>
          ) : (
            <button className="bg-purple-500 text-white px-4 py-2 rounded" onClick={handleLogin}>
              Login
            </button>
          )}
        </div>
      </div>
    </nav>
  )
}