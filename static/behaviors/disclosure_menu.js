// Overflow menus built on <details> (.mui-action-row__more) close like menus do:
// on a click outside, on Escape (focus returns to the trigger), and when another
// opens. They open under the trigger's start edge and flip to its end edge only
// when the start-aligned menu would leave the viewport. Native <details> does none
// of this, so before 0.19.3 an open "More" stayed open until its own trigger was
// clicked again.
(function () {
  if (window.__muiDisclosureMenuBound) return;
  window.__muiDisclosureMenuBound = true;
  var SELECTOR = 'details.mui-action-row__more';
  function openMenus() { return Array.prototype.slice.call(document.querySelectorAll(SELECTOR + '[open]')); }
  function close(details) {
    var panel = details.querySelector('.mui-action-row__overflow');
    if (!panel) { details.open = false; return; }
    window.MaudUI.closeOverlay(panel, function () { details.open = false; });
  }
  document.addEventListener('click', function (event) {
    var summary = event.target.closest?.(SELECTOR + ' > summary');
    if (!summary) return;
    var details = summary.parentElement;
    var panel = details.querySelector('.mui-action-row__overflow');
    if (panel?.getAttribute('data-state') === 'closing') {
      event.preventDefault(); window.MaudUI.cancelOverlayExit(panel);
    } else if (details.open) { event.preventDefault(); close(details); }
  });
  function align(details) {
    var panel = details.querySelector('.mui-action-row__overflow');
    if (!panel) return;
    details.removeAttribute('data-align');
    var rect = panel.getBoundingClientRect();
    var rtl = getComputedStyle(details).direction === 'rtl';
    var overflows = rtl ? rect.left < 0 : rect.right > document.documentElement.clientWidth;
    if (overflows) details.setAttribute('data-align', 'end');
  }
  document.addEventListener('toggle', function (event) {
    var details = event.target;
    if (!(details instanceof HTMLDetailsElement) || !details.matches(SELECTOR) || !details.open) return;
    openMenus().forEach(function (other) { if (other !== details) close(other); });
    align(details);
  }, true);
  document.addEventListener('pointerdown', function (event) {
    openMenus().forEach(function (details) { if (!details.contains(event.target)) close(details); });
  });
  document.addEventListener('keydown', function (event) {
    if (event.key !== 'Escape') return;
    openMenus().forEach(function (details) {
      var summary = details.querySelector('summary');
      if (summary && details.contains(document.activeElement)) summary.focus();
      close(details);
    });
  });
})();
