# Round 6: live workflow evidence

Real headless Chrome, driven through CDP in `examples/night-6-live.mjs`, before library changes. Both live origins were visited as receptionist; manager was used to complete the lodge payment. All entered data is fictional and marked UI test N6. Raw observations: `night-6-live-events.jsonl`; screenshots: `night-6-shots/`.

Created guest **GUES-acbec954**, booking **BOOK-f553d7e6**, cash payment **PAYM-calm-river-7a0b**. The payment reached **Completed** and booking **CheckedIn**, verified after navigation. The three-step GuestCheckIn journey itself failed at its final request; the direct booking action completed check-in. The unfinished journey remains visible at step 3 for reproduction. No seeded records were edited.

| Tried, in user words | What happened | What I expected | Owner / response |
| --- | --- | --- | --- |
| Create a guest without entering a name | A browser bubble appeared, then vanished; no message remained beside the field | Keep the problem beside the field and read it when I return | Library: inline validation and accessible field wiring; reproduced in neutral |
| Enter an email and phone | Both fields are plain text; there is no email format check or phone keyboard hint | Appropriate input types | App: choose email/tel types |
| Save the new guest and booking | Create stayed enabled with the same label; no busy indication or saved confirmation | Tell me saving is underway and when it finishes | Library: opt-in pending state and persistent result notice; app must call the result API |
| Choose Leila for the booking | The saved guest was Theo; selecting Leila again in Edit still saved Theo | Save the selected guest | App: serializes unchecked radios. Native FormData correctly contained Leila. Browser-only workaround disabled unchecked radios for ONE correction submission; app source was not changed |
| Choose a room | Choices contain only UI-101/UI-201/UI-301, without room names or availability | Know which room I am choosing | App: relation labels and availability |
| Enter departure before arrival | Inline date explanation appeared and departure received focus | Explain and focus the invalid date | Existing date-range library behavior passed; preserve it |
| Edit the booking | Page title became the special-request text; repeated “Field notes” and disabled computed fields interrupted the form | A short record title and editable fields | App: title selection, disclosure and field projection |
| Confirm the booking | Modal said “Confirm Action” and “execute”; status changed to Confirmed below while the header stayed Pending | Name the guest/action and update the whole record | Library: concrete confirmation composition; app: copy and refresh all affected fragments |
| Dismiss confirmation with Escape, then confirm with keyboard | Escape closed and returned focus; Tab/Enter confirmed payment | Stable native keyboard operation | Passed after correcting CDP key encoding; early driver key attempts are not app failures |
| Add a payment as receptionist | Offered form failed with 403 and a long access-policy error/code | Either allow payment or explain what role can take it before I fill it in | App: role affordances/permissions; library: human line before diagnostics |
| Leave payment method blank | Asterisk says required, but the select has no required attribute | Stop me beside Method | Library: native-select lacked required/error props; added them. App: pass required semantics |
| Enter R1,500 as manager | Related row showed a dash; detail showed R150,000.00 | The same R1,500 in every view | App: money conversion/formatting and related-row value |
| Process the cash payment | Payment reached Completed; action feedback was absent or generic “Created successfully” | “Payment recorded” with amount/reference | Library notice contract; app supplies accurate action-specific text |
| Find the check-in journey | It is under Settings, then Journeys, then Start, then another Start | Start check-in near the booking action | App: journey placement and duplicate launch |
| Select the booking in check-in | Choices use references or special requests, including cancelled/checked-out bookings | Guest, room, dates and eligible bookings | App: picker projection and eligibility |
| Verify the guest at step 2 | Only Mark Complete appeared; no guest details or verification instruction | See whose identity I am checking | App: step content; library: header with guest context and progress |
| Continue to check-in step 3 | A second complete shell appeared inside the step, with repeated navigation and technical workflow text | One compact step and the booking summary | App: request/render the record fragment; library: reusable wizard header |
| Finish the journey | HTTP 415; two notices only said “Request failed:” | Useful recovery text and a completed journey | App: request encoding/duplicate handlers/progression; library: failure notice fallback |
| Search for Leila with Enter | Found guest; name repeated in snippet, oversized low-contrast kind badge, no result count | Scan one identity, useful details and a count | Library: results block; app supplies distinct snippet and search data |
| Search for an unknown guest | Plain “No records match” with no recovery action | Keep the query and offer a clear next step | Library: query-aware empty results and clear-search link |
| Cancel a guest edit | Returned to the original unchanged guest | Discard the pending change | Passed in lodge; neutral create cancel also returned correctly |
| Use More drawer at 390 and press Escape | Drawer closed and focus returned, but More still exposed aria-expanded=true | Expanded state agrees with what is open | Library: synchronize cancel and close; reproduced in both apps |
| Open the empty Notes list | “No note records yet. Create one” appeared as a bare sentence | A clear “Create your first note” action and why it helps | Library: first-record empty state; app chooses entity wording and permission |

The test booking's guest relation was corrected explicitly; this does not count as a passing unmodified create/edit path. Payment processing was a demo cash state transition, not a card charge. The amount display bug was retained as evidence, not silently worked around. Background `/api/views` and favicon 404s were observed but did not block these journeys.

App changes and deployment are outside this repository's authorized file scope. Library fixes below require consumers to adopt their APIs; the live applications still have the defects listed above.
