# Duhurile Pârjolului (`duhuri_pirjol`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play duhuri_pirjol --seed 1`. Sursa implementării: `polytabletop-games/src/duhuri_pirjol.rs` (motor comun `polytabletop-games/src/t5_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T5.tsv`.

> Variantă originală pentru doi: pachete fixe construite de joc (Clanul Cremenii contra Clanul Cenușii), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Fiecare are 5 scuturi: lovitura directă reușită rupe 1 scut (2 dacă atacul e cel puțin 4) și îl dă în mână adversarului; lovitura fără scuturi câștigă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

Cărțile (zece unități și cinci vrăji) și cele două pachete fixe sunt definite în `SPEC` din fișierul sursă; informația ascunsă (mâinile, pachetele, scuturile) este protejată prin `set_viewer` (HOTSEAT_SAFE), iar AI-ul are adâncime fixă 0 și nu citește informația ascunsă a adversarului.
