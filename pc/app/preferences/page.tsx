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

export default function PreferencesPage() {
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
          setError("Unable to load preferences.");
        }
      }
    };

    loadStatus();
    return () => {
      cancelled = true;
    };
  }, []);

  const preferenceCards = [
    {
      title: "Streaming",
      description: "Codec, quality, and refresh targeting.",
      entries: [
        status.settings.codec,
        `Quality ${status.settings.quality}%`,
        `Refresh cap ${status.settings.refreshCapHz} Hz`,
      ],
    },
    {
      title: "Input",
      description: "Pen + touch behavior defaults.",
      entries: [status.settings.inputMode, "Palm rejection: On", "Pressure smoothing: Medium"],
    },
    {
      title: "Transport",
      description: "USB/Wi-Fi handoff priorities.",
      entries: [
        status.transport.aoapAttached ? "USB active" : "Prefer USB",
        status.transport.tcpListening ? "Wi-Fi ready" : "Wi-Fi idle",
        "Auto-reconnect: Enabled",
      ],
    },
  ];
  return (
    <div className="app-shell">
      <header className="topbar">
        <div>
          <div className="wordmark">UberDisplay</div>
          <div className="tagline">Preferences</div>
        </div>
        <div className="topbar-actions">
          <Link className="ghost-button" href="/">Back to Home</Link>
          <Link className="ghost-button" href="/diagnostics">Diagnostics</Link>
        </div>
      </header>
      <nav className="tab-bar" aria-label="Primary">
        <Link className="tab-link" href="/">Home</Link>
        <Link className="tab-link" href="/diagnostics">Diagnostics</Link>
        <Link className="tab-link active" href="/preferences">Preferences</Link>
      </nav>

      <main className="content-grid">
        <section className="hero">
          <div className="hero-title">Tune the host profile.</div>
          <p className="hero-body">
            Adjust codec, input, and transport defaults. These settings apply to
            new pairing sessions and can be overridden per device.
          </p>
          <div className="hero-actions">
            <button className="primary-button" type="button">Save Changes</button>
            <button className="secondary-button" type="button">Reset Defaults</button>
          </div>
        </section>

        <section className="card status-card">
          <div className="card-header">
            <div className="card-title">Profiles</div>
            <div className="card-subtitle">Stored configuration sets</div>
          </div>
          <div className="device-list">
            <div className="device-row">
              <div>
                <div className="device-name">Studio</div>
                <div className="device-meta">High fidelity for pen work</div>
              </div>
              <button className="pill-button" type="button">Active</button>
            </div>
            <div className="device-row">
              <div>
                <div className="device-name">Mobile</div>
                <div className="device-meta">Balanced for quick sharing</div>
              </div>
              <button className="secondary-button" type="button">Activate</button>
            </div>
          </div>
        </section>

        <section className="card connect-card">
          <div className="card-header">
            <div className="card-title">Defaults</div>
            <div className="card-subtitle">Adjust key behavior</div>
          </div>
          <div className="settings-grid">
            {preferenceCards.map((card) => (
              <div className="setting" key={card.title}>
                <div className="setting-label">{card.title}</div>
                <div className="setting-value">{card.description}</div>
                <div className="device-meta">
                  {card.entries.join(" • ")}
                </div>
              </div>
            ))}
          </div>
          <div className="divider" />
          <div className="connect-actions">
            <button className="secondary-button" type="button">Manage Presets</button>
            <Link className="ghost-button" href="/">Return</Link>
          </div>
          {error && <div className="form-error">{error}</div>}
        </section>
      </main>

      <footer className="footer">
        <div>Preferences saved locally</div>
        <div>Sync: Off</div>
      </footer>
    </div>
  );
}
