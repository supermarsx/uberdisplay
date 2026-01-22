"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import Toast, { type ToastState } from "./components/Toast";

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

export default function HomePage() {
  const [status, setStatus] = useState<AppStatus>(fallbackStatus);
  const [devices, setDevices] = useState<AppStatus["devices"]>(fallbackStatus.devices);
  const [pairingOpen, setPairingOpen] = useState(false);
  const [form, setForm] = useState<DeviceForm>(initialForm);
  const [toast, setToast] = useState<ToastState>(null);
  const [editingDeviceId, setEditingDeviceId] = useState<string | null>(null);
  const [codecSelection, setCodecSelection] = useState<CodecSelection | null>(null);
  const [sessionStats, setSessionStats] = useState<SessionStats>(fallbackStats);
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
    let statsTimer: ReturnType<typeof setInterval> | null = null;

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
    loadSessionStats();
    statsTimer = setInterval(loadSessionStats, 2500);
    return () => {
      cancelled = true;
      if (statsTimer) {
        clearInterval(statsTimer);
      }
    };
  }, []);

  const invokeTauri = async <T,>(command: string, args?: Record<string, unknown>) => {
    const { invoke } = await import("@tauri-apps/api/tauri");
    return invoke<T>(command, args);
  };

  const pushToast = (message: string, type: "info" | "success" | "error" = "info") => {
    setToast({ message, type });
  };

  const refreshDevices = async () => {
    try {
      const list = await invokeTauri<AppStatus["devices"]>("list_devices");
      setDevices(list ?? []);
      pushToast("Device list refreshed.", "success");
    } catch (err) {
      pushToast("Unable to refresh devices.", "error");
      console.error(err);
    }
  };

  const handlePairSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    if (!form.name.trim()) {
      pushToast("Device name is required.", "error");
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
      pushToast("Device paired.", "success");
    } catch (err) {
      pushToast("Unable to save device.", "error");
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
    setToast(null);
  };

  const handleEditSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    if (!editingDeviceId) {
      return;
    }
    if (!form.name.trim()) {
      pushToast("Device name is required.", "error");
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
      pushToast("Device updated.", "success");
    } catch (err) {
      pushToast("Unable to update device.", "error");
      console.error(err);
    }
  };

  const handleRemove = async (deviceId: string) => {
    try {
      const list = await invokeTauri<AppStatus["devices"]>("remove_device", { deviceId });
      setDevices(list ?? []);
      pushToast("Device removed.", "success");
    } catch (err) {
      pushToast("Unable to remove device.", "error");
      console.error(err);
    }
  };

  const handleConnect = async (deviceId: string) => {
    try {
      const list = await invokeTauri<AppStatus["devices"]>("connect_device", { deviceId });
      setDevices(list ?? []);
      pushToast("Connection requested.", "success");
    } catch (err) {
      pushToast("Unable to connect device.", "error");
      console.error(err);
    }
  };

  const handleStartSession = async () => {
    try {
      await invokeTauri("start_session");
      pushToast("Session start requested.", "success");
    } catch (err) {
      pushToast("Unable to start session.", "error");
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
      pushToast(`TCP connected. Selected codec: ${selection.codecName}.`, "success");
    } catch (err) {
      pushToast("Unable to connect via TCP.", "error");
      console.error(err);
    }
  };

  const handleTcpDisconnect = async () => {
    try {
      await invokeTauri("tcp_disconnect");
      pushToast("TCP disconnected.", "success");
    } catch (err) {
      pushToast("Unable to disconnect TCP.", "error");
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
      <Toast toast={toast} onClear={() => setToast(null)} />
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
                setToast(null);
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
          <div className="status-metrics compact">
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
            <div>
              <div className="metric-label">Live FPS</div>
              <div className="metric-value">{sessionStats.fps.toFixed(1)}</div>
            </div>
            <div>
              <div className="metric-label">Bitrate</div>
              <div className="metric-value">{sessionStats.bitrateKbps} kbps</div>
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
              <div className="form-actions">
                <button
                  className="secondary-button"
                  type="button"
                  onClick={() => {
                    setPairingOpen(false);
                    setEditingDeviceId(null);
                    setForm(initialForm);
                    setToast(null);
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
          <details className="accordion">
            <summary>Open TCP Session</summary>
            <div className="accordion-body">
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
            </div>
          </details>
          {codecSelection && (
            <div className="form-note">
              Negotiated codec: {codecSelection.codecName} (host {codecSelection.hostMask}, client {codecSelection.clientMask})
            </div>
          )}
          <div className="divider" />
          <div className="connect-actions">
            <div className="connect-actions-links">
              <button className="ghost-button" type="button" onClick={refreshDevices}>Refresh</button>
              <Link className="ghost-button" href="/diagnostics">View Logs</Link>
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
