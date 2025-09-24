
import { useAuth } from '../contexts/AuthContext'

export default function Home() {
  const { user, loading } = useAuth()

  const apiBaseUrl = import.meta.env.VITE_API_URL || "http://localhost:8080";

  const handleLogin = () => {
    window.location.href = `${apiBaseUrl}/api/auth/login`
  }

  return (
    <div className="flex justify-center flex-col items-center p-10">
        <div className="text-center">
            <h1 className="text-6xl font-bold text-white">OWC Fantasy</h1>
            <p className="text-3xl text-gray-400 pt-5 max-w-2xl ">Draft your very own OWC team and score points based on your team's individual performance!</p>
         </div>
         <div className="w-full flex justify-center">
            {loading ? null : user ? (
              <div className="text-center">
                <h2 className="text-2xl text-white mb-4">Welcome back, {user.username}!</h2>
              </div>
            ) : (
              <button 
                className="bg-purple-500 text-white px-4 py-2 my-5 rounded-md text-2xl"
                onClick={handleLogin}
              >
                Login with osu!
              </button>
            )}
         </div>
    </div>
  )
}