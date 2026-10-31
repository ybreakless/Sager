pub fn get_dashboard_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Sager - Game Companion & Cloud Save Shield</title>
    <link rel="icon" type="image/png" href="/api/logo">
    <style>
        :root {
            --bg-base: #000;
            --bg-surface: #000;
            --bg-card: #111;
            --bg-card-hover: #222;
            --bg-input: #000;
            --border: #333;
            --border-highlight: #555;
            --text-main: #fff;
            --text-muted: #bbb;
            --primary: #fff;
            --primary-glow: rgba(255,255,255,0.2);
            --gold: #fff;
            --gold-glow: rgba(255,255,255,0.2);
            --success: #fff;
            --success-glow: rgba(255,255,255,0.2);
            --danger: #fff;
            --purple: #fff;
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, Roboto, sans-serif;
            -webkit-font-smoothing: antialiased;
        }

        body {
            background-color: var(--bg-base);
            color: var(--text-main);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            overflow-x: hidden;
            font-size: clamp(13px, 1vw, 16px);
        }

        /* ── Top Header / ProtonVPN Companion Bar ── */
        header {
            background: linear-gradient(180deg, #121a2d 0%, var(--bg-surface) 100%);
            border-bottom: 1px solid var(--border);
            padding: 12px 24px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            position: sticky;
            top: 0;
            z-index: 100;
        }

        .logo-area {
            display: flex;
            align-items: center;
            gap: 12px;
            cursor: pointer;
        }

        .logo-img {
            width: 36px;
            height: 36px;
            border-radius: 8px;
            box-shadow: 0 0 12px var(--primary-glow);
        }

        .logo-title {
            font-size: 20px;
            font-weight: 800;
            letter-spacing: 1.2px;
            background: #38bdf8;
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }

        .logo-badge {
            background: rgba(16, 185, 129, 0.15);
            color: var(--success);
            border: 1px solid rgba(16, 185, 129, 0.4);
            border-radius: 20px;
            padding: 2px 8px;
            font-size: 11px;
            font-weight: 700;
            display: flex;
            align-items: center;
            gap: 5px;
        }

        .status-dot {
            width: 7px;
            height: 7px;
            background-color: var(--success);
            border-radius: 50%;
            box-shadow: 0 0 8px var(--success);
            animation: pulse-dot 2s infinite;
        }

        @keyframes pulse-dot {
            0%, 100% { opacity: 1; transform: scale(1); }
            50% { opacity: 0.5; transform: scale(0.85); }
        }

        .header-controls {
            display: flex;
            align-items: center;
            gap: 10px;
        }

        /* ── Navigation Tabs ── */
        .nav-tabs {
            display: flex;
            background-color: var(--bg-surface);
            border-bottom: 1px solid var(--border);
            padding: 0 24px;
            gap: 4px;
            overflow-x: auto;
        }

        .nav-tab {
            padding: 12px 18px;
            cursor: pointer;
            color: var(--text-muted);
            font-size: 13px;
            font-weight: 600;
            border-bottom: 2px solid transparent;
            transition: all 0.2s ease;
            display: flex;
            align-items: center;
            gap: 7px;
            white-space: nowrap;
        }

        .nav-tab:hover {
            color: var(--text-main);
            background-color: rgba(255, 255, 255, 0.02);
        }

        .nav-tab.active {
            color: var(--primary);
            border-bottom-color: var(--primary);
            background-color: rgba(56, 189, 248, 0.04);
        }

        /* ── Main Container ── */
        main {
            flex: 1;
            padding: clamp(12px, 2vw, 28px);
            max-width: min(100%, 1440px);
            margin: 0 auto;
            width: 100%;
            transition: all 0.3s ease;
        }

        @media (max-width: 720px) {
            header { flex-wrap: wrap; gap: 10px; padding: 10px 14px; }
            .header-controls { width: 100%; justify-content: flex-end; flex-wrap: wrap; }
            .nav-tabs { padding: 0 12px; }
            .hero-header { flex-direction: column; align-items: flex-start; gap: 8px; }
            .hero-backup-btn { font-size: 13px; padding: 12px 14px; }
            .cloud-sync-strip { flex-direction: column; align-items: flex-start; }
            .ach-matrix-grid { grid-template-columns: 1fr; }
        }

        @media (min-width: 1440px) {
            main { max-width: 1360px; }
        }

        /* ── ProtonVPN Hero Shield Card ── */
        .hero-shield-card {
            background: #111;
            border: 1px solid #293a5a;
            border-radius: 14px;
            padding: 20px;
            margin-bottom: 20px;
            position: relative;
            overflow: hidden;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.4);
        }

        .hero-shield-card::before {
            content: '';
            position: absolute;
            top: -60px;
            right: -60px;
            width: 160px;
            height: 160px;
            background: radial-gradient(circle, rgba(56, 189, 248, 0.2) 0%, transparent 70%);
            border-radius: 50%;
            pointer-events: none;
        }

        .hero-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 16px;
        }

        .hero-title-group {
            display: flex;
            align-items: center;
            gap: 10px;
        }

        .shield-icon {
            font-size: 26px;
            filter: drop-shadow(0 0 10px rgba(56, 189, 248, 0.5));
        }

        .hero-title {
            font-size: 16px;
            font-weight: 800;
            color: #fff;
            letter-spacing: 0.5px;
        }

        .hero-subtitle {
            font-size: 11px;
            color: var(--text-muted);
        }

        .hero-backup-btn {
            width: 100%;
            background: #0284c7;
            color: white;
            border: none;
            border-radius: 10px;
            padding: 14px 20px;
            font-size: 15px;
            font-weight: 700;
            letter-spacing: 0.5px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 10px;
            box-shadow: 0 6px 20px rgba(37, 99, 235, 0.4);
            transition: all 0.2s ease;
        }

        .hero-backup-btn:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 25px rgba(56, 189, 248, 0.5);
            background: #0369a1;
        }

        .hero-backup-btn:active {
            transform: translateY(0);
        }

        /* ── ProtonVPN Quick Controls Switchboard ── */
        .switchboard-card {
            background-color: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 16px;
            margin-bottom: 20px;
        }

        .card-header-sm {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 14px;
            font-size: 13px;
            font-weight: 700;
            color: var(--primary);
            text-transform: uppercase;
            letter-spacing: 0.8px;
        }

        .switch-row {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 10px 0;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        }

        .switch-row:last-child {
            border-bottom: none;
        }

        .switch-label {
            font-size: 13px;
            font-weight: 600;
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .switch-desc {
            font-size: 11px;
            color: var(--text-muted);
            margin-top: 2px;
        }

        /* Toggle Switch UI */
        .toggle-switch {
            position: relative;
            display: inline-block;
            width: 44px;
            height: 24px;
        }

        .toggle-switch input {
            opacity: 0;
            width: 0;
            height: 0;
        }

        .slider {
            position: absolute;
            cursor: pointer;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: #333;
            border: 1px solid var(--border);
            transition: .3s;
            border-radius: 24px;
        }

        .slider:before {
            position: absolute;
            content: "";
            height: 16px;
            width: 16px;
            left: 3px;
            bottom: 3px;
            background-color: #666;
            transition: .3s;
            border-radius: 50%;
        }

        input:checked + .slider {
            background: #0ea5e9;
            border-color: #38bdf8;
        }

        input:checked + .slider:before {
            transform: translateX(20px);
            background-color: #ffffff;
            box-shadow: 0 0 8px rgba(255, 255, 255, 0.8);
        }

        /* Launcher Dropdown & Pills */
        .launcher-pill-group {
            display: flex;
            flex-wrap: wrap;
            gap: 6px;
            margin-top: 8px;
        }

        .launcher-pill {
            background-color: var(--bg-input);
            border: 1px solid var(--border);
            color: var(--text-muted);
            padding: 5px 10px;
            border-radius: 6px;
            font-size: 11px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
        }

        .launcher-pill:hover {
            color: var(--text-main);
            border-color: #3b82f6;
        }

        .launcher-pill.active {
            background: rgba(56, 189, 248, 0.15);
            border-color: var(--primary);
            color: var(--primary);
            box-shadow: 0 0 10px var(--primary-glow);
        }

        /* ── Cloud Sync Status Strip ── */
        .cloud-sync-strip {
            background: rgba(14, 20, 36, 0.8);
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 12px 16px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 20px;
            gap: 12px;
        }

        .cloud-info {
            display: flex;
            align-items: center;
            gap: 10px;
        }

        .cloud-badge {
            font-size: 11px;
            font-weight: 700;
            padding: 3px 8px;
            border-radius: 6px;
        }

        .cloud-btn {
            background-color: #1e293b;
            border: 1px solid var(--border);
            color: var(--text-main);
            padding: 6px 12px;
            border-radius: 6px;
            font-size: 11px;
            font-weight: 600;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 6px;
            transition: all 0.2s;
        }

        .cloud-btn:hover {
            background-color: #27364f;
            border-color: var(--primary);
        }

        /* ── Achievement Matrix & Overall Progress ── */
        .ach-overview-banner {
            background: #182238;
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 18px 20px;
            margin-bottom: 20px;
        }

        .ach-progress-bar-bg {
            background-color: #0b1120;
            border-radius: 8px;
            height: 12px;
            width: 100%;
            overflow: hidden;
            margin: 12px 0 8px 0;
            border: 1px solid rgba(255, 255, 255, 0.05);
        }

        .ach-progress-bar-fill {
            height: 100%;
            background: #f59e0b;
            box-shadow: 0 0 12px var(--gold-glow);
            transition: width 0.6s cubic-bezier(0.16, 1, 0.3, 1);
        }

        .filter-pill-bar {
            display: flex;
            gap: 8px;
            margin-bottom: 16px;
            flex-wrap: wrap;
        }

        .filter-pill {
            background: var(--bg-card);
            border: 1px solid var(--border);
            color: var(--text-muted);
            padding: 6px 14px;
            border-radius: 20px;
            font-size: 12px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
        }

        .filter-pill:hover {
            color: var(--text-main);
            border-color: var(--primary);
        }

        .filter-pill.active {
            background: #555;
            color: var(--primary);
            border-color: var(--primary);
            box-shadow: 0 0 10px var(--primary-glow);
        }

        /* Achievement Cards Grid */
        .ach-matrix-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
            gap: 12px;
        }

        .matrix-card {
            background-color: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 14px;
            display: flex;
            gap: 12px;
            position: relative;
            overflow: hidden;
            transition: all 0.2s ease;
        }

        .matrix-card.unlocked {
            border-color: rgba(251, 191, 36, 0.4);
            background: #152033;
        }

        .matrix-card.unlocked::before {
            content: '';
            position: absolute;
            left: 0;
            top: 0;
            bottom: 0;
            width: 4px;
            background: #fbbf24;
        }

        .matrix-card.locked {
            opacity: 0.65;
            background-color: #0d1322;
        }

        .matrix-card.locked:hover {
            opacity: 0.95;
            border-color: #3b82f6;
        }

        .matrix-card.locked::before {
            content: '';
            position: absolute;
            left: 0;
            top: 0;
            bottom: 0;
            width: 4px;
            background-color: #334155;
        }

        .ach-card-icon {
            font-size: 24px;
            display: flex;
            align-items: center;
            justify-content: center;
            width: 44px;
            height: 44px;
            border-radius: 8px;
            background: rgba(0, 0, 0, 0.3);
            flex-shrink: 0;
        }

        .matrix-card.unlocked .ach-card-icon {
            box-shadow: 0 0 12px var(--gold-glow);
            background: rgba(251, 191, 36, 0.1);
        }

        .ach-card-content {
            flex: 1;
            min-width: 0;
        }

        .ach-card-title {
            font-size: 14px;
            font-weight: 700;
            color: var(--text-main);
            margin-bottom: 3px;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .matrix-card.unlocked .ach-card-title {
            color: #fef08a;
        }

        .ach-card-desc {
            font-size: 12px;
            color: var(--text-muted);
            line-height: 1.4;
            margin-bottom: 6px;
        }

        .ach-card-footer {
            display: flex;
            justify-content: space-between;
            align-items: center;
            font-size: 10.5px;
            color: #64748b;
        }

        .ach-launcher-badge {
            background: rgba(255, 255, 255, 0.05);
            padding: 1px 6px;
            border-radius: 4px;
            color: var(--primary);
            font-weight: 600;
        }

        /* ── Buttons & Forms ── */
        .btn {
            background: #0ea5e9;
            color: white;
            border: none;
            border-radius: 8px;
            padding: 8px 16px;
            font-size: 13px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s ease;
            display: inline-flex;
            align-items: center;
            gap: 6px;
        }

        .btn:hover {
            background: #38bdf8;
            transform: translateY(-1px);
        }

        .btn-sm {
            padding: 5px 10px;
            font-size: 11px;
            border-radius: 6px;
        }

        .btn-gold {
            background: #f59e0b;
            color: #1c1300;
            font-weight: 700;
        }

        .btn-gold:hover {
            background: #fbbf24;
        }

        .btn-bg-action {
            background: #10b981;
            color: #fff;
            font-weight: 700;
        }

        .btn-bg-action:hover {
            background: #10b981;
            box-shadow: 0 0 12px var(--success-glow);
        }

        .btn-appear-action {
            background: #6366f1;
            color: #fff;
            font-weight: 700;
        }

        .btn-appear-action:hover {
            background: #818cf8;
            box-shadow: 0 0 12px rgba(99, 102, 241, 0.4);
        }

        .form-input {
            width: 100%;
            background-color: var(--bg-input);
            border: 1px solid var(--border);
            color: var(--text-main);
            padding: 8px 12px;
            border-radius: 6px;
            font-size: 13px;
            outline: none;
            transition: border-color 0.2s;
        }

        .form-input:focus {
            border-color: var(--primary);
            box-shadow: 0 0 8px var(--primary-glow);
        }

        /* ── Games List Table ── */
        .table-container {
            overflow-x: auto;
            border-radius: 10px;
            border: 1px solid var(--border);
            background-color: var(--bg-card);
        }

        table {
            width: 100%;
            border-collapse: collapse;
            font-size: 12.5px;
            text-align: left;
        }

        th {
            background-color: rgba(255, 255, 255, 0.02);
            color: var(--text-muted);
            padding: 10px 14px;
            font-weight: 600;
            border-bottom: 1px solid var(--border);
        }

        td {
            padding: 10px 14px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.04);
            vertical-align: middle;
        }

        tr:hover td {
            background-color: var(--bg-card-hover);
        }

        /* ── Tab Switcher Visibility ── */
        .tab-content {
            display: none;
        }

        .tab-content.active {
            display: block;
            animation: fadeIn 0.25s ease-out;
        }

        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(4px); }
            to { opacity: 1; transform: translateY(0); }
        }

        .toast-msg {
            position: fixed;
            bottom: 24px;
            right: 24px;
            background: #1e293b;
            color: #fff;
            padding: 12px 20px;
            border-radius: 8px;
            border: 1px solid var(--primary);
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
            font-size: 13px;
            z-index: 1000;
            display: none;
            animation: slideUp 0.3s ease;
        }

        @keyframes slideUp {
            from { transform: translateY(20px); opacity: 0; }
            to { transform: translateY(0); opacity: 1; }
        }
    </style>
</head>
<body>

    <!-- Top Header / Bar -->
    <header>
        <div class="logo-area">
            <img src="/api/logo" alt="Sager" class="logo-img" />
            <div>
                <div class="logo-title">SAGER</div>
                <div style="font-size: 10px; color: var(--text-muted); font-weight: 600;">CLOUD SHIELD COMPANION</div>
            </div>
            <div class="logo-badge">
                <div class="status-dot"></div>
                <span id="header-status-label">ACTIVE</span>
            </div>
        </div>

        <div class="header-controls">
            <!-- Run in Background Button -->
            <button class="btn btn-sm btn-bg-action" id="btn-run-bg" onclick="runInBackground()" title="Hide console window & run silently in Windows background">
                <span>🛡️</span>
                <span>Run in Background</span>
            </button>

            <!-- Make Console Appear Button -->
            <button class="btn btn-sm btn-appear-action" id="btn-make-appear" onclick="makeConsoleAppear()" title="Make Sager console window appear on desktop">
                <span>👁️</span>
                <span>Make Appear</span>
            </button>
        </div>
    </header>

    <!-- Navigation Tabs -->
    <nav class="nav-tabs">
        <div class="nav-tab active" onclick="switchTab('tab-companion')">⚡ Companion</div>
        <div class="nav-tab" onclick="switchTab('tab-achievements')">🏆 Achievements Matrix</div>
        <div class="nav-tab" onclick="switchTab('tab-saves')">💾 Game Saves</div>
        <div class="nav-tab" onclick="switchTab('tab-cloud')">☁️ Cloud & Restore</div>
        <div class="nav-tab" onclick="switchTab('tab-vault')">📦 Vault & Mobile Sync</div>
        <div class="nav-tab" onclick="switchTab('tab-test')">🧪 Test Center</div>
        <div class="nav-tab" onclick="switchTab('tab-settings')">⚙️ Settings</div>
    </nav>

    <!-- Main Workspace -->
    <main>

        <!-- ═══════════════════════════════════════════════════════════════ -->
        <!-- TAB 1: PROTONVPN COMPANION VIEW (HERO & QUICK SWITCHBOARD)     -->
        <!-- ═══════════════════════════════════════════════════════════════ -->
        <div id="tab-companion" class="tab-content active">

            <!-- Hero Shield Card -->
            <div class="hero-shield-card">
                <div class="hero-header">
                    <div class="hero-title-group">
                        <div class="shield-icon">🛡️</div>
                        <div>
                            <div class="hero-title" id="hero-shield-title">GAME SAVE SHIELD ACTIVE</div>
                            <div class="hero-subtitle" id="hero-shield-sub">Monitoring Steam, Goldberg & Local saves</div>
                        </div>
                    </div>
                    <div id="hero-sync-indicator" style="font-size: 11px; color: var(--success); font-weight: 700;">
                        ● SYNCED
                    </div>
                </div>

                <!-- Giant One-Click Backup Button -->
                <button class="hero-backup-btn" onclick="backupAllGames()">
                    <span>💾</span>
                    <span>BACKUP ALL GAMES TO GOOGLE DRIVE</span>
                </button>

                <!-- Next Backup / Auto-Save Notice -->
                <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 14px; font-size: 11.5px; color: var(--text-muted);">
                    <span id="next-backup-notice">⏱️ Auto-Save: 24x / day (Every 60m)</span>
                    <span id="last-backup-badge" style="color: var(--primary); font-weight: 600;">Cloud: Ready</span>
                </div>
            </div>

            <!-- Google Drive Cloud Status Strip -->
            <div class="cloud-sync-strip" id="gdrive-status-strip">
                <div class="cloud-info">
                    <span style="font-size: 20px;">☁️</span>
                    <div>
                        <div style="font-size: 12.5px; font-weight: 700;" id="gdrive-status-title">Google Drive for Desktop</div>
                        <div style="font-size: 11px; color: var(--text-muted);" id="gdrive-status-path">Target: Auto-Detecting...</div>
                    </div>
                </div>
                <div style="display: flex; gap: 8px; align-items: center;" id="gdrive-action-container">
                    <span class="cloud-badge" id="gdrive-pill-badge" style="background: rgba(16, 185, 129, 0.2); color: var(--success);">Running</span>
                    <button class="cloud-btn" onclick="openBackupFolder()">📂 Open</button>
                </div>
            </div>

            <!-- ProtonVPN Quick Controls Switchboard -->
            <div class="switchboard-card">
                <div class="card-header-sm">
                    <span>⚡ Quick Controls</span>
                    <span style="font-size: 10px; color: var(--text-muted);">Real-Time Windows Sync</span>
                </div>

                <!-- 1. Auto-Backup Toggle -->
                <div class="switch-row">
                    <div>
                        <div class="switch-label">⏱️ Auto-Backup Game Saves</div>
                        <div class="switch-desc">Background snapshotting to Google Drive</div>
                    </div>
                    <label class="toggle-switch">
                        <input type="checkbox" id="switch-auto-backup" onchange="toggleQuickSetting('auto_backup_enabled', this.checked)">
                        <span class="slider"></span>
                    </label>
                </div>

                <!-- 2. Achievement Sound Alerts -->
                <div class="switch-row">
                    <div>
                        <div class="switch-label">🔔 Steam Achievement Sound (deck_ui)</div>
                        <div class="switch-desc">Authentic Steam unlock chime (deck_ui_achievement_toast.wav)</div>
                    </div>
                    <label class="toggle-switch">
                        <input type="checkbox" id="switch-sound" onchange="toggleQuickSetting('sound_enabled', this.checked)">
                        <span class="slider"></span>
                    </label>
                </div>

                <!-- 3. Desktop Notifications -->
                <div class="switch-row">
                    <div>
                        <div class="switch-label">💬 Steam TopMost In-Game Overlay</div>
                        <div class="switch-desc">Sleek overlay appears over running games without losing focus</div>
                    </div>
                    <label class="toggle-switch">
                        <input type="checkbox" id="switch-notif" onchange="toggleQuickSetting('notifications_enabled', this.checked)">
                        <span class="slider"></span>
                    </label>
                </div>

                <!-- 4. Preferred Game Launcher Selector -->
                <div class="switch-row" style="flex-direction: column; align-items: flex-start;">
                    <div style="width: 100%; display: flex; justify-content: space-between; align-items: center;">
                        <div>
                            <div class="switch-label">🎮 Preferred Game Launcher</div>
                            <div class="switch-desc">Choose launcher for tracking & in-game sync</div>
                        </div>
                        <span id="active-launcher-badge" style="font-size: 11px; font-weight: 700; color: var(--primary);">Goldberg</span>
                    </div>
                    <div class="launcher-pill-group">
                        <button class="launcher-pill active" onclick="selectLauncher('Goldberg SteamEmu')">Goldberg SteamEmu</button>
                        <button class="launcher-pill" onclick="selectLauncher('Steam')">Steam</button>
                        <button class="launcher-pill" onclick="selectLauncher('Project Lightning')">Project Lightning</button>
                        <button class="launcher-pill" onclick="selectLauncher('Custom / Direct')">Custom / Direct</button>
                    </div>
                </div>

                <!-- 5. Show in Launcher Toggle -->
                <div class="switch-row">
                    <div>
                        <div class="switch-label">🎯 Show Achievements in Launcher</div>
                        <div class="switch-desc">Synchronize unlock stats with launcher</div>
                    </div>
                    <label class="toggle-switch">
                        <input type="checkbox" id="switch-launcher-sync" onchange="toggleQuickSetting('show_in_launcher', this.checked)">
                        <span class="slider"></span>
                    </label>
                </div>
            </div>

            <!-- 🕶️ Background Execution & Windows Visibility Controls -->
            <div class="switchboard-card" style="border-color: rgba(56, 189, 248, 0.4); background: #15223c;">
                <div class="card-header-sm">
                    <span style="color: var(--primary);">🕶️ Background Execution & Window Controls</span>
                    <span id="console-live-badge" class="cloud-badge" style="background: rgba(56, 189, 248, 0.15); color: var(--primary);">Tray Active</span>
                </div>

                <div style="display: flex; gap: 10px; margin-bottom: 12px; flex-wrap: wrap;">
                    <button class="btn btn-gold" style="flex: 1; min-width: 170px; justify-content: center; padding: 10px 14px;" onclick="runInBackground()">
                        <span>🛡️</span>
                        <span>Run in Background (Hide Window)</span>
                    </button>

                    <button class="btn" style="flex: 1; min-width: 170px; justify-content: center; padding: 10px 14px; background: #3b82f6;" onclick="makeConsoleAppear()">
                        <span>👁️</span>
                        <span>Make Console Appear</span>
                    </button>
                </div>

                <div style="font-size: 11.5px; color: var(--text-muted); line-height: 1.4; margin-bottom: 12px;">
                    💡 <strong>Run in Background:</strong> Hides the console terminal window so Sager operates silently in your Windows system tray. Cloud save sync and achievement tracking will run 24/7 in the background.<br>
                    💡 <strong>Make Console Appear:</strong> Restores the live console window to your desktop so you can see live Rust engine logs.
                </div>

                <!-- Windows Startup Toggle -->
                <div class="switch-row" style="padding-top: 10px; border-top: 1px solid rgba(255, 255, 255, 0.08);">
                    <div>
                        <div class="switch-label">🚀 Windows Startup (Silent Auto-Run)</div>
                        <div class="switch-desc">Launch Sager automatically in silent background mode on PC boot</div>
                    </div>
                    <button class="btn btn-sm" id="btn-toggle-startup" onclick="toggleWindowsStartup()" style="background: rgba(16, 185, 129, 0.15); color: var(--success); border: 1px solid rgba(16, 185, 129, 0.4);">
                        ● Active (Boot ON)
                    </button>
                </div>
            </div>

            <!-- Quick Games Snapshot Preview -->
            <div class="switchboard-card">
                <div class="card-header-sm">
                    <span>🎮 Detected Games (<span id="quick-games-count">0</span>)</span>
                    <button class="btn btn-sm" onclick="switchTab('tab-saves')">View All</button>
                </div>
                <div id="quick-games-list" style="display: flex; flex-direction: column; gap: 8px;">
                    <!-- Rendered dynamically -->
                </div>
            </div>

        </div>

        <!-- ═══════════════════════════════════════════════════════════════ -->
        <!-- TAB 2: ACHIEVEMENTS MATRIX (UNLOCKED VS LOCKED OVERALL VIEW)    -->
        <!-- ═══════════════════════════════════════════════════════════════ -->
        <div id="tab-achievements" class="tab-content">
            
            <!-- Overall Progress Banner -->
            <div class="ach-overview-banner">
                <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 10px;">
                    <div>
                        <div style="font-size: 18px; font-weight: 800; color: #fff;" id="matrix-game-title">LEGO Batman: Legacy of the Dark Knight</div>
                        <div style="font-size: 12px; color: var(--text-muted);" id="matrix-game-meta">Goldberg SteamEmu • AppID 2215200</div>
                    </div>
                    <div style="text-align: right;">
                        <span style="font-size: 24px; font-weight: 800; color: var(--gold);" id="matrix-percent-label">3.1%</span>
                        <div style="font-size: 11px; color: var(--text-muted);" id="matrix-fraction-label">2 of 64 Unlocked</div>
                    </div>
                </div>

                <div class="ach-progress-bar-bg">
                    <div class="ach-progress-bar-fill" id="matrix-progress-fill" style="width: 3.1%;"></div>
                </div>

                <!-- Game Switcher Selector -->
                <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 12px; flex-wrap: wrap; gap: 10px;">
                    <div style="display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 12px; color: var(--text-muted);">Select Game:</span>
                        <select id="game-matrix-select" class="form-input" style="width: auto; padding: 4px 10px; font-weight: 600;" onchange="changeMatrixGame(this.value)">
                            <!-- Populated dynamically -->
                        </select>
                    </div>
                    <div style="display: flex; gap: 6px; flex-wrap: wrap;">
                        <button class="btn btn-sm" id="btn-open-steam-app" onclick="openCurrentGameInSteam()" style="background: rgba(102, 192, 244, 0.15); color: #66c0f4; border: 1px solid rgba(102, 192, 244, 0.4);">🎮 Open in Steam</button>
                        <button class="btn btn-sm btn-gold" onclick="triggerTestUnlock()">🔔 Test Alert</button>
                        <button class="btn btn-sm" onclick="loadAchievementsMatrix()">🔄 Refresh</button>
                    </div>
                </div>
            </div>

            <!-- Filter Pills (All / Unlocked / Locked) & Search -->
            <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 10px; margin-bottom: 16px;">
                <div class="filter-pill-bar" style="margin-bottom: 0;">
                    <button class="filter-pill active" id="pill-filter-all" onclick="filterMatrixView('all')">All (<span id="count-all">0</span>)</button>
                    <button class="filter-pill" id="pill-filter-unlocked" onclick="filterMatrixView('unlocked')">🏆 Unlocked (<span id="count-unlocked">0</span>)</button>
                    <button class="filter-pill" id="pill-filter-locked" onclick="filterMatrixView('locked')">🔒 Locked (<span id="count-locked">0</span>)</button>
                </div>

                <input type="text" id="ach-matrix-search" class="form-input" placeholder="🔍 Search achievements..." style="max-width: 260px;" oninput="applyMatrixSearch()" />
            </div>

            <!-- Achievement Matrix Cards Grid -->
            <div class="ach-matrix-grid" id="ach-matrix-container">
                <!-- Rendered dynamically -->
            </div>

        </div>

        <!-- ═══════════════════════════════════════════════════════════════ -->
        <!-- TAB 3: GAME SAVES MANAGER                                       -->
        <!-- ═══════════════════════════════════════════════════════════════ -->
        <div id="tab-saves" class="tab-content">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; flex-wrap: wrap; gap: 10px;">
                <div>
                    <h2 style="font-size: 18px; font-weight: 800;">🎮 Detected Game Saves</h2>
                    <p style="font-size: 12px; color: var(--text-muted);">Sager scans Steam, Goldberg, LocalAppData, and Saved Games</p>
                </div>
                <div style="display: flex; gap: 8px;">
                    <input type="text" id="game-search" class="form-input" placeholder="Search games..." style="width: 220px;" oninput="filterGames()" />
                    <button class="btn btn-sm" onclick="loadGames()">🔄 Scan Now</button>
                </div>
            </div>

            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th>Game Name</th>
                            <th>Category</th>
                            <th>Save Path</th>
                            <th>Size</th>
                            <th>Files</th>
                            <th>Action</th>
                        </tr>
                    </thead>
                    <tbody id="games-table-body">
                        <!-- Populated dynamically -->
                    </tbody>
                </table>
            </div>
        </div>

        <!-- ═══════════════════════════════════════════════════════════════ -->
        <!-- TAB 4: CLOUD ARCHIVES & RESTORE                                -->
        <!-- ═══════════════════════════════════════════════════════════════ -->
        <div id="tab-cloud" class="tab-content">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; flex-wrap: wrap; gap: 10px;">
                <div>
                    <h2 style="font-size: 18px; font-weight: 800;">☁️ Cloud Archives (Google Drive)</h2>
                    <p style="font-size: 12px; color: var(--text-muted);">Compressed zip archives with JSON manifests ready to restore to any PC</p>
                </div>
                <button class="btn btn-sm" onclick="openBackupFolder()">📂 Open Google Drive Folder</button>
            </div>

            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th>Game</th>
                            <th>Archive Filename</th>
                            <th>Backup Date</th>
                            <th>Size</th>
                            <th>Target Save Path</th>
                            <th>Restore</th>
                        </tr>
                    </thead>
                    <tbody id="backups-table-body">
                        <!-- Populated dynamically -->
                    </tbody>
                </table>
            </div>
        </div>

        <!-- ═══════════════════════════════════════════════════════════════ -->
        <!-- TAB 5: TEST CENTER & DIAGNOSTICS                               -->
        <!-- ═══════════════════════════════════════════════════════════════ -->
        <div id="tab-test" class="tab-content">
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 16px; margin-bottom: 20px;">
                
                <!-- Quick Tests Card -->
                <div class="switchboard-card">
                    <div class="card-header-sm">🧪 Interactive Test Suite</div>
                    <div style="display: flex; flex-direction: column; gap: 10px;">
                        <button class="btn" onclick="testSound()">🔔 Test Steam Achievement Sound</button>
                        <button class="btn btn-gold" onclick="triggerTestUnlock()">🏆 Test Steam In-Game TopMost Overlay & Chime</button>
                        <button class="btn" onclick="runWriteSpeedTest()">⚡ Run Google Drive Write Benchmark</button>
                        <button class="btn" onclick="runFullDiagnostics()">🔬 Run Full System Diagnostics</button>
                    </div>
                </div>

                <!-- Custom Simulator Card -->
                <div class="switchboard-card">
                    <div class="card-header-sm">🎮 Custom Achievement Simulator</div>
                    <div style="display: flex; flex-direction: column; gap: 8px;">
                        <input type="text" id="sim-game" class="form-input" placeholder="Game Name (e.g. LEGO Batman)" value="LEGO Batman: Legacy of the Dark Knight" />
                        <input type="text" id="sim-title" class="form-input" placeholder="Achievement Title" value="Gotham's Guardian Unlocked!" />
                        <input type="text" id="sim-desc" class="form-input" placeholder="Description" value="Defeated all villains in Gotham City" />
                        <button class="btn btn-gold" onclick="simulateCustomAchievement()">🚀 Trigger Toast & Save</button>
                    </div>
                </div>

            </div>

            <!-- Diagnostics Output Console -->
            <div class="switchboard-card">
                <div class="card-header-sm">
                    <span>📋 Live Diagnostics Console</span>
                    <button class="btn btn-sm" onclick="clearConsole()">Clear</button>
                </div>
                <div id="console-output" style="background: #050811; border: 1px solid var(--border); border-radius: 8px; padding: 14px; font-family: 'Consolas', monospace; font-size: 12px; height: 240px; overflow-y: auto; line-height: 1.6; color: #38bdf8;">
                </div>
            </div>
        </div>

        <!-- ═══════════════════════════════════════════════════════════════ -->
        <!-- TAB 6: VAULT EXPORT & IMPORT / MOBILE SYNC                      -->
        <!-- ═══════════════════════════════════════════════════════════════ -->
        <div id="tab-vault" class="tab-content">
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 16px; margin-bottom: 20px;">
                
                <!-- Export Vault Card -->
                <div class="switchboard-card">
                    <div class="card-header-sm">
                        <span>📦 Export Sager Vault</span>
                        <span class="cloud-badge" style="background: rgba(16, 185, 129, 0.15); color: var(--success);">Zero Loss Backup</span>
                    </div>
                    <div style="font-size: 12.5px; color: var(--text-muted); line-height: 1.5; margin-bottom: 16px;">
                        Export your complete trophy unlock history, discovered games, backup configuration, and manifest into a single portable bundle (<code>.json</code> / <code>.sager</code>). Carry your achievements anywhere or restore on a new machine.
                    </div>
                    <button class="btn btn-gold" onclick="exportVaultBundle()" style="width: 100%; justify-content: center; padding: 12px;">
                        <span>📦</span>
                        <span>Download Sager Vault Bundle (.json)</span>
                    </button>
                    <div style="margin-top: 12px; font-size: 11.5px; color: var(--text-muted); text-align: center;">
                        Compatible with Desktop Sager & Android Companion App
                    </div>
                </div>

                <!-- Import Vault Card -->
                <div class="switchboard-card">
                    <div class="card-header-sm">
                        <span>📥 Import Sager Vault</span>
                        <span class="cloud-badge" style="background: rgba(56, 189, 248, 0.15); color: var(--primary);">Auto-Merge</span>
                    </div>
                    <div style="font-size: 12.5px; color: var(--text-muted); line-height: 1.5; margin-bottom: 14px;">
                        Import a previously exported vault or bundle from another PC or Android companion. Achievements are intelligently merged into your local registry without overwriting existing unlocks.
                    </div>
                    <div style="display: flex; flex-direction: column; gap: 10px;">
                        <input type="file" id="vault-import-input" accept=".json,.sager" class="form-input" style="padding: 8px;" />
                        <button class="btn" onclick="importVaultBundle()" style="width: 100%; justify-content: center; padding: 12px; background: #059669;">
                            <span>📥</span>
                            <span>Import & Merge into Local Vault</span>
                        </button>
                    </div>
                </div>

            </div>

            <!-- Android Companion & Local Home Wi-Fi Sync Card -->
            <div class="switchboard-card" style="border-color: rgba(99, 102, 241, 0.4); background: #0e1526;">
                <div class="card-header-sm">
                    <span style="color: #818cf8; font-size: 15px; font-weight: 800;">📱 Sager Android Companion & Home Wi-Fi Sync</span>
                    <span id="wifi-status-badge" class="cloud-badge" style="background: rgba(16, 185, 129, 0.15); color: var(--success);">🟢 Ready for LAN Sync</span>
                </div>

                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 16px; margin: 16px 0;">
                    
                    <div style="display: flex; flex-direction: column; gap: 10px;">
                        <div style="font-size: 13px; font-weight: 700; color: #fff;">🏠 Home Network Connection Details:</div>
                        <div style="background: rgba(0, 0, 0, 0.35); border: 1px solid var(--border); border-radius: 8px; padding: 12px; font-family: monospace; font-size: 13px;">
                            <div style="color: var(--text-muted); font-size: 11px;">Desktop Local IP (Home Wi-Fi):</div>
                            <div style="font-weight: 800; color: var(--gold); font-size: 15px;" id="lan-ip-display">Loading LAN IP...</div>
                            <div style="color: var(--text-muted); font-size: 11px; margin-top: 8px;">Mobile Sync API URL:</div>
                            <div style="color: var(--primary); font-size: 12px; word-break: break-all;" id="mobile-sync-url-display">http://.../api/mobile/sync</div>
                        </div>

                        <div style="display: flex; gap: 8px;">
                            <button class="btn btn-sm btn-gold" onclick="testMobileSyncPayload()">📱 Test Mobile Payload</button>
                            <button class="btn btn-sm" onclick="copyMobileSyncUrl()">📋 Copy Sync URL</button>
                        </div>
                    </div>

                    <div style="display: flex; flex-direction: column; gap: 8px; font-size: 12.5px; color: var(--text-muted); line-height: 1.5;">
                        <div style="font-size: 13px; font-weight: 700; color: #fff;">⚡ How It Works on the Go & at Home:</div>
                        <div>1️⃣ <strong>Show Friends Anywhere:</strong> Sager Android caches all your trophies and unlocked achievements offline in your phone. Show off your gaming feats anywhere with zero internet required.</div>
                        <div>2️⃣ <strong>Automatic Home Wi-Fi Update:</strong> When you get home and connect to Wi-Fi, tap <em>Sync with Sager PC</em> on your phone. It pulls the latest unlocks directly from this PC over your local network!</div>
                        <div>3️⃣ <strong>100% Private:</strong> Pure local LAN sync. No third-party servers, no logins, no telemetry.</div>
                    </div>

                </div>

                <div style="border-top: 1px solid rgba(255, 255, 255, 0.08); padding-top: 14px; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 10px;">
                    <div style="font-size: 12px; color: var(--text-muted);">
                        📦 <strong>Android APK Release:</strong> Standalone package available in the <a href="https://github.com/ybreakless/Sager/releases" target="_blank" style="color: var(--primary); text-decoration: underline;">GitHub Releases</a> section.
                    </div>
                    <a href="https://github.com/ybreakless/Sager/releases" target="_blank" class="btn btn-sm" style="background: #4f46e5; color: #fff;">
                        🤖 Get Android App (GitHub Release)
                    </a>
                </div>
            </div>
        </div>

        <!-- ═══════════════════════════════════════════════════════════════ -->
        <!-- TAB 7: SETTINGS                                                -->
        <!-- ═══════════════════════════════════════════════════════════════ -->
        <div id="tab-settings" class="tab-content">
            <div class="switchboard-card">
                <div class="card-header-sm">⚙️ Sager Configuration & Schedule</div>
                <form id="config-form" onsubmit="saveSettings(event)" style="display: flex; flex-direction: column; gap: 16px;">
                    
                    <div>
                        <label style="font-size: 13px; font-weight: 700; color: var(--primary);">⏰ Auto-Backup Frequency (Times per Day)</label>
                        <div style="display: flex; gap: 8px; flex-wrap: wrap; margin-top: 8px;">
                            <button type="button" class="launcher-pill" onclick="setPreset(1)">1x / Day</button>
                            <button type="button" class="launcher-pill" onclick="setPreset(6)">6x / Day</button>
                            <button type="button" class="launcher-pill" onclick="setPreset(12)">12x / Day</button>
                            <button type="button" class="launcher-pill active" id="btn-preset-24" onclick="setPreset(24)">24x / Day (Hourly) ⭐</button>
                            <button type="button" class="launcher-pill" onclick="setPreset(48)">48x / Day</button>
                        </div>
                        <div style="display: flex; align-items: center; gap: 10px; margin-top: 8px;">
                            <input type="number" id="cfg-bpd" class="form-input" min="1" max="144" value="24" oninput="updateFreqNotice()" style="width: 80px;" />
                            <span id="freq-notice" style="font-size: 12px; color: var(--gold); font-weight: 700;">⚡ Backing up 24 times a day (every 60 minutes)</span>
                        </div>
                        <input type="hidden" id="cfg-interval" value="60" />
                    </div>

                    <div>
                        <label style="font-size: 13px; font-weight: 700;">Google Drive / Cloud Backup Directory</label>
                        <input type="text" id="cfg-backup-dir" class="form-input" style="margin-top: 6px;" required />
                    </div>

                    <div>
                        <label style="font-size: 13px; font-weight: 700;">Google Drive Executable Path (Optional)</label>
                        <input type="text" id="cfg-gdrive-exe" class="form-input" placeholder="Auto-detect" style="margin-top: 6px;" />
                    </div>

                    <button type="submit" class="btn" style="align-self: flex-start; padding: 10px 24px;">💾 Save Settings</button>
                </form>
            </div>
        </div>

    </main>

    <!-- Toast Notification Banner -->
    <div id="toast" class="toast-msg"></div>

    <!-- Background Confirmation Modal -->
    <div id="bg-modal" style="display: none; position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.75); z-index: 9999; align-items: center; justify-content: center; backdrop-filter: blur(4px);">
        <div style="background: #111a2f; border: 1px solid #38bdf8; border-radius: 14px; padding: 26px; max-width: 440px; text-align: center; box-shadow: 0 16px 40px rgba(0,0,0,0.8); animation: slideUp 0.3s ease;">
            <div style="font-size: 42px; margin-bottom: 8px;">🛡️</div>
            <div style="font-size: 18px; font-weight: 800; color: #fff;">Sager is Running in the Background!</div>
            <p style="font-size: 13px; color: var(--text-muted); margin: 12px 0 20px 0; line-height: 1.5;">
                The console window has been hidden.<br>
                Sager is now operating silently in your <strong>Windows System Tray</strong> (near your clock).<br><br>
                Your game saves are automatically backed up to Google Drive and achievements will pop real-time alerts.
            </p>
            <div style="display: flex; gap: 10px; justify-content: center;">
                <button class="btn btn-gold" onclick="makeConsoleAppear()">👁️ Make Console Appear</button>
                <button class="btn" onclick="dismissBgModal()">Got It, Keep in Tray</button>
            </div>
        </div>
    </div>

    <!-- ═══════════════════════════════════════════════════════════════ -->
    <!-- JAVASCRIPT CONTROLLER                                          -->
    <!-- ═══════════════════════════════════════════════════════════════ -->
    <script>
        let allDiscoveredGames = [];
        let allAchievementSummaries = [];
        let selectedGameIndex = 0;
        let activeMatrixFilter = 'all';

        // ── Tab Navigation ──
        function switchTab(tabId) {
            document.querySelectorAll('.tab-content').forEach(el => el.classList.remove('active'));
            document.querySelectorAll('.nav-tab').forEach(el => el.classList.remove('active'));
            
            const target = document.getElementById(tabId);
            if (target) target.classList.add('active');

            document.querySelectorAll('.nav-tab').forEach(tab => {
                if (tab.getAttribute('onclick') && tab.getAttribute('onclick').includes(tabId)) {
                    tab.classList.add('active');
                }
            });

            if (tabId === 'tab-achievements') {
                loadAchievementsMatrix();
            } else if (tabId === 'tab-saves') {
                loadGames();
            } else if (tabId === 'tab-cloud') {
                loadBackups();
            } else if (tabId === 'tab-vault') {
                loadNetworkInfo();
            } else if (tabId === 'tab-test') {
                runFullDiagnostics();
            }
        }

        // ── Toast & Console ──
        function showToast(msg) {
            const t = document.getElementById('toast');
            t.textContent = msg;
            t.style.display = 'block';
            setTimeout(() => { t.style.display = 'none'; }, 4000);
        }

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
        }

        function formatBytes(bytes) {
            if (!bytes || bytes === 0) return '0 B';
            const k = 1024;
            const sizes = ['B', 'KB', 'MB', 'GB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
        }

        // ── Quick Controls: Switches & Launcher ──
        async function toggleQuickSetting(field, val) {
            try {
                const payload = {};
                payload[field] = val;
                const res = await fetch('/api/launcher/config', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(payload)
                });
                const d = await res.json();
                showToast(`✅ ${field.replace(/_/g, ' ')} updated`);
                logToConsole(`Updated setting ${field}: ${val}`, 'PASS');
            } catch (e) {
                logToConsole('Error saving setting: ' + e, 'ERROR');
            }
        }

        async function selectLauncher(launcherName) {
            document.querySelectorAll('.launcher-pill').forEach(btn => {
                btn.classList.toggle('active', btn.textContent.trim() === launcherName);
            });
            document.getElementById('active-launcher-badge').textContent = launcherName;

            try {
                await fetch('/api/launcher/config', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ preferred_launcher: launcherName })
                });
                showToast(`🚀 Active Launcher set to: ${launcherName}`);
                logToConsole(`Launcher switched to: ${launcherName}`, 'PASS');
            } catch (e) {
                console.error(e);
            }
        }

        // ── Achievements Matrix: Unlocked vs Locked ──
        async function loadAchievementsMatrix() {
            try {
                const res = await fetch('/api/achievements/overview');
                allAchievementSummaries = await res.json();

                if (allAchievementSummaries.length === 0) {
                    document.getElementById('ach-matrix-container').innerHTML = `
                        <div style="grid-column: 1/-1; text-align: center; padding: 40px; color: var(--text-muted);">
                            No achievements tracked yet. Play games or trigger a test notification!
                        </div>
                    `;
                    return;
                }

                // Populate game selector dropdown
                const select = document.getElementById('game-matrix-select');
                select.innerHTML = allAchievementSummaries.map((g, idx) => `
                    <option value="${idx}">${g.game_name} (${g.unlocked_count}/${g.total_count})</option>
                `).join('');

                renderSelectedGameMatrix();
            } catch (e) {
                console.error('Error loading achievements matrix:', e);
            }
        }

        function changeMatrixGame(idx) {
            selectedGameIndex = parseInt(idx) || 0;
            renderSelectedGameMatrix();
        }

        function renderSelectedGameMatrix() {
            const game = allAchievementSummaries[selectedGameIndex];
            if (!game) return;

            document.getElementById('matrix-game-title').textContent = game.game_name;
            document.getElementById('matrix-game-meta').textContent = `${game.launcher} • AppID: ${game.app_id || 'N/A'}`;
            document.getElementById('matrix-percent-label').textContent = `${game.percentage.toFixed(1)}%`;
            document.getElementById('matrix-fraction-label').textContent = `${game.unlocked_count} of ${game.total_count} Unlocked`;
            document.getElementById('matrix-progress-fill').style.width = `${game.percentage}%`;

            document.getElementById('count-all').textContent = game.total_count;
            document.getElementById('count-unlocked').textContent = game.unlocked_count;
            document.getElementById('count-locked').textContent = game.locked_count;

            applyMatrixSearch();
        }

        function filterMatrixView(filter) {
            activeMatrixFilter = filter;
            document.querySelectorAll('.filter-pill').forEach(p => p.classList.remove('active'));
            const activeBtn = document.getElementById(`pill-filter-${filter}`);
            if (activeBtn) activeBtn.classList.add('active');
            applyMatrixSearch();
        }

        function applyMatrixSearch() {
            const game = allAchievementSummaries[selectedGameIndex];
            if (!game) return;

            const query = (document.getElementById('ach-matrix-search').value || '').toLowerCase().trim();
            const container = document.getElementById('ach-matrix-container');

            let pool = [];
            if (activeMatrixFilter === 'all') {
                pool = [...game.unlocked, ...game.locked];
            } else if (activeMatrixFilter === 'unlocked') {
                pool = game.unlocked;
            } else {
                pool = game.locked;
            }

            if (query) {
                pool = pool.filter(a => 
                    a.title.toLowerCase().includes(query) || 
                    a.description.toLowerCase().includes(query) ||
                    a.id.toLowerCase().includes(query)
                );
            }

            if (pool.length === 0) {
                container.innerHTML = `<div style="grid-column: 1/-1; text-align: center; padding: 30px; color: var(--text-muted);">No matching achievements found.</div>`;
                return;
            }

            container.innerHTML = pool.map(a => {
                const isUnlocked = a.unlocked;
                const timeStr = a.unlock_time ? new Date(a.unlock_time * 1000).toLocaleString() : '';
                const safeTitle = (a.title || '').replace(/'/g, "\\'");
                const safeGame = (a.game_name || game.game_name || '').replace(/'/g, "\\'");
                const safeDesc = (a.description || '').replace(/'/g, "\\'");
                return `
                    <div class="matrix-card ${isUnlocked ? 'unlocked' : 'locked'}">
                        <div class="ach-card-icon">${isUnlocked ? '🏆' : '🔒'}</div>
                        <div class="ach-card-content">
                            <div class="ach-card-title">
                                <span>${a.title}</span>
                                <span style="font-size: 11px; font-weight: 700; color: ${isUnlocked ? 'var(--success)' : '#64748b'};">
                                    ${isUnlocked ? 'UNLOCKED' : 'LOCKED'}
                                </span>
                            </div>
                            <div class="ach-card-desc">${a.description || 'Secret achievement objective'}</div>
                            <div class="ach-card-footer" style="display: flex; justify-content: space-between; align-items: center; gap: 8px;">
                                <span>${isUnlocked ? `Earned: ${timeStr}` : 'Objective not yet completed'}</span>
                                <div style="display: flex; gap: 6px; align-items: center;">
                                    ${isUnlocked ? `<button class="btn btn-sm btn-gold" style="padding: 2px 8px; font-size: 11px;" onclick="replaySteamAlert('${safeTitle}', '${safeGame}', '${safeDesc}')">🔔 Pop Overlay</button>` : ''}
                                    <span class="ach-launcher-badge">${a.launcher || game.launcher}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                `;
            }).join('');
        }

        // ── Game Saves & Backups ──
        async function loadGames() {
            try {
                const res = await fetch('/api/games');
                allDiscoveredGames = await res.json();
                renderGamesTable(allDiscoveredGames);
                renderQuickGames(allDiscoveredGames);
            } catch (e) {
                console.error(e);
            }
        }

        function renderQuickGames(games) {
            document.getElementById('quick-games-count').textContent = games.length;
            const container = document.getElementById('quick-games-list');
            if (games.length === 0) {
                container.innerHTML = '<div style="font-size: 12px; color: var(--text-muted); text-align: center; padding: 10px;">Scanning games...</div>';
                return;
            }
            container.innerHTML = games.slice(0, 5).map(g => `
                <div style="display: flex; justify-content: space-between; align-items: center; padding: 8px 12px; background: rgba(255,255,255,0.02); border-radius: 8px; border: 1px solid var(--border);">
                    <div>
                        <div style="font-size: 13px; font-weight: 700; color: var(--text-main);">${g.name}</div>
                        <div style="font-size: 11px; color: var(--text-muted);">${g.save_category} • ${formatBytes(g.total_size_bytes)}</div>
                    </div>
                    <button class="btn btn-sm" onclick="backupSingleGame('${encodeURIComponent(g.name)}', '${encodeURIComponent(g.source_path)}')">💾 Backup</button>
                </div>
            `).join('');
        }

        function filterGames() {
            const q = (document.getElementById('game-search').value || '').toLowerCase();
            const filtered = allDiscoveredGames.filter(g => 
                g.name.toLowerCase().includes(q) || 
                g.save_category.toLowerCase().includes(q) ||
                g.source_path.toLowerCase().includes(q)
            );
            renderGamesTable(filtered);
        }

        function renderGamesTable(games) {
            const tbody = document.getElementById('games-table-body');
            if (games.length === 0) {
                tbody.innerHTML = '<tr><td colspan="6" style="text-align:center; color: var(--text-muted);">No games found.</td></tr>';
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

        async function loadBackups() {
            const tbody = document.getElementById('backups-table-body');
            tbody.innerHTML = '<tr><td colspan="6" style="text-align:center;">Loading cloud archives...</td></tr>';
            try {
                const res = await fetch('/api/backups');
                const data = await res.json();
                let rows = '';
                let total = 0;

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
                                    <button class="btn btn-gold btn-sm" onclick="restoreBackup('${encodeURIComponent(arc.full_path)}')">🔄 Restore</button>
                                </td>
                            </tr>
                        `;
                    });
                });

                tbody.innerHTML = total === 0 ? '<tr><td colspan="6" style="text-align:center;">No backups found yet. Click Backup All Saves!</td></tr>' : rows;
            } catch (e) {
                tbody.innerHTML = '<tr><td colspan="6" style="color:var(--danger);">Error loading backups</td></tr>';
            }
        }

        async function backupAllGames() {
            showToast('⏳ Backing up all game saves to Google Drive...');
            logToConsole('Initiating backup for all discovered game saves...', 'INFO');
            try {
                const res = await fetch('/api/backup', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ all: true })
                });
                const d = await res.json();
                showToast('✅ ' + d.message);
                logToConsole(d.message, 'PASS');
                loadBackups();
                loadGames();
            } catch (e) {
                logToConsole('Backup error: ' + e, 'ERROR');
            }
        }

        async function backupSingleGame(name, path) {
            const decodedName = decodeURIComponent(name);
            showToast('⏳ Backing up ' + decodedName + '...');
            try {
                const res = await fetch('/api/backup', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        game_name: decodedName,
                        source_path: decodeURIComponent(path)
                    })
                });
                const d = await res.json();
                showToast('✅ ' + d.message);
                logToConsole(d.message, 'PASS');
                loadBackups();
            } catch (e) {
                console.error(e);
            }
        }

        async function restoreBackup(zipPath) {
            const decoded = decodeURIComponent(zipPath);
            if (!confirm('Are you sure you want to restore this save? (A safety backup of your existing local save will be created automatically)')) return;

            showToast('🔄 Restoring save archive...');
            logToConsole(`Restoring backup: ${decoded}`, 'INFO');
            try {
                const res = await fetch('/api/restore', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ zip_path: decoded })
                });
                const d = await res.json();
                if (d.success) {
                    showToast('✅ ' + d.message);
                    logToConsole(d.message, 'PASS');
                } else {
                    showToast('❌ ' + d.message);
                    logToConsole(d.message, 'ERROR');
                }
            } catch (e) {
                logToConsole('Restore failed: ' + e, 'ERROR');
            }
        }

        // ── Google Drive & Status ──
        async function loadDriveStatus() {
            try {
                const res = await fetch('/api/drive-status');
                const d = await res.json();
                const badge = document.getElementById('gdrive-pill-badge');
                const pathEl = document.getElementById('gdrive-status-path');
                const actionContainer = document.getElementById('gdrive-action-container');

                pathEl.textContent = `Target: ${d.backup_dir}`;

                if (d.google_drive_running) {
                    badge.textContent = 'Running';
                    badge.style.backgroundColor = 'rgba(16, 185, 129, 0.2)';
                    badge.style.color = 'var(--success)';
                    actionContainer.innerHTML = `
                        <span class="cloud-badge" style="background: rgba(16, 185, 129, 0.2); color: var(--success);">Running</span>
                        <button class="cloud-btn" onclick="openBackupFolder()">📂 Open</button>
                    `;
                } else if (d.google_drive_installed) {
                    badge.textContent = 'Stopped';
                    badge.style.backgroundColor = 'rgba(239, 68, 68, 0.2)';
                    badge.style.color = 'var(--danger)';
                    actionContainer.innerHTML = `
                        <button class="btn btn-sm btn-gold" onclick="launchGoogleDrive()">🚀 Launch Drive</button>
                        <button class="cloud-btn" onclick="openBackupFolder()">📂 Open</button>
                    `;
                } else {
                    badge.textContent = 'Not Installed';
                    badge.style.backgroundColor = 'rgba(148, 163, 184, 0.2)';
                    badge.style.color = 'var(--text-muted)';
                }
            } catch (e) { console.error(e); }
        }

        async function launchGoogleDrive() {
            showToast('🚀 Launching Google Drive for Desktop...');
            try {
                const res = await fetch('/api/launch-gdrive', { method: 'POST' });
                const d = await res.json();
                showToast(d.message);
                logToConsole(d.message, d.success ? 'PASS' : 'WARN');
                setTimeout(loadDriveStatus, 3000);
            } catch (e) {
                console.error(e);
            }
        }

        async function openBackupFolder() {
            try {
                await fetch('/api/open-backup-folder', { method: 'POST' });
            } catch (e) { console.error(e); }
        }

        // ── Interactive Tests ──
        async function testSound() {
            try {
                const audio = document.getElementById('steam-chime');
                if (audio) {
                    audio.currentTime = 0;
                    audio.play().catch(e => console.log('Audio autoplay prevented:', e));
                }
                await fetch('/api/test-sound', { method: 'POST' });
                showToast('🔔 Steam achievement sound played (deck_ui_achievement_toast.wav)!');
                logToConsole('Played authentic Steam achievement unlock chime (deck_ui_achievement_toast.wav)', 'PASS');
            } catch (e) { console.error(e); }
        }

        async function triggerTestUnlock() {
            try {
                const audio = document.getElementById('steam-chime');
                if (audio) {
                    audio.currentTime = 0;
                    audio.play().catch(e => console.log('Audio autoplay prevented:', e));
                }
                const res = await fetch('/api/achievements/test', { method: 'POST' });
                const d = await res.json();
                showToast('🔔 ' + d.message);
                logToConsole('Triggered Steam In-Game TopMost Overlay & Chime', 'PASS');
                setTimeout(loadAchievementsMatrix, 500);
            } catch (e) { console.error(e); }
        }

        async function replaySteamAlert(title, game_name, description) {
            try {
                const audio = document.getElementById('steam-chime');
                if (audio) {
                    audio.currentTime = 0;
                    audio.play().catch(e => console.log('Audio autoplay prevented:', e));
                }
                await fetch('/api/achievements/replay-overlay', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ title, game_name, description })
                });
                showToast(`🔔 Steam in-game overlay popped for '${title}'!`);
                logToConsole(`Displayed Steam in-game topmost overlay & chime for '${title}' in ${game_name}`, 'PASS');
            } catch (e) { console.error(e); }
        }

        async function openCurrentGameInSteam() {
            const game = allAchievementSummaries[selectedGameIndex];
            if (!game || !game.app_id || game.app_id === '0') {
                showToast('⚠️ No Steam AppID associated with this game');
                return;
            }
            try {
                await fetch('/api/achievements/open-steam', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ app_id: game.app_id })
                });
                showToast(`🚀 Opening Steam Community achievements for AppID ${game.app_id}...`);
                logToConsole(`Launching Steam achievement interface: steam://open/achievements/${game.app_id}`, 'INFO');
            } catch (e) { console.error(e); }
        }

        async function simulateCustomAchievement() {
            const game = document.getElementById('sim-game').value.trim() || 'LEGO Batman: Legacy of the Dark Knight';
            const title = document.getElementById('sim-title').value.trim() || 'Gotham Guardian';
            const desc = document.getElementById('sim-desc').value.trim() || 'Defeated all villains in Gotham City';

            try {
                const audio = document.getElementById('steam-chime');
                if (audio) {
                    audio.currentTime = 0;
                    audio.play().catch(e => console.log('Audio autoplay prevented:', e));
                }
                const res = await fetch('/api/test/custom-achievement', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ game_name: game, title, description: desc, app_id: '2215200' })
                });
                const d = await res.json();
                showToast('🔔 ' + d.message);
                logToConsole(`Steam in-game overlay popped: "${title}" in "${game}"`, 'PASS');
                setTimeout(loadAchievementsMatrix, 500);
            } catch (e) { console.error(e); }
        }

        async function runWriteSpeedTest() {
            logToConsole('Testing Google Drive write latency...', 'INFO');
            try {
                const res = await fetch('/api/test/write-test', { method: 'POST' });
                const d = await res.json();
                showToast('⚡ ' + d.message);
                logToConsole(`Write benchmark PASSED in ${d.latency_ms.toFixed(2)} ms on ${d.path}`, 'PASS');
            } catch (e) { console.error(e); }
        }

        async function runFullDiagnostics() {
            logToConsole('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━', 'INFO');
            logToConsole('Running full Sager diagnostic health scan...', 'INFO');
            try {
                const res = await fetch('/api/test/diagnostics');
                const d = await res.json();
                logToConsole(`[Google Drive] Status: ${d.google_drive.running ? 'Running' : 'Stopped'} | Latency: ${d.google_drive.write_latency_ms.toFixed(2)} ms`, 'PASS');
                logToConsole(`[Save Scanner] Discovered ${d.save_scanner.games_count} games with saves (${formatBytes(d.save_scanner.total_size_bytes)})`, 'PASS');
                logToConsole(`[Watcher] Goldberg: ${d.watcher.goldberg_detected} | Steam: ${d.watcher.steam_detected}`, 'PASS');
                logToConsole(`[Cloud Archives] ${d.cloud_backups.total_archives} zip snapshots ready in Google Drive`, 'PASS');
                logToConsole('All diagnostics completed successfully!', 'SUCCESS');
                logToConsole('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━', 'INFO');
            } catch (e) { console.error(e); }
        }

        // ── Config & Settings ──
        async function loadConfig() {
            try {
                const res = await fetch('/api/config');
                const cfg = await res.json();

                document.getElementById('switch-auto-backup').checked = cfg.auto_backup_enabled;
                document.getElementById('switch-sound').checked = cfg.sound_enabled;
                document.getElementById('switch-notif').checked = cfg.notifications_enabled;
                document.getElementById('switch-launcher-sync').checked = cfg.show_in_launcher !== false;

                const launcher = cfg.preferred_launcher || 'Goldberg SteamEmu';
                document.querySelectorAll('.launcher-pill').forEach(btn => {
                    btn.classList.toggle('active', btn.textContent.trim() === launcher);
                });
                document.getElementById('active-launcher-badge').textContent = launcher;

                document.getElementById('cfg-backup-dir').value = cfg.backup_directory;
                document.getElementById('cfg-gdrive-exe').value = cfg.google_drive_exe || '';
                const bpd = cfg.backups_per_day || 24;
                document.getElementById('cfg-bpd').value = bpd;
                updateFreqNotice();
            } catch (e) { console.error(e); }
        }

        function setPreset(times) {
            document.getElementById('cfg-bpd').value = times;
            updateFreqNotice();
        }

        function updateFreqNotice() {
            let times = parseInt(document.getElementById('cfg-bpd').value) || 24;
            times = Math.max(1, Math.min(144, times));
            const minutes = Math.max(1, Math.round(1440 / times));
            document.getElementById('cfg-interval').value = minutes;
            const hours = (minutes / 60).toFixed(1).replace('.0', '');
            const text = minutes >= 60 ? `every ${hours} hour${hours !== '1' ? 's' : ''}` : `every ${minutes} minutes`;
            document.getElementById('freq-notice').textContent = `⚡ Backing up ${times} times a day (${text})`;
            document.getElementById('next-backup-notice').textContent = `⏱️ Auto-Save: ${times}x / day (${text})`;
        }

        async function saveSettings(e) {
            e.preventDefault();
            const bpd = parseInt(document.getElementById('cfg-bpd').value) || 24;
            const interval = Math.max(1, Math.round(1440 / bpd));
            const payload = {
                backup_directory: document.getElementById('cfg-backup-dir').value,
                auto_backup_interval_minutes: interval,
                backups_per_day: bpd,
                google_drive_exe: document.getElementById('cfg-gdrive-exe').value.trim() || null
            };

            try {
                const res = await fetch('/api/config', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(payload)
                });
                showToast(`✅ Saved! Backing up ${bpd}x a day to Google Drive.`);
                logToConsole(`Configuration saved. Target: ${payload.backup_directory}`, 'PASS');
                loadDriveStatus();
            } catch (e) { console.error(e); }
        }

        // ── Background & Visibility Handlers ──
        async function runInBackground() {
            showToast('🛡️ Sending Sager to background tray...');
            logToConsole('Requesting background silent execution (hiding console window)...', 'INFO');
            try {
                const res = await fetch('/api/system/run-background', { method: 'POST' });
                const d = await res.json();
                logToConsole(d.message, 'PASS');
                const modal = document.getElementById('bg-modal');
                if (modal) modal.style.display = 'flex';
                updateConsoleVisibilityBadge(false);
            } catch (e) {
                logToConsole('Error sending to background: ' + e, 'ERROR');
            }
        }

        async function makeConsoleAppear() {
            showToast('👁️ Restoring console window to desktop...');
            logToConsole('Requesting console window restore to foreground...', 'INFO');
            try {
                const res = await fetch('/api/system/make-appear', { method: 'POST' });
                const d = await res.json();
                showToast('✅ ' + d.message);
                logToConsole(d.message, 'PASS');
                dismissBgModal();
                updateConsoleVisibilityBadge(true);
            } catch (e) {
                logToConsole('Error restoring console: ' + e, 'ERROR');
            }
        }

        async function toggleWindowsStartup() {
            try {
                const res = await fetch('/api/system/toggle-startup', { method: 'POST' });
                const d = await res.json();
                showToast(d.message);
                logToConsole(d.message, d.installed ? 'PASS' : 'WARN');
                updateStartupButton(d.installed);
            } catch (e) {
                console.error(e);
            }
        }

        function dismissBgModal() {
            const modal = document.getElementById('bg-modal');
            if (modal) modal.style.display = 'none';
        }

        function updateStartupButton(installed) {
            const btn = document.getElementById('btn-toggle-startup');
            if (!btn) return;
            if (installed) {
                btn.textContent = '● Active (Boot ON)';
                btn.style.background = 'rgba(16, 185, 129, 0.15)';
                btn.style.color = 'var(--success)';
                btn.style.borderColor = 'rgba(16, 185, 129, 0.4)';
            } else {
                btn.textContent = '○ Disabled';
                btn.style.background = 'rgba(239, 68, 68, 0.15)';
                btn.style.color = 'var(--danger)';
                btn.style.borderColor = 'rgba(239, 68, 68, 0.4)';
            }
        }

        function updateConsoleVisibilityBadge(visible) {
            const badge = document.getElementById('console-live-badge');
            if (!badge) return;
            if (visible) {
                badge.textContent = 'Console: Visible';
                badge.style.background = 'rgba(251, 191, 36, 0.15)';
                badge.style.color = 'var(--gold)';
            } else {
                badge.textContent = 'Tray: Silent Background';
                badge.style.background = 'rgba(56, 189, 248, 0.15)';
                badge.style.color = 'var(--primary)';
            }
        }

        async function loadSystemStatus() {
            try {
                const res = await fetch('/api/status');
                const d = await res.json();
                if (d.startup_installed !== undefined) {
                    updateStartupButton(d.startup_installed);
                }
                if (d.console_visible !== undefined) {
                    updateConsoleVisibilityBadge(d.console_visible);
                }
            } catch (e) {
                console.error(e);
            }
        }

        // ── Vault Export & Import / Mobile Sync Handlers ──
        let cachedMobileSyncUrl = '';

        async function loadNetworkInfo() {
            try {
                const res = await fetch('/api/network/info');
                const d = await res.json();
                cachedMobileSyncUrl = d.mobile_sync_url;
                const ipEl = document.getElementById('lan-ip-display');
                if (ipEl) ipEl.textContent = `${d.local_ip}:${d.port} (${d.pc_name})`;
                const urlEl = document.getElementById('mobile-sync-url-display');
                if (urlEl) urlEl.textContent = d.mobile_sync_url;
            } catch (e) {
                console.error('Error loading network info:', e);
            }
        }

        function exportVaultBundle() {
            showToast('📦 Preparing Sager Vault export bundle...');
            logToConsole('Exporting full Sager Vault bundle (.json)...', 'INFO');
            window.location.href = '/api/vault/export';
        }

        async function importVaultBundle() {
            const input = document.getElementById('vault-import-input');
            if (!input || !input.files || input.files.length === 0) {
                showToast('⚠️ Please select a .json or .sager vault file to import');
                return;
            }
            const file = input.files[0];
            showToast('📥 Reading and verifying vault file: ' + file.name);
            logToConsole(`Reading vault bundle: ${file.name} (${file.size} bytes)...`, 'INFO');

            try {
                const text = await file.text();
                const res = await fetch('/api/vault/import', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: text
                });
                const d = await res.json();
                if (d.success) {
                    showToast('✅ ' + d.message);
                    logToConsole(d.message, 'PASS');
                    loadAchievementsMatrix();
                } else {
                    showToast('❌ Import error: ' + d.message);
                    logToConsole('Import failed: ' + d.message, 'ERROR');
                }
            } catch (e) {
                showToast('❌ Error parsing vault file: ' + e);
                logToConsole('Error importing vault: ' + e, 'ERROR');
            }
        }

        async function testMobileSyncPayload() {
            showToast('📱 Fetching Mobile LAN Sync payload...');
            logToConsole('Testing GET /api/mobile/sync (payload sent to Android companion)...', 'INFO');
            try {
                const res = await fetch('/api/mobile/sync');
                const d = await res.json();
                logToConsole(`Mobile Sync OK: ${d.unlocked_achievements_count} trophies across ${d.total_games} games`, 'PASS');
                showToast(`✅ Android payload verified: ${d.unlocked_achievements_count} trophies ready for phone`);
            } catch (e) {
                logToConsole('Mobile sync test failed: ' + e, 'ERROR');
            }
        }

        function copyMobileSyncUrl() {
            if (!cachedMobileSyncUrl) {
                showToast('⚠️ Network URL not ready yet');
                return;
            }
            navigator.clipboard.writeText(cachedMobileSyncUrl).then(() => {
                showToast('📋 Copied Mobile Sync URL to clipboard!');
            }).catch(() => {
                showToast('📋 Mobile Sync URL: ' + cachedMobileSyncUrl);
            });
        }

        // ── Initialization ──
        window.addEventListener('DOMContentLoaded', () => {
            loadConfig();
            loadDriveStatus();
            loadGames();
            loadAchievementsMatrix();
            loadSystemStatus();
            loadNetworkInfo();
        });
    </script>
    <audio id="steam-chime" preload="auto" src="/api/sound/steam-achievement.wav"></audio>
</body>
</html>
"#
}
