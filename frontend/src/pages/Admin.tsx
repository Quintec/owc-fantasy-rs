import { useState } from 'react';
import { useRound } from '../contexts/RoundContext';
import { parseMultiplayerLinks, eliminatePlayers, unEliminatePlayers } from '../api/getPlayers';
import { useAuth } from '../contexts/AuthContext';

export default function Admin() {
  const { round } = useRound();
  const { user } = useAuth();
  const [linksText, setLinksText] = useState('');
  const [status, setStatus] = useState<string | null>(null);
  const [country, setCountry] = useState<string>('');

  const submitLinks = async () => {
    try {
      setStatus('Submitting links...');
      const links = linksText.split('\n').map(l => l.trim()).filter(Boolean);
      const res = await parseMultiplayerLinks(links, round);
      setStatus('Parsed: ' + JSON.stringify(res));
    } catch (err: any) {
      setStatus('Error: ' + (err?.message || String(err)));
    }
  };

  const eliminateTeam = async () => {
    try {
      setStatus('Eliminating players...');
      const res = await eliminatePlayers(country || undefined);
      setStatus('Eliminated player ids: ' + JSON.stringify(res.eliminated) + (country ? ` (filtered by country=${country})` : ''));
    } catch (err: any) {
      setStatus('Error: ' + (err?.message || String(err)));
    }
  };

  if (!user) return <div className="p-5 text-white">You must be logged in to use admin tools.</div>;

  return (
    <div className="p-6 text-white">
      <h1 className="text-2xl font-bold mb-4">Admin</h1>

      <div className="mb-4">
        <label className="mr-2">Round (auto):</label>
        <span className="ml-2 font-semibold">{round}</span>
      </div>

      <div className="mb-4">
        <label className="block mb-1">Multiplayer links (one per line):</label>
        <textarea value={linksText} onChange={(e) => setLinksText(e.target.value)} className="w-full bg-gray-900 p-2 rounded h-40" />
        <button onClick={submitLinks} className="mt-2 bg-purple-600 px-4 py-2 rounded">Parse links</button>
      </div>

      <div className="mb-4">
        <label className="block mb-1">Eliminate players in tournament:</label>
        <input placeholder="Optional country code (e.g. US)" value={country} onChange={e => setCountry(e.target.value.toUpperCase())} className="bg-gray-900 p-2 rounded mr-2 w-48" />
        <button onClick={eliminateTeam} className="bg-red-600 px-4 py-2 rounded mr-2">Eliminate players</button>
        <button onClick={async () => {
          try {
            setStatus('Un-eliminating players...');
            const res = await unEliminatePlayers(country || undefined);
            setStatus('Un-eliminated player ids: ' + JSON.stringify(res.uneliminated) + (country ? ` (filtered by country=${country})` : ''));
          } catch (err: any) {
            setStatus('Error: ' + (err?.message || String(err)));
          }
        }} className="bg-green-600 px-4 py-2 rounded">Un-eliminate players</button>
      </div>

      {status && (
        <div className="mt-4 p-2 bg-gray-800 rounded">
          <pre className="whitespace-pre-wrap">{status}</pre>
        </div>
      )}
    </div>
  );
}
