# FreqChain

Frequency sidechaining plugin

![](docs/FreqChain%20UI.png)

### Features

- Frequency sidechaining
- Sidechain signal
  - Input gain
  - Mono/Stereo switch
  - Detail (spectral smoothing)
  - Precision (spectral sharpening)
  - Per-bin temporal smoothing
  - Standard 7-band EQ
  - Solo/Audition (currently only with input gain, mono, and EQ applied)

### Controls

- Shift+drag: granular dragging
- Double-click/Ctrl+click (Command on macOS): reset to default
  - Applies to sliders, knobs, and band nodes (resets frequency/gain)
- (Shift+)scroll: changes Q value when hovering a band node
  - (slider and knob scrolling is planned)

## Building

`cargo xtask bundle freqchain --release`

> [!NOTE]  
> `xtask` is an aliased command in `.cargo/config`