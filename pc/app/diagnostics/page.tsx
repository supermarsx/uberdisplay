 "use client";

import Link from "next/link";
import { useEffect, useState } from "react";

type AppStatus = {
  protocolVersion: number;
  driver: { installed: boolean; active: boolean };
  transport: { tcpListening: boolean; tcpConnections: number; aoapAttached: boolean };
  settings: { codec: string; quality: number; refreshCapHz: number; inputMode: string };
  devices: Array<{
    id: string;
    name: string;
    transport: string;
    status: string;
    lastSeen?: string | null;
  }>;
};

const fallbackStatus: AppStatus = {
  protocolVersion: 4,
  driver: { installed: false, active: false },
  transport: { tcpListening: false, tcpConnections: 0, aoapAttached: false },
  settings: { codec: "H.264 High", quality: 80, refreshCapHz: 120, inputMode: "Touch + Pen" },
  devices: [],
};

const logEntries = [
  { time: "00:12:41", message: "USB transport ready. AOAP handshake idle." },
  { time: "00:12:08", message: "Wi-Fi listener active on port 1445." },
  { time: "00:11:52", message: "Display driver installed, awaiting session." },
];

export default function DiagnosticsPage() {
  const [status, setStatus] = useState<AppStatus>(fallbackStatus);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    const loadStatus = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const data = await invoke<AppStatus>("app_status");
        if (!cancelled) {
          setStatus(data);
          setError(null);
        }
      } catch (_error) {
        if (!cancelled) {
          setStatus(fallbackStatus);
          setError("Unable to load diagnostics.");
        }
      }
    };

    loadStatus();
    return () => {
      cancelled = true;
    };
  }, []);
  return (
    <div className="app-shell">
      <header className="topbar">
        <div>
          <div className="wordmark">UberDisplay</div>
          <div className="tagline">Diagnostics</div>
        </div>
        <div className="topbar-actions">
          <Link className="ghost-button" href="/">Back to Home</Link>
          <Link className="ghost-button" href="/preferences">Preferences</Link>
        </div>
      </header>
      <nav className="tab-bar" aria-label="Primary">
        <Link className="tab-link" href="/">Home</Link>
        <Link className="tab-link active" href="/diagnostics">Diagnostics</Link>
        <Link className="tab-link" href="/preferences">Preferences</Link>
      </nav>

      <main className="content-grid">
        <section className="card status-card">
          <div className="card-header">
            <div className="card-title">Live Signals</div>
            <div className="card-subtitle">Connection + driver telemetry</div>
          </div>
          <div className="chip-grid">
            <div className="chip">
              <span className={`chip-dot ${status.driver.installed ? "ok" : "warn"}`} />
              {status.driver.installed ? "Driver Ready" : "Driver Missing"}
            </div>
            <div className="chip">
              <span className="chip-dot idle" />
              Encoder Standby
            </div>
            <div className="chip">
              <span className={`chip-dot ${status.transport.tcpListening ? "ok" : "warn"}`} />
              {status.transport.tcpListening ? "Wi-Fi Listener" : "Wi-Fi Offline"}
            </div>
          </div>
          <div className="status-metrics">
            <div>
              <div className="metric-label">Capture Mode</div>
              <div className="metric-value">Hybrid (USB + Wi-Fi)</div>
            </div>
            <div>
              <div className="metric-label">Host IP</div>
              <div className="metric-value">192.168.1.42</div>
            </div>
            <div>
              <div className="metric-label">Active Sessions</div>
              <div className="metric-value">{status.transport.tcpConnections}</div>
            </div>
          </div>
          {error && <div className="form-error">{error}</div>}
        </section>

        <section className="card connect-card">
          <div className="card-header">
            <div className="card-title">Event Log</div>
            <div className="card-subtitle">Most recent host events</div>
          </div>
          <div className="device-list">
            {logEntries.map((entry) => (
              <div className="device-row" key={`${entry.time}-${entry.message}`}>
                <div>
                  <div className="device-name">{entry.message}</div>
                  <div className="device-meta">{entry.time}</div>
                </div>
                <div className="device-actions">
                  <button className="pill-button" type="button">Details</button>
                </div>
              </div>
            ))}
          </div>
          <div className="divider" />
          <div className="connect-actions">
            <button className="secondary-button" type="button">Export Logs</button>
            <Link className="ghost-button" href="/">Return</Link>
          </div>
        </section>
      </main>

      <footer className="footer">
        <div>Last refresh: just now</div>
        <div>Diagnostics mode</div>
      </footer>
    </div>
  );
}
