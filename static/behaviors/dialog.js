(function () {
  'use strict';
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = new WeakSet(), openers = new WeakMap();
  const focusable = dialog => Array.from(dialog.querySelectorAll('a[href],button:not([disabled]),input:not([disabled]):not([type="hidden"]),select:not([disabled]),textarea:not([disabled]),summary,[tabindex]:not([tabindex="-1"])'))
    .filter(n => n.getClientRects().length && !n.closest('[inert]') && getComputedStyle(n).visibility !== 'hidden');
  function trigger(button) {
    if (initialized.has(button)) return;
    initialized.add(button);
    button.addEventListener('click', () => {
      const dialog = document.getElementById(button.getAttribute('data-target'));
      if (!dialog?.showModal || dialog.open) return;
      ui.init(dialog); openers.set(dialog, button); dialog.showModal();
      dialog.querySelector('[autofocus]')?.focus();
    });
  }
  function attach(dialog, backdrop) {
    if (initialized.has(dialog)) return;
    initialized.add(dialog);
    dialog.addEventListener('click', event => {
      const close = event.target.closest?.('[data-mui-close]');
      if (close && close.closest('dialog') === dialog) { event.preventDefault(); dialog.close(); return; }
      if (backdrop && event.target === dialog) {
        const r = dialog.getBoundingClientRect();
        if (event.clientX < r.left || event.clientX > r.right || event.clientY < r.top || event.clientY > r.bottom) dialog.close();
      }
    });
    dialog.addEventListener('keydown', event => {
      if (event.key !== 'Tab' || !dialog.open) return;
      const items = focusable(dialog), first = items[0], last = items[items.length - 1];
      if (!first) { event.preventDefault(); return; }
      // Keep Tab at the modal's boundaries. Native Chrome may otherwise move
      // Shift+Tab into browser chrome rather than around the dialog controls.
      if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
      else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
    });
    dialog.addEventListener('close', () => {
      const opener = openers.get(dialog);
      if (opener?.isConnected && opener.getClientRects().length) opener.focus();
      openers.delete(dialog);
    });
    // Native cancel handles Escape for both dialog types. No forced choice.
  }
  ui.behaviors['dialog-trigger'] = trigger;
  ui.behaviors['alert-dialog-trigger'] = trigger;
  ui.behaviors.dialog = dialog => attach(dialog, true);
  ui.behaviors['alert-dialog'] = dialog => attach(dialog, false);
  ui.init();
})();
