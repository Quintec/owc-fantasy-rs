import { useAuth } from "../contexts/AuthContext"
import { Link } from "react-router-dom";
export default function Navbar() {
  const { user, loading, logout } = useAuth()

  const apiBaseUrl = import.meta.env.VITE_API_URL || "http://localhost:8080";

  const handleLogin = () => {
    window.location.href = `${apiBaseUrl}/api/auth/login`
  }

  const handleLogout = async () => {
    await logout()
  }

  return (
    <nav>
      <div className="flex justify-between bg-gray-800 items-center">
        <ul className="flex text-white gap-5 p-5">
          <li>
            <Link to="/" className="hover:text-purple-400 transition-colors">
              Home
            </Link>
          </li>
          <li>
            <Link to="/team" className="hover:text-purple-400 transition-colors">
              Team
            </Link>
          </li>
          <li>
            <Link to="/leaderboard" className="hover:text-purple-400 transition-colors">
              Leaderboards
            </Link>
          </li>
          <li>
            <Link to="/faq" className="hover:text-purple-400 transition-colors">
              FAQ
            </Link>
          </li>
        </ul>

        <div className="p-3">
          {loading ? null : user ? (
            <div className="flex items-center gap-3 text-white">
              {user.id ? (
                <img src={`https://a.ppy.sh/${user.id}`} alt="avatar" className="w-8 h-8 rounded-full" />
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