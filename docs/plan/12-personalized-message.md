# 12. Personalized daily message

## 12.1 Profile settings

Let the user configure:

- Role and career stage.
- Current priorities.
- Preferred tone.
- Formality.
- Warmth.
- Concision.
- Humor.
- Metaphor density.
- Whether literary language is welcome.
- Topics or expressions to avoid.

Offer tone presets such as:

- Warm and cutesy.
- Calm and direct.
- Scholarly and austere.
- Literary and introspective.
- Energetic coach.
- Minimalist.

Prefer descriptive style controls such as “19th-century philosophical prose” rather than promising exact imitation of a particular writer.

## 12.2 Generation policy

Generate once per day and cache the result.

Context should be limited to:

- The user’s chosen name.
- Role.
- Today’s principal milestone.
- Recent completion or streak.
- Explicit tone settings.

Do not send Scratchpad bodies, research Logs, imported annotations, paper text, or sensitive project content unless the user explicitly opts in to that context and provider. Sharing a note across local projects does not grant AI consent. A chosen milestone is optional; missing one must not trigger a required planning step.

Fallback message templates must exist for:

- Offline mode.
- Provider failure.
- Missing API key.
- Safety or privacy settings.

The message should encourage without falsely claiming knowledge of the user’s emotional state.

---

[← Previous](11-native-pdf.md) · [Plan index](README.md) · [Next →](13-technical-architecture.md)
