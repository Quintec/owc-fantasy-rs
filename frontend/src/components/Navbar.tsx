import { useState } from "react"; 
import { useAuth } from "../contexts/AuthContext";
import { Link } from "react-router-dom";

export default function Navbar() {
  const { user, loading, logout } = useAuth();
  
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);

  const apiBaseUrl = import.meta.env.VITE_API_URL || "http://localhost:8080";

  const handleLogin = () => {
    window.location.href = `${apiBaseUrl}/api/auth/login`;
  };

  const handleLogout = async () => {
    await logout();
    setIsDropdownOpen(false); 
  };

  return (
    <nav>
      <div className="flex justify-between bg-gray-800 items-center">
        <ul className="flex text-white gap-3 sm:gap-5 p-3 sm:p-5 items-center">
          <li>
            <Link to="/" className="hover:text-purple-400 transition-colors">Home</Link>
          </li>
          <li>
            <Link to="/team" className="hover:text-purple-400 transition-colors">Team</Link>
          </li>
          <li>
            <Link to="/leaderboard" className="hover:text-purple-400 transition-colors">Leaderboards</Link>
          </li>
          <li>
            <Link to="/scores" className="hover:text-purple-400 transition-colors">Scores</Link>
          </li>
        </ul>

        <div className="p-3">
          {loading ? null : user ? (
            <div className="relative">
              <button 
                onClick={() => setIsDropdownOpen(!isDropdownOpen)} 
                className="flex items-center gap-2 sm:gap-3 text-white focus:outline-none hover:bg-gray-700 p-1 sm:p-2 rounded transition-colors"
              >
                {user.id ? (
                  <img src={`https://a.ppy.sh/${user.id}`} alt="avatar" className="w-8 h-8 rounded-full" />
                ) : (
                  <div className="w-8 h-8 rounded-full bg-gray-600" />
                )}
                <span className="hidden sm:inline">{user.username}</span>
                <svg className={`w-4 h-4 transition-transform ${isDropdownOpen ? 'rotate-180' : ''}`} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 9l-7 7-7-7"></path></svg>
              </button>

              {isDropdownOpen && (
                <div className="absolute right-0 mt-2 w-48 bg-gray-700 rounded-md shadow-lg z-50 py-1">
                  <button 
                    className="block w-full text-left px-4 py-2 text-sm text-white hover:bg-red-500 hover:text-white transition-colors" 
                    onClick={handleLogout}
                  >
                    Logout
                  </button>
                </div>
              )}
            </div>
          ) : (
            /* UPDATED LOGIN BUTTON: 
               1. text-sm and smaller padding (px-3 py-1) on mobile
               2. Regular size (sm:px-4 sm:text-base) on larger screens
               3. whitespace-nowrap prevents the text from wrapping vertically
            */
            <button 
              className="bg-purple-500 text-white text-sm px-3 py-1 sm:px-4 sm:py-2 sm:text-base rounded whitespace-nowrap transition-transform active:scale-95" 
              onClick={handleLogin}
            >
              Login
            </button>
          )}
        </div>
      </div>
    </nav>
  );
}