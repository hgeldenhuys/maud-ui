// Presentation-only delay: callers retain their existing aria and focus policy.
(function () {
  const ui = window.MaudUI;
  if (!ui) return;
  const pending = new WeakMap();
  ui.cancelOverlayExit = function (panel) {
    pending.get(panel)?.();
  };
  ui.closeOverlay = function (panel, hide) {
    if (pending.has(panel) || panel.hidden) return;
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) { hide(); return; }
    const state = panel.getAttribute('data-state');
    const inert = panel.inert;
    panel.setAttribute('data-state', 'closing');
    panel.inert = true;
    const style = getComputedStyle(panel);
    const ms = value => value.trim().endsWith('ms') ? parseFloat(value) : parseFloat(value) * 1000;
    const duration = ms(style.getPropertyValue('--mui-motion-fast')) || 0;
    let timer;
    function cleanup() {
      clearTimeout(timer);
      panel.removeEventListener('animationend', finish);
      panel.inert = inert;
      if (state === null) panel.removeAttribute('data-state');
      else panel.setAttribute('data-state', state);
      pending.delete(panel);
    }
    function finish(event) {
      if (event && (event.target !== panel || event.animationName !== 'mui-overlay-exit')) return;
      cleanup(); hide();
    }
    pending.set(panel, cleanup);
    panel.addEventListener('animationend', finish);
    timer = setTimeout(() => finish(), duration);
  };
})();
