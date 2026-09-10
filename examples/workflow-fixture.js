// Example application adapter. Only the local browser-test server persists these requests.
window.fixtureSubmits = 0;
window.fixtureFinished = 0;
if (!new URLSearchParams(location.search).has('native')) document.querySelectorAll('form[data-mui="form-feedback"]').forEach(form => {
  form.addEventListener('submit', async event => {
    event.preventDefault(); window.fixtureSubmits++;
    const body = new URLSearchParams(new FormData(form));
    window.fixturePayload = Object.fromEntries(body);
    try {
      const response = await fetch(form.action, { method: 'POST', body });
      const result = await response.json();
      MaudUI.formFeedback(form, result);
      if (form.id === 'check-in-form' && result.status === 'success') {
        form.closest('dialog').close();
        MaudUI.notice(document.getElementById('check-in-result'), result);
      }
    } catch (error) {
      MaudUI.formFeedback(form, { status: 'error', message: 'The request could not be completed.', description: 'Your entries are still here. Try again.', details: error.message });
    } finally { window.fixtureFinished++; }
  });
});
document.getElementById('next-step').addEventListener('click', event => {
  const header = document.querySelector('.mui-wizard-header');
  const steps = header.querySelectorAll('li');
  steps[1].removeAttribute('aria-current'); steps[1].dataset.step = 'complete';
  steps[1].querySelector('.mui-wizard-header__number').textContent = '✓';
  const completed = document.createElement('span'); completed.className = 'mui-sr-only'; completed.textContent = ' · Complete'; steps[1].append(completed);
  steps[2].setAttribute('aria-current', 'step'); steps[2].dataset.step = 'current';
  header.querySelector('.mui-wizard-header__progress').textContent = 'Step 3 of 3 · Check in';
  const title = document.getElementById('step-title'); title.textContent = 'Ready to check in Leila'; title.focus(); event.target.disabled = true;
});
