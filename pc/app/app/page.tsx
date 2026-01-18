"use client";

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
  transport: { tcpListening: true, tcpConnections: 0, aoapAttached: false },
  settings: { codec: "H.264 High", quality: 80, refreshCapHz: 120, inputMode: "Touch + Pen" },
  devices: [],
};

export default function HomePage() {
  const [status, setStatus] = useState<AppStatus>(fallbackStatus);

  useEffect(() => {
    let cancelled = false;

    const loadStatus = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const data = await invoke<AppStatus>("app_status");
        if (!cancelled) {
          setStatus(data);
        }
      } catch (_error) {
        if (!cancelled) {
          setStatus(fallbackStatus);
        }
      }
    };

    loadStatus();
    return () => {
      cancelled = true;
    };
  }, []);

  const driverChip = status.driver.installed
    ? status.driver.active
      ? "Driver OK"
      : "Driver Idle"
    : "Driver Missing";
  const usbChip = status.transport.aoapAttached ? "USB Attached" : "USB Idle";
  const wifiChip = status.transport.tcpListening
    ? `Wi-Fi Ready (${status.transport.tcpConnections})`
    : "Wi-Fi Offline";

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
              <span className={`chip-dot ${status.driver.installed ? "ok" : "warn"}`} />
              {driverChip}
            </div>
            <div className="chip">
              <span className={`chip-dot ${status.transport.aoapAttached ? "ok" : "warn"}`} />
              {usbChip}
            </div>
            <div className="chip">
              <span className="chip-dot idle" />
              Encoder Standby
            </div>
            <div className="chip">
              <span className={`chip-dot ${status.transport.tcpListening ? "ok" : "warn"}`} />
              {wifiChip}
            </div>
          </div>
          <div className="status-metrics">
            <div>
              <div className="metric-label">Active Display</div>
              <div className="metric-value">Virtual Canvas 01</div>
            </div>
            <div>
              <div className="metric-label">Driver</div>
              <div className="metric-value">
                {status.driver.installed ? (status.driver.active ? "Active" : "Installed") : "Missing"}
              </div>
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
            {status.devices.length === 0 ? (
              <div className="device-row muted">
                <div>
                  <div className="device-name">No paired devices yet</div>
                  <div className="device-meta">Use Pair Device to add one.</div>
                </div>
                <button className="pill-button" type="button">Pair</button>
              </div>
            ) : (
              status.devices.map((device) => (
                <div className="device-row" key={device.id}>
                  <div>
                    <div className="device-name">{device.name}</div>
                    <div className="device-meta">
                      {device.transport} - {device.status}
                      {device.lastSeen ? ` - ${device.lastSeen}` : ""}
                    </div>
                  </div>
                  <button className="pill-button" type="button">Connect</button>
                </div>
              ))
            )}
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
              <div className="setting-value">{status.settings.codec}</div>
            </div>
            <div className="setting">
              <div className="setting-label">Quality</div>
              <div className="setting-value">{status.settings.quality}%</div>
            </div>
            <div className="setting">
              <div className="setting-label">Refresh</div>
              <div className="setting-value">Auto ({status.settings.refreshCapHz} Hz)</div>
            </div>
            <div className="setting">
              <div className="setting-label">Input</div>
              <div className="setting-value">{status.settings.inputMode}</div>
            </div>
          </div>
        </section>
      </main>

      <footer className="footer">
        <div>Host Agent 0.1 - Protocol v{status.protocolVersion}</div>
        <div>Latency mode: Balanced</div>
      </footer>
    </div>
  );
}
