pub fn get_dashboard_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Sager - Game Companion & Cloud Sync</title>
    <link rel="icon" type="image/png" href="/api/logo">
    <style>
        :root {
            --bg-base: #090d16;
            --bg-card: #131c2e;
            --bg-hover: #1c2942;
            --border: #22324e;
            --text-main: #f1f5f9;
            --text-muted: #94a3b8;
            --primary: #38bdf8;
            --primary-hover: #0ea5e9;
            --gold: #fbbf24;
            --gold-glow: rgba(251, 191, 36, 0.2);
            --success: #10b981;
            --danger: #ef4444;
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, Roboto, sans-serif;
        }

        body {
            background-color: var(--bg-base);
            color: var(--text-main);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }

        header {
            background: linear-gradient(180deg, #162035 0%, var(--bg-card) 100%);
            border-bottom: 1px solid var(--border);
            padding: 16px 32px;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .logo-area {
            display: flex;
            align-items: center;
            gap: 14px;
        }

        .logo-title {
            font-size: 22px;
            font-weight: 800;
            letter-spacing: 1.5px;
            background: linear-gradient(90deg, #38bdf8, #818cf8, #fbbf24);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }

        .logo-tagline {
            font-size: 12px;
            color: var(--text-muted);
        }

        .quick-actions {
            display: flex;
            gap: 10px;
        }

        .nav-tabs {
            display: flex;
            background-color: var(--bg-card);
            border-bottom: 1px solid var(--border);
            padding: 0 32px;
            gap: 4px;
        }

        .nav-tab {
            padding: 14px 20px;
            cursor: pointer;
            color: var(--text-muted);
            font-size: 14px;
            font-weight: 600;
            border-bottom: 2px solid transparent;
            transition: all 0.2s ease;
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .nav-tab:hover {
            color: var(--text-main);
            background-color: rgba(255, 255, 255, 0.02);
        }

        .nav-tab.active {
            color: var(--primary);
            border-bottom-color: var(--primary);
            background-color: rgba(56, 189, 248, 0.05);
        }

        main {
            padding: 24px 32px;
            flex: 1;
            max-width: 1400px;
            width: 100%;
            margin: 0 auto;
        }

        .tab-content {
            display: none;
        }

        .tab-content.active {
            display: block;
        }

        .status-strip {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
            gap: 16px;
            margin-bottom: 24px;
        }

        .status-card {
            background-color: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 16px;
            display: flex;
            flex-direction: column;
            gap: 6px;
        }

        .status-label {
            font-size: 12px;
            text-transform: uppercase;
            color: var(--text-muted);
            letter-spacing: 0.5px;
        }

        .status-value {
            font-size: 17px;
            font-weight: 700;
            color: var(--text-main);
            display: flex;
            align-items: center;
            gap: 8px;
            word-break: break-all;
        }

        .badge-live {
            background-color: rgba(16, 185, 129, 0.2);
            color: var(--success);
            padding: 2px 8px;
            border-radius: 9999px;
            font-size: 11px;
            font-weight: 600;
            display: inline-flex;
            align-items: center;
            gap: 4px;
        }

        .badge-live::before {
            content: '';
            width: 6px;
            height: 6px;
            background-color: var(--success);
            border-radius: 50%;
        }

        .btn {
            background-color: var(--primary);
            color: #031525;
            border: none;
            padding: 8px 16px;
            border-radius: 6px;
            font-size: 13px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s ease;
            display: inline-flex;
            align-items: center;
            gap: 6px;
            text-decoration: none;
        }

        .btn:hover {
            background-color: var(--primary-hover);
            transform: translateY(-1px);
        }

        .btn-secondary {
            background-color: #22324e;
            color: var(--text-main);
        }

        .btn-secondary:hover {
            background-color: #2c4166;
        }

        .btn-gold {
            background-color: var(--gold);
            color: #2d1800;
        }

        .btn-gold:hover {
            background-color: #f59e0b;
        }

        .btn-sm {
            padding: 5px 10px;
            font-size: 12px;
        }

        .card {
            background-color: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 20px;
            margin-bottom: 24px;
        }

        .card-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 16px;
        }

        .card-title {
            font-size: 16px;
            font-weight: 700;
            display: flex;
            align-items: center;
            gap: 8px;
        }

        table {
            width: 100%;
            border-collapse: collapse;
            text-align: left;
            font-size: 13px;
        }

        th {
            background-color: rgba(255, 255, 255, 0.02);
            color: var(--text-muted);
            padding: 12px 16px;
            border-bottom: 1px solid var(--border);
            font-weight: 600;
        }

        td {
            padding: 12px 16px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
            vertical-align: middle;
        }

        tr:hover td {
            background-color: var(--bg-hover);
        }

        .ach-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
            gap: 16px;
        }

        .ach-card {
            background-color: #162035;
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 16px;
            display: flex;
            gap: 14px;
            position: relative;
            overflow: hidden;
            transition: all 0.2s ease;
        }

        .ach-card:hover {
            border-color: var(--gold);
            box-shadow: 0 4px 20px var(--gold-glow);
        }

        .ach-card::before {
            content: '';
            position: absolute;
            left: 0;
            top: 0;
            bottom: 0;
            width: 4px;
            background: linear-gradient(180deg, var(--gold) 0%, #f59e0b 100%);
        }

        .ach-badge {
            width: 48px;
            height: 48px;
            background: linear-gradient(135deg, rgba(251, 191, 36, 0.2) 0%, rgba(245, 158, 11, 0.1) 100%);
            border: 1px solid rgba(251, 191, 36, 0.4);
            border-radius: 8px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 24px;
            flex-shrink: 0;
        }

        .ach-info {
            display: flex;
            flex-direction: column;
            gap: 4px;
            min-width: 0;
        }

        .ach-game {
            font-size: 11px;
            font-weight: 700;
            color: var(--primary);
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .ach-title {
            font-size: 15px;
            font-weight: 700;
            color: var(--text-main);
        }

        .ach-desc {
            font-size: 12px;
            color: var(--text-muted);
            line-height: 1.4;
        }

        .ach-date {
            font-size: 11px;
            color: #64748b;
            margin-top: 4px;
        }

        .form-group {
            margin-bottom: 16px;
        }

        .form-label {
            display: block;
            font-size: 13px;
            font-weight: 600;
            margin-bottom: 6px;
            color: var(--text-main);
        }

        .form-hint {
            font-size: 12px;
            color: var(--text-muted);
            margin-bottom: 8px;
            line-height: 1.4;
        }

        .form-input {
            width: 100%;
            padding: 10px 14px;
            background-color: #0b111e;
            border: 1px solid var(--border);
            border-radius: 6px;
            color: var(--text-main);
            font-size: 14px;
            outline: none;
            transition: border-color 0.2s;
        }

        .form-input:focus {
            border-color: var(--primary);
        }

        .preset-btn {
            background: #162035;
            border: 1px solid var(--border);
            color: var(--text-main);
            padding: 8px 14px;
            border-radius: 6px;
            font-size: 12px;
            cursor: pointer;
            font-weight: 600;
            transition: all 0.2s ease;
        }

        .preset-btn:hover {
            background: #1c2942;
            border-color: var(--primary);
        }

        .preset-btn.active {
            background: rgba(56, 189, 248, 0.15);
            border-color: var(--primary);
            color: var(--primary);
        }

        .diag-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 16px;
            margin-bottom: 24px;
        }

        .diag-card {
            background: #162035;
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 16px;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        .diag-title {
            font-size: 14px;
            font-weight: 700;
            color: var(--text-main);
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .diag-detail {
            font-size: 12px;
            color: var(--text-muted);
            line-height: 1.5;
        }

        .toast-msg {
            position: fixed;
            bottom: 24px;
            right: 24px;
            background-color: var(--bg-card);
            border: 1px solid var(--primary);
            color: var(--text-main);
            padding: 14px 20px;
            border-radius: 8px;
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
            display: none;
            z-index: 1000;
            animation: slideUp 0.3s ease;
        }

        @keyframes slideUp {
            from { transform: translateY(20px); opacity: 0; }
            to { transform: translateY(0); opacity: 1; }
        }
    </style>
</head>
<body>
    <header>
        <div class="logo-area">
            <img src="/api/logo" alt="Sager" style="width: 44px; height: 44px; object-fit: contain; border-radius: 10px; filter: drop-shadow(0 4px 12px rgba(14, 165, 233, 0.4));" />
            <div>
                <div class="logo-title">SAGER</div>
                <div class="logo-tagline">Steam Achievements & Game Save Cloud Sync</div>
            </div>
        </div>
        <div class="quick-actions">
            <button class="btn" onclick="backupAllGames()">💾 Backup All Saves</button>
            <button class="btn btn-secondary" onclick="openBackupFolder()">📂 Open Google Drive Folder</button>
            <button class="btn btn-gold" onclick="switchTab('tab-diagnostics')">🧪 Test Center</button>
        </div>
    </header>

    <div class="nav-tabs">
        <div class="nav-tab active" onclick="switchTab('tab-saves')">💾 Game Saves & Cloud Restore</div>
        <div class="nav-tab" onclick="switchTab('tab-achievements')">🏆 Steam Achievements Notifier</div>
        <div class="nav-tab" onclick="switchTab('tab-diagnostics')">🧪 Test Center & Diagnostics</div>
        <div class="nav-tab" onclick="switchTab('tab-settings')">⚙️ Options & Backup Schedule</div>
    </div>

    <main>
        <!-- Tab 1: Game Saves -->
        <div id="tab-saves" class="tab-content active">
            <div class="status-strip">
                <div class="status-card" id="gdrive-status-card">
                    <div class="status-label">Google Drive Status</div>
                    <div class="status-value" id="status-gdrive">
                        <span id="gdrive-badge" class="badge-live" style="display:none;">Checking...</span>
                    </div>
                    <div id="gdrive-action" style="margin-top: 6px;"></div>
                </div>
                <div class="status-card">
                    <div class="status-label">Backup Target</div>
                    <div class="status-value" id="status-backup-dir" style="font-size: 13px;">Detecting...</div>
                </div>
                <div class="status-card">
                    <div class="status-label">Auto-Backup Engine</div>
                    <div class="status-value"><span class="badge-live">Active</span> <span id="status-interval" style="font-size: 13px; color: var(--gold); font-weight: 700;">24x / day</span></div>
                </div>
                <div class="status-card">
                    <div class="status-label">Discovered Games</div>
                    <div class="status-value" id="status-games-count">0</div>
                </div>
                <div class="status-card">
                    <div class="status-label">Total Cloud Snapshots</div>
                    <div class="status-value" id="status-backups-count">0</div>
                </div>
            </div>

            <div class="card">
                <div class="card-header" style="flex-wrap: wrap; gap: 12px;">
                    <div class="card-title">🎮 Discovered PC Game Saves (Ludusavi Engine)</div>
                    <div style="display: flex; gap: 10px; align-items: center;">
                        <input type="text" id="game-search" class="form-input" placeholder="🔍 Search games..." oninput="filterGames()" style="padding: 6px 12px; font-size: 12px; width: 220px;" />
                        <button class="btn btn-secondary btn-sm" onclick="loadGames()">🔄 Rescan Saves</button>
                    </div>
                </div>
                <table>
                    <thead>
                        <tr>
                            <th>Game Title</th>
                            <th>Category</th>
                            <th>Save Path</th>
                            <th>Save Size</th>
                            <th>Files</th>
                            <th>Actions</th>
                        </tr>
                    </thead>
                    <tbody id="games-table-body">
                        <tr><td colspan="6" style="text-align: center; color: var(--text-muted);">Scanning save directories...</td></tr>
                    </tbody>
                </table>
            </div>

            <div class="card">
                <div class="card-header">
                    <div class="card-title">☁️ Available Cloud Backups for Restore (Google Drive)</div>
                    <button class="btn btn-secondary btn-sm" onclick="loadBackups()">🔄 Refresh List</button>
                </div>
                <table>
                    <thead>
                        <tr>
                            <th>Game Title</th>
                            <th>Backup Archive</th>
                            <th>Created Timestamp</th>
                            <th>Size</th>
                            <th>Original Path</th>
                            <th>Actions</th>
                        </tr>
                    </thead>
                    <tbody id="backups-table-body">
                        <tr><td colspan="6" style="text-align: center; color: var(--text-muted);">Loading backups...</td></tr>
                    </tbody>
                </table>
            </div>
        </div>

        <!-- Tab 2: Achievements -->
        <div id="tab-achievements" class="tab-content">
            <div class="status-strip">
                <div class="status-card">
                    <div class="status-label">Achievement Monitor</div>
                    <div class="status-value"><span class="badge-live">Watching</span> <span style="font-size: 13px; color: var(--text-muted);">Goldberg / Project Lightning / Steam</span></div>
                </div>
                <div class="status-card">
                    <div class="status-label">Total Unlocked Achievements</div>
                    <div class="status-value" id="status-ach-count" style="color: var(--gold);">0</div>
                </div>
                <div class="status-card">
                    <div class="status-label">Tray Toast & Sound Alert</div>
                    <div class="status-value"><span class="badge-live">Enabled</span></div>
                </div>
            </div>

            <div class="card">
                <div class="card-header">
                    <div class="card-title">🏆 Unlocked Achievements Feed (Steam & Project Lightning)</div>
                    <button class="btn btn-gold btn-sm" onclick="testAchievement()">🔔 Test Notification</button>
                </div>
                <div class="ach-grid" id="achievements-container">
                    <p style="color: var(--text-muted);">No achievements logged yet. Unlock achievements in games or click 'Test Notification' to verify!</p>
                </div>
            </div>
        </div>

        <!-- Tab 3: Test Center & Diagnostics -->
        <div id="tab-diagnostics" class="tab-content">
            <div class="card" style="background: linear-gradient(135deg, #101c33 0%, #172642 100%); border-color: var(--primary);">
                <div class="card-header">
                    <div>
                        <div class="card-title" style="font-size: 18px; color: var(--primary);">🧪 Sager Interactive Test Center & Diagnostic Suite</div>
                        <div style="font-size: 13px; color: var(--text-muted); margin-top: 4px;">Easily test every single subsystem of Sager, trigger notifications, benchmark Google Drive write speeds, and inspect your saves before pushing changes.</div>
                    </div>
                    <button class="btn btn-gold" onclick="runFullDiagnostics()">🚀 Run Full Diagnostics</button>
                </div>
            </div>

            <!-- Diagnostics Cards Grid -->
            <div class="diag-grid" id="diag-cards-grid">
                <div class="diag-card" id="diag-card-gdrive">
                    <div class="diag-title">
                        <span>☁️ Google Drive Sync</span>
                        <span class="badge-live" id="diag-badge-gdrive">Checking...</span>
                    </div>
                    <div class="diag-detail" id="diag-detail-gdrive">Process, permissions and sync directory status.</div>
                    <div style="display: flex; gap: 8px; margin-top: 6px;">
                        <button class="btn btn-sm btn-secondary" onclick="runWriteSpeedTest()">⚡ Test Write Speed</button>
                        <button class="btn btn-sm" onclick="launchGoogleDrive()">🚀 Start GDrive</button>
                    </div>
                </div>

                <div class="diag-card" id="diag-card-watcher">
                    <div class="diag-title">
                        <span>🎮 Achievement Watcher</span>
                        <span class="badge-live" id="diag-badge-watcher">Active</span>
                    </div>
                    <div class="diag-detail" id="diag-detail-watcher">Monitoring Project Lightning, Goldberg & Steam locations.</div>
                    <div style="display: flex; gap: 8px; margin-top: 6px;">
                        <button class="btn btn-sm btn-gold" onclick="testAchievement()">🔔 Trigger Test Alert</button>
                    </div>
                </div>

                <div class="diag-card" id="diag-card-scanner">
                    <div class="diag-title">
                        <span>💾 Save Game Scanner</span>
                        <span class="badge-live" id="diag-badge-scanner">Ready</span>
                    </div>
                    <div class="diag-detail" id="diag-detail-scanner">Scanning PC save directories (Ludusavi engine).</div>
                    <div style="display: flex; gap: 8px; margin-top: 6px;">
                        <button class="btn btn-sm btn-secondary" onclick="loadGames(); logToConsole('🔄 Save games rescan initiated.')">🔄 Rescan Saves</button>
                    </div>
                </div>

                <div class="diag-card" id="diag-card-backup">
                    <div class="diag-title">
                        <span>📦 Backup & Restore</span>
                        <span class="badge-live" id="diag-badge-backup">Ready</span>
                    </div>
                    <div class="diag-detail" id="diag-detail-backup">Zip compression with JSON manifests and safety snapshots.</div>
                    <div style="display: flex; gap: 8px; margin-top: 6px;">
                        <button class="btn btn-sm" onclick="backupAllGames()">💾 Test Full Backup</button>
                    </div>
                </div>

                <div class="diag-card" id="diag-card-sound">
                    <div class="diag-title">
                        <span>🔔 Audio & Notifications</span>
                        <span class="badge-live" id="diag-badge-sound">Enabled</span>
                    </div>
                    <div class="diag-detail" id="diag-detail-sound">Windows Balloon/Toast and asterisk audio chime.</div>
                    <div style="display: flex; gap: 8px; margin-top: 6px;">
                        <button class="btn btn-sm btn-secondary" onclick="testSound()">🔊 Test Sound</button>
                    </div>
                </div>

                <div class="diag-card" id="diag-card-paths">
                    <div class="diag-title">
                        <span>📁 Directories & Startup</span>
                        <span class="badge-live" id="diag-badge-paths">Linked</span>
                    </div>
                    <div class="diag-detail" id="diag-detail-paths">Config directory and Windows Startup integration.</div>
                    <div style="display: flex; gap: 8px; margin-top: 6px;">
                        <button class="btn btn-sm btn-secondary" onclick="openConfigFolder()">📂 Config Folder</button>
                        <button class="btn btn-sm btn-secondary" onclick="openBackupFolder()">📂 Backup Folder</button>
                    </div>
                </div>
            </div>

            <!-- Custom Simulator -->
            <div class="card">
                <div class="card-header">
                    <div class="card-title">🔔 Custom Achievement Pop Simulator</div>
                </div>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 14px; margin-bottom: 14px;">
                    <div>
                        <label class="form-label">Game Name</label>
                        <input type="text" id="sim-game" class="form-input" value="Project Lightning" />
                    </div>
                    <div>
                        <label class="form-label">Achievement Title</label>
                        <input type="text" id="sim-title" class="form-input" value="Master of Lightning" />
                    </div>
                    <div>
                        <label class="form-label">Description</label>
                        <input type="text" id="sim-desc" class="form-input" value="Successfully synchronized cloud saves & unlocked achievement!" />
                    </div>
                </div>
                <button class="btn btn-gold" onclick="simulateCustomAchievement()">🚀 Trigger Toast Notification Now</button>
            </div>

            <!-- Live Terminal Log -->
            <div class="card">
                <div class="card-header">
                    <div class="card-title">📜 Live Activity & Test Console</div>
                    <div style="display: flex; gap: 8px;">
                        <button class="btn btn-secondary btn-sm" onclick="copyConsole()">📋 Copy Log</button>
                        <button class="btn btn-secondary btn-sm" onclick="clearConsole()">🗑️ Clear</button>
                    </div>
                </div>
                <div id="console-output" style="background: #050811; border: 1px solid var(--border); border-radius: 8px; padding: 14px; font-family: 'Consolas', 'Courier New', monospace; font-size: 12px; height: 260px; overflow-y: auto; line-height: 1.6; color: #38bdf8;">
                </div>
            </div>
        </div>

        <!-- Tab 4: Settings -->
        <div id="tab-settings" class="tab-content">
            <div class="card">
                <div class="card-header">
                    <div class="card-title">⚙️ Options & Backup Schedule</div>
                </div>
                <form id="config-form" onsubmit="saveSettings(event)">
                    <!-- Backup Frequency / Times per Day -->
                    <div class="form-group" style="background: rgba(255, 255, 255, 0.02); border: 1px solid var(--border); border-radius: 8px; padding: 18px; margin-bottom: 20px;">
                        <label class="form-label" style="font-size: 14px; color: var(--primary);">⏰ Backup Frequency: How many times a day?</label>
                        <div class="form-hint">Choose how frequently Sager automatically saves your game progress to Google Drive. This ensures your latest saves are ready to restore whenever you switch between your laptop and desktop PC!</div>
                        
                        <div class="frequency-presets" style="display: flex; flex-wrap: wrap; gap: 8px; margin: 14px 0;">
                            <button type="button" class="preset-btn" onclick="setPreset(1)">1x / Day (Every 24h)</button>
                            <button type="button" class="preset-btn" onclick="setPreset(2)">2x / Day (Every 12h)</button>
                            <button type="button" class="preset-btn" onclick="setPreset(4)">4x / Day (Every 6h)</button>
                            <button type="button" class="preset-btn" onclick="setPreset(12)">12x / Day (Every 2h)</button>
                            <button type="button" class="preset-btn active" id="btn-preset-24" onclick="setPreset(24)">24x / Day (Hourly) ⭐</button>
                            <button type="button" class="preset-btn" onclick="setPreset(48)">48x / Day (Every 30m)</button>
                        </div>

                        <div style="display: flex; align-items: center; gap: 14px; margin-top: 12px; flex-wrap: wrap;">
                            <span style="font-size: 13px; color: var(--text-muted);">Custom times per day:</span>
                            <input type="number" id="cfg-bpd" class="form-input" min="1" max="144" value="24" oninput="updateFreqNotice()" style="width: 90px; text-align: center; font-weight: bold;" />
                            <span id="freq-notice" style="font-size: 13px; color: var(--gold); font-weight: 700;">⚡ Backing up 24 times a day (every 60 minutes)</span>
                        </div>
                        <input type="hidden" id="cfg-interval" value="60" />
                    </div>

                    <div class="form-group">
                        <label class="form-label">Google Drive / Cloud Backup Directory</label>
                        <div class="form-hint">Directory where your save zip archives and manifests are stored. Point this to your Google Drive folder on both your laptop and desktop PC for instant cross-PC syncing!</div>
                        <input type="text" id="cfg-backup-dir" class="form-input" required />
                    </div>

                    <div class="form-group">
                        <label class="form-label">Google Drive Executable Path (Optional)</label>
                        <div class="form-hint">Custom path to GoogleDriveFS.exe. Leave blank for automatic detection.</div>
                        <input type="text" id="cfg-gdrive-exe" class="form-input" placeholder="Auto-detect" />
                    </div>

                    <div style="display: flex; flex-wrap: wrap; gap: 24px; align-items: center; margin-bottom: 24px; padding: 14px; background: rgba(255,255,255,0.01); border-radius: 6px; border: 1px solid var(--border);">
                        <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                            <input type="checkbox" id="cfg-sound" /> Enable Achievement Audio Chime
                        </label>
                        <button type="button" class="btn btn-secondary btn-sm" onclick="testSound()">🔊 Test Sound</button>
                        <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                            <input type="checkbox" id="cfg-notif" /> Enable Windows Desktop Notifications
                        </label>
                        <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                            <input type="checkbox" id="cfg-auto-backup" /> Enable Automatic Background Backups
                        </label>
                    </div>

                    <button type="submit" class="btn">💾 Save Options</button>
                </form>
            </div>
        </div>
    </main>

    <div id="toast" class="toast-msg"></div>

    <script>
        let allDiscoveredGames = [];

        function logToConsole(msg, level = 'INFO') {
            const el = document.getElementById('console-output');
            if (!el) return;
            const time = new Date().toLocaleTimeString();
            let color = '#38bdf8';
            if (level === 'PASS' || level === 'SUCCESS') color = '#10b981';
            if (level === 'WARN') color = '#fbbf24';
            if (level === 'ERROR') color = '#ef4444';
            
            const line = document.createElement('div');
            line.innerHTML = `<span style="color: #64748b;">[${time}]</span> <span style="color: ${color}; font-weight: 700;">[${level}]</span> ${msg}`;
            el.appendChild(line);
            el.scrollTop = el.scrollHeight;
        }

        function clearConsole() {
            const el = document.getElementById('console-output');
            if (el) el.innerHTML = '';
            logToConsole('Console cleared.', 'INFO');
        }

        function copyConsole() {
            const el = document.getElementById('console-output');
            if (!el) return;
            navigator.clipboard.writeText(el.innerText).then(() => {
                showToast('📋 Console log copied to clipboard!');
            });
        }

        function showToast(msg) {
            const t = document.getElementById('toast');
            t.textContent = msg;
            t.style.display = 'block';
            setTimeout(() => { t.style.display = 'none'; }, 4000);
        }

        function switchTab(tabId) {
            document.querySelectorAll('.tab-content').forEach(el => el.classList.remove('active'));
            document.querySelectorAll('.nav-tab').forEach(el => el.classList.remove('active'));
            const target = document.getElementById(tabId);
            if (target) target.classList.add('active');
            
            // Highlight nav tab
            document.querySelectorAll('.nav-tab').forEach(tab => {
                if (tab.getAttribute('onclick') && tab.getAttribute('onclick').includes(tabId)) {
                    tab.classList.add('active');
                }
            });

            if (tabId === 'tab-diagnostics') {
                runFullDiagnostics();
            }
        }

        function formatBytes(bytes) {
            if (bytes === 0) return '0 B';
            const k = 1024;
            const sizes = ['B', 'KB', 'MB', 'GB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
        }

        function setPreset(times) {
            document.getElementById('cfg-bpd').value = times;
            document.querySelectorAll('.preset-btn').forEach(btn => btn.classList.remove('active'));
            if (event && event.currentTarget) {
                event.currentTarget.classList.add('active');
            }
            updateFreqNotice();
        }

        function updateFreqNotice() {
            let times = parseInt(document.getElementById('cfg-bpd').value) || 24;
            if (times < 1) times = 1;
            if (times > 144) times = 144;
            const minutes = Math.max(1, Math.round(1440 / times));
            document.getElementById('cfg-interval').value = minutes;
            const notice = document.getElementById('freq-notice');
            if (minutes >= 60) {
                const hours = (minutes / 60).toFixed(1).replace('.0', '');
                notice.textContent = `⚡ Backing up ${times} times a day (every ${hours} hour${hours !== '1' ? 's' : ''})`;
            } else {
                notice.textContent = `⚡ Backing up ${times} times a day (every ${minutes} minutes)`;
            }
        }

        async function testSound() {
            try {
                await fetch('/api/test-sound', { method: 'POST' });
                showToast('🔊 Audio chime played!');
                logToConsole('Played Windows audio alert chime (MB_ICONASTERISK)', 'PASS');
            } catch (e) {
                logToConsole('Error playing audio: ' + e, 'ERROR');
            }
        }

        async function openConfigFolder() {
            try {
                await fetch('/api/open-config-folder', { method: 'POST' });
                logToConsole('Opened config folder %APPDATA%\\Sager in Windows Explorer', 'INFO');
            } catch (e) {
                console.error(e);
            }
        }

        async function runWriteSpeedTest() {
            logToConsole('Testing Google Drive backup directory write latency...', 'INFO');
            try {
                const res = await fetch('/api/test/write-test', { method: 'POST' });
                const d = await res.json();
                if (d.success) {
                    showToast('✅ ' + d.message);
                    logToConsole(`Write latency test PASSED in ${d.latency_ms.toFixed(2)} ms on ${d.path}`, 'PASS');
                } else {
                    showToast('❌ ' + d.message);
                    logToConsole(`Write test FAILED on ${d.path}: ${d.message}`, 'ERROR');
                }
            } catch (e) {
                logToConsole('Write test error: ' + e, 'ERROR');
            }
        }

        async function simulateCustomAchievement() {
            const game = document.getElementById('sim-game').value.trim() || 'Project Lightning';
            const title = document.getElementById('sim-title').value.trim() || 'Achievement Unlocked!';
            const desc = document.getElementById('sim-desc').value.trim() || 'Custom test alert';

            logToConsole(`Triggering custom achievement toast: "${title}" in "${game}"`, 'INFO');
            try {
                const res = await fetch('/api/test/custom-achievement', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ game_name: game, title: title, description: desc })
                });
                const d = await res.json();
                showToast('🔔 ' + d.message);
                logToConsole(`Windows Tray Balloon alert delivered: "${title}"`, 'PASS');
                setTimeout(loadAchievements, 500);
            } catch (e) {
                logToConsole('Failed to trigger achievement: ' + e, 'ERROR');
            }
        }

        async function runFullDiagnostics() {
            logToConsole('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━', 'INFO');
            logToConsole('Starting full system diagnostics check...', 'INFO');

            try {
                const res = await fetch('/api/test/diagnostics');
                const d = await res.json();

                // 1. Google Drive
                const gd = d.google_drive;
                const gdriveBadge = document.getElementById('diag-badge-gdrive');
                const gdriveDetail = document.getElementById('diag-detail-gdrive');
                if (gd.running) {
                    gdriveBadge.textContent = 'Running';
                    gdriveBadge.style.color = 'var(--success)';
                    gdriveDetail.textContent = `Process active. Backup dir: ${gd.backup_dir} (Write test: ${gd.write_latency_ms.toFixed(2)} ms)`;
                    logToConsole(`[Google Drive] Process Running | Latency: ${gd.write_latency_ms.toFixed(2)} ms | Path: ${gd.backup_dir}`, 'PASS');
                } else {
                    gdriveBadge.textContent = gd.installed ? 'Stopped' : 'Not Installed';
                    gdriveBadge.style.color = gd.installed ? 'var(--gold)' : 'var(--danger)';
                    gdriveDetail.textContent = `Process not running. Backup dir: ${gd.backup_dir} (Write test: ${gd.write_latency_ms.toFixed(2)} ms)`;
                    logToConsole(`[Google Drive] Process is stopped. Target: ${gd.backup_dir}`, 'WARN');
                }

                // 2. Watcher
                const w = d.watcher;
                const watcherDetail = document.getElementById('diag-detail-watcher');
                const watcherParts = [];
                if (w.goldberg_detected) watcherParts.push('Goldberg');
                if (w.project_lightning_detected) watcherParts.push('Project Lightning');
                if (w.steam_detected) watcherParts.push('Steam');
                watcherDetail.textContent = `Active locations: ${watcherParts.join(', ') || 'Custom configured'}. Unlocked history: ${d.achievements.total_unlocked} trophies.`;
                logToConsole(`[Achievement Watcher] Locations: ${watcherParts.join(', ')} | Trophies: ${d.achievements.total_unlocked}`, 'PASS');

                // 3. Scanner
                const s = d.save_scanner;
                const scannerDetail = document.getElementById('diag-detail-scanner');
                scannerDetail.textContent = `Found ${s.games_count} games with saves (${formatBytes(s.total_size_bytes)} total on disk).`;
                logToConsole(`[Save Scanner] Discovered ${s.games_count} game saves (${formatBytes(s.total_size_bytes)})`, 'PASS');

                // 4. Backup & Restore
                const b = d.cloud_backups;
                const backupDetail = document.getElementById('diag-detail-backup');
                backupDetail.textContent = `${b.total_archives} cloud snapshots ready for restore (${formatBytes(b.total_size_bytes)}).`;
                logToConsole(`[Cloud Archives] ${b.total_archives} zip snapshots available (${formatBytes(b.total_size_bytes)})`, 'PASS');

                // 5. Sound & System
                const sys = d.system;
                const soundDetail = document.getElementById('diag-detail-sound');
                soundDetail.textContent = `Notifications: ${sys.notifications_enabled ? 'ON' : 'OFF'} | Audio Chimes: ${sys.sound_enabled ? 'ON' : 'OFF'}`;

                // 6. Startup
                const pathsDetail = document.getElementById('diag-detail-paths');
                pathsDetail.textContent = `Startup Shortcut: ${sys.startup_shortcut_installed ? 'Installed (Autostart ON)' : 'Not detected'}. Config: ${sys.config_path}`;
                logToConsole(`[Windows Startup] Auto-start shortcut: ${sys.startup_shortcut_installed ? 'Installed' : 'Missing'}`, sys.startup_shortcut_installed ? 'PASS' : 'WARN');

                logToConsole('All diagnostics completed successfully!', 'SUCCESS');
                logToConsole('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━', 'INFO');
            } catch (e) {
                logToConsole('Diagnostics error: ' + e, 'ERROR');
            }
        }

        async function loadDriveStatus() {
            try {
                const res = await fetch('/api/drive-status');
                const d = await res.json();
                const badge = document.getElementById('gdrive-badge');
                const action = document.getElementById('gdrive-action');
                const card = document.getElementById('gdrive-status-card');
                badge.style.display = 'inline-flex';

                if (d.google_drive_running) {
                    badge.textContent = 'Running';
                    badge.style.backgroundColor = 'rgba(16, 185, 129, 0.2)';
                    badge.style.color = 'var(--success)';
                    card.style.borderColor = 'var(--success)';
                    action.innerHTML = '';
                } else if (d.google_drive_installed) {
                    badge.textContent = 'Stopped';
                    badge.style.backgroundColor = 'rgba(239, 68, 68, 0.2)';
                    badge.style.color = 'var(--danger)';
                    card.style.borderColor = 'var(--danger)';
                    action.innerHTML = '<button class="btn btn-sm" onclick="launchGoogleDrive()">🚀 Launch Google Drive</button>';
                } else {
                    badge.textContent = 'Not Installed';
                    badge.style.backgroundColor = 'rgba(148, 163, 184, 0.2)';
                    badge.style.color = 'var(--text-muted)';
                    card.style.borderColor = 'var(--border)';
                    action.innerHTML = '<span style="font-size:11px;color:var(--text-muted);">Install Google Drive for Desktop</span>';
                }
            } catch (e) { console.error(e); }
        }

        async function launchGoogleDrive() {
            showToast('🚀 Launching Google Drive...');
            logToConsole('Launching Google Drive desktop executable...', 'INFO');
            try {
                const res = await fetch('/api/launch-gdrive', { method: 'POST' });
                const data = await res.json();
                if (data.success) {
                    showToast('✅ ' + data.message);
                    logToConsole(data.message, 'PASS');
                    setTimeout(loadDriveStatus, 3000);
                } else {
                    showToast('❌ ' + data.message);
                    logToConsole(data.message, 'ERROR');
                }
            } catch (e) {
                showToast('❌ Failed to launch Google Drive');
                logToConsole('Launch error: ' + e, 'ERROR');
            }
        }

        async function loadStatus() {
            try {
                const res = await fetch('/api/status');
                const data = await res.json();
                document.getElementById('status-backup-dir').textContent = data.backup_dir;
                document.getElementById('status-games-count').textContent = data.games_count;
                document.getElementById('status-ach-count').textContent = data.achievements_count;
                const bpd = data.backups_per_day || Math.max(1, Math.round(1440 / (data.auto_backup_interval || 60)));
                document.getElementById('status-interval').textContent = `${bpd}x / day (Every ${data.auto_backup_interval}m)`;
            } catch (e) {
                console.error(e);
            }
        }

        function filterGames() {
            const query = (document.getElementById('game-search').value || '').toLowerCase();
            const filtered = allDiscoveredGames.filter(g => 
                g.name.toLowerCase().includes(query) || 
                g.save_category.toLowerCase().includes(query) ||
                g.source_path.toLowerCase().includes(query)
            );
            renderGamesTable(filtered);
        }

        function renderGamesTable(games) {
            const tbody = document.getElementById('games-table-body');
            if (games.length === 0) {
                tbody.innerHTML = '<tr><td colspan="6" style="text-align:center; color: var(--text-muted);">No matching game saves found.</td></tr>';
                return;
            }
            tbody.innerHTML = games.map(g => `
                <tr>
                    <td style="font-weight: 700; color: var(--text-main);">${g.name}</td>
                    <td><span style="background: rgba(255,255,255,0.06); padding: 2px 6px; border-radius: 4px; font-size: 11px;">${g.save_category}</span></td>
                    <td style="font-family: monospace; font-size: 11px; color: var(--text-muted);">${g.source_path}</td>
                    <td>${formatBytes(g.total_size_bytes)}</td>
                    <td>${g.files_count}</td>
                    <td>
                        <button class="btn btn-sm" onclick="backupSingleGame('${encodeURIComponent(g.name)}', '${encodeURIComponent(g.source_path)}')">💾 Backup</button>
                    </td>
                </tr>
            `).join('');
        }

        async function loadGames() {
            const tbody = document.getElementById('games-table-body');
            tbody.innerHTML = '<tr><td colspan="6" style="text-align:center;">Scanning save games...</td></tr>';
            try {
                const res = await fetch('/api/games');
                allDiscoveredGames = await res.json();
                renderGamesTable(allDiscoveredGames);
                document.getElementById('status-games-count').textContent = allDiscoveredGames.length;
            } catch (e) {
                tbody.innerHTML = '<tr><td colspan="6" style="color:var(--danger);">Error scanning games</td></tr>';
            }
        }

        async function loadBackups() {
            const tbody = document.getElementById('backups-table-body');
            tbody.innerHTML = '<tr><td colspan="6" style="text-align:center;">Loading cloud archives...</td></tr>';
            try {
                const res = await fetch('/api/backups');
                const data = await res.json();
                let total = 0;
                let rows = '';

                data.forEach(group => {
                    const gameName = group[0];
                    const archives = group[1];
                    total += archives.length;

                    archives.forEach(arc => {
                        const dateStr = arc.manifest ? arc.manifest.backup_date_str : 'Unknown';
                        const origPath = arc.manifest ? arc.manifest.original_path : 'Original save path';
                        rows += `
                            <tr>
                                <td style="font-weight: 700; color: var(--primary);">${gameName}</td>
                                <td>${arc.file_name}</td>
                                <td>${dateStr}</td>
                                <td>${formatBytes(arc.size_bytes)}</td>
                                <td style="font-family: monospace; font-size: 11px; color: var(--text-muted);">${origPath}</td>
                                <td>
                                    <button class="btn btn-gold btn-sm" onclick="restoreBackup('${encodeURIComponent(arc.full_path)}')">🔄 Restore to PC</button>
                                </td>
                            </tr>
                        `;
                    });
                });

                document.getElementById('status-backups-count').textContent = total;
                if (total === 0) {
                    tbody.innerHTML = '<tr><td colspan="6" style="text-align:center;">No backups found in Google Drive yet. Click "Backup All Saves" above!</td></tr>';
                } else {
                    tbody.innerHTML = rows;
                }
            } catch (e) {
                tbody.innerHTML = '<tr><td colspan="6" style="color:var(--danger);">Error loading backups</td></tr>';
            }
        }

        async function loadAchievements() {
            try {
                const res = await fetch('/api/achievements');
                const list = await res.json();
                const container = document.getElementById('achievements-container');
                document.getElementById('status-ach-count').textContent = list.length;

                if (list.length === 0) {
                    container.innerHTML = '<p style="color: var(--text-muted);">No achievements logged yet. Unlock achievements in games or click "Test Notification"!</p>';
                    return;
                }

                container.innerHTML = list.map(a => {
                    const timeStr = a.unlock_time ? new Date(a.unlock_time * 1000).toLocaleString() : 'Recently';
                    return `
                        <div class="ach-card">
                            <div class="ach-badge">🏆</div>
                            <div class="ach-info">
                                <div class="ach-game">${a.game_name}</div>
                                <div class="ach-title">${a.title}</div>
                                <div class="ach-desc">${a.description || 'Achievement unlocked'}</div>
                                <div class="ach-date">Unlocked: ${timeStr}</div>
                            </div>
                        </div>
                    `;
                }).join('');
            } catch (e) {
                console.error(e);
            }
        }

        async function loadConfig() {
            try {
                const res = await fetch('/api/config');
                const cfg = await res.json();
                document.getElementById('cfg-backup-dir').value = cfg.backup_directory;
                document.getElementById('cfg-interval').value = cfg.auto_backup_interval_minutes;
                document.getElementById('cfg-sound').checked = cfg.sound_enabled;
                document.getElementById('cfg-notif').checked = cfg.notifications_enabled;
                document.getElementById('cfg-auto-backup').checked = cfg.auto_backup_enabled;
                document.getElementById('cfg-gdrive-exe').value = cfg.google_drive_exe || '';

                const bpd = cfg.backups_per_day || Math.max(1, Math.round(1440 / (cfg.auto_backup_interval_minutes || 60)));
                document.getElementById('cfg-bpd').value = bpd;
                updateFreqNotice();

                // Highlight matching preset button if any
                document.querySelectorAll('.preset-btn').forEach(btn => {
                    btn.classList.toggle('active', btn.textContent.startsWith(bpd + 'x'));
                });
            } catch (e) {
                console.error(e);
            }
        }

        async function saveSettings(e) {
            e.preventDefault();
            const gdriveExe = document.getElementById('cfg-gdrive-exe').value.trim();
            const bpd = parseInt(document.getElementById('cfg-bpd').value) || 24;
            const interval = Math.max(1, Math.round(1440 / bpd));

            const config = {
                backup_directory: document.getElementById('cfg-backup-dir').value,
                auto_backup_interval_minutes: interval,
                backups_per_day: bpd,
                sound_enabled: document.getElementById('cfg-sound').checked,
                notifications_enabled: document.getElementById('cfg-notif').checked,
                auto_backup_enabled: document.getElementById('cfg-auto-backup').checked,
                google_drive_exe: gdriveExe || null
            };

            const res = await fetch('/api/config', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(config)
            });

            if (res.ok) {
                showToast(`✅ Saved! Backing up ${bpd} times a day to Google Drive.`);
                logToConsole(`Configuration saved. Backup target: ${config.backup_directory}, schedule: ${bpd}x daily`, 'PASS');
                loadStatus();
                loadBackups();
            } else {
                showToast('❌ Failed to save configuration');
            }
        }

        async function backupAllGames() {
            showToast('⏳ Backing up all game saves to Google Drive...');
            logToConsole('Triggered backup for all discovered games...', 'INFO');
            const res = await fetch('/api/backup', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ all: true })
            });
            const data = await res.json();
            showToast('✅ ' + data.message);
            logToConsole(data.message, 'PASS');
            loadBackups();
            loadStatus();
        }

        async function backupSingleGame(name, path) {
            const decodedName = decodeURIComponent(name);
            showToast('⏳ Backing up ' + decodedName + '...');
            logToConsole(`Backing up game save: ${decodedName}`, 'INFO');
            const res = await fetch('/api/backup', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    game_name: decodedName,
                    source_path: decodeURIComponent(path)
                })
            });
            const data = await res.json();
            showToast('✅ ' + data.message);
            logToConsole(data.message, 'PASS');
            loadBackups();
        }

        async function restoreBackup(zipPath) {
            const decodedZip = decodeURIComponent(zipPath);
            if (!confirm('Are you sure you want to restore this save? (A safety backup of your existing local save will be created automatically)')) {
                return;
            }
            showToast('⏳ Restoring save files...');
            logToConsole(`Restoring save archive: ${decodedZip}`, 'INFO');
            const res = await fetch('/api/restore', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ zip_path: decodedZip })
            });
            const data = await res.json();
            if (data.success) {
                showToast('✅ Save restored successfully! ' + data.message);
                logToConsole(data.message, 'PASS');
            } else {
                showToast('❌ Restore failed: ' + data.message);
                logToConsole('Restore error: ' + data.message, 'ERROR');
            }
        }

        async function openBackupFolder() {
            await fetch('/api/open-folder', { method: 'POST' });
            logToConsole('Opened Google Drive backup folder in Windows Explorer', 'INFO');
        }

        async function testAchievement() {
            showToast('🔔 Triggering test achievement pop...');
            logToConsole('Sending test achievement pop notification to Windows tray...', 'INFO');
            await fetch('/api/achievements/test', { method: 'POST' });
            logToConsole('Delivered test notification: "Sager Master Unlocked"', 'PASS');
            setTimeout(loadAchievements, 500);
        }

        // Initialize dashboard
        loadStatus();
        loadDriveStatus();
        loadGames();
        loadBackups();
        loadAchievements();
        loadConfig();

        logToConsole('🚀 Sager Web UI connected & initialized', 'SUCCESS');
        logToConsole('Connected to backend at http://127.0.0.1:3939', 'INFO');

        // Auto refresh achievements, status, and drive status
        setInterval(() => {
            loadAchievements();
            loadStatus();
            loadDriveStatus();
        }, 4000);
    </script>
</body>
</html>
"#
}
