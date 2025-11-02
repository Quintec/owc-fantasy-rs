import React, { createContext, useContext, useMemo, useState } from 'react';

export type RoundType = 'ro16' | 'qf' | 'sf' | 'f' | 'gf';

type RoundContextType = {
  round: RoundType;
  setRound: (r: RoundType) => void;
  isDraftOpen: boolean;
};

// The ordered rounds used by the tournament. The index corresponds to how many
// drafting periods have opened since the start date.
export const ROUNDS: RoundType[] = ['ro16', 'qf', 'sf', 'f', 'gf'];

// Start of the first drafting period (Monday 00:00 UTC prior to the weekend of Nov 15).
// For this tournament RO16 weekend begins Nov 15, 2025, so the drafting period opens on Monday 2025-11-10 00:00 UTC.
const ROUND_START_UTC = Date.UTC(2025, 10 /* Nov, zero-based */, 10, 0, 0, 0);

function computeRound(): RoundType {
  const now = new Date();
  const nowUtc = Date.UTC(
    now.getUTCFullYear(),
    now.getUTCMonth(),
    now.getUTCDate(),
    now.getUTCHours(),
    now.getUTCMinutes(),
    now.getUTCSeconds(),
  );

  const msPerWeek = 7 * 24 * 60 * 60 * 1000;
  const weeksElapsed = Math.floor((nowUtc - ROUND_START_UTC) / msPerWeek);
  const index = Math.max(0, Math.min(ROUNDS.length - 1, weeksElapsed));
  return ROUNDS[index];
}

const RoundContext = createContext<RoundContextType | undefined>(undefined);

function computeIsDraftOpen(): boolean {
  const now = new Date();
  const nowUtc = Date.UTC(
    now.getUTCFullYear(),
    now.getUTCMonth(),
    now.getUTCDate(),
    now.getUTCHours(),
    now.getUTCMinutes(),
    now.getUTCSeconds(),
  );

  const msPerWeek = 7 * 24 * 60 * 60 * 1000;
  // determine which week we're in relative to ROUND_START_UTC
  const weeksElapsed = Math.floor((nowUtc - ROUND_START_UTC) / msPerWeek);
  const weekStart = ROUND_START_UTC + weeksElapsed * msPerWeek;

  // drafting window: Monday 00:00 UTC (weekStart) -> Friday 00:00 UTC (weekStart + 4 days)
  const draftStart = weekStart;
  const draftEnd = weekStart + 4 * 24 * 60 * 60 * 1000;

  // drafting should not be open before the very first round starts
  if (nowUtc < ROUND_START_UTC) return false;

  return nowUtc >= draftStart && nowUtc < draftEnd;
}

export const RoundProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const autoRound = useMemo(() => computeRound(), []);
  const [round, setRound] = useState<RoundType>(autoRound);
  const isDraftOpen = useMemo(() => computeIsDraftOpen(), []);
  return <RoundContext.Provider value={{ round, setRound, isDraftOpen }}>{children}</RoundContext.Provider>;
};

export const useRound = () => {
  const ctx = useContext(RoundContext);
  if (!ctx) throw new Error('useRound must be used inside RoundProvider');
  return ctx;
};
