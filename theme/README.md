# Vox theme (vox-shared/theme)

The single source of color for every Vox surface, in a dark and a light theme. Edit `tokens.json`, then run `node build.mjs`.

- `vox-web/src/app/theme.css` and `vox-desktop/src/styles/vox-theme.css` — written directly: `--vox-*` CSS variables for `:root`/`[data-theme="dark"]` and `[data-theme="light"]`.
- `dist/VoxTheme.kt` — Compose `VoxColors` (dark), `VoxLightColors` and `VoxRadius` for Android.

Apps set `data-theme` on `<html>` from the user's preference (`auto`, `light` or `dark`, stored in localStorage as `vox-theme`). Components never use literal colors; they use these variables or the utilities mapped from them.
