interface Torrent {
  id: number
  name: string
  progress: number
  downloaded: string
  total: string
  peers: number
  eta: string
  status: 'downloading' | 'paused'
  downSpeed?: string
  upSpeed?: string
}
