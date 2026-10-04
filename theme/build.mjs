import { readFileSync, writeFileSync } from "node:fs";

const t = JSON.parse(readFileSync(new URL("./tokens.json", import.meta.url), "utf8"));
const kebab = (s) => s.replace(/[A-Z]/g, (c) => "-" + c.toLowerCase());
const pascal = (s) => s[0].toUpperCase() + s.slice(1);
const out = (name, body) => writeFileSync(new URL(`./dist/${name}`, import.meta.url), body);

const vars = [
  ...Object.entries(t.color).map(([k, v]) => `  --vox-${kebab(k)}: ${v};`),
  ...Object.entries(t.radius).map(([k, v]) => `  --vox-radius-${k}: ${v}px;`),
];
out(
  "vox-theme.css",
  `:root {\n${vars.join("\n")}\n  color-scheme: dark;\n}\n`,
);

const kt = [
  "package com.vox.theme",
  "",
  "import androidx.compose.ui.graphics.Color",
  "import androidx.compose.ui.unit.dp",
  "",
  "object VoxColors {",
  ...Object.entries(t.color).map(
    ([k, v]) => `    val ${pascal(k)} = Color(0xFF${v.slice(1).toUpperCase()})`,
  ),
  "}",
  "",
  "object VoxRadius {",
  ...Object.entries(t.radius).map(([k, v]) => `    val ${pascal(k)} = ${v}.dp`),
  "}",
  "",
].join("\n");
out("VoxTheme.kt", kt);
