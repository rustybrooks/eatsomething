export function genPlayNext() {
  return '/wwm';
}

export function genActivePuzzles() {
  return '/wwm/active';
}

export function genPuzzleBrowse(leagueSlug: string, answerId: string, username: string = null) {
  let url = `/wwm/puzzles/${leagueSlug}/${answerId}/browse`;
  if (username) {
    url += `/${username}`;
  }
  return url;
}

export function genPuzzlePlay(leagueSlug: string, answerId: string) {
  return `/wwm/puzzles/${leagueSlug}/${answerId}/play`;
}

export function genLeague(leagueSlug: string) {
  return `/wwm/leagues/${leagueSlug}`;
}

export function genLeagues() {
  return `/wwm/leagues`;
}

export function genJoinLeague(leagueSlug: string, inviteCode: string = null, answerId: string = null) {
  let url = `/wwm/leagues/${leagueSlug}/join`;
  if (inviteCode) {
    url += `/${inviteCode}`;
  }
  return url;
}

export function genJoinLeagueAndPlay(leagueSlug: string, answerId: string = null) {
  return `/wwm/leagues/${leagueSlug}/join_play/${answerId}`;
}

export function genLeagueNew() {
  return '/wwm/leagues/new';
}

export function genUser(username: string) {
  return `/wwm/users/${username}`;
}
