# Șase Lumi în Flăcări (`sase_lumi_in_flacari`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sase_lumi_in_flacari --seed 1`. Sursa implementării: `polytabletop-games/src/sase_lumi_in_flacari.rs` (motor comun `polytabletop-games/src/t5_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T5.tsv`.

> Variantă originală pentru doi: pachete fixe construite de joc (Legiunea Aurie contra Roiul Verde), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9. Fiecare are 3 zone cu 8 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: nicio unitate nu are boală de invocare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

Cărțile (zece unități și cinci vrăji) și cele două pachete fixe sunt definite în `SPEC` din fișierul sursă; informația ascunsă (mâinile, pachetele, scuturile) este protejată prin `set_viewer` (HOTSEAT_SAFE), iar AI-ul are adâncime fixă 0 și nu citește informația ascunsă a adversarului.
