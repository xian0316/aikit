// ═══ AIKit 前端 v2 ═══
const { invoke } = window.__TAURI__?.core ?? {};
const { save: dialogSave, open: dialogOpen } = window.__TAURI__?.dialog ?? {};

// ─── Tabler Icons (inline SVG, zero dependency) ───
const ICONS = {
  package:      '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 3l8 4.5v9L12 21l-8-4.5v-9zm0 9l8-4.5M12 12v9m0-9L4 7.5m12-2.25l-8 4.5"/></svg>',
  check:        '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0-18 0"/><path d="m9 12l2 2l4-4"/></svg>',
  globe:        '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0-18 0m.6-3h16.8M3.6 15h16.8"/><path d="M11.5 3a17 17 0 0 0 0 18m1-18a17 17 0 0 1 0 18"/></svg>',
  settings:     '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 0 0-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 0 0-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 0 0-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 0 0-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 0 0 1.066-2.573c-.94-1.543.826-3.31 2.37-2.37c1 .608 2.296.07 2.572-1.065"/><path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0-6 0"/></svg>',
  search:       '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0-14 0m18 11l-6-6"/></svg>',
  plus:         '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14m-7-7h14"/></svg>',
  arrowLeft:    '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14M5 12l6 6m-6-6l6-6"/></svg>',
  refresh:      '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 11A8.1 8.1 0 0 0 4.5 9M4 5v4h4m-4 4a8.1 8.1 0 0 0 15.5 2m.5 4v-4h-4"/></svg>',
  download:     '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 17v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2M7 11l5 5l5-5m-5-7v12"/></svg>',
  upload:       '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 17v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2M7 9l5-5l5 5m-5-5v12"/></svg>',
  trash:        '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 7h16m-10 4v6m4-6v6M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2l1-12M9 7V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v3"/></svg>',
  alertTriangle: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 9v4m-1.637-9.409L2.257 17.125a1.914 1.914 0 0 0 1.636 2.871h16.214a1.914 1.914 0 0 0 1.636-2.87L13.637 3.59a1.914 1.914 0 0 0-3.274 0M12 16h.01"/></svg>',
  folderOpen:   '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/></svg>',
  inbox:        '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2z"/><path d="M4 13h3l3 3h4l3-3h3"/></svg>',
  loader:       '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3a9 9 0 1 0 9 9"/></svg>',
  helpCircle:   '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0-18 0m9 5v.01"/><path d="M12 13.5a1.5 1.5 0 0 1 1-1.5a2.6 2.6 0 1 0-3-4"/></svg>',
  fileText:     '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 3v4a1 1 0 0 0 1 1h4"/><path d="M17 21H7a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7l5 5v11a2 2 0 0 1-2 2M9 9h1m-1 4h6m-6 4h6"/></svg>',
  moodEmpty:    '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0-18 0m6-2h.01M15 10h.01M9 15h6"/></svg>',
  externalLink: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 6H6a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-6"/><path d="M11 13l9-9"/><path d="M15 4h5v5"/></svg>',
  refresh:      '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 11a8.1 8.1 0 0 0-15.5-2"/><path d="M4 4v5h5"/><path d="M4 13a8.1 8.1 0 0 0 15.5 2"/><path d="M20 20v-5h-5"/></svg>',
  rocket:       '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91 0z"/><path d="M12 15l-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/></svg>',
};

function icon(name, size) {
  const svg = ICONS[name] || '';
  if (size) return svg.replace(/width="\d+"/, `width="${size}"`).replace(/height="\d+"/, `height="${size}"`);
  return svg;
}

function repoUrl(owner, name) {
  return `https://github.com/${owner}/${name}`;
}

async function openExternal(url) {
  try {
    const shell = window.__TAURI__?.shell;
    if (shell?.open) { await shell.open(url); return; }
  } catch (e) { /* fallthrough */ }
  window.open(url, '_blank');
}

const api = {
  repo: {
    list: ()                      => invoke('repo_list'),
    add: (url, branch, subdir)    => invoke('repo_add', { url, branch, subdir }),
    del: (repoId)                 => invoke('repo_delete', { repoId }),
    pull: (repoId)                => invoke('repo_pull', { repoId }),
  },
  skills: {
    scan: (repoId)                => invoke('skills_scan', { repoId }),
    install: (skill)              => invoke('skills_install', { skill }),
    toggleApp: (skillName, toolId, enabled) => invoke('skills_toggle_app', { skillName, toolId, enabled }),
    uninstall: (skillName)        => invoke('skills_uninstall', { skillName }),
    checkUpdates: ()              => invoke('skills_check_updates'),
    applyUpdate: (dirName, repoId)=> invoke('skills_apply_update', { dirName, repoId }),
  },
  installed: { list: () => invoke('installed_list') },
  tools:     {
    list: ()                          => invoke('tools_list'),
    checkInstalled: (toolId)          => invoke('tools_check_installed', { toolId }),
  },
  settings:  {
    get: ()           => invoke('settings_get'),
    set: (partial)    => invoke('settings_set', { partial }),
  },
  logs: {
    read: (limit, level) => invoke('logs_read', { limit, level }),
    clear: ()            => invoke('logs_clear'),
  },
  dir: {
    open: (dirPath) => invoke('dir_open', { dirPath }),
    list: ()        => invoke('dir_list'),
    size: (dirPath) => invoke('dir_size', { dirPath }),
  },
  config: {
    export: (filePath) => invoke('config_export', { filePath }),
    import: (filePath) => invoke('config_import', { filePath }),
  },
  skillsSh: {
    search:   (q)       => invoke('skills_sh_search', { query: q }),
    trending: ()        => invoke('skills_sh_trending'),
  },
};

const state = {
  repos: [],
  skills: [],
  installed: [],
  tools: [],
  settings: {},
  view: 'my-skills',
  tab: 'repos',     // discover 页的子页签: repos | skills-sh
  currentRepoId: null,
  search: '',
  updates: [],
  appVersion: '',
};

// ═══ 导航结构 ═══
const NAV = [
  { group: '技能库', items: [
    { id: 'my-skills', icon: 'check',  label: '我的技能' },
    { id: 'discover',  icon: 'package', label: '发现' },
  ]},
  { group: '管理', items: [
    { id: 'settings',  icon: 'settings', label: '设置' },
  ]},
];

const App = {
  async init() {
    if (!invoke) {
      document.getElementById('content').innerHTML =
        '<div class="empty-state"><div class="icon">' + icon('alertTriangle', 40) + '</div><p>请在 Tauri 环境中运行</p></div>';
      return;
    }
    try {
      state.repos     = await api.repo.list();
      state.installed = await api.installed.list();
      state.tools     = await api.tools.list();
      state.settings  = await api.settings.get();
      try { state.appVersion = await window.__TAURI__.app.getVersion(); } catch {}
      this.renderSidebar();
      this.renderToolbar();
      this.renderContent();
      this.renderRepoCount();
    } catch (e) {
      this.toast('初始化失败: ' + e, 'error');
    }
  },

  // ═══ 侧边栏 ═══
  renderSidebar() {
    const nav = document.getElementById('sidebar-nav');
    nav.innerHTML = NAV.map(section => `
      <div class="nav-group">
        <div class="nav-group-label muted">${section.group}</div>
        ${section.items.map(item => `
          <button class="nav-item ${state.view === item.id ? 'active' : ''}"
            data-view="${item.id}" onclick="App.switchView('${item.id}')">
            <span class="nav-icon">${icon(item.icon, 16)}</span><span>${item.label}</span>
          </button>`).join('')}
      </div>`).join('');
  },

  switchView(view) {
    state.view = view;
    state.currentRepoId = null;
    state.search = '';
    this.renderSidebar();
    this.renderToolbar();
    this.renderContent();
  },

  switchTab(tab) {
    state.tab = tab;
    state.currentRepoId = null;
    state.search = '';
    this.renderToolbar();
    this.renderContent();
  },

  // ═══ 工具栏 ═══
  renderToolbar() {
    const tb = document.getElementById('toolbar');
    const searchHTML = `<input type="text" id="search-input" placeholder="搜索..." value="${state.search}"
      oninput="App.onSearch(this.value)" style="flex:1;max-width:350px">`;

    if (state.view === 'my-skills') {
      tb.innerHTML = `<h3>我的技能</h3>${searchHTML}<div class="toolbar-spacer"></div>
        <button class="btn btn-sm btn-outline" onclick="App.checkUpdates()">检查更新</button>`;
    } else if (state.view === 'discover') {
      if (state.currentRepoId) {
        // 仓库详情
        const repo = state.repos.find(r => r.id === state.currentRepoId);
        tb.innerHTML = `<button class="btn btn-outline btn-sm" onclick="App.backToRepos()">${icon('arrowLeft', 14)} 返回</button>
          <h3 style="margin:0">${repo ? this.esc(repo.owner+'/'+repo.name) : ''}</h3>
          ${repo ? `<button class="icon-btn" title="在 GitHub 中打开" onclick="openExternal('${repoUrl(repo.owner, repo.name)}')">${icon('externalLink', 16)}</button>` : ''}
          <div class="toolbar-spacer"></div>${searchHTML}
          <button class="btn btn-sm btn-outline" onclick="App.refreshSkills()">刷新</button>`;
      } else {
        // 发现页：tab 切换 + 搜索 + 仓库管理
        const tabBtns = `<div class="flex gap-sm">
          <button class="btn btn-sm ${state.tab==='repos'?'':'btn-outline'}" onclick="App.switchTab('repos')">仓库</button>
          <button class="btn btn-sm ${state.tab==='skills-sh'?'':'btn-outline'}" onclick="App.switchTab('skills-sh')">skills.sh</button>
        </div>`;
        tb.innerHTML = `${tabBtns}${searchHTML}<div class="toolbar-spacer"></div>
          <button class="btn btn-sm btn-outline" onclick="App.showAddRepo()">仓库管理</button>`;
      }
    } else if (state.view === 'settings') {
      tb.innerHTML = `<h3>设置</h3>`;
    }
  },

  // ═══ 内容区 ═══
  renderContent() {
    const c = document.getElementById('content');
    switch (state.view) {
      case 'my-skills':   return this.renderMySkills(c);
      case 'discover':
        if (state.currentRepoId) return this.renderSkillsView(c);
        if (state.tab === 'skills-sh') return this.renderSkillsSh(c);
        return this.renderReposView(c);
      case 'settings':    return this.renderSettings(c);
    }
  },

  // ─── 我的技能（已安装 + 工具开关）───
  renderMySkills(c) {
    let list = state.installed;
    if (state.search) {
      const q = state.search.toLowerCase();
      list = list.filter(i => (i.skillName||'').toLowerCase().includes(q));
    }
    if (list.length === 0) {
      c.innerHTML = `<div class="empty-state"><div class="icon">${icon('moodEmpty', 40)}</div>
        <p>${state.search ? '没有匹配' : '还没有安装任何技能，去仓库浏览安装'}</p></div>`;
      return;
    }

    c.innerHTML = list.map(i => {
      const apps = i.apps || {};
      const enabledCount = Object.values(apps).filter(Boolean).length;
      const hasUpdate = state.updates.some(u => u.dirName === i.dirName);
      // 工具开关：卡片样式
      const toggles = state.tools.map(t => {
        const on = apps[t.id] === true;
        const toolInstalled = t.installed !== false;
        if (!toolInstalled) {
          return `<span class="toggle-btn disabled" title="未安装">${t.name}</span>`;
        }
        return `<button class="toggle-btn ${on ? 'on' : ''}" onclick="App.toggleApp('${this.esc(i.dirName)}','${t.id}',${!on})">${t.name}</button>`;
      }).join(' ');

      return `<div class="repo-item ${hasUpdate ? 'has-update' : ''}" style="flex-direction:column;align-items:stretch;gap:var(--sp-3);padding:var(--sp-4) var(--sp-5)">
        <div class="flex" style="justify-content:space-between">
          <div>
            <div class="repo-name">${this.esc(i.skillName)}${hasUpdate ? '<span class="tag update">可更新</span>' : ''}</div>
            <div class="muted" style="margin-top:2px">${enabledCount > 0 ? enabledCount + ' 个工具已启用' : '未启用任何工具'}</div>
          </div>
          <div class="flex gap-sm">
            ${hasUpdate ? `<button class="btn btn-sm btn-update" onclick="App.applyUpdate('${this.esc(i.dirName)}')">${icon('download', 14)} 更新</button>` : ''}
            <button class="btn btn-sm btn-red" onclick="App.uninstallSkill('${this.esc(i.dirName)}')">移除</button>
          </div>
        </div>
        <div class="flex gap-sm" style="flex-wrap:wrap">${toggles}</div>
      </div>`;
    }).join('');
    this.updateStatus(`${list.length} 个已安装${state.updates.length ? ' | ' + state.updates.length + ' 个可更新' : ''}`);
  },

  // ─── 仓库列表 ───
  async renderReposView(c) {
    let repos = state.repos;
    if (state.search) {
      const q = state.search.toLowerCase();
      repos = repos.filter(r => (r.owner+'/'+r.name).toLowerCase().includes(q));
    }
    if (repos.length === 0) {
      c.innerHTML = `<div class="empty-state"><div class="icon">${icon('package', 40)}</div>
        <p>${state.search ? '没有匹配' : '还没有添加仓库，去仓库管理添加'}</p></div>`;
      return;
    }
    c.innerHTML = `<div class="card-grid">${repos.map(r => `
      <div class="skill-card repo-card" onclick="App.enterRepo('${r.id}')" style="cursor:pointer">
        <div class="card-row">
          <div class="name">${this.esc(r.owner)}/${this.esc(r.name)}</div>
          <button class="icon-btn" title="在 GitHub 中打开" onclick="event.stopPropagation();openExternal('${repoUrl(r.owner, r.name)}')">${icon('externalLink', 16)}</button>
        </div>
        <div class="desc muted">分支: ${this.esc(r.branch)}</div>
        <div class="meta"><span class="tag" id="count-${r.id}">${icon('loader', 14)} 加载中</span></div>
      </div>`).join('')}</div>`;
    this.updateStatus(`${repos.length} 个仓库`);

    for (const r of repos) {
      try {
        const result = await api.skills.scan(r.id);
        const count = (result.skills||[]).length;
        const el = document.getElementById('count-'+r.id);
        if (el) el.textContent = `${count} 个技能`;
      } catch {
        const el = document.getElementById('count-'+r.id);
        if (el) el.innerHTML = icon('alertTriangle', 14);
      }
    }
  },

  // ─── 进入仓库看技能 ───
  async enterRepo(repoId) {
    state.currentRepoId = repoId;
    state.search = '';
    this.renderToolbar();
    document.getElementById('content').innerHTML =
      `<div class="empty-state"><div class="icon">${icon('loader', 40)}</div><p>加载中...</p></div>`;
    await this.loadSkills(repoId);
  },

  backToRepos() {
    state.currentRepoId = null;
    state.skills = [];
    this.renderToolbar();
    this.renderContent();
  },

  renderSkillsView(c) {
    let skills = state.skills;
    if (state.search) {
      const q = state.search.toLowerCase();
      skills = skills.filter(s =>
        (s.name||'').toLowerCase().includes(q) ||
        (s.description||'').toLowerCase().includes(q)
      );
    }
    if (skills.length === 0) {
      c.innerHTML = `<div class="empty-state"><div class="icon">${icon('search', 40)}</div><p>没有匹配的技能</p></div>`;
      return;
    }
    c.innerHTML = `<div class="card-grid">${skills.map(s => this.skillCard(s)).join('')}</div>`;
    const inst = skills.filter(s => s.installed).length;
    this.updateStatus(`共 ${skills.length} 个 | ${inst} 已安装`);
  },

  skillCard(s) {
    const tags = [];
    if (s.version) tags.push(`<span class="tag">v${this.esc(s.version)}</span>`);
    if (s.installed) tags.push(`<span class="tag installed">installed</span>`);

    const idx = state.skills.indexOf(s);
    const btn = s.installed
      ? `<button class="btn btn-sm btn-outline" onclick="App.doUninstall(${idx})">移除</button>`
      : `<button class="btn btn-sm" onclick="App.doInstall(${idx})">安装</button>`;

    return `<div class="skill-card ${s.installed ? 'installed' : ''}">
      <div class="name">${this.esc(s.name)}</div>
      <div class="desc">${this.esc(s.description) || '<span class="muted">无描述</span>'}</div>
      <div class="meta">${tags.join('')}</div>
      <div class="actions">${btn}</div>
    </div>`;
  },

  // ─── skills.sh（社区搜索）───
  async renderSkillsSh(c) {
    if (!state.search || !state.search.trim()) {
      // 没搜索时显示热门
      c.innerHTML = `<div class="empty-state"><div class="icon">${icon('globe', 40)}</div>
        <p>输入关键词搜索社区技能</p>
        <button class="btn btn-sm btn-outline" onclick="App.loadTrending()">加载热门技能</button></div>`;
      return;
    }

    c.innerHTML = `<div class="empty-state"><div class="icon">${icon('loader', 40)}</div><p>搜索 "${this.esc(state.search)}"...</p></div>`;

    try {
      const result = await api.skillsSh.search(state.search);
      const skills = result.skills || [];
      if (skills.length === 0) {
        c.innerHTML = `<div class="empty-state"><div class="icon">${icon('search', 40)}</div><p>没有找到匹配的技能</p></div>`;
        return;
      }
      c.innerHTML = `<div class="card-grid">${skills.map(s => `
        <div class="skill-card">
          <div class="name">${this.esc(s.name)}</div>
          <div class="desc muted">${this.esc(s.source)}</div>
          <div class="meta">
            <span class="tag">${icon('download', 12)} ${this.esc(s.installs)}</span>
          </div>
          <div class="actions">
            <button class="btn btn-sm" onclick="App.installFromSkillsSh('${this.esc(s.source)}')">添加并安装</button>
          </div>
        </div>`).join('')}</div>`;
      this.updateStatus(`找到 ${skills.length} 个技能`);
    } catch (e) {
      c.innerHTML = `<div class="empty-state"><div class="icon">${icon('alertTriangle', 40)}</div><p>搜索失败: ${this.esc(String(e))}</p></div>`;
    }
  },

  async loadTrending() {
    const c = document.getElementById('content');
    c.innerHTML = `<div class="empty-state"><div class="icon">${icon('loader', 40)}</div><p>加载中...</p></div>`;
    try {
      // skills.sh 没有 trending API，用多个热门关键词搜索聚合
      const hotKeywords = ['git', 'react', 'code', 'python', 'test', 'docker', 'api', 'build'];
      const all = [];
      const seen = new Set();
      for (const kw of hotKeywords) {
        try {
          const r = await api.skillsSh.search(kw);
          for (const s of (r.skills||[])) {
            if (!seen.has(s.id)) { seen.add(s.id); all.push(s); }
          }
        } catch {}
      }
      // 按安装数排序
      all.sort((a,b) => (b.installs||0) - (a.installs||0));
      const top = all.slice(0, 50);

      if (!top.length) {
        c.innerHTML = `<div class="empty-state"><div class="icon">${icon('moodEmpty', 40)}</div><p>暂无数据</p></div>`;
        return;
      }
      c.innerHTML = `<div class="card-grid">${top.map(s => `
        <div class="skill-card">
          <div class="name">${this.esc(s.name)}</div>
          <div class="desc muted">${this.esc(s.source)}</div>
          <div class="meta"><span class="tag">${icon('download', 12)} ${this.esc(s.installs)}</span></div>
          <div class="actions">
            <button class="btn btn-sm" onclick="App.installFromSkillsSh('${this.esc(s.source)}')">添加并安装</button>
          </div>
        </div>`).join('')}</div>`;
      this.updateStatus(`${top.length} 个热门技能`);
    } catch (e) {
      c.innerHTML = `<div class="empty-state"><div class="icon">${icon('alertTriangle', 40)}</div><p>加载失败</p></div>`;
    }
  },

  // 从 skills.sh 安装：source 格式如 "github/awesome-copilot"，即 owner/repo
  async installFromSkillsSh(source) {
    // source 格式: "owner/repo" 或 "github/owner/repo"
    const parts = source.split('/');
    let owner, repo;
    if (parts[0] === 'github' && parts.length >= 3) {
      owner = parts[1];
      repo = parts[2];
    } else if (parts.length >= 2) {
      owner = parts[0];
      repo = parts[1];
    } else {
      this.toast('无法解析技能来源: ' + source, 'error');
      return;
    }

    const url = `https://github.com/${owner}/${repo}`;
    if (!confirm(`将添加仓库 ${owner}/${repo} 并进入浏览技能？`)) return;

    this.toast('正在 clone...', 'info');
    try {
      // 先添加仓库
      await api.repo.add(url, 'main', '');
      state.repos = await api.repo.list();
      this.renderRepoCount();

      // 找到新添加的仓库并进入
      const repoId = `${owner}__${repo}`;
      state.view = 'discover';
      state.tab = 'repos';
      this.renderSidebar();
      await this.enterRepo(repoId);
      this.toast('仓库已添加', 'success');
    } catch (e) {
      // 可能已存在，尝试直接进入
      const repoId = `${owner}__${repo}`;
      if (state.repos.find(r => r.id === repoId)) {
        state.view = 'discover';
        state.tab = 'repos';
        this.renderSidebar();
        await this.enterRepo(repoId);
      } else {
        this.toast('添加失败: ' + e, 'error');
      }
    }
  },

  // ─── 仓库管理（已合并到弹窗）───

  // ═══ 仓库管理 ═══
  renderSettings(c) {
    const s = state.settings;
    const appVersion = state.appVersion || '1.0.0';
    c.innerHTML = `
      <div style="max-width:600px">
        <div class="mb-2">
          <h4>关于</h4>
          <div class="flex gap-sm" style="margin-top:8px;align-items:center">
            <span class="muted">AIKit v${appVersion}</span>
            <button class="btn btn-sm btn-outline" id="check-update-btn" onclick="App.checkAppUpdate()">${icon('refresh', 14)} 检查更新</button>
          </div>
        </div>
        <div class="mb-2">
          <h4>日志</h4>
          <div class="flex gap-sm" style="margin-top:8px;flex-wrap:wrap">
            <label class="flex gap-sm"><input type="checkbox" ${s.logEnabled?'checked':''} onchange="App.setSetting('logEnabled',this.checked)"> 启用</label>
            <select onchange="App.setSetting('logLevel',this.value)">
              ${['error','warn','info','debug','trace'].map(lv=>`<option value="${lv}" ${s.logLevel===lv?'selected':''}>${lv}</option>`).join('')}
            </select>
            <button class="btn btn-sm btn-outline" onclick="App.viewLogs()">查看</button>
            <button class="btn btn-sm btn-red" onclick="App.clearLogs()">清空</button>
          </div>
        </div>
        <div class="mb-2">
          <h4>配置</h4>
          <div class="flex gap-sm" style="margin-top:8px">
            <button class="btn btn-sm btn-outline" onclick="App.exportConfig()">${icon('upload', 14)} 导出</button>
            <button class="btn btn-sm btn-outline" onclick="App.importConfig()">${icon('download', 14)} 导入</button>
          </div>
        </div>
        <div class="mb-2">
          <h4>目录</h4>
          <div id="dir-list" style="margin-top:8px">加载中...</div>
        </div>
      </div>`;
    this.loadDirList();
  },

  // ═══ 数据操作 ═══
  async loadSkills(repoId) {
    try {
      const result = await api.skills.scan(repoId);
      state.skills = result.skills || [];
      this.renderContent();
    } catch (e) { this.toast('加载失败: ' + e, 'error'); }
  },

  async refreshSkills() {
    if (!state.currentRepoId) return;
    this.toast('刷新中...', 'info');
    await this.loadSkills(state.currentRepoId);
  },

  onSearch(val) {
    state.search = val;
    this.renderContent();
  },

  onGlobalSearch(val) {
    // 全局搜索：同步到 state.search 并触发当前页搜索
    const searchInput = document.getElementById('search-input');
    if (searchInput) searchInput.value = val;
    state.search = val;
    this.renderContent();
  },

  // ═══ 仓库管理 ═══
  showAddRepo() {
    const modal = document.getElementById('modal');
    document.getElementById('modal-overlay').classList.remove('hidden');
    const repoListHTML = state.repos.length > 0 ? `
      <div style="margin-bottom:18px;border-top:1px solid var(--border-subtle);padding-top:16px">
        <div class="muted" style="margin-bottom:8px;font-size:12px;font-weight:600">已添加仓库</div>
        ${state.repos.map(r => `
          <div class="flex" style="justify-content:space-between;padding:6px 0">
            <span style="font-size:13px">${this.esc(r.owner)}/${this.esc(r.name)}</span>
            <div class="flex gap-sm">
              <button class="icon-btn" title="在 GitHub 中打开" onclick="openExternal('${repoUrl(r.owner, r.name)}')">${icon('externalLink', 14)}</button>
              <button class="btn btn-sm btn-red" onclick="App.deleteRepo('${r.id}','${this.esc(r.owner+'/'+r.name)}')">删除</button>
            </div>
          </div>
        `).join('')}
      </div>` : '';

    modal.innerHTML = `
      <h2>仓库管理</h2>
      <div class="modal-field">
        <label>GitHub 地址</label>
        <input type="text" id="repo-url" placeholder="如 https://github.com/anthropics/skills"
          oninput="App.parseRepoUrl()" style="width:100%">
        <div id="repo-parsed" class="muted" style="margin-top:4px;font-size:12px"></div>
      </div>
      <div class="modal-field">
        <label>分支</label><input type="text" id="repo-branch" value="main">
      </div>
      <div class="modal-field">
        <label>子目录（可选）</label><input type="text" id="repo-subdir" placeholder="如 skills">
      </div>
      <div class="modal-actions">
        <button class="btn btn-outline" onclick="App.closeModal()">取消</button>
        <button class="btn" onclick="App.doAddRepo()">添加</button>
      </div>
      ${repoListHTML}
    `;
  },

  parseRepoUrl() {
    const url = document.getElementById('repo-url').value.trim();
    const hint = document.getElementById('repo-parsed');
    if (!url) { hint.textContent = ''; return; }
    const m = url.match(/github\.com\/([^/\s]+)\/([^/\s#?]+)/);
    if (m) {
      const tree = url.match(/\/tree\/([^/]+)(\/(.+))?/);
      if (tree) {
        document.getElementById('repo-branch').value = tree[1];
        if (tree[3]) document.getElementById('repo-subdir').value = tree[3];
      }
      hint.innerHTML = `<span style="color:var(--success);display:inline-flex;align-items:center;gap:4px">${icon('check', 14)} ${m[1]}/${m[2].replace(/\.git$/,'')}</span>`;
    } else {
      hint.innerHTML = `<span style="color:var(--danger);display:inline-flex;align-items:center;gap:4px">${icon('alertTriangle', 14)} 无法识别</span>`;
    }
  },

  async doAddRepo() {
    const url = document.getElementById('repo-url').value.trim();
    const branch = document.getElementById('repo-branch').value.trim() || 'main';
    const subdir = document.getElementById('repo-subdir').value.trim();
    if (!url) { this.toast('请填写地址', 'warn'); return; }
    this.closeModal();
    this.toast('正在 clone...', 'info');
    try {
      await api.repo.add(url, branch, subdir);
      state.repos = await api.repo.list();
      this.renderContent();
      this.renderRepoCount();
      this.toast('添加成功', 'success');
    } catch (e) { this.toast('添加失败: ' + e, 'error'); }
  },

  async deleteRepo(id, name) {
    if (!confirm(`删除仓库 ${name}？`)) return;
    try {
      await api.repo.del(id);
      state.repos = await api.repo.list();
      this.renderContent();
      this.renderRepoCount();
      this.toast('已删除', 'success');
    } catch (e) { this.toast('删除失败: ' + e, 'error'); }
  },

  async pullRepo(id) {
    this.toast('拉取中...', 'info');
    try {
      await api.repo.pull(id);
      this.toast('已更新', 'success');
      if (state.currentRepoId === id) await this.loadSkills(id);
    } catch (e) { this.toast('失败: ' + e, 'error'); }
  },

  // ═══ 安装/卸载（SSOT 模式）═══

  // 安装技能到 SSOT（不选工具）
  async doInstall(idx) {
    const skill = state.skills[idx];
    if (!skill) return;
    try {
      this.toast('安装中...', 'info');
      const result = await api.skills.install(skill);
      if (result.alreadyInstalled) {
        this.toast('该技能已安装', 'warn');
        return;
      }
      this.toast('安装成功，可在「我的技能」中启用工具', 'success');
      this.loadSkills(state.currentRepoId);
      this.updateInstalled();
    } catch (e) { this.toast('安装失败: ' + e, 'error'); }
  },

  // 卸载技能（从 SSOT + 所有工具目录）
  async doUninstall(idx) {
    const skill = state.skills[idx];
    if (!skill) return;
    if (!confirm(`卸载「${skill.name}」？\n将从 SSOT 和所有工具目录移除。`)) return;
    try {
      await api.skills.uninstall(skill.dirName);
      this.toast('已卸载', 'success');
      this.loadSkills(state.currentRepoId);
      this.updateInstalled();
    } catch (e) { this.toast('卸载失败: ' + e, 'error'); }
  },

  // 从我的技能页卸载
  async uninstallSkill(dirName) {
    if (!confirm(`卸载「${dirName}」？`)) return;
    try {
      await api.skills.uninstall(dirName);
      this.toast('已卸载', 'success');
      this.updateInstalled();
    } catch (e) { this.toast('失败: ' + e, 'error'); }
  },

  // 切换工具开关
  async toggleApp(skillName, toolId, enabled) {
    try {
      await api.skills.toggleApp(skillName, toolId, enabled);
      this.toast(`${enabled ? '已同步到' : '已移除'} ${state.tools.find(t=>t.id===toolId)?.name || toolId}`, 'success');
      await this.updateInstalled();
    } catch (e) { this.toast('失败: ' + e, 'error'); }
  },

  async checkUpdates() {
    this.toast('检查中...', 'info');
    for (const r of state.repos) { try { await api.repo.pull(r.id); } catch {} }
    const result = await api.skills.checkUpdates();
    state.updates = result.updates || [];
    if (!state.updates.length) {
      this.toast('全部最新', 'success');
      if (state.view === 'my-skills') this.renderContent();
      return;
    }
    this.toast(`${state.updates.length} 个可更新`, 'warn');
    if (state.view === 'my-skills') this.renderContent();
    else if (state.currentRepoId) await this.loadSkills(state.currentRepoId);
  },

  async applyUpdate(dirName) {
    const u = state.updates.find(x => x.dirName === dirName);
    if (!u) return;
    this.toast('更新中...', 'info');
    try {
      await api.skills.applyUpdate(u.dirName, u.repoId);
      state.updates = state.updates.filter(x => x.dirName !== dirName);
      await this.updateInstalled();
      const enabledTools = Object.entries((state.installed.find(i => i.dirName === dirName) || {}).apps || {})
        .filter(([, on]) => on).map(([tid]) => state.tools.find(t => t.id === tid)?.name).filter(Boolean);
      this.toast(enabledTools.length ? `已更新，已同步到 ${enabledTools.join('、')}` : '已更新', 'success');
    } catch (e) { this.toast('更新失败: ' + e, 'error'); }
  },

  // ─── 应用自身更新（Tauri updater）───
  async checkAppUpdate() {
    const btn = document.getElementById('check-update-btn');
    const setBtn = (html) => { if (btn) btn.innerHTML = html; };
    setBtn(icon('loader', 14) + ' 检查中...');
    try {
      const update = await window.__TAURI__.updater.check();
      if (!update) {
        this.toast('已是最新版本', 'success');
        setBtn(icon('refresh', 14) + ' 检查更新');
        return;
      }
      this.toast(`发现新版本 v${update.version}，正在下载...`, 'info');
      let contentLength = 0, downloaded = 0;
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            contentLength = event.data.contentLength || 0;
            setBtn(icon('download', 14) + ' 下载中 0%');
            break;
          case 'Progress':
            downloaded += event.data.chunkLength || 0;
            if (contentLength > 0) {
              const pct = Math.min(100, Math.round(downloaded / contentLength * 100));
              setBtn(icon('download', 14) + ` 下载中 ${pct}%`);
            }
            break;
          case 'Finished':
            setBtn(icon('rocket', 14) + ' 安装中...');
            break;
        }
      });
      this.toast('下载完成，即将重启...', 'success');
      await window.__TAURI__.process.relaunch();
    } catch (e) {
      this.toast('检查更新失败: ' + e, 'error');
      setBtn(icon('refresh', 14) + ' 检查更新');
    }
  },

  // ═══ 设置操作 ═══
  async setSetting(key, value) {
    const partial = {}; partial[key] = value;
    state.settings = await api.settings.set(partial);
    this.toast('已保存', 'success');
  },

  async loadDirList() {
    const el = document.getElementById('dir-list');
    if (!el) return;
    try {
      const dirs = await api.dir.list();
      this._dirPaths = []; // 缓存所有路径，按索引引用

      let html = '<table style="width:100%;font-size:12px"><tbody>';

      html += '<tr><td colspan="2" style="padding-top:8px;font-weight:600;color:var(--text)">工具目录</td></tr>';
      let idx = 0;
      for (const [k, v] of Object.entries(dirs.app)) {
        this._dirPaths.push(v);
        html += `<tr><td class="muted" style="padding:3px 0;width:120px">${k}</td><td style="padding:3px 0">
          <span class="muted" style="word-break:break-all">${this.esc(v)}</span>
          <button class="btn btn-sm btn-outline" style="margin-left:8px;padding:2px 8px" onclick="App.openDir(${idx})">打开</button>
        </td></tr>`;
        idx++;
      }

      html += '<tr><td colspan="2" style="padding-top:12px;font-weight:600;color:var(--text)">Skills 目录</td></tr>';
      for (const [k, v] of Object.entries(dirs.tools)) {
        const t = state.tools.find(x => x.id === k);
        const isInstalled = t && t.installed !== false;
        this._dirPaths.push(v);
        if (isInstalled) {
          html += `<tr><td class="muted" style="padding:3px 0;width:120px">${t ? t.name : k}</td><td style="padding:3px 0">
            <span class="muted" style="word-break:break-all">${this.esc(v)}</span>
            <button class="btn btn-sm btn-outline" style="margin-left:8px;padding:2px 8px" onclick="App.openDir(${idx})">打开</button>
          </td></tr>`;
        } else {
          html += `<tr><td class="muted" style="padding:3px 0;width:120px">${t ? t.name : k}</td><td style="padding:3px 0">
            <span class="tag" style="color:var(--text-dim)">未安装</span>
          </td></tr>`;
        }
        idx++;
      }

      html += '</tbody></table>';
      el.innerHTML = html;
    } catch { el.innerHTML = '加载失败'; }
  },

  async openDir(idx) {
    const p = this._dirPaths ? this._dirPaths[idx] : null;
    if (!p) { this.toast('路径不存在', 'error'); return; }
    try { await api.dir.open(p); } catch (e) { this.toast('打开失败: ' + e, 'error'); }
  },

  async viewLogs() {
    const logs = await api.logs.read(100);
    document.getElementById('modal-overlay').classList.remove('hidden');
    document.getElementById('modal').innerHTML = `
      <h2>操作日志</h2>
      <div style="max-height:400px;overflow-y:auto">
        ${logs.length === 0 ? '<p class="muted text-center">暂无日志</p>' :
          logs.map(l => `<div class="log-entry ${l.level}">
            <span class="log-time">${l.time}</span>
            <span class="log-level">${l.level}</span>
            <span>${this.esc(l.action)} ${l.detail ? this.esc(JSON.stringify(l.detail)) : ''}</span>
          </div>`).join('')}
      </div>
      <div class="modal-actions"><button class="btn btn-outline" onclick="App.closeModal()">关闭</button></div>`;
  },

  async clearLogs() { if (confirm('清空日志？')) { await api.logs.clear(); this.toast('已清空', 'success'); } },

  async exportConfig() {
    try {
      const fp = await dialogSave({ title: '导出', defaultPath: `aikit-${Date.now()}.json`, filters: [{ name: 'JSON', extensions: ['json'] }] });
      if (!fp) return;
      await api.config.export(fp);
      this.toast('已导出', 'success');
    } catch (e) { this.toast('失败: ' + e, 'error'); }
  },

  async importConfig() {
    try {
      const fp = await dialogOpen({ title: '导入', filters: [{ name: 'JSON', extensions: ['json'] }] });
      if (!fp) return;
      const p = typeof fp === 'string' ? fp : fp.path;
      await api.config.import(p);
      state.repos = await api.repo.list();
      state.installed = await api.installed.list();
      state.settings = await api.settings.get();
      this.renderContent();
      this.toast('已导入', 'success');
    } catch (e) { this.toast('失败: ' + e, 'error'); }
  },

  // ═══ 辅助 ═══
  async updateInstalled() {
    state.installed = await api.installed.list();
    this.renderRepoCount();
    if (state.view === 'my-skills') this.renderContent();
  },

  renderRepoCount() {
    document.getElementById('repo-count').innerHTML =
      `<span class="muted">${state.repos.length} 个仓库 | ${state.installed.length} 个已安装</span>`;
  },

  updateStatus(text) {
    const el = document.getElementById('status-text');
    if (el) el.textContent = text;
  },

  closeModal() { document.getElementById('modal-overlay').classList.add('hidden'); },

  esc(s) {
    if (!s) return '';
    return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;').replace(/'/g,'&#39;');
  },

  toast(msg, type = 'info') {
    const c = document.getElementById('toast-container');
    const el = document.createElement('div');
    el.className = `toast ${type}`;
    el.textContent = msg;
    c.appendChild(el);
    setTimeout(() => el.remove(), 3500);
  },
};

window.addEventListener('DOMContentLoaded', () => App.init());
