"use client";

import Link from "next/link";
import { useEffect, useState } from "react";

type AppStatus = {
  protocolVersion: number;
  driver: { installed: boolean; active: boolean };
  transport: { tcpListening: boolean; tcpConnections: number; aoapAttached: boolean };
  settings: { codec: string; quality: number; refreshCapHz: number; keyframeInterval: number; inputMode: string };
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
  devices: [],
};

export default function PreferencesPage() {
  const [status, setStatus] = useState<AppStatus>(fallbackStatus);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [form, setForm] = useState(fallbackStatus.settings);

  useEffect(() => {
    let cancelled = false;

    const loadStatus = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const data = await invoke<AppStatus>("app_status");
        if (!cancelled) {
          setStatus(data);
          setError(null);
          setForm(data.settings);
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

  const handleSave = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      const payload = {
        codec: form.codec,
        quality: Number(form.quality),
        refreshCapHz: Number(form.refreshCapHz),
        keyframeInterval: Number(form.keyframeInterval),
        inputMode: form.inputMode,
      };
      const saved = await invoke<AppStatus["settings"]>("update_settings", { settings: payload });
      setStatus((prev) => ({ ...prev, settings: saved }));
      setForm(saved);
      setError(null);
      setNotice("Preferences saved.");
    } catch (err) {
      setError("Unable to save preferences.");
      console.error(err);
    }
  };

  const handleReset = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      const saved = await invoke<AppStatus["settings"]>("reset_settings");
      setStatus((prev) => ({ ...prev, settings: saved }));
      setForm(saved);
      setError(null);
      setNotice("Preferences reset.");
    } catch (err) {
      setError("Unable to reset preferences.");
      console.error(err);
    }
  };

  const handleManagePresets = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("record_action", { message: "Manage presets opened" });
      setNotice("Presets manager requested.");
    } catch (err) {
      setError("Unable to open presets.");
      console.error(err);
    }
  };

  const handleActivateProfile = async (profile: string) => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("record_action", { message: `Activated profile ${profile}` });
      setNotice(`Profile ${profile} activated.`);
    } catch (err) {
      setError("Unable to activate profile.");
      console.error(err);
    }
  };

  const preferenceCards = [
    {
      title: "Streaming",
      description: "Codec, quality, and refresh targeting.",
      entries: [
        status.settings.codec,
        `Quality ${status.settings.quality}%`,
        `Refresh cap ${status.settings.refreshCapHz} Hz`,
        `Keyframe interval ${status.settings.keyframeInterval}f`,
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

  const codecOptions = [
    "H.264 High",
    "H.265 HEVC",
    "H.266",
    "AV1",
    "VP9",
    "EVC (xevd/xeve)",
    "MPEG-5 LCEVC",
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
            <button className="primary-button" type="button" onClick={handleSave}>Save Changes</button>
            <button className="secondary-button" type="button" onClick={handleReset}>Reset Defaults</button>
          </div>
          <form className="pair-form settings-form">
            <div className="form-grid">
              <label className="form-field">
                <span className="form-label">Codec</span>
                <select
                  className="form-input"
                  value={form.codec}
                  onChange={(event) => setForm({ ...form, codec: event.target.value })}
                >
                  {codecOptions.map((codec) => (
                    <option key={codec} value={codec}>
                      {codec}
                    </option>
                  ))}
                </select>
              </label>
              <label className="form-field">
                <span className="form-label">Quality</span>
                <input
                  className="form-input"
                  type="number"
                  min={10}
                  max={100}
                  value={form.quality}
                  onChange={(event) => setForm({ ...form, quality: Number(event.target.value) })}
                />
              </label>
              <label className="form-field">
                <span className="form-label">Refresh Cap (Hz)</span>
                <input
                  className="form-input"
                  type="number"
                  min={30}
                  max={240}
                  value={form.refreshCapHz}
                  onChange={(event) =>
                    setForm({ ...form, refreshCapHz: Number(event.target.value) })
                  }
                />
              </label>
              <label className="form-field">
                <span className="form-label">Keyframe Interval (frames)</span>
                <input
                  className="form-input"
                  type="number"
                  min={15}
                  max={300}
                  value={form.keyframeInterval}
                  onChange={(event) =>
                    setForm({ ...form, keyframeInterval: Number(event.target.value) })
                  }
                />
              </label>
              <label className="form-field">
                <span className="form-label">Input Mode</span>
                <input
                  className="form-input"
                  value={form.inputMode}
                  onChange={(event) => setForm({ ...form, inputMode: event.target.value })}
                />
              </label>
            </div>
          </form>
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
              <button className="pill-button" type="button" onClick={() => handleActivateProfile("Studio")}>Active</button>
            </div>
            <div className="device-row">
              <div>
                <div className="device-name">Mobile</div>
                <div className="device-meta">Balanced for quick sharing</div>
              </div>
              <button className="secondary-button" type="button" onClick={() => handleActivateProfile("Mobile")}>Activate</button>
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
            <button className="secondary-button" type="button" onClick={handleManagePresets}>Manage Presets</button>
            <Link className="ghost-button" href="/">Return</Link>
          </div>
          {error && <div className="form-error">{error}</div>}
          {notice && <div className="form-note">{notice}</div>}
        </section>
      </main>

      <footer className="footer">
        <div>Preferences saved locally</div>
        <div>Sync: Off</div>
      </footer>
    </div>
  );
}
