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
  const [platform, setPlatform] = useState("unknown");
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
  const [serviceStatus, setServiceStatus] = useState<string | null>(null);
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
  const [profiles, setProfiles] = useState([
    { id: "studio", name: "Studio", description: "High fidelity for pen work", active: true, editing: false },
    { id: "mobile", name: "Mobile", description: "Balanced for quick sharing", active: false, editing: false },
  ]);
  const [defaultsConfig, setDefaultsConfig] = useState({
    palmRejection: true,
    pressureSmoothing: "Medium",
    autoReconnect: true,
    usbPriority: "Prefer USB",
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

    const loadPlatform = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const value = await invoke<string>("platform_name");
        if (!cancelled) {
          setPlatform(value);
        }
      } catch (_error) {
        if (!cancelled) {
          setPlatform("unknown");
        }
      }
    };

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

    loadPlatform();
    loadStatus();
    loadDisplays();
    loadVirtualDisplays();
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    let cancelled = false;
    if (platform !== "windows") {
      return () => {
        cancelled = true;
      };
    }

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

    loadDriverStatus();
    loadDriverPipe();
    return () => {
      cancelled = true;
    };
  }, [platform]);

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
      const detail = err instanceof Error ? err.message : String(err);
      if (detail.includes("pipe unavailable")) {
        pushToast("Service offline. Install/start the VDD service first.", "error");
      } else {
        pushToast("Unable to run driver action.", "error");
      }
      console.error(err);
    }
  };

  const handleServiceAction = async (action: "install" | "start" | "stop" | "query") => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      if (action === "install") {
        await invoke("install_vdd_service");
        pushToast("Service install requested.", "success");
      } else if (action === "start") {
        await invoke("start_vdd_service");
        pushToast("Service start requested.", "success");
      } else if (action === "stop") {
        await invoke("stop_vdd_service");
        pushToast("Service stop requested.", "success");
      } else {
        const status = await invoke<string>("query_vdd_service");
        setServiceStatus(status);
        pushToast("Service status refreshed.", "success");
      }
    } catch (err) {
      pushToast("Unable to run service action.", "error");
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
      await invoke("record_action", { message: "Manage profiles opened" });
      pushToast("Profiles manager requested.", "success");
    } catch (err) {
      pushToast("Unable to open profiles.", "error");
      console.error(err);
    }
  };

  const handleActivateProfile = async (profile: string) => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("record_action", { message: `Activated profile ${profile}` });
      setProfiles((prev) =>
        prev.map((item) => ({
          ...item,
          active: item.name === profile,
        }))
      );
      pushToast(`Profile ${profile} activated.`, "success");
    } catch (err) {
      pushToast("Unable to activate profile.", "error");
      console.error(err);
    }
  };

  const handleEditProfile = (id: string) => {
    setProfiles((prev) =>
      prev.map((profile) =>
        profile.id === id ? { ...profile, editing: !profile.editing } : profile
      )
    );
  };

  const handleProfileChange = (id: string, key: "name" | "description", value: string) => {
    setProfiles((prev) =>
      prev.map((profile) => (profile.id === id ? { ...profile, [key]: value } : profile))
    );
  };

  const handleSaveProfiles = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("record_action", { message: "Profiles updated (local only)" });
      setProfiles((prev) => prev.map((profile) => ({ ...profile, editing: false })));
      pushToast("Profiles updated.", "success");
    } catch (err) {
      pushToast("Unable to save profiles.", "error");
      console.error(err);
    }
  };

  const handleDefaultsSave = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/tauri");
      await invoke("record_action", { message: "Defaults updated (local only)" });
      pushToast("Defaults updated.", "success");
    } catch (err) {
      pushToast("Unable to update defaults.", "error");
      console.error(err);
    }
  };

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
        <section className="hero hero--prefs">
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
            <div className="form-grid prefs-grid">
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
                <span className="form-note">Pick the default codec. Session negotiation can override this.</span>
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
                <span className="form-note">Higher quality increases bitrate.</span>
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
                <span className="form-note">Auto clamps to device capability.</span>
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
                <span className="form-note">Shorter intervals recover faster after loss.</span>
              </label>
              <label className="form-field">
                <span className="form-label">Input Mode</span>
                <input
                  className="form-input"
                  value={form.inputMode}
                  onChange={(event) => setForm({ ...form, inputMode: event.target.value })}
                />
                <span className="form-note">Shown on the host and sent to clients.</span>
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
          <div className="form-note">Applies to the current session; device permissions still gate input.</div>
        </section>

        {platform === "windows" && (
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
            {driverPing === false && (
              <div className="form-note">Service not running. Install and start the VDD service.</div>
            )}
            <div className="form-actions align-left">
              <button className="secondary-button" type="button" onClick={() => handleDriverAction("install")}>Install</button>
              <button className="secondary-button" type="button" onClick={() => handleDriverAction("enable")}>Enable</button>
              <button className="secondary-button" type="button" onClick={() => handleDriverAction("disable")}>Disable</button>
              <button className="secondary-button" type="button" onClick={() => handleDriverAction("status")}>Refresh</button>
            </div>
            <div className="form-note">Windows only. Install requires admin once.</div>
            <div className="form-actions align-left">
              <button className="ghost-button" type="button" onClick={() => handleServiceAction("install")}>Install Service</button>
              <button className="ghost-button" type="button" onClick={() => handleServiceAction("start")}>Start Service</button>
              <button className="ghost-button" type="button" onClick={() => handleServiceAction("stop")}>Stop Service</button>
              <button className="ghost-button" type="button" onClick={() => handleServiceAction("query")}>Service Status</button>
            </div>
            {serviceStatus && <div className="form-note">Service: {serviceStatus}</div>}
            {driverSettingsRaw && <div className="form-note">Pipe: {driverSettingsRaw}</div>}
          </section>
        )}

        {platform === "windows" && (
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
            <div className="form-grid prefs-grid">
              <label className="form-field">
                <span className="form-label">GPU Name</span>
                <input
                  className="form-input"
                  value={driverGpuName}
                  onChange={(event) => setDriverGpuName(event.target.value)}
                  placeholder='NVIDIA GeForce'
                />
                <span className="form-note">Exact adapter name from Windows display settings.</span>
              </label>
              <div className="form-actions align-left">
                <button className="secondary-button" type="button" onClick={handleDriverSetGpu}>Apply GPU</button>
              </div>
            </div>
          </section>
        )}

        {(platform === "windows" || platform === "linux") && (
        <section className="card settings-card">
          <div className="card-header">
            <div className="card-title">Display Targets</div>
            <div className="card-subtitle">Assign sessions to a display output</div>
          </div>
          <div className="form-grid prefs-grid">
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
            <div className="form-actions align-left">
              <button className="secondary-button" type="button" onClick={handleDisplayTargetSave}>Apply</button>
            </div>
          </div>
          <div className="divider" />
          <div className="card-header">
            <div className="card-title">Virtual Displays</div>
            <div className="card-subtitle">Create or remove virtual outputs</div>
          </div>
          <div className="form-grid prefs-grid">
            <label className="form-field">
              <span className="form-label">Label</span>
              <input
                className="form-input"
                value={virtualDisplayLabel}
                onChange={(event) => setVirtualDisplayLabel(event.target.value)}
              />
              <span className="form-note">Used when requesting new outputs.</span>
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
              <span className="form-note">Windows uses SETDISPLAYCOUNT, Linux starts Xvfb.</span>
            </label>
            <div className="form-actions align-left">
              <button className="secondary-button" type="button" onClick={handleCreateVirtualDisplay}>Create</button>
              <button className="secondary-button" type="button" onClick={handleSetVirtualDisplayCount}>Apply Count</button>
            </div>
          </div>
          <div className="device-list spacious">
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
        )}

        {platform === "linux" && (
          <section className="card status-card">
            <div className="card-header">
              <div className="card-title">Linux Virtual Display</div>
              <div className="card-subtitle">Xvfb base display + resolution</div>
            </div>
            <div className="form-grid prefs-grid">
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
              <div className="form-actions align-left">
                <button className="secondary-button" type="button" onClick={handleLinuxConfigApply}>Apply Linux Config</button>
              </div>
            </div>
            <div className="form-note">Linux-only: restarts Xvfb instances if already running.</div>
          </section>
        )}

        <section className="card status-card">
          <div className="card-header">
            <div className="card-title">Profiles</div>
            <div className="card-subtitle">Stored configuration sets</div>
          </div>
          <div className="device-list">
            {profiles.map((profile) => (
              <div className="device-row" key={profile.id}>
                <div>
                  {profile.editing ? (
                    <>
                      <input
                        className="form-input"
                        value={profile.name}
                        onChange={(event) => handleProfileChange(profile.id, "name", event.target.value)}
                      />
                      <input
                        className="form-input"
                        value={profile.description}
                        onChange={(event) =>
                          handleProfileChange(profile.id, "description", event.target.value)
                        }
                      />
                    </>
                  ) : (
                    <>
                      <div className="device-name">{profile.name}</div>
                      <div className="device-meta">{profile.description}</div>
                    </>
                  )}
                </div>
                <div className="device-actions">
                  <button
                    className={profile.active ? "pill-button" : "secondary-button"}
                    type="button"
                    onClick={() => handleActivateProfile(profile.name)}
                  >
                    {profile.active ? "Active" : "Activate"}
                  </button>
                  <button className="ghost-button" type="button" onClick={() => handleEditProfile(profile.id)}>
                    {profile.editing ? "Cancel" : "Edit"}
                  </button>
                </div>
              </div>
            ))}
          </div>
          <div className="form-actions align-left">
            <button className="secondary-button" type="button" onClick={handleSaveProfiles}>Save Profiles</button>
          </div>
        </section>

        <section className="card connect-card">
          <div className="card-header">
            <div className="card-title">Defaults</div>
            <div className="card-subtitle">Adjust key behavior</div>
          </div>
          <div className="form-grid prefs-grid">
            <label className="form-field">
              <span className="form-label">Palm Rejection</span>
              <select
                className="form-input"
                value={defaultsConfig.palmRejection ? "on" : "off"}
                onChange={(event) =>
                  setDefaultsConfig({ ...defaultsConfig, palmRejection: event.target.value === "on" })
                }
              >
                <option value="on">On</option>
                <option value="off">Off</option>
              </select>
            </label>
            <label className="form-field">
              <span className="form-label">Pressure Smoothing</span>
              <select
                className="form-input"
                value={defaultsConfig.pressureSmoothing}
                onChange={(event) =>
                  setDefaultsConfig({ ...defaultsConfig, pressureSmoothing: event.target.value })
                }
              >
                <option value="Low">Low</option>
                <option value="Medium">Medium</option>
                <option value="High">High</option>
              </select>
            </label>
            <label className="form-field">
              <span className="form-label">Auto Reconnect</span>
              <select
                className="form-input"
                value={defaultsConfig.autoReconnect ? "on" : "off"}
                onChange={(event) =>
                  setDefaultsConfig({ ...defaultsConfig, autoReconnect: event.target.value === "on" })
                }
              >
                <option value="on">On</option>
                <option value="off">Off</option>
              </select>
            </label>
            <label className="form-field">
              <span className="form-label">USB Priority</span>
              <select
                className="form-input"
                value={defaultsConfig.usbPriority}
                onChange={(event) =>
                  setDefaultsConfig({ ...defaultsConfig, usbPriority: event.target.value })
                }
              >
                <option value="Prefer USB">Prefer USB</option>
                <option value="Prefer Wi-Fi">Prefer Wi-Fi</option>
                <option value="Balanced">Balanced</option>
              </select>
            </label>
          </div>
          <div className="form-actions align-left">
            <button className="secondary-button" type="button" onClick={handleDefaultsSave}>Save Defaults</button>
          </div>
          <div className="divider" />
          <div className="connect-actions">
            <button className="secondary-button" type="button" onClick={handleManagePresets}>Manage Profiles</button>
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
