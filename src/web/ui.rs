pub fn get_dashboard_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Sager - Game Companion & Cloud Sync</title>
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
            gap: 12px;
        }

        .logo-icon {
            width: 40px;
            height: 40px;
            background: linear-gradient(135deg, #0ea5e9 0%, #6366f1 100%);
            border-radius: 10px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 22px;
            box-shadow: 0 4px 12px rgba(14, 165, 233, 0.3);
        }

        .logo-title {
            font-size: 22px;
            font-weight: 800;
            letter-spacing: 1px;
            background: linear-gradient(90deg, #38bdf8, #818cf8);
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
            grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
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
            font-size: 18px;
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
            <div class="logo-icon">🎮</div>
            <div>
                <div class="logo-title">SAGER</div>
                <div class="logo-tagline">Steam Achievements & Game Save Cloud Backup</div>
            </div>
        </div>
        <div class="quick-actions">
            <button class="btn" onclick="backupAllGames()">💾 Backup All Saves</button>
            <button class="btn btn-secondary" onclick="openBackupFolder()">📂 Open Google Drive Folder</button>
            <button class="btn btn-gold" onclick="testAchievement()">🔔 Test Achievement Alert</button>
        </div>
    </header>

    <div class="nav-tabs">
        <div class="nav-tab active" onclick="switchTab('tab-saves')">💾 Game Saves & Cloud Restore</div>
        <div class="nav-tab" onclick="switchTab('tab-achievements')">🏆 Steam Achievements Notifier</div>
        <div class="nav-tab" onclick="switchTab('tab-settings')">⚙️ Settings & Paths</div>
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
                    <div class="status-value"><span class="badge-live">Active</span> <span id="status-interval" style="font-size: 13px; color: var(--text-muted);">Every 60 min</span></div>
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
                <div class="card-header">
                    <div class="card-title">🎮 Discovered PC Game Saves (Ludusavi Engine)</div>
                    <button class="btn btn-secondary btn-sm" onclick="loadGames()">🔄 Rescan Saves</button>
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

        <!-- Tab 3: Settings -->
        <div id="tab-settings" class="tab-content">
            <div class="card">
                <div class="card-header">
                    <div class="card-title">⚙️ Sager Configuration</div>
                </div>
                <form id="config-form" onsubmit="saveSettings(event)">
                    <div class="form-group">
                        <label class="form-label">Google Drive / Cloud Backup Directory</label>
                        <div class="form-hint">Path where your save zip archives and manifests are stored. Set this to your Google Drive folder on both your laptop and PC for automatic cross-machine save syncing!</div>
                        <input type="text" id="cfg-backup-dir" class="form-input" required />
                    </div>

                    <div class="form-group">
                        <label class="form-label">Google Drive Executable Path (Optional)</label>
                        <div class="form-hint">Custom path to GoogleDriveFS.exe. Leave blank for auto-detection.</div>
                        <input type="text" id="cfg-gdrive-exe" class="form-input" placeholder="Auto-detect" />
                    </div>

                    <div class="form-group">
                        <label class="form-label">Auto-Backup Interval (Minutes)</label>
                        <input type="number" id="cfg-interval" class="form-input" min="5" max="1440" required />
                    </div>

                    <div style="display: flex; gap: 24px; margin-bottom: 20px;">
                        <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                            <input type="checkbox" id="cfg-sound" /> Enable Achievement Pop Sound
                        </label>
                        <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                            <input type="checkbox" id="cfg-notif" /> Enable Windows Desktop Notifications
                        </label>
                        <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                            <input type="checkbox" id="cfg-auto-backup" /> Enable Automatic Background Backups
                        </label>
                    </div>

                    <button type="submit" class="btn">💾 Save Settings</button>
                </form>
            </div>
        </div>
    </main>

    <div id="toast" class="toast-msg"></div>

    <script>
        function showToast(msg) {
            const t = document.getElementById('toast');
            t.textContent = msg;
            t.style.display = 'block';
            setTimeout(() => { t.style.display = 'none'; }, 4000);
        }

        function switchTab(tabId) {
            document.querySelectorAll('.tab-content').forEach(el => el.classList.remove('active'));
            document.querySelectorAll('.nav-tab').forEach(el => el.classList.remove('active'));
            document.getElementById(tabId).classList.add('active');
            event.currentTarget.classList.add('active');
        }

        function formatBytes(bytes) {
            if (bytes === 0) return '0 B';
            const k = 1024;
            const sizes = ['B', 'KB', 'MB', 'GB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
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
            try {
                const res = await fetch('/api/launch-gdrive', { method: 'POST' });
                const data = await res.json();
                if (data.success) {
                    showToast('✅ ' + data.message);
                    setTimeout(loadDriveStatus, 3000);
                } else {
                    showToast('❌ ' + data.message);
                }
            } catch (e) { showToast('❌ Failed to launch Google Drive'); }
        }

        async function loadStatus() {
            try {
                const res = await fetch('/api/status');
                const data = await res.json();
                document.getElementById('status-backup-dir').textContent = data.backup_dir;
                document.getElementById('status-games-count').textContent = data.games_count;
                document.getElementById('status-ach-count').textContent = data.achievements_count;
                document.getElementById('status-interval').textContent = 'Every ' + data.auto_backup_interval + ' min';
            } catch (e) {
                console.error(e);
            }
        }

        async function loadGames() {
            const tbody = document.getElementById('games-table-body');
            tbody.innerHTML = '<tr><td colspan="6" style="text-align:center;">Scanning save games...</td></tr>';
            try {
                const res = await fetch('/api/games');
                const games = await res.json();
                if (games.length === 0) {
                    tbody.innerHTML = '<tr><td colspan="6" style="text-align:center;">No game saves found in standard locations.</td></tr>';
                    return;
                }
                tbody.innerHTML = games.map(g => `
                    <tr>
                        <td style="font-weight: 700;">${g.name}</td>
                        <td><span style="background: rgba(255,255,255,0.06); padding: 2px 6px; border-radius: 4px; font-size: 11px;">${g.save_category}</span></td>
                        <td style="font-family: monospace; font-size: 11px; color: var(--text-muted);">${g.source_path}</td>
                        <td>${formatBytes(g.total_size_bytes)}</td>
                        <td>${g.files_count}</td>
                        <td>
                            <button class="btn btn-sm" onclick="backupSingleGame('${encodeURIComponent(g.name)}', '${encodeURIComponent(g.source_path)}')">💾 Backup</button>
                        </td>
                    </tr>
                `).join('');
                document.getElementById('status-games-count').textContent = games.length;
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
            } catch (e) {
                console.error(e);
            }
        }

        async function saveSettings(e) {
            e.preventDefault();
            const gdriveExe = document.getElementById('cfg-gdrive-exe').value.trim();
            const config = {
                backup_directory: document.getElementById('cfg-backup-dir').value,
                auto_backup_interval_minutes: parseInt(document.getElementById('cfg-interval').value),
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
                showToast('✅ Configuration saved successfully!');
                loadStatus();
                loadBackups();
            } else {
                showToast('❌ Failed to save configuration');
            }
        }

        async function backupAllGames() {
            showToast('⏳ Backing up all game saves to Google Drive...');
            const res = await fetch('/api/backup', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ all: true })
            });
            const data = await res.json();
            showToast('✅ ' + data.message);
            loadBackups();
            loadStatus();
        }

        async function backupSingleGame(name, path) {
            showToast('⏳ Backing up ' + decodeURIComponent(name) + '...');
            const res = await fetch('/api/backup', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    game_name: decodeURIComponent(name),
                    source_path: decodeURIComponent(path)
                })
            });
            const data = await res.json();
            showToast('✅ ' + data.message);
            loadBackups();
        }

        async function restoreBackup(zipPath) {
            if (!confirm('Are you sure you want to restore this save? (A safety backup of your existing local save will be created automatically)')) {
                return;
            }
            showToast('⏳ Restoring save files...');
            const res = await fetch('/api/restore', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ zip_path: decodeURIComponent(zipPath) })
            });
            const data = await res.json();
            if (data.success) {
                showToast('✅ Save restored successfully! ' + data.message);
            } else {
                showToast('❌ Restore failed: ' + data.message);
            }
        }

        async function openBackupFolder() {
            await fetch('/api/open-folder', { method: 'POST' });
        }

        async function testAchievement() {
            showToast('🔔 Triggering test achievement pop...');
            await fetch('/api/achievements/test', { method: 'POST' });
            setTimeout(loadAchievements, 500);
        }

        // Initialize dashboard
        loadStatus();
        loadDriveStatus();
        loadGames();
        loadBackups();
        loadAchievements();
        loadConfig();

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
