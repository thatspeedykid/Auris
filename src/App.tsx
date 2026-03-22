import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

type DspState = 'checking' | 'active' | 'inactive' | 'driver_missing' | 'error'

interface EqFilter {
  filter_type: string
  fc: number
  gain: number
  q: number
  enabled: boolean
}

interface HeadphoneProfile {
  name: string
  device_match: string[]
  preamp_db: number
  filters: EqFilter[]
}

interface Preset {
  name: string
  description: string
  bass_boost_db: number
  mid_db: number
  treble_db: number
}

interface EqState {
  enabled: boolean
  active_profile: string
  active_preset: string
  desser_enabled: boolean
  desser_strength: number
}

function App() {
  const [dspStatus, setDspStatus] = useState<DspState>('checking')
  const [version, setVersion] = useState('0.1.0-alpha')
  const [activeProfile, setActiveProfile] = useState<HeadphoneProfile | null>(null)
  const [profiles, setProfiles] = useState<HeadphoneProfile[]>([])
  const [presets, setPresets] = useState<Preset[]>([])
  const [eqState, setEqState] = useState<EqState | null>(null)
  const [activeTab, setActiveTab] = useState<'eq' | 'apps' | 'privacy'>('eq')
  const [eqEnabled, setEqEnabled] = useState(true)
  const [desserEnabled, setDesserEnabled] = useState(true)

  useEffect(() => {
    const poll = () => {
      invoke<string>('get_dsp_status').then(s => setDspStatus(s as DspState)).catch(() => setDspStatus('error'))
    }
    poll()
    const interval = setInterval(poll, 5000)

    invoke<string>('get_version').then(setVersion).catch(() => {})
    invoke<HeadphoneProfile>('get_active_profile').then(setActiveProfile).catch(() => {})
    invoke<HeadphoneProfile[]>('get_headphone_profiles').then(setProfiles).catch(() => {})
    invoke<Preset[]>('get_presets').then(setPresets).catch(() => {})
    invoke<EqState>('get_eq_state').then(s => {
      setEqState(s)
      setEqEnabled(s.enabled)
      setDesserEnabled(s.desser_enabled)
    }).catch(() => {})

    return () => clearInterval(interval)
  }, [])

  const badgeLabel: Record<DspState, string> = {
    checking: 'Checking...', active: 'DSP active',
    inactive: 'DSP inactive', driver_missing: 'Install FxSound', error: 'Error',
  }

  const filterColor = (gain: number) => {
    if (gain > 0) return '#34d399'
    if (gain < 0) return '#f87171'
    return '#6b7280'
  }

  const barHeight = (gain: number) => Math.abs(gain) * 6 // px per dB

  return (
    <div className="app">
      <header className="app-header">
        <div className="logo">
          <span className="logo-text">Auris</span>
          <span className="logo-by">by PrivacyChase</span>
        </div>
        <div className="header-right">
          <span className="version-tag">v{version}</span>
          <div className={`status-badge status-${dspStatus}`}>{badgeLabel[dspStatus]}</div>
        </div>
      </header>

      {/* Headphone profile bar */}
      <div className="profile-bar">
        <div className="profile-icon">🎧</div>
        <div className="profile-info">
          <span className="profile-name">{activeProfile?.name ?? 'Detecting headphones...'}</span>
          <span className="profile-sub">
            {activeProfile ? `${activeProfile.filters.length} EQ filters · Preamp ${activeProfile.preamp_db} dB` : 'Connect headphones for auto-profile'}
          </span>
        </div>
        <div className="profile-toggle">
          <button
            className={`toggle-btn ${eqEnabled ? 'on' : 'off'}`}
            onClick={() => {
              setEqEnabled(e => !e)
              invoke('set_eq_enabled', { enabled: !eqEnabled })
            }}
          >
            {eqEnabled ? 'ON' : 'OFF'}
          </button>
        </div>
      </div>

      {/* Tab bar */}
      <div className="tab-bar">
        {(['eq', 'apps', 'privacy'] as const).map(tab => (
          <button
            key={tab}
            className={`tab-btn ${activeTab === tab ? 'active' : ''}`}
            onClick={() => setActiveTab(tab)}
          >
            {tab === 'eq' ? 'EQ' : tab === 'apps' ? 'Per-App' : 'Privacy'}
          </button>
        ))}
      </div>

      <main className="app-main">
        {/* EQ Tab */}
        {activeTab === 'eq' && (
          <div className="eq-panel">
            {/* EQ bar visualizer */}
            {activeProfile && activeProfile.filters.length > 0 && (
              <div className="eq-viz">
                <div className="eq-bars">
                  {activeProfile.filters.map((f, i) => (
                    <div key={i} className="eq-bar-col">
                      <div className="eq-bar-wrap">
                        {f.gain > 0 && (
                          <div className="eq-bar boost" style={{ height: barHeight(f.gain), background: filterColor(f.gain) }} />
                        )}
                        <div className="eq-bar-zero" />
                        {f.gain < 0 && (
                          <div className="eq-bar cut" style={{ height: barHeight(f.gain), background: filterColor(f.gain) }} />
                        )}
                      </div>
                      <span className="eq-bar-label">{f.fc >= 1000 ? `${(f.fc/1000).toFixed(f.fc >= 10000 ? 0 : 1)}k` : `${f.fc}`}</span>
                      <span className="eq-bar-gain" style={{ color: filterColor(f.gain) }}>
                        {f.gain > 0 ? '+' : ''}{f.gain.toFixed(1)}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Presets */}
            <div className="section-label">Presets</div>
            <div className="preset-grid">
              {presets.filter(p => p.name !== 'flat').map(p => (
                <button
                  key={p.name}
                  className={`preset-btn ${eqState?.active_preset === p.name ? 'active' : ''}`}
                  title={p.description}
                >
                  {p.description}
                </button>
              ))}
            </div>

            {/* De-esser */}
            <div className="desser-row">
              <div className="desser-info">
                <span className="desser-title">De-esser</span>
                <span className="desser-sub">Cuts harsh "shhh" sibilance on voices & YouTube (6–9 kHz)</span>
              </div>
              <button
                className={`toggle-btn ${desserEnabled ? 'on' : 'off'}`}
                onClick={() => setDesserEnabled(e => !e)}
              >
                {desserEnabled ? 'ON' : 'OFF'}
              </button>
            </div>
          </div>
        )}

        {/* Per-app Tab */}
        {activeTab === 'apps' && (
          <div className="apps-panel">
            <div className="section-label">App profiles — auto-switch when you change apps</div>
            <div className="app-list">
              {[
                { icon: '🌐', name: 'YouTube / Browser', sub: 'Chrome, Edge, Firefox', preset: 'Headphone EQ + De-esser ON', color: '#34d399' },
                { icon: '🎵', name: 'Spotify', sub: 'spotify.exe', preset: 'Headphone EQ · De-esser OFF', color: '#a3e635' },
                { icon: '🎮', name: 'Games', sub: 'All other apps', preset: 'Gaming preset', color: '#60a5fa' },
                { icon: '🎙', name: 'Discord', sub: 'discord.exe', preset: 'Voice preset + Aggressive de-esser', color: '#c084fc' },
              ].map((app, i) => (
                <div key={i} className="app-row">
                  <span className="app-icon">{app.icon}</span>
                  <div className="app-row-info">
                    <span className="app-row-name">{app.name}</span>
                    <span className="app-row-sub">{app.sub}</span>
                  </div>
                  <span className="app-row-preset" style={{ color: app.color }}>{app.preset}</span>
                </div>
              ))}
            </div>
            <div className="coming-soon">⟳ Auto-switching wires in Phase 2 — profiles are ready</div>
          </div>
        )}

        {/* Privacy Tab */}
        {activeTab === 'privacy' && (
          <div className="privacy-panel">
            <div className="privacy-score">
              <span className="privacy-score-num">0</span>
              <span className="privacy-score-label">network calls since launch</span>
            </div>
            <div className="privacy-list">
              {[
                { check: true, label: 'No telemetry or crash reporting' },
                { check: true, label: 'No analytics or usage tracking' },
                { check: true, label: 'All EQ processing is 100% local' },
                { check: true, label: 'No accounts or sign-in required' },
                { check: true, label: 'Open source — AGPL v3.0' },
                { check: false, label: 'Auto-update (opt-in, coming v1.0)' },
              ].map((item, i) => (
                <div key={i} className="privacy-row">
                  <span className={item.check ? 'check-yes' : 'check-no'}>{item.check ? '✓' : '◦'}</span>
                  <span>{item.label}</span>
                </div>
              ))}
            </div>
          </div>
        )}
      </main>

      <footer className="app-footer">
        <a href="https://github.com/thatspeedykid/Auris" target="_blank" rel="noopener noreferrer">
          Source code
        </a>
        <span>Built by PrivacyChase — software that respects you.</span>
      </footer>
    </div>
  )
}

export default App
