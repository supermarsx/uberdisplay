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
    inputPermissions?: {
      enableInput: boolean;
      touch: boolean;
      pen: boolean;
      keyboard: boolean;
    };
  }>;
};

type CodecSelection = {
  codecId: number;
  codecName: string;
  hostMask: number;
  clientMask: number;
};

type DeviceForm = {
  name: string;
  transport: string;
  status: string;
  inputPermissions: {
    enableInput: boolean;
    touch: boolean;
    pen: boolean;
    keyboard: boolean;
  };
};

const fallbackStatus: AppStatus = {
  protocolVersion: 4,
  driver: { installed: false, active: false },
  transport: { tcpListening: true, tcpConnections: 0, aoapAttached: false },
  settings: { codec: "H.264 High", quality: 80, refreshCapHz: 120, keyframeInterval: 60, inputMode: "Touch + Pen" },
  session: { lifecycle: "idle" },
  devices: [],
};

const initialForm: DeviceForm = {
  name: "",
  transport: "USB",
  status: "Paired",
  inputPermissions: {
    enableInput: true,
    touch: true,
    pen: true,
    keyboard: true,
  },
};

const createId = () => {
  if (typeof crypto !== "undefined" && "randomUUID" in crypto) {
    return crypto.randomUUID();
  }
  return `device-${Date.now()}-${Math.floor(Math.random() * 10000)}`;
};

export default function HomePage() {
  const [status, setStatus] = useState<AppStatus>(fallbackStatus);
  const [devices, setDevices] = useState<AppStatus["devices"]>(fallbackStatus.devices);
  const [pairingOpen, setPairingOpen] = useState(false);
  const [form, setForm] = useState<DeviceForm>(initialForm);
  const [error, setError] = useState<string | null>(null);
  const [editingDeviceId, setEditingDeviceId] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [codecSelection, setCodecSelection] = useState<CodecSelection | null>(null);
  const [tcpForm, setTcpForm] = useState({
    host: "",
    port: 1445,
    width: 2560,
    height: 1600,
    hostWidth: 2560,
    hostHeight: 1600,
    encoderId: 1,
    codecs: {
      h265: true,
      av1: true,
      h264: true,
      vp9: true,
      evc: false,
      lcevc: false,
    },
  });

  useEffect(() => {
    let cancelled = false;

    const loadStatus = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/tauri");
        const data = await invoke<AppStatus>("app_status");
        if (!cancelled) {
          setStatus(data);
          setDevices(data.devices ?? []);
        }
      } catch (_error) {
        if (!cancelled) {
          setStatus(fallbackStatus);
          setDevices(fallbackStatus.devices);
        }
      }
    };

    loadStatus();
    return () => {
      cancelled = true;
    };
  }, []);

  const invokeTauri = async <T,>(command: string, args?: Record<string, unknown>) => {
    const { invoke } = await import("@tauri-apps/api/tauri");
    return invoke<T>(command, args);
  };

  const refreshDevices = async () => {
    try {
      const list = await invokeTauri<AppStatus["devices"]>("list_devices");
      setDevices(list ?? []);
      setError(null);
      setNotice("Device list refreshed.");
    } catch (err) {
      setError("Unable to refresh devices.");
      console.error(err);
    }
  };

  const handlePairSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    if (!form.name.trim()) {
      setError("Device name is required.");
      return;
    }

    try {
      const device = {
        id: createId(),
        name: form.name.trim(),
        transport: form.transport,
        status: form.status.trim() || "Paired",
        lastSeen: "Just now",
        inputPermissions: {
          enableInput: form.inputPermissions.enableInput,
          touch: form.inputPermissions.touch,
          pen: form.inputPermissions.pen,
          keyboard: form.inputPermissions.keyboard,
        },
      };
      const list = await invokeTauri<AppStatus["devices"]>("upsert_device", { device });
      setDevices(list ?? []);
      setForm(initialForm);
      setPairingOpen(false);
      setError(null);
      setNotice("Device paired.");
    } catch (err) {
      setError("Unable to save device.");
      console.error(err);
    }
  };

  const handleEditOpen = (device: AppStatus["devices"][number]) => {
    setForm({
      name: device.name,
      transport: device.transport,
      status: device.status,
      inputPermissions: {
        enableInput: device.inputPermissions?.enableInput ?? true,
        touch: device.inputPermissions?.touch ?? true,
        pen: device.inputPermissions?.pen ?? true,
        keyboard: device.inputPermissions?.keyboard ?? true,
      },
    });
    setEditingDeviceId(device.id);
    setPairingOpen(true);
    setError(null);
  };

  const handleEditSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    if (!editingDeviceId) {
      return;
    }
    if (!form.name.trim()) {
      setError("Device name is required.");
      return;
    }

    try {
      const device = {
        id: editingDeviceId,
        name: form.name.trim(),
        transport: form.transport,
        status: form.status.trim() || "Paired",
        lastSeen: "Just now",
        inputPermissions: {
          enableInput: form.inputPermissions.enableInput,
          touch: form.inputPermissions.touch,
          pen: form.inputPermissions.pen,
          keyboard: form.inputPermissions.keyboard,
        },
      };
      const list = await invokeTauri<AppStatus["devices"]>("upsert_device", { device });
      setDevices(list ?? []);
      setForm(initialForm);
      setEditingDeviceId(null);
      setPairingOpen(false);
      setError(null);
      setNotice("Device updated.");
    } catch (err) {
      setError("Unable to update device.");
      console.error(err);
    }
  };

  const handleRemove = async (deviceId: string) => {
    try {
      const list = await invokeTauri<AppStatus["devices"]>("remove_device", { deviceId });
      setDevices(list ?? []);
      setError(null);
      setNotice("Device removed.");
    } catch (err) {
      setError("Unable to remove device.");
      console.error(err);
    }
  };

  const handleConnect = async (deviceId: string) => {
    try {
      const list = await invokeTauri<AppStatus["devices"]>("connect_device", { deviceId });
      setDevices(list ?? []);
      setError(null);
      setNotice("Connection requested.");
    } catch (err) {
      setError("Unable to connect device.");
      console.error(err);
    }
  };

  const handleStartSession = async () => {
    try {
      await invokeTauri("start_session");
      setError(null);
      setNotice("Session start requested.");
    } catch (err) {
      setError("Unable to start session.");
      console.error(err);
    }
  };

  const handleAddVirtualDisplay = async () => {
    try {
      await invokeTauri("add_virtual_display");
      setError(null);
      setNotice("Virtual display requested.");
    } catch (err) {
      setError("Unable to add virtual display.");
      console.error(err);
    }
  };

  const handleInputToggle = async (
    label: string,
    enabled: boolean,
    nextPermissions: typeof inputControls
  ) => {
    try {
      await invokeTauri("set_session_input_permissions", {
        permissions: {
          enableInput: nextPermissions.enableInput,
          touch: nextPermissions.touch,
          pen: nextPermissions.pen,
          keyboard: nextPermissions.keyboard,
        },
      });
      await invokeTauri("record_action", {
        message: `Remote input ${label} ${enabled ? "enabled" : "disabled"}`,
      });
      setNotice(`Remote input ${label} ${enabled ? "enabled" : "disabled"}.`);
      setError(null);
    } catch (err) {
      setError("Unable to update remote input.");
      console.error(err);
    }
  };

  const codecMaskFromForm = () => {
    let mask = 0;
    if (tcpForm.codecs.h264) mask |= 1 << 0;
    if (tcpForm.codecs.h265) mask |= 1 << 1;
    if (tcpForm.codecs.av1) mask |= 1 << 2;
    if (tcpForm.codecs.vp9) mask |= 1 << 3;
    if (tcpForm.codecs.evc) mask |= 1 << 5;
    if (tcpForm.codecs.lcevc) mask |= 1 << 6;
    return mask;
  };

  const handleTcpConnect = async () => {
    try {
      const selection = await invokeTauri<CodecSelection>("tcp_connect_and_configure", {
        host: tcpForm.host.trim(),
        port: tcpForm.port,
        width: tcpForm.width,
        height: tcpForm.height,
        hostWidth: tcpForm.hostWidth,
        hostHeight: tcpForm.hostHeight,
        encoderId: tcpForm.encoderId,
        clientCodecMask: codecMaskFromForm(),
      });
      setCodecSelection(selection);
      setNotice(`TCP connected. Selected codec: ${selection.codecName}.`);
      setError(null);
    } catch (err) {
      setError("Unable to connect via TCP.");
      console.error(err);
    }
  };

  const handleTcpDisconnect = async () => {
    try {
      await invokeTauri("tcp_disconnect");
      setNotice("TCP disconnected.");
      setError(null);
    } catch (err) {
      setError("Unable to disconnect TCP.");
      console.error(err);
    }
  };
  const driverChip = status.driver.installed
    ? status.driver.active
      ? "Driver OK"
      : "Driver Idle"
    : "Driver Missing";
  const usbChip = status.transport.aoapAttached ? "USB Attached" : "USB Idle";
  const wifiChip = status.transport.tcpListening
    ? `Wi-Fi Ready (${status.transport.tcpConnections})`
    : "Wi-Fi Offline";
  const sessionLifecycle = status.session?.lifecycle ?? "idle";
  const sessionLabel = sessionLifecycle.charAt(0).toUpperCase() + sessionLifecycle.slice(1);
  const [inputControls, setInputControls] = useState({
    enableInput: true,
    captureOnConnect: true,
    touch: true,
    pen: true,
    keyboard: true,
  });

  const formatLastSeen = (value?: string | null) => {
    if (!value) {
      return "";
    }
    if (/^\d+$/.test(value)) {
      const timestamp = Number(value) * 1000;
      return new Date(timestamp).toLocaleTimeString();
    }
    return value;
  };

  return (
    <div className="app-shell">
      <header className="topbar">
        <div>
          <div className="wordmark">UberDisplay</div>
          <div className="tagline">Host Atelier</div>
        </div>
        <div className="topbar-actions">
          <Link className="ghost-button" href="/diagnostics">Diagnostics</Link>
          <Link className="ghost-button" href="/preferences">Preferences</Link>
        </div>
      </header>
      <nav className="tab-bar" aria-label="Primary">
        <Link className="tab-link active" href="/">Home</Link>
        <Link className="tab-link" href="/diagnostics">Diagnostics</Link>
        <Link className="tab-link" href="/preferences">Preferences</Link>
      </nav>

      <main className="content-grid">
        <section className="hero">
          <div className="hero-title">A second canvas for every device.</div>
          <p className="hero-body">
            Drive a paired Android display with buttery low-latency capture, stylus
            precision, and adaptive transport. USB-first, Wi-Fi ready.
          </p>
          <div className="hero-actions">
            <button className="primary-button" type="button" onClick={handleStartSession}>Start Session</button>
            <button
              className="secondary-button"
              type="button"
              onClick={() => {
                setPairingOpen((open) => !open);
                setEditingDeviceId(null);
                setForm(initialForm);
                setError(null);
              }}
            >
              Pair Device
            </button>
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
            <div>
              <div className="metric-label">Session</div>
              <div className="metric-value">{sessionLabel}</div>
            </div>
          </div>
        </section>

        <section className="card connect-card">
          <div className="card-header">
            <div className="card-title">Connect</div>
            <div className="card-subtitle">Select a display profile</div>
          </div>
          <div className="device-list">
            {devices.length === 0 ? (
              <div className="device-row muted">
                <div>
                  <div className="device-name">No paired devices yet</div>
                  <div className="device-meta">Use Pair Device to add one.</div>
                </div>
                <button className="pill-button" type="button" onClick={() => setPairingOpen(true)}>
                  Pair
                </button>
              </div>
            ) : (
              devices.map((device) => (
                <div className="device-row" key={device.id}>
                  <div>
                    <div className="device-name">{device.name}</div>
                    <div className="device-meta">
                      {device.transport} - {device.status}
                      {device.lastSeen ? ` - ${formatLastSeen(device.lastSeen)}` : ""}
                    </div>
                  </div>
                  <div className="device-actions">
                    <button className="pill-button" type="button" onClick={() => handleConnect(device.id)}>Connect</button>
                    <button
                      className="secondary-button"
                      type="button"
                      onClick={() => handleEditOpen(device)}
                    >
                      Edit
                    </button>
                    <button
                      className="ghost-button"
                      type="button"
                      onClick={() => handleRemove(device.id)}
                    >
                      Remove
                    </button>
                  </div>
                </div>
              ))
            )}
          </div>
          {pairingOpen && (
            <form
              className="pair-form"
              onSubmit={editingDeviceId ? handleEditSubmit : handlePairSubmit}
            >
              <div className="form-grid">
                <label className="form-field">
                  <span className="form-label">Device Name</span>
                  <input
                    className="form-input"
                    value={form.name}
                    onChange={(event) => setForm({ ...form, name: event.target.value })}
                    placeholder="Galaxy Tab S8"
                    required
                  />
                </label>
                <label className="form-field">
                  <span className="form-label">Transport</span>
                  <select
                    className="form-input"
                    value={form.transport}
                    onChange={(event) => setForm({ ...form, transport: event.target.value })}
                  >
                    <option value="USB">USB</option>
                    <option value="Wi-Fi">Wi-Fi</option>
                  </select>
                </label>
                <label className="form-field">
                  <span className="form-label">Status</span>
                  <input
                    className="form-input"
                    value={form.status}
                    onChange={(event) => setForm({ ...form, status: event.target.value })}
                    placeholder="Paired"
                  />
                </label>
                <div className="form-field">
                  <span className="form-label">Input Permissions</span>
                  <div className="form-toggle-row">
                    <label className="form-toggle">
                      <input
                        type="checkbox"
                        checked={form.inputPermissions.enableInput}
                        onChange={(event) =>
                          setForm({
                            ...form,
                            inputPermissions: {
                              ...form.inputPermissions,
                              enableInput: event.target.checked,
                            },
                          })
                        }
                      />
                      Enable Input
                    </label>
                    <label className="form-toggle">
                      <input
                        type="checkbox"
                        checked={form.inputPermissions.touch}
                        onChange={(event) =>
                          setForm({
                            ...form,
                            inputPermissions: {
                              ...form.inputPermissions,
                              touch: event.target.checked,
                            },
                          })
                        }
                      />
                      Touch
                    </label>
                    <label className="form-toggle">
                      <input
                        type="checkbox"
                        checked={form.inputPermissions.pen}
                        onChange={(event) =>
                          setForm({
                            ...form,
                            inputPermissions: {
                              ...form.inputPermissions,
                              pen: event.target.checked,
                            },
                          })
                        }
                      />
                      Pen
                    </label>
                    <label className="form-toggle">
                      <input
                        type="checkbox"
                        checked={form.inputPermissions.keyboard}
                        onChange={(event) =>
                          setForm({
                            ...form,
                            inputPermissions: {
                              ...form.inputPermissions,
                              keyboard: event.target.checked,
                            },
                          })
                        }
                      />
                      Keyboard
                    </label>
                  </div>
                </div>
              </div>
              {error && <div className="form-error">{error}</div>}
              <div className="form-actions">
                <button
                  className="secondary-button"
                  type="button"
                  onClick={() => {
                    setPairingOpen(false);
                    setEditingDeviceId(null);
                    setForm(initialForm);
                    setError(null);
                  }}
                >
                  Cancel
                </button>
                <button className="primary-button" type="submit">
                  {editingDeviceId ? "Update Device" : "Save Device"}
                </button>
              </div>
            </form>
          )}
          <div className="divider" />
          <div className="card-header">
            <div className="card-title">TCP Session</div>
            <div className="card-subtitle">Manual connect + configure</div>
          </div>
          <form className="pair-form settings-form">
            <div className="form-grid">
              <label className="form-field">
                <span className="form-label">Device IP</span>
                <input
                  className="form-input"
                  value={tcpForm.host}
                  onChange={(event) => setTcpForm({ ...tcpForm, host: event.target.value })}
                  placeholder="192.168.1.42"
                  required
                />
              </label>
              <label className="form-field">
                <span className="form-label">Port</span>
                <input
                  className="form-input"
                  type="number"
                  value={tcpForm.port}
                  onChange={(event) =>
                    setTcpForm({ ...tcpForm, port: Number(event.target.value) })
                  }
                />
              </label>
              <label className="form-field">
                <span className="form-label">Width</span>
                <input
                  className="form-input"
                  type="number"
                  value={tcpForm.width}
                  onChange={(event) =>
                    setTcpForm({ ...tcpForm, width: Number(event.target.value) })
                  }
                />
              </label>
              <label className="form-field">
                <span className="form-label">Height</span>
                <input
                  className="form-input"
                  type="number"
                  value={tcpForm.height}
                  onChange={(event) =>
                    setTcpForm({ ...tcpForm, height: Number(event.target.value) })
                  }
                />
              </label>
              <label className="form-field">
                <span className="form-label">Host Width</span>
                <input
                  className="form-input"
                  type="number"
                  value={tcpForm.hostWidth}
                  onChange={(event) =>
                    setTcpForm({ ...tcpForm, hostWidth: Number(event.target.value) })
                  }
                />
              </label>
              <label className="form-field">
                <span className="form-label">Host Height</span>
                <input
                  className="form-input"
                  type="number"
                  value={tcpForm.hostHeight}
                  onChange={(event) =>
                    setTcpForm({ ...tcpForm, hostHeight: Number(event.target.value) })
                  }
                />
              </label>
              <label className="form-field">
                <span className="form-label">Encoder Id</span>
                <input
                  className="form-input"
                  type="number"
                  value={tcpForm.encoderId}
                  onChange={(event) =>
                    setTcpForm({ ...tcpForm, encoderId: Number(event.target.value) })
                  }
                />
              </label>
              <div className="form-field">
                <span className="form-label">Client Codecs</span>
                <div className="form-toggle-row">
                  <label className="form-toggle">
                    <input
                      type="checkbox"
                      checked={tcpForm.codecs.h265}
                      onChange={(event) =>
                        setTcpForm({
                          ...tcpForm,
                          codecs: { ...tcpForm.codecs, h265: event.target.checked },
                        })
                      }
                    />
                    H.265
                  </label>
                  <label className="form-toggle">
                    <input
                      type="checkbox"
                      checked={tcpForm.codecs.av1}
                      onChange={(event) =>
                        setTcpForm({
                          ...tcpForm,
                          codecs: { ...tcpForm.codecs, av1: event.target.checked },
                        })
                      }
                    />
                    AV1
                  </label>
                  <label className="form-toggle">
                    <input
                      type="checkbox"
                      checked={tcpForm.codecs.h264}
                      onChange={(event) =>
                        setTcpForm({
                          ...tcpForm,
                          codecs: { ...tcpForm.codecs, h264: event.target.checked },
                        })
                      }
                    />
                    H.264
                  </label>
                  <label className="form-toggle">
                    <input
                      type="checkbox"
                      checked={tcpForm.codecs.vp9}
                      onChange={(event) =>
                        setTcpForm({
                          ...tcpForm,
                          codecs: { ...tcpForm.codecs, vp9: event.target.checked },
                        })
                      }
                    />
                    VP9
                  </label>
                  <label className="form-toggle">
                    <input
                      type="checkbox"
                      checked={tcpForm.codecs.evc}
                      onChange={(event) =>
                        setTcpForm({
                          ...tcpForm,
                          codecs: { ...tcpForm.codecs, evc: event.target.checked },
                        })
                      }
                    />
                    EVC
                  </label>
                  <label className="form-toggle">
                    <input
                      type="checkbox"
                      checked={tcpForm.codecs.lcevc}
                      onChange={(event) =>
                        setTcpForm({
                          ...tcpForm,
                          codecs: { ...tcpForm.codecs, lcevc: event.target.checked },
                        })
                      }
                    />
                    LCEVC
                  </label>
                </div>
              </div>
            </div>
            <div className="form-actions">
              <button className="secondary-button" type="button" onClick={handleTcpDisconnect}>
                Disconnect
              </button>
              <button className="primary-button" type="button" onClick={handleTcpConnect}>
                Connect + Configure
              </button>
            </div>
          </form>
          {codecSelection && (
            <div className="form-note">
              Negotiated codec: {codecSelection.codecName} (host {codecSelection.hostMask}, client {codecSelection.clientMask})
            </div>
          )}
          <div className="divider" />
          <div className="connect-actions">
            <button className="secondary-button" type="button" onClick={handleAddVirtualDisplay}>Add Virtual Display</button>
            <div className="connect-actions-links">
              <button className="ghost-button" type="button" onClick={refreshDevices}>Refresh</button>
              <Link className="ghost-button" href="/diagnostics">View Logs</Link>
            </div>
          </div>
          {notice && <div className="form-note">{notice}</div>}
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
              <div className="setting-label">Keyframe</div>
              <div className="setting-value">{status.settings.keyframeInterval} frames</div>
            </div>
            <div className="setting">
              <div className="setting-label">Input</div>
              <div className="setting-value">{status.settings.inputMode}</div>
            </div>
          </div>
          <div className="divider" />
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
      </main>

      <footer className="footer">
        <div>Host Agent 0.1 - Protocol v{status.protocolVersion}</div>
        <div>Latency mode: Balanced</div>
      </footer>
    </div>
  );
}
