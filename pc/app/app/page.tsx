export default function HomePage() {
  // TODO: Replace static status data with live Tauri signals once transport is wired.
  return (
    <div className="app-shell">
      <header className="topbar">
        <div>
          <div className="wordmark">UberDisplay</div>
          <div className="tagline">Host Atelier</div>
        </div>
        <div className="topbar-actions">
          <button className="ghost-button" type="button">Diagnostics</button>
          <button className="ghost-button" type="button">Preferences</button>
        </div>
      </header>

      <main className="content-grid">
        <section className="hero">
          <div className="hero-title">A second canvas for every device.</div>
          <p className="hero-body">
            Drive a paired Android display with buttery low-latency capture, stylus
            precision, and adaptive transport. USB-first, Wi-Fi ready.
          </p>
          <div className="hero-actions">
            <button className="primary-button" type="button">Start Session</button>
            <button className="secondary-button" type="button">Pair Device</button>
          </div>
        </section>

        <section className="card status-card">
          <div className="card-header">
            <div className="card-title">System Status</div>
            <div className="card-subtitle">Live from the host pipeline</div>
          </div>
          <div className="chip-grid">
            <div className="chip">
              <span className="chip-dot ok" />
              Driver OK
            </div>
            <div className="chip">
              <span className="chip-dot warn" />
              USB Idle
            </div>
            <div className="chip">
              <span className="chip-dot idle" />
              Encoder Standby
            </div>
            <div className="chip">
              <span className="chip-dot ok" />
              Wi-Fi Ready
            </div>
          </div>
          <div className="status-metrics">
            <div>
              <div className="metric-label">Active Display</div>
              <div className="metric-value">Virtual Canvas 01</div>
            </div>
            <div>
              <div className="metric-label">Transport</div>
              <div className="metric-value">USB-AOAP</div>
            </div>
            <div>
              <div className="metric-label">Stream</div>
              <div className="metric-value">2560 x 1600 @ 60 FPS</div>
            </div>
          </div>
        </section>

        <section className="card connect-card">
          <div className="card-header">
            <div className="card-title">Connect</div>
            <div className="card-subtitle">Select a display profile</div>
          </div>
          <div className="device-list">
            <div className="device-row">
              <div>
                <div className="device-name">Galaxy Tab S8</div>
                <div className="device-meta">USB • Paired</div>
              </div>
              <button className="pill-button" type="button">Resume</button>
            </div>
            <div className="device-row">
              <div>
                <div className="device-name">Pixel Fold</div>
                <div className="device-meta">Wi-Fi • Last seen 2m ago</div>
              </div>
              <button className="pill-button" type="button">Connect</button>
            </div>
            <div className="device-row muted">
              <div>
                <div className="device-name">Studio Display</div>
                <div className="device-meta">Virtual driver missing</div>
              </div>
              <button className="pill-button" type="button">Install Driver</button>
            </div>
          </div>
          <div className="divider" />
          <div className="connect-actions">
            <button className="secondary-button" type="button">Add Virtual Display</button>
            <button className="ghost-button" type="button">View Logs</button>
          </div>
        </section>

        <section className="card settings-card">
          <div className="card-header">
            <div className="card-title">Session Defaults</div>
            <div className="card-subtitle">Applied to new sessions</div>
          </div>
          <div className="settings-grid">
            <div className="setting">
              <div className="setting-label">Codec</div>
              <div className="setting-value">H.264 High</div>
            </div>
            <div className="setting">
              <div className="setting-label">Quality</div>
              <div className="setting-value">80%</div>
            </div>
            <div className="setting">
              <div className="setting-label">Refresh</div>
              <div className="setting-value">Auto (120 Hz)</div>
            </div>
            <div className="setting">
              <div className="setting-label">Input</div>
              <div className="setting-value">Touch + Pen</div>
            </div>
          </div>
        </section>
      </main>

      <footer className="footer">
        <div>Host Agent 0.1 • Protocol v4</div>
        <div>Latency mode: Balanced</div>
      </footer>
    </div>
  );
}
