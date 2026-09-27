# Vox Shared

A small Rust crate for types that would otherwise end up copy-pasted (and drifting) across [vox-core](https://github.com/vox-suite/vox-core) and [vox-desktop](https://github.com/vox-suite/vox-desktop).

Right now it holds one thing: `sms`, the SMS-classification taxonomy and result shape used by both vox-core's cloud extractor and vox-desktop's on-device one, so they can't silently disagree on categories.

## Adding to it

Add a module here only for something that's *already* duplicated verbatim in two places in the Vox codebases — not speculatively. If you're not sure, it probably doesn't belong here yet.
