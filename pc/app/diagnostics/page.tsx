"use client";

import Link from "next/link";
import { useEffect, useState } from "react";

type AppStatus = {
  protocolVersion: number;
  driver: { installed: boolean; active: boolean };
  transport: { tcpListening: boolean; tcpConnections: number; aoapAttached: boolean };
  settings: { codec: string; quality: number; refreshCapHz: number; keyframeInterval: number; inputMode: string };
  session: { lifecycle: string };
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
  settings: { codec: "H.264 High", quality: 80, refreshCapHz: 120, keyframeInterval: 60, inputMode: "Touch + Pen" },
  session: { lifecycle: "idle" },
  devices: [],
};

type HostLogEntry = {
  timestamp: number;
  message: string;
};

type DisplayInfo = {
  id: string;
  name: string;
  active: boolean;
  primary: boolean;
  width: number;
  height: number;
  refreshHz: number;
  isVirtual: boolean;
};

type SessionStats = {
  fps: number;
  bitrateKbps: number;
  framesSent: number;
  framesAcked: number;
  lastFrameBytes: number;
  queueDepth: number;
  dxgiTimeouts: number;
  dxgiAccessLost: number;
  dxgiFailures: number;
  dxgiLastBytes: number;
  capturePath: string;
  captureScale: string;
};

const fallbackLogs: HostLogEntry[] = [
  { timestamp: 0, message: "USB transport ready. AOAP handshake idle." },
  { timestamp: 0, message: "Wi-Fi listener active on port 1445." },
  { timestamp: 0, message: "Display driver installed, awaiting session." },
];

const fallbackStats: SessionStats = {
  fps: 0,
  bitrateKbps: 0,
  framesSent: 0,
  framesAcked: 0,
  lastFrameBytes: 0,
  queueDepth: 0,
  dxgiTimeouts: 0,
  dxgiAccessLost: 0,
  dxgiFailures: 0,
  dxgiLastBytes: 0,
  capturePath: "Unknown",
  captureScale: "Unknown",
};

export default function DiagnosticsPage() {
  const [status, setStatus] = useState<AppStatus>(fallbackStatus);
  const [error, setError] = useState<string | null>(null);
  const [logs, setLogs] = useState<HostLogEntry[]>(fallbackLogs);
  const [notice, setNotice] = useState<string | null>(null);
  const [sessionStats, setSessionStats] = useState<SessionStats>(fallbackStats);
  const [displays, setDisplays] = useState<DisplayInfo[]>([]);

  useEffect(() => {
    let cancelled = false;
    let statsTimer: ReturnType<typeof setInterval> | null = null;

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

    const loadLogs = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const data = await invoke<HostLogEntry[]>("list_logs");
        if (!cancelled) {
          setLogs(data.length ? data.slice().reverse() : fallbackLogs);
        }
      } catch (_error) {
        if (!cancelled) {
          setLogs(fallbackLogs);
        }
      }
    };

    const loadDisplays = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const data = await invoke<DisplayInfo[]>("list_displays");
        if (!cancelled) {
          setDisplays(data ?? []);
        }
      } catch (_error) {
        if (!cancelled) {
          setDisplays([]);
        }
      }
    };

    const loadSessionStats = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const data = await invoke<SessionStats>("session_stats_snapshot");
        if (!cancelled) {
          setSessionStats(data);
        }
      } catch (_error) {
        if (!cancelled) {
          setSessionStats(fallbackStats);
        }
      }
    };

    loadStatus();
    loadLogs();
    loadDisplays();
    loadSessionStats();
    statsTimer = setInterval(loadSessionStats, 2000);
    return () => {
      cancelled = true;
      if (statsTimer) {
        clearInterval(statsTimer);
      }
    };
  }, []);

  const handleExportLogs = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      const path = await invoke<string>("export_logs");
      setNotice(`Logs exported to ${path}`);
    } catch (err) {
      setError("Unable to export logs.");
      console.error(err);
    }
  };

  const handleExportDiagnostics = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      const path = await invoke<string>("export_diagnostics");
      setNotice(`Diagnostics exported to ${path}`);
    } catch (err) {
      setError("Unable to export diagnostics.");
      console.error(err);
    }
  };

  const handleLogDetails = async (message: string) => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("record_action", { message: `Viewed log details: ${message}` });
    } catch (err) {
      console.error(err);
    }
  };

  const handleClearLogView = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("clear_logs");
      setLogs([]);
      setNotice("Event log cleared.");
      setError(null);
    } catch (err) {
      setError("Unable to clear logs.");
      console.error(err);
    }
  };

  const formatTimestamp = (timestamp: number) => {
    if (!timestamp) {
      return "Just now";
    }
    return new Date(timestamp * 1000).toLocaleTimeString();
  };
  const sessionLifecycle = status.session?.lifecycle ?? "idle";
  const sessionLabel = sessionLifecycle.charAt(0).toUpperCase() + sessionLifecycle.slice(1);
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
            <div>
              <div className="metric-label">Session</div>
              <div className="metric-value">{sessionLabel}</div>
            </div>
            <div>
              <div className="metric-label">Encoder FPS</div>
              <div className="metric-value">{sessionStats.fps.toFixed(1)}</div>
            </div>
            <div>
              <div className="metric-label">Bitrate</div>
              <div className="metric-value">{sessionStats.bitrateKbps} kbps</div>
            </div>
            <div>
              <div className="metric-label">Frames (Sent/Acked)</div>
              <div className="metric-value">
                {sessionStats.framesSent} / {sessionStats.framesAcked}
              </div>
            </div>
            <div>
              <div className="metric-label">DXGI Timeouts</div>
              <div className="metric-value">{sessionStats.dxgiTimeouts}</div>
            </div>
            <div>
              <div className="metric-label">DXGI Access Lost</div>
              <div className="metric-value">{sessionStats.dxgiAccessLost}</div>
            </div>
            <div>
              <div className="metric-label">DXGI Failures</div>
              <div className="metric-value">{sessionStats.dxgiFailures}</div>
            </div>
            <div>
              <div className="metric-label">Capture Path</div>
              <div className="metric-value">{sessionStats.capturePath}</div>
            </div>
            <div>
              <div className="metric-label">Capture Scale</div>
              <div className="metric-value">{sessionStats.captureScale}</div>
            </div>
          </div>
          {error && <div className="form-error">{error}</div>}
        </section>

        <section className="card connect-card">
          <div className="card-header log-header">
            <div>
              <div className="card-title">Event Log</div>
              <div className="card-subtitle">Most recent host events</div>
            </div>
            <div className="log-count">{logs.length}</div>
          </div>
          <div className="device-list log-list">
            {logs.map((entry) => (
              <div className="device-row" key={`${entry.timestamp}-${entry.message}`}>
                <div>
                  <div className="device-name">{entry.message}</div>
                  <div className="device-meta">{formatTimestamp(entry.timestamp)}</div>
                </div>
                <div className="device-actions">
                  <button className="pill-button" type="button" onClick={() => handleLogDetails(entry.message)}>Details</button>
                </div>
              </div>
            ))}
          </div>
          <div className="divider" />
          <div className="connect-actions">
            <button className="secondary-button" type="button" onClick={handleClearLogView}>Clear View</button>
            <button className="secondary-button" type="button" onClick={handleExportLogs}>Export Logs</button>
            <button className="secondary-button" type="button" onClick={handleExportDiagnostics}>Export Diagnostics</button>
            <Link className="ghost-button" href="/">Return</Link>
          </div>
          {notice && <div className="form-note">{notice}</div>}
        </section>
        <section className="card settings-card">
        <div className="card-header">
          <div className="card-title">Displays</div>
          <div className="card-subtitle">Detected physical + virtual outputs</div>
        </div>
        <div className="device-list">
          {displays.length === 0 ? (
            <div className="device-row muted">
              <div>
                <div className="device-name">No displays detected</div>
                <div className="device-meta">Check driver installation or permissions.</div>
              </div>
            </div>
          ) : (
            displays.map((display) => (
              <div className="device-row" key={display.id}>
                <div>
                  <div className="device-name">{display.name || display.id}</div>
                  <div className="device-meta">
                    {display.width} x {display.height} @ {display.refreshHz} Hz
                    {display.primary ? " • Primary" : ""}
                    {display.active ? " • Active" : ""}
                    {display.isVirtual ? " • Virtual" : ""}
                  </div>
                </div>
              </div>
            ))
          )}
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
