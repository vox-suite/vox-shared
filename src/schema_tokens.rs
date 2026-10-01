/*!
* The canonical 24-slot color/icon palette for `data_schemas.color_token`/
* `icon_token`. The server only ever stores an index 0-23 (see
* `vox-core/migrations/20260929000001_schema_display_tokens.sql`); this
* module is the one place that says what each index actually *means* --
* every renderer (vox-desktop's CSS, vox-android's Compose port) mirrors
* these exact values instead of inventing its own.
*
* Colors are defined in OKLCH, not hex: at a fixed lightness and chroma,
* varying only hue gives 24 genuinely evenly-perceived colors, which
* plain hex/HSL picks don't -- HSL's perceived brightness swings wildly
* by hue at a fixed L. L/C here (72%, 0.14) are tuned to sit alongside
* vox-desktop's existing accent colors (coral-pulse, electric-sky,
* success-green), which cluster in roughly that same range.
*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklch {
    /// Lightness, 0.0-1.0.
    pub l: f32,
    /// Chroma, roughly 0.0-0.4 in practice.
    pub c: f32,
    /// Hue, degrees, 0.0-360.0.
    pub h: f32,
}

pub const TOKEN_COUNT: usize = 24;

const PALETTE_L: f32 = 0.72;
const PALETTE_C: f32 = 0.14;

/// 24 hues, 15 degrees apart, starting near vox-desktop's existing
/// coral-pulse accent so the new palette reads as an extension of it
/// rather than a clash.
pub const COLOR_TOKENS: [Oklch; TOKEN_COUNT] = [
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 20.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 35.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 50.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 65.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 80.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 95.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 110.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 125.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 140.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 155.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 170.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 185.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 200.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 215.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 230.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 245.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 260.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 275.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 290.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 305.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 320.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 335.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 350.0,
    },
    Oklch {
        l: PALETTE_L,
        c: PALETTE_C,
        h: 5.0,
    },
];

/// 24 lucide icon names (kebab-case, matching lucide-react's naming) for
/// common personal-data categories. Each Rust/TS/Kotlin renderer looks
/// this name up in its own icon library -- the name is the shared
/// contract, not a specific icon component.
pub const ICON_TOKENS: [&str; TOKEN_COUNT] = [
    "wallet",
    "heart-pulse",
    "utensils",
    "car",
    "home",
    "briefcase",
    "plane",
    "dumbbell",
    "book-open",
    "music",
    "film",
    "camera",
    "gamepad-2",
    "shopping-bag",
    "coffee",
    "pill",
    "moon",
    "cloud-sun",
    "phone-call",
    "message-circle",
    "map-pin",
    "party-popper",
    "users",
    "laptop",
];

/// Reference OKLCH -> sRGB conversion (Bjorn Ottosson's OKLab formulas).
/// Ports of this palette to a renderer with no native OKLCH support
/// (e.g. Android's Compose `Color`, which has no OKLCH ColorSpace) should
/// mirror this exact algorithm rather than hand-picking approximate hex
/// equivalents, so the actual rendered colors stay consistent with the
/// CSS `oklch()` values vox-desktop renders natively.
pub fn oklch_to_srgb(color: Oklch) -> [u8; 3] {
    let h_rad = color.h.to_radians();
    let a = color.c * h_rad.cos();
    let b = color.c * h_rad.sin();

    let l_ = color.l + 0.396_337_78 * a + 0.215_803_76 * b;
    let m_ = color.l - 0.105_561_346 * a - 0.063_854_17 * b;
    let s_ = color.l - 0.089_484_18 * a - 1.291_485_5 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    let r_lin = 4.076_741_7 * l - 3.307_711_6 * m + 0.230_969_94 * s;
    let g_lin = -1.268_438 * l + 2.609_757_4 * m - 0.341_319_38 * s;
    let b_lin = -0.0041960863 * l - 0.703_418_6 * m + 1.707_614_7 * s;

    [
        gamma_encode(r_lin),
        gamma_encode(g_lin),
        gamma_encode(b_lin),
    ]
}

fn gamma_encode(linear: f32) -> u8 {
    let c = linear.clamp(0.0, 1.0);
    let encoded = if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    (encoded.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// `oklch(L% C H)`, directly usable as a CSS color value.
pub fn oklch_css(color: Oklch) -> String {
    format!(
        "oklch({}% {} {})",
        (color.l * 100.0).round(),
        color.c,
        color.h
    )
}

pub fn color_token(index: i32) -> Option<Oklch> {
    usize::try_from(index)
        .ok()
        .and_then(|i| COLOR_TOKENS.get(i))
        .copied()
}

pub fn icon_token(index: i32) -> Option<&'static str> {
    usize::try_from(index)
        .ok()
        .and_then(|i| ICON_TOKENS.get(i))
        .copied()
}
