"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import Toast, { type ToastState } from "../components/Toast";

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

export default function PreferencesPage() {
  const [status, setStatus] = useState<AppStatus>(fallbackStatus);
  const [toast, setToast] = useState<ToastState>(null);
  const [form, setForm] = useState(fallbackStatus.settings);
  const [displays, setDisplays] = useState<DisplayInfo[]>([]);
  const [virtualDisplays, setVirtualDisplays] = useState<DisplayInfo[]>([]);
  const [displayTarget, setDisplayTarget] = useState("auto");
  const [virtualDisplayLabel, setVirtualDisplayLabel] = useState("UberDisplay");
  const [virtualDisplayCount, setVirtualDisplayCount] = useState(1);
  const [driverStatus, setDriverStatus] = useState<{
    installed: boolean;
    status: string;
    details?: { friendlyName?: string; instanceId?: string; pnpDeviceID?: string };
  } | null>(null);
  const [driverPing, setDriverPing] = useState<boolean | null>(null);
  const [driverSettingsRaw, setDriverSettingsRaw] = useState<string | null>(null);
  const [driverToggles, setDriverToggles] = useState({
    logging: false,
    debug: false,
    hdrPlus: false,
    sdr10: false,
    customEdid: false,
    preventSpoof: false,
    ceaOverride: false,
    hardwareCursor: false,
  });
  const [driverGpuName, setDriverGpuName] = useState("");
  const [linuxConfig, setLinuxConfig] = useState({
    baseDisplay: 99,
    width: 2560,
    height: 1600,
    depth: 24,
  });
  const [inputControls, setInputControls] = useState({
    enableInput: true,
    captureOnConnect: true,
    touch: true,
    pen: true,
    keyboard: true,
  });

  useEffect(() => {
    let cancelled = false;

    const loadStatus = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const data = await invoke<AppStatus>("app_status");
        if (!cancelled) {
          setStatus(data);
          setForm(data.settings);
        }
      } catch (_error) {
        if (!cancelled) {
          setStatus(fallbackStatus);
        }
      }
    };

    const loadDriverStatus = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const data = await invoke<{
          installed: boolean;
          status: string;
          details?: { friendlyName?: string; instanceId?: string; pnpDeviceID?: string };
        }>("virtual_driver_status");
        if (!cancelled) {
          setDriverStatus(data);
        }
      } catch (_error) {
        if (!cancelled) {
          setDriverStatus(null);
        }
      }
    };

    const loadDriverPipe = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const ping = await invoke<boolean>("driver_pipe_ping");
        const settings = await invoke<string | null>("driver_pipe_get_settings");
        if (!cancelled) {
          setDriverPing(ping);
          setDriverSettingsRaw(settings ?? null);
          if (settings) {
            setDriverToggles((prev) => ({
              ...prev,
              debug: /DEBUG=true/i.test(settings),
              logging: /LOG=true/i.test(settings),
            }));
          }
        }
      } catch (_error) {
        if (!cancelled) {
          setDriverPing(false);
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

    const loadVirtualDisplays = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const data = await invoke<DisplayInfo[]>("list_virtual_displays");
        const count = await invoke<number>("virtual_display_count");
        if (!cancelled) {
          setVirtualDisplays(data ?? []);
          setVirtualDisplayCount(Math.max(1, count || 1));
        }
      } catch (_error) {
        if (!cancelled) {
          setVirtualDisplays([]);
        }
      }
    };

    loadStatus();
    loadDriverStatus();
    loadDriverPipe();
    loadDisplays();
    loadVirtualDisplays();
    return () => {
      cancelled = true;
    };
  }, []);

  const pushToast = (message: string, type: "info" | "success" | "error" = "info") => {
    setToast({ message, type });
  };

  const handleInputToggle = async (
    label: string,
    enabled: boolean,
    nextPermissions: typeof inputControls
  ) => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("set_session_input_permissions", {
        permissions: {
          enableInput: nextPermissions.enableInput,
          touch: nextPermissions.touch,
          pen: nextPermissions.pen,
          keyboard: nextPermissions.keyboard,
        },
      });
      await invoke("record_action", {
        message: `Remote input ${label} ${enabled ? "enabled" : "disabled"}`,
      });
      pushToast(`Remote input ${label} ${enabled ? "enabled" : "disabled"}.`, "success");
    } catch (err) {
      pushToast("Unable to update remote input.", "error");
      console.error(err);
    }
  };

  const handleDisplayTargetSave = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      const payload = displayTarget === "auto" ? null : displayTarget;
      await invoke("set_session_display_target", { displayId: payload });
      pushToast("Display target updated.", "success");
    } catch (err) {
      pushToast("Unable to update display target.", "error");
      console.error(err);
    }
  };

  const handleCreateVirtualDisplay = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("create_virtual_display", { label: virtualDisplayLabel });
      pushToast("Virtual display creation requested.", "success");
    } catch (err) {
      pushToast("Unable to create virtual display.", "error");
      console.error(err);
    }
  };

  const handleRemoveVirtualDisplay = async (displayId: string) => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("remove_virtual_display", { displayId });
      pushToast("Virtual display removal requested.", "success");
    } catch (err) {
      pushToast("Unable to remove virtual display.", "error");
      console.error(err);
    }
  };

  const handleSetVirtualDisplayCount = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("set_virtual_display_count", { count: virtualDisplayCount });
      pushToast(`Virtual display count set to ${virtualDisplayCount}.`, "success");
    } catch (err) {
      pushToast("Unable to update virtual display count.", "error");
      console.error(err);
    }
  };

  const handleDriverAction = async (action: string) => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("virtual_driver_action", { action });
      pushToast(`Driver action: ${action}.`, "success");
    } catch (err) {
      pushToast("Unable to run driver action.", "error");
      console.error(err);
    }
  };

  const handleDriverToggle = async (key: keyof typeof driverToggles, enabled: boolean) => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      const optionMap: Record<string, string> = {
        logging: "logging",
        debug: "debug",
        hdrPlus: "hdr_plus",
        sdr10: "sdr10",
        customEdid: "custom_edid",
        preventSpoof: "prevent_spoof",
        ceaOverride: "cea_override",
        hardwareCursor: "hardware_cursor",
      };
      await invoke("driver_pipe_set_toggle", { option: optionMap[key], enabled });
      setDriverToggles((prev) => ({ ...prev, [key]: enabled }));
      pushToast(`Driver toggle updated.`, "success");
    } catch (err) {
      pushToast("Unable to update driver toggle.", "error");
      console.error(err);
    }
  };

  const handleDriverSetGpu = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("driver_pipe_set_gpu", { name: driverGpuName });
      pushToast("Driver GPU set.", "success");
    } catch (err) {
      pushToast("Unable to set driver GPU.", "error");
      console.error(err);
    }
  };

  const handleLinuxConfigApply = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("set_linux_vdd_config", {
        baseDisplay: linuxConfig.baseDisplay,
        width: linuxConfig.width,
        height: linuxConfig.height,
        depth: linuxConfig.depth,
      });
      pushToast("Linux VDD config applied.", "success");
    } catch (err) {
      pushToast("Unable to apply Linux VDD config.", "error");
      console.error(err);
    }
  };

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
      pushToast("Preferences saved.", "success");
    } catch (err) {
      pushToast("Unable to save preferences.", "error");
      console.error(err);
    }
  };

  const handleReset = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      const saved = await invoke<AppStatus["settings"]>("reset_settings");
      setStatus((prev) => ({ ...prev, settings: saved }));
      setForm(saved);
      pushToast("Preferences reset.", "success");
    } catch (err) {
      pushToast("Unable to reset preferences.", "error");
      console.error(err);
    }
  };

  const handleManagePresets = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("record_action", { message: "Manage presets opened" });
      pushToast("Presets manager requested.", "success");
    } catch (err) {
      pushToast("Unable to open presets.", "error");
      console.error(err);
    }
  };

  const handleActivateProfile = async (profile: string) => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("record_action", { message: `Activated profile ${profile}` });
      pushToast(`Profile ${profile} activated.`, "success");
    } catch (err) {
      pushToast("Unable to activate profile.", "error");
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
      <Toast toast={toast} onClear={() => setToast(null)} />
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
            <div className="card-title">Remote Input</div>
            <div className="card-subtitle">Control input capture from the device</div>
          </div>
          <div className="form-toggle-row">
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={inputControls.enableInput}
                onChange={(event) => {
                  const next = event.target.checked;
                  const nextState = { ...inputControls, enableInput: next };
                  setInputControls(nextState);
                  handleInputToggle("master", next, nextState);
                }}
              />
              Enable Input
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={inputControls.captureOnConnect}
                onChange={(event) => {
                  const next = event.target.checked;
                  const nextState = { ...inputControls, captureOnConnect: next };
                  setInputControls(nextState);
                  handleInputToggle("auto-capture", next, nextState);
                }}
              />
              Capture on Connect
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={inputControls.touch}
                onChange={(event) => {
                  const next = event.target.checked;
                  const nextState = { ...inputControls, touch: next };
                  setInputControls(nextState);
                  handleInputToggle("touch", next, nextState);
                }}
              />
              Touch
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={inputControls.pen}
                onChange={(event) => {
                  const next = event.target.checked;
                  const nextState = { ...inputControls, pen: next };
                  setInputControls(nextState);
                  handleInputToggle("pen", next, nextState);
                }}
              />
              Pen
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={inputControls.keyboard}
                onChange={(event) => {
                  const next = event.target.checked;
                  const nextState = { ...inputControls, keyboard: next };
                  setInputControls(nextState);
                  handleInputToggle("keyboard", next, nextState);
                }}
              />
              Keyboard
            </label>
          </div>
        </section>

        <section className="card status-card">
          <div className="card-header">
            <div className="card-title">Driver Manager</div>
            <div className="card-subtitle">Install, enable, and inspect driver status</div>
          </div>
          <div className="status-metrics compact">
            <div>
              <div className="metric-label">Installed</div>
              <div className="metric-value">{driverStatus?.installed ? "Yes" : "No"}</div>
            </div>
            <div>
              <div className="metric-label">Status</div>
              <div className="metric-value">{driverStatus?.status ?? "Unknown"}</div>
            </div>
            <div>
              <div className="metric-label">Device</div>
              <div className="metric-value">{driverStatus?.details?.friendlyName ?? "—"}</div>
            </div>
            <div>
              <div className="metric-label">Pipe</div>
              <div className="metric-value">{driverPing ? "Online" : "Offline"}</div>
            </div>
          </div>
          <div className="form-actions">
            <button className="secondary-button" type="button" onClick={() => handleDriverAction("install")}>Install</button>
            <button className="secondary-button" type="button" onClick={() => handleDriverAction("enable")}>Enable</button>
            <button className="secondary-button" type="button" onClick={() => handleDriverAction("disable")}>Disable</button>
            <button className="secondary-button" type="button" onClick={() => handleDriverAction("status")}>Refresh</button>
          </div>
          {driverSettingsRaw && <div className="form-note">Pipe: {driverSettingsRaw}</div>}
        </section>

        <section className="card settings-card">
          <div className="card-header">
            <div className="card-title">Driver Pipe Toggles</div>
            <div className="card-subtitle">HDR, logging, and driver flags</div>
          </div>
          <div className="form-toggle-row">
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={driverToggles.hdrPlus}
                onChange={(event) => handleDriverToggle("hdrPlus", event.target.checked)}
              />
              HDR+
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={driverToggles.sdr10}
                onChange={(event) => handleDriverToggle("sdr10", event.target.checked)}
              />
              SDR10
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={driverToggles.logging}
                onChange={(event) => handleDriverToggle("logging", event.target.checked)}
              />
              Logging
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={driverToggles.debug}
                onChange={(event) => handleDriverToggle("debug", event.target.checked)}
              />
              Debug Logs
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={driverToggles.customEdid}
                onChange={(event) => handleDriverToggle("customEdid", event.target.checked)}
              />
              Custom EDID
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={driverToggles.preventSpoof}
                onChange={(event) => handleDriverToggle("preventSpoof", event.target.checked)}
              />
              Prevent Spoof
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={driverToggles.ceaOverride}
                onChange={(event) => handleDriverToggle("ceaOverride", event.target.checked)}
              />
              CEA Override
            </label>
            <label className="form-toggle">
              <input
                type="checkbox"
                checked={driverToggles.hardwareCursor}
                onChange={(event) => handleDriverToggle("hardwareCursor", event.target.checked)}
              />
              Hardware Cursor
            </label>
          </div>
          <div className="form-grid">
            <label className="form-field">
              <span className="form-label">GPU Name</span>
              <input
                className="form-input"
                value={driverGpuName}
                onChange={(event) => setDriverGpuName(event.target.value)}
                placeholder='NVIDIA GeForce'
              />
            </label>
            <div className="form-actions">
              <button className="secondary-button" type="button" onClick={handleDriverSetGpu}>Apply GPU</button>
            </div>
          </div>
        </section>

        <section className="card settings-card">
          <div className="card-header">
            <div className="card-title">Display Targets</div>
            <div className="card-subtitle">Assign sessions to a display output</div>
          </div>
          <div className="form-grid">
            <label className="form-field">
              <span className="form-label">Target Display</span>
              <select
                className="form-input"
                value={displayTarget}
                onChange={(event) => setDisplayTarget(event.target.value)}
              >
                <option value="auto">Auto (Primary)</option>
                {displays.map((display) => (
                  <option key={display.id} value={display.id}>
                    {display.name || display.id} ({display.width}x{display.height}@{display.refreshHz}Hz)
                  </option>
                ))}
              </select>
            </label>
            <div className="form-actions">
              <button className="secondary-button" type="button" onClick={handleDisplayTargetSave}>Apply</button>
            </div>
          </div>
          <div className="divider" />
          <div className="card-header">
            <div className="card-title">Virtual Displays</div>
            <div className="card-subtitle">Create or remove virtual outputs</div>
          </div>
          <div className="form-grid">
            <label className="form-field">
              <span className="form-label">Label</span>
              <input
                className="form-input"
                value={virtualDisplayLabel}
                onChange={(event) => setVirtualDisplayLabel(event.target.value)}
              />
            </label>
            <label className="form-field">
              <span className="form-label">Display Count</span>
              <input
                className="form-input"
                type="number"
                min={1}
                max={12}
                value={virtualDisplayCount}
                onChange={(event) => setVirtualDisplayCount(Number(event.target.value))}
              />
            </label>
            <div className="form-actions">
              <button className="secondary-button" type="button" onClick={handleCreateVirtualDisplay}>Create</button>
              <button className="secondary-button" type="button" onClick={handleSetVirtualDisplayCount}>Apply Count</button>
            </div>
          </div>
          <div className="device-list">
            {virtualDisplays.length === 0 ? (
              <div className="device-row muted">
                <div>
                  <div className="device-name">No virtual displays detected</div>
                  <div className="device-meta">Install the driver and create a display.</div>
                </div>
              </div>
            ) : (
              virtualDisplays.map((display) => (
                <div className="device-row" key={display.id}>
                  <div>
                    <div className="device-name">{display.name || display.id}</div>
                    <div className="device-meta">
                      {display.width} x {display.height} @ {display.refreshHz} Hz
                      {display.active ? " • Active" : ""}
                    </div>
                  </div>
                  <div className="device-actions">
                    <button className="ghost-button" type="button" onClick={() => handleRemoveVirtualDisplay(display.id)}>
                      Remove
                    </button>
                  </div>
                </div>
              ))
            )}
          </div>
        </section>

        <section className="card status-card">
          <div className="card-header">
            <div className="card-title">Linux Virtual Display</div>
            <div className="card-subtitle">Xvfb base display + resolution</div>
          </div>
          <div className="form-grid">
            <label className="form-field">
              <span className="form-label">Base Display</span>
              <input
                className="form-input"
                type="number"
                min={1}
                value={linuxConfig.baseDisplay}
                onChange={(event) => setLinuxConfig({ ...linuxConfig, baseDisplay: Number(event.target.value) })}
              />
            </label>
            <label className="form-field">
              <span className="form-label">Width</span>
              <input
                className="form-input"
                type="number"
                min={320}
                value={linuxConfig.width}
                onChange={(event) => setLinuxConfig({ ...linuxConfig, width: Number(event.target.value) })}
              />
            </label>
            <label className="form-field">
              <span className="form-label">Height</span>
              <input
                className="form-input"
                type="number"
                min={240}
                value={linuxConfig.height}
                onChange={(event) => setLinuxConfig({ ...linuxConfig, height: Number(event.target.value) })}
              />
            </label>
            <label className="form-field">
              <span className="form-label">Depth</span>
              <input
                className="form-input"
                type="number"
                min={16}
                max={32}
                value={linuxConfig.depth}
                onChange={(event) => setLinuxConfig({ ...linuxConfig, depth: Number(event.target.value) })}
              />
            </label>
            <div className="form-actions">
              <button className="secondary-button" type="button" onClick={handleLinuxConfigApply}>Apply Linux Config</button>
            </div>
          </div>
          <div className="form-note">Linux-only: restarts Xvfb instances if already running.</div>
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
        </section>
      </main>

      <footer className="footer">
        <div>Preferences saved locally</div>
        <div>Sync: Off</div>
      </footer>
    </div>
  );
}
