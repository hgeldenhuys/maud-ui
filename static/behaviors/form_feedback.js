(function () {
  'use strict';
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = new WeakSet();
  const pending = new WeakMap();
  const focusQueued = new WeakSet();
  let nextId = 0;
  const fields = form => Array.from(form.elements).filter(n => /^(INPUT|SELECT|TEXTAREA)$/.test(n.tagName));
  const enabled = n => !n.disabled && n.type !== 'hidden' && !n.readOnly;
  const errorNodes = field => Array.from(field.querySelectorAll('.mui-field__error')).filter(n => n.closest('.mui-field, .mui-radio-group') === field);
  function fieldFor(control) { return control.closest('.mui-field, .mui-radio-group'); }
  function controlsFor(field) {
    const id = field.getAttribute('data-field-control') || field.querySelector('label[for]')?.htmlFor;
    const control = id && document.getElementById(id);
    if (control && field.contains(control)) {
      if (control.type === 'radio') return Array.from(field.querySelectorAll('input[type="radio"]')).filter(n => n.name === control.name);
      return [control];
    }
    return Array.from(field.querySelectorAll('input,select,textarea')).filter(n => fieldFor(n) === field);
  }
  function wire(field) {
    const errors = errorNodes(field).filter(n => n.textContent.trim());
    const descriptions = Array.from(field.querySelectorAll('.mui-field__description')).filter(n => n.closest('.mui-field, .mui-radio-group') === field);
    const nodes = [...descriptions, ...errors];
    nodes.forEach(n => { if (!n.id) n.id = 'mui-field-message-' + (++nextId); });
    controlsFor(field).forEach(control => {
      const previous = (control.getAttribute('data-mui-field-links') || '').split(/\s+/);
      const existing = (control.getAttribute('aria-describedby') || '').split(/\s+/).filter(id => id && !previous.includes(id));
      const links = nodes.map(n => n.id);
      const all = Array.from(new Set([...existing, ...links])).join(' ');
      if (all) control.setAttribute('aria-describedby', all); else control.removeAttribute('aria-describedby');
      control.setAttribute('data-mui-field-links', links.join(' '));
      if (errors.length) {
        control.setAttribute('aria-invalid', 'true'); control.setAttribute('data-mui-field-invalid', '');
      } else if (control.hasAttribute('data-mui-field-invalid')) {
        control.removeAttribute('aria-invalid'); control.removeAttribute('data-mui-field-invalid');
      }
    });
    field.classList.toggle('mui-field--invalid', errors.length > 0);
    if (errors.length) field.setAttribute('data-invalid', 'true'); else field.removeAttribute('data-invalid');
  }
  function clear(control) {
    const field = fieldFor(control);
    if (!field) return;
    errorNodes(field).forEach(n => n.remove()); wire(field);
  }
  function showError(control, message) {
    let field = fieldFor(control);
    // Do not rearrange caller markup. Unwrapped controls get a message beside them.
    if (!field) {
      const base = (control.id || (control.id = 'mui-control-' + (++nextId))) + '-feedback';
      let id = control.getAttribute('data-mui-error-id') || base;
      let error = document.getElementById(id);
      while (error && error.getAttribute('data-mui-control-error') !== control.id) { id = base + '-' + (++nextId); error = document.getElementById(id); }
      if (!error) { error = document.createElement('p'); error.id = id; error.className = 'mui-field__error'; control.after(error); }
      control.setAttribute('data-mui-error-id', id);
      error.textContent = message; error.setAttribute('data-mui-control-error', control.id);
      const links = new Set((control.getAttribute('aria-describedby') || '').split(/\s+/).filter(Boolean));
      links.add(id); control.setAttribute('aria-describedby', Array.from(links).join(' ')); control.setAttribute('aria-invalid', 'true');
      return;
    }
    clear(control);
    const error = document.createElement('p'); error.className = 'mui-field__error'; error.textContent = message;
    field.append(error); wire(field);
  }
  function clearControl(control) {
    clear(control);
    if (!fieldFor(control) && control.id) {
      const error = document.getElementById(control.getAttribute('data-mui-error-id') || control.id + '-feedback');
      if (error?.getAttribute('data-mui-control-error') === control.id) {
        const links = (control.getAttribute('aria-describedby') || '').split(/\s+/).filter(id => id && id !== error.id).join(' ');
        error.remove(); if (links) control.setAttribute('aria-describedby', links); else control.removeAttribute('aria-describedby');
        control.removeAttribute('aria-invalid'); control.removeAttribute('data-mui-error-id');
      }
    }
  }
  function focusFirst(form) {
    const first = fields(form).find(n => enabled(n) && n.getAttribute('aria-invalid') === 'true' && n.getClientRects().length);
    if (first) first.focus();
    return first;
  }
  function resultRegion(form) {
    let region = form.querySelector('[data-mui-form-result]');
    if (!region) { region = document.createElement('div'); region.setAttribute('data-mui-form-result', ''); region.hidden = true; form.append(region); }
    return region;
  }
  // Safe text-only result API. A failure always has a human sentence before diagnostics.
  ui.notice = function (region, result = {}) {
    const failure = result.status === 'error';
    region.classList.add('mui-notice'); region.setAttribute('data-tone', failure ? 'error' : result.status === 'success' ? 'success' : 'info');
    region.setAttribute('role', failure ? 'alert' : 'status'); region.setAttribute('aria-atomic', 'true'); region.tabIndex = -1;
    region.replaceChildren(); region.hidden = false;
    const line = document.createElement('p'); line.className = 'mui-notice__message';
    line.textContent = result.message?.trim() || (failure ? 'Your changes could not be saved.' : 'Changes saved.'); region.append(line);
    if (result.description) { const p = document.createElement('p'); p.className = 'mui-notice__description'; p.textContent = result.description; region.append(p); }
    if (result.details) {
      const details = document.createElement('details'), summary = document.createElement('summary'), text = document.createElement('pre');
      details.className = 'mui-notice__details'; summary.textContent = 'Details'; text.textContent = result.details;
      details.append(summary, text); region.append(details);
    }
    return region;
  };
  // Does not disable fields: native FormData still contains the user's inputs.
  // Mirror the successful named submitter while it is disabled, exactly once.
  ui.formPending = function (form, value = true, submitter) {
    if (value && pending.has(form)) return;
    if (!value) {
      const state = pending.get(form);
      if (state?.mirror) state.mirror.remove();
      form.querySelectorAll('[data-mui-submitter-mirror]').forEach(n => n.remove());
      Array.from(form.elements).filter(n => n.hasAttribute('data-mui-disabled-before')).forEach(button => {
        button.disabled = button.getAttribute('data-mui-disabled-before') === 'true';
        button.removeAttribute('data-mui-disabled-before');
        const aria = button.getAttribute('data-mui-aria-disabled-before');
        if (aria === '__absent__') button.removeAttribute('aria-disabled'); else if (aria !== null) button.setAttribute('aria-disabled', aria);
        button.removeAttribute('data-mui-aria-disabled-before');
        const content = button.querySelector('[data-mui-submit-content]');
        if (content) { button.querySelector('[data-mui-submit-progress]')?.remove(); content.replaceWith(...content.childNodes); }
      });
      form.removeAttribute('aria-busy'); pending.delete(form); return;
    }
    const buttons = Array.from(form.elements).filter(n => n.type === 'submit' || n.type === 'image' || n.type === 'reset');
    let mirror;
    if (submitter?.name && !submitter.disabled && submitter.type !== 'image') {
      mirror = document.createElement('input'); mirror.type = 'hidden'; mirror.name = submitter.name; mirror.value = submitter.value;
      mirror.setAttribute('data-mui-submitter-mirror', ''); form.append(mirror);
    }
    pending.set(form, { mirror }); form.setAttribute('aria-busy', 'true');
    buttons.forEach(button => {
      button.setAttribute('data-mui-disabled-before', String(button.disabled));
      button.setAttribute('data-mui-aria-disabled-before', button.getAttribute('aria-disabled') ?? '__absent__');
      button.disabled = true; button.setAttribute('aria-disabled', 'true');
    });
    if (submitter?.tagName === 'BUTTON') {
      const content = document.createElement('span'); content.setAttribute('data-mui-submit-content', ''); content.hidden = true;
      content.append(...submitter.childNodes); const progress = document.createElement('span'); progress.setAttribute('data-mui-submit-progress', '');
      progress.textContent = form.getAttribute('data-pending-label') || 'Saving…'; submitter.append(content, progress);
    }
  };
  ui.formFeedback = function (form, result = {}) {
    ui.formPending(form, false);
    fields(form).forEach(clearControl);
    const region = ui.notice(resultRegion(form), result);
    if (result.status === 'error') {
      for (const [name, message] of Object.entries(result.errors || {})) {
        fields(form).filter(n => n.name === name || n.id === name).filter(enabled).forEach(n => showError(n, String(message)));
      }
      if (!focusFirst(form)) region.focus();
    }
    return region;
  };
  ui.behaviors['field-feedback'] = function (field) { wire(field); };
  ui.behaviors['form-feedback'] = function (form) {
    if (initialized.has(form)) return;
    initialized.add(form);
    ui.formPending(form, false); // Recover restored history snapshots.
    form.querySelectorAll('.mui-field, .mui-radio-group').forEach(wire);
    if (form.querySelector('.mui-field__error')) queueMicrotask(() => focusFirst(form));
    form.addEventListener('submit', event => {
      if (pending.has(form)) { event.preventDefault(); event.stopImmediatePropagation(); return; }
      // The dialog method is a local dismissal, not a request.
      if ((event.submitter?.formMethod || form.method) === 'dialog') return;
      if (!form.noValidate && !event.submitter?.formNoValidate && !form.checkValidity()) { event.preventDefault(); return; }
      const region = form.querySelector('[data-mui-form-result]'); if (region) region.hidden = true;
      ui.formPending(form, true, event.submitter);
    }, true);
    form.addEventListener('reset', event => queueMicrotask(() => {
      if (event.defaultPrevented) return;
      ui.formPending(form, false); fields(form).forEach(clearControl);
      const region = form.querySelector('[data-mui-form-result]'); if (region) region.hidden = true;
    }));
  };
  document.addEventListener('invalid', event => {
    const control = event.target, form = control.form;
    if (!form || form.getAttribute('data-mui') !== 'form-feedback') return;
    event.preventDefault(); showError(control, control.validationMessage || 'Check this field.');
    if (!focusQueued.has(form)) {
      focusQueued.add(form); queueMicrotask(() => { focusQueued.delete(form); focusFirst(form); });
    }
  }, true);
  const edit = event => {
    const control = event.target;
    if (control.form?.getAttribute('data-mui') !== 'form-feedback' || control.getAttribute('aria-invalid') !== 'true') return;
    clearControl(control);
    if (!control.validity.valid) showError(control, control.validationMessage);
  };
  document.addEventListener('input', edit); document.addEventListener('change', edit);
  // Form fragments can be replaced wholesale, or only an individual error field.
  document.addEventListener('htmx:afterSettle', event => {
    const field = event.target.closest?.('.mui-field, .mui-radio-group');
    if (field) {
      wire(field);
      const form = controlsFor(field)[0]?.form;
      if (form?.getAttribute('data-mui') === 'form-feedback' && errorNodes(field).length) focusFirst(form);
    }
  });
  window.addEventListener('pageshow', event => {
    if (event.persisted) document.querySelectorAll('form[data-mui="form-feedback"]').forEach(form => ui.formPending(form, false));
  });
  // Gallery-only local confirmation examples, never a transport adapter.
  ui.behaviors['confirm-demo'] = wrapper => {
    if (initialized.has(wrapper)) return;
    initialized.add(wrapper);
    wrapper.addEventListener('submit', event => {
      event.preventDefault();
      setTimeout(() => ui.formFeedback(event.target, { status: 'success', message: 'Check-in confirmed in this example.' }), 500);
    });
  };
  ui.init();
})();
