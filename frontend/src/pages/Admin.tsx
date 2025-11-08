import { useState, useEffect } from 'react';
import { useRound } from '../contexts/RoundContext';
import { parseMultiplayerLinks, eliminatePlayers, unEliminatePlayers, getAllPlayers, importPlayersFromParticipants } from '../api/players';
import type { PlayerProps } from '../types';
import { useAuth } from '../contexts/AuthContext';

export default function Admin() {
  const { round } = useRound();
  const { user } = useAuth();
  const [linksText, setLinksText] = useState('');
  const [participantsText, setParticipantsText] = useState('');
  const [status, setStatus] = useState<string | null>(null);
  const [country, setCountry] = useState<string>('');
  const [players, setPlayers] = useState<PlayerProps[]>([]);
  const [scoreDiffs, setScoreDiffs] = useState<Record<number, number>>({});

  const submitLinks = async () => {
    try {
      setStatus('Submitting links...');
      const links = linksText.split('\n').map(l => l.trim()).filter(Boolean);
      const res = await parseMultiplayerLinks(links, round);

      // Notify how many links were added
      setStatus(`Added ${links.length} multiplayer links`);

      // Refresh players and compute score diffs
      const before: Record<number, number> = {};
      players.forEach(p => { before[p.id] = p.score ?? 0; });

      const updated = await getAllPlayers();
      const diffs: Record<number, number> = {};
      updated.forEach((p: PlayerProps) => {
        const oldScore = before[p.id] ?? 0;
        const newScore = (p.score ?? 0);
        const delta = newScore - oldScore;
        if (delta !== 0) diffs[p.id] = delta;
      });

      setPlayers(updated);
      setScoreDiffs(diffs);

      // Also append parse response for debugging if available
      if (res && typeof res === 'object') {
        // leave status as the Added message, but also log res
        console.debug('parseMultiplayerLinks response:', res);
      }
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

  // load players initially
  useEffect(() => {
    let mounted = true;
    (async () => {
      try {
        const p = await getAllPlayers();
        if (!mounted) return;
        setPlayers(p);
      } catch (e) {
        console.error('Failed to load players for admin page', e);
      }
    })();
    return () => { mounted = false; };
  }, []);

  if (!user) return <div className="p-5 text-white">You must be logged in to use admin tools.</div>;

  const importPlayers = async () => {
    try {
      setStatus('Sending participants text to backend for import...');
      
      // Backend handles OAuth and DB insertion
      const result = await importPlayersFromParticipants(participantsText);
      
      setStatus(`Successfully imported ${result.count} players!${result.errors.length > 0 ? ` Errors: ${result.errors.slice(0, 3).join(', ')}` : ''}`);
      
      // Refresh players list
      const updated = await getAllPlayers();
      setPlayers(updated);
      setParticipantsText(''); // Clear the textarea
    } catch (err: any) {
      const errorMsg = err?.response?.data?.error || err?.message || String(err);
      setStatus('Error importing players: ' + errorMsg);
    }
  };

  return (
    <div className="p-6 text-white">
      <h1 className="text-2xl font-bold mb-4">Admin</h1>

      <div className="mb-4">
        <label className="mr-2">Round (auto):</label>
        <span className="ml-2 font-semibold">{round}</span>
      </div>

      {/* Import Players Section */}
      <div className="mb-6 p-4 bg-gray-800 rounded">
        <h2 className="text-xl font-semibold mb-2">Import Players from Tournament</h2>
        <p className="text-sm text-gray-400 mb-2">
          Paste the participants table from the osu! wiki (markdown format).
          The system will extract player IDs, fetch their data via OAuth2, and add them to the database.
        </p>
        <label className="block mb-1">Participants markdown:</label>
        <textarea 
          value={participantsText} 
          onChange={(e) => setParticipantsText(e.target.value)} 
          className="w-full bg-gray-900 p-2 rounded h-60 font-mono text-sm"
          placeholder="Paste markdown table here, e.g.:&#10;| ::{ flag=US }:: | **United States** | **[player1](https://osu.ppy.sh/users/123)**, [player2](https://osu.ppy.sh/users/456) |"
        />
        <button onClick={importPlayers} className="mt-2 bg-blue-600 px-4 py-2 rounded hover:bg-blue-700">
          Import Players
        </button>
      </div>

      <div className="mb-4">
        <label className="block mb-1">Multiplayer links (one per line):</label>
        <textarea value={linksText} onChange={(e) => setLinksText(e.target.value)} className="w-full bg-gray-900 p-2 rounded h-40" />
        <button onClick={submitLinks} className="mt-2 bg-purple-600 px-4 py-2 rounded">Parse links</button>
      </div>

      <div className="mb-4">
        <label className="block mb-1">Eliminate players in tournament:</label>
        <input placeholder="Optional country code (e.g. US)" value={country} onChange={e => setCountry(e.target.value.toUpperCase())} className="bg-gray-900 p-2 rounded mr-2 w-100" />
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

      <div className="mt-6">
        <h2 className="text-xl font-semibold mb-2">Players</h2>
        <div className="overflow-auto max-h-96 bg-gray-900 p-2 rounded">
          <table className="w-full table-auto">
            <thead>
              <tr className="text-left text-sm text-gray-400">
                <th className="px-2">#</th>
                <th className="px-2">Player</th>
                <th className="px-2">Country</th>
                <th className="px-2">Score</th>
                <th className="px-2">Δ</th>
              </tr>
            </thead>
            <tbody>
              {players.map((p, idx) => {
                const delta = scoreDiffs[p.id] ?? 0;
                return (
                  <tr key={p.id} className="border-t border-gray-800 text-white text-sm">
                    <td className="px-2 py-2">{idx + 1}</td>
                    <td className="px-2 py-2 flex items-center gap-2">
                      <img src={`https://a.ppy.sh/${p.id}`} className="h-8 w-8 rounded-full" alt="avatar" />
                      <span>{p.username}</span>
                    </td>
                    <td className="px-2 py-2">{p.country}</td>
                    <td className="px-2 py-2">{p.score ?? 0}</td>
                    <td className={`px-2 py-2 ${delta > 0 ? 'text-green-400' : delta < 0 ? 'text-red-400' : 'text-gray-400'}`}>
                      {delta > 0 ? `+${delta}` : delta < 0 ? `${delta}` : '-'}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
