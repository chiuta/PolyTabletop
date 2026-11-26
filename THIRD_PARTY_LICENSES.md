# Licențe terțe / Third-party licenses

## Fonturi încorporate în `polytabletop-gui` (`polytabletop-gui/assets/fonts/`)

Fonturile sunt încorporate în binar cu `include_bytes!`; nu se descarcă nimic la rulare.
Fișierele Cormorant Garamond, Lora și JetBrains Mono provin din subseturile `latin` + `latin-ext`
distribuite de Google Fonts (aceleași pe care le încorporează aplicațiile „Fără Frontiere” de pe
chiuta.ro), convertite din WOFF2 în TTF și îmbinate cu `fontTools.merge`; nu s-a modificat niciun contur.

| Font | Fișier | Licență | Text complet |
|---|---|---|---|
| Cormorant Garamond (Regular, SemiBold) | `CormorantGaramond-*.ttf` | SIL OFL 1.1 | `polytabletop-gui/assets/fonts/OFL-CormorantGaramond.txt` |
| Lora (Regular, Medium) | `Lora-*.ttf` | SIL OFL 1.1 | `polytabletop-gui/assets/fonts/OFL-Lora.txt` |
| JetBrains Mono (Regular) | `JetBrainsMono-Regular.ttf` | SIL OFL 1.1 | `polytabletop-gui/assets/fonts/OFL-JetBrainsMono.txt` |
| DejaVu Sans (rezervă pentru simboluri) | `DejaVuSans.ttf` (nemodificat) | Bitstream Vera / DejaVu (permisivă) | `polytabletop-gui/assets/fonts/LICENSE-DejaVu.txt` |

* Cormorant Garamond: Copyright 2015 The Cormorant Project Authors (https://github.com/CatharsisFonts/Cormorant)
* Lora: Copyright 2011 The Lora Project Authors (https://github.com/cyrealtype/Lora-Cyrillic)
* JetBrains Mono: Copyright 2020 The JetBrains Mono Project Authors (https://github.com/JetBrains/JetBrainsMono)
* DejaVu: Copyright (c) 2003 Bitstream, Inc.; modificările DejaVu sunt domeniu public.

Fonturile OFL nu sunt vândute separat de program; sunt redistribuite împreună cu textul licenței.

## Dependențe Rust

Licențele dependențelor Rust (egui/eframe, etc.) sunt cele ale crate-urilor respective
(MIT / Apache-2.0 / etc.); vezi `cargo tree` și `Cargo.lock`.
