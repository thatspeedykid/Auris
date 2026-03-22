import { useEffect, useState } from 'react'
import './App.css'

function App() {
  const [dspStatus, setDspStatus] = useState<'checking' | 'active' | 'inactive'>('checking')

  useEffect(() => {
    // TODO: wire to Tauri backend DSP status command
    // invoke('get_dsp_status').then(setDspStatus)
    setTimeout(() => setDspStatus('inactive'), 800)
  }, [])

  return (
    <div className="app">
      <header className="app-header">
        <div className="logo">
          <span className="logo-text">Auris</span>
          <span className="logo-by">by PrivacyChase</span>
        </div>
        <div className={`status-badge status-${dspStatus}`}>
          {dspStatus === 'checking' && 'Checking DSP…'}
          {dspStatus === 'active' && 'DSP active'}
          {dspStatus === 'inactive' && 'DSP not connected'}
        </div>
      </header>

      <main className="app-main">
        <div className="alpha-notice">
          <span className="alpha-tag">v0.1.0-alpha</span>
          <p>
            This is an early development build. The audio engine is being wired up.
            No audio processing is active yet.
          </p>
        </div>

        <div className="roadmap">
          <h2>What's coming</h2>
          <ul>
            <li className="done">✓ Project scaffolding</li>
            <li className="done">✓ Tauri shell + React UI</li>
            <li className="done">✓ GitHub Actions CI/CD</li>
            <li className="wip">⟳ FxSound DSP wired to Rust backend</li>
            <li className="wip">⟳ Audio passthrough confirmed</li>
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
          href="https://github.com/privacychase/auris"
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
