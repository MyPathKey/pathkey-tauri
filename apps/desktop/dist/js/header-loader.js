(() => {
  const script = document.currentScript;

  // Resolve a path relative to this script's folder (NOT the page)
  const base = new URL('.', script.src);
  function resolveUrl(relOrAbs) {
    try {
      // If it's already absolute (http:, file:, asset:, etc.), URL() keeps it absolute
      return new URL(relOrAbs, base).href;
    } catch {
      // Fallback to original string
      return relOrAbs;
    }
  }

  const headerAttr = script.getAttribute('data-header') || '../partials/header.html';
  const cssAttr    = script.getAttribute('data-css')    || '../css/header.css';
  const headerURL  = resolveUrl(headerAttr);
  const cssURL     = resolveUrl(cssAttr);

  // 1) Ensure CSS once
  function ensureCss(href) {
    if (document.querySelector(`link[rel="stylesheet"][href="${href}"]`)) return;
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = href;
    link.onload = () => console.info('[header-loader] CSS loaded:', href);
    link.onerror = (e) => console.error('[header-loader] CSS failed:', href, e);
    document.head.appendChild(link);
  }

  // 2) Create placeholder at top of body (if needed)
  function ensurePlaceholder() {
    if (document.querySelector('body > header') || document.getElementById('header-placeholder')) return;
    const ph = document.createElement('div');
    ph.id = 'header-placeholder';
    document.body.insertAdjacentElement('afterbegin', ph);
  }

  // 3) Inject header.html
  async function injectHeader() {
    if (document.querySelector('body > header')) {
      console.info('[header-loader] header already present, skipping');
      return;
    }
    ensurePlaceholder();
    try {
      const res = await fetch(headerURL);
      if (!res.ok) throw new Error(`HTTP ${res.status} ${res.statusText}`);
      const html = await res.text();
      const placeholder = document.getElementById('header-placeholder');
      placeholder.innerHTML = html;
      console.info('[header-loader] injected header from', headerURL);

      // Wire mobile menu (only if present)
      const btn = document.getElementById('menuBtn');
      const panel = document.getElementById('mobileMenu');
      if (btn && panel) {
        btn.addEventListener('click', () => {
          const open = panel.hasAttribute('hidden');
          if (open) {
            panel.removeAttribute('hidden');
            panel.classList.add('open');
            btn.setAttribute('aria-expanded', 'true');
          } else {
            panel.setAttribute('hidden', '');
            panel.classList.remove('open');
            btn.setAttribute('aria-expanded', 'false');
          }
        });
        document.addEventListener('click', (e) => {
          if (!panel.contains(e.target) && !btn.contains(e.target)) {
            panel.setAttribute('hidden', '');
            panel.classList.remove('open');
            btn.setAttribute('aria-expanded', 'false');
          }
        }, { capture: true });
      }
    } catch (err) {
      console.error('[header-loader] failed to inject header from', headerURL, err);
    }
  }

  function start() {
    ensureCss(cssURL);
    injectHeader();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', start);
  } else {
    start();
  }
})();
