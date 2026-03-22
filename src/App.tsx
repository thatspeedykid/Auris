import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

type DspState = 'checking' | 'active' | 'inactive' | 'driver_missing' | 'error'

function App() {
  const [dspStatus, setDspStatus] = useState<DspState>('checking')
  const [version, setVersion] = useState('0.1.0-alpha')

  useEffect(() => {
    invoke<string>('get_dsp_status')
      .then(s => setDspStatus(s as DspState))
      .catch(() => setDspStatus('error'))

    invoke<string>('get_version')
      .then(setVersion)
      .catch(() => {})

    const interval = setInterval(() => {
      invoke<string>('get_dsp_status')
        .then(s => setDspStatus(s as DspState))
        .catch(() => setDspStatus('error'))
    }, 5000)

    return () => clearInterval(interval)
  }, [])

  const badgeLabel: Record<DspState, string> = {
    checking:       'Checking...',
    active:         'DSP active',
    inactive:       'DSP inactive',
    driver_missing: 'Install FxSound driver',
    error:          'Error',
  }

  return (
    <div className="app">
      <header className="app-header">
        <div className="logo">
          <span className="logo-text">Auris</span>
          <span className="logo-by">by PrivacyChase</span>
        </div>
        <div className={`status-badge status-${dspStatus}`}>
          {badgeLabel[dspStatus]}
        </div>
      </header>

      <main className="app-main">
        <div className="alpha-notice">
          <span className="alpha-tag">v{version}</span>
          <p>
            {dspStatus === 'active'
              ? 'FxSound driver detected. Audio engine ready for Phase 1 wiring.'
              : 'This is an early development build. The audio engine is being wired up. No audio processing is active yet.'}
          </p>
        </div>

        <div className="roadmap">
          <h2>What's coming</h2>
          <ul>
            <li className="done">✓ Project scaffolding</li>
            <li className="done">✓ Tauri shell + React UI</li>
            <li className="done">✓ GitHub Actions CI/CD</li>
            <li className={dspStatus === 'active' ? 'done' : 'wip'}>
              {dspStatus === 'active' ? '✓' : '⟳'} FxSound DSP driver detected
            </li>
            <li className="wip">⟳ DSP FFI wiring (Phase 1)</li>
            <li>◦ EQ visualizer + presets</li>
            <li>◦ Per-app audio profiles</li>
            <li>◦ Headphone auto-detection (AutoEQ)</li>
            <li>◦ Mic noise suppression (DeepFilterNet 3)</li>
            <li>◦ AI scene detection</li>
          </ul>
        </div>
      </main>

      <footer className="app-footer">
        <span>Privacy audit log: <strong>0 network calls</strong></span>
        <a
          href="https://github.com/thatspeedykid/Auris"
          target="_blank"
          rel="noopener noreferrer"
        >
          Source code
        </a>
      </footer>
    </div>
  )
}

export default App
