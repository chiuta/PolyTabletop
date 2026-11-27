# Gardienele de cristal (`gardiene_de_cristal`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play gardiene_de_cristal --seed 1`. Sursa: `polytabletop-games/src/gardiene_de_cristal.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Cercul Lunii contra Cercul Umbrei), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Rezerva de energie primește +2 la fiecare tură proprie (maximum 9); ce nu cheltui se păstrează. Câștigi când viața adversarului ajunge la 0 (start 22); lovitura directă scade viața cu atacul unității. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, armură, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: vindecare, lovitură, binecuvântare, trezire, întărire. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Cercul Lunii, J2 Cercul Umbrei; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Fetiță Cu Stea | 1 | 2/1 | — |
| Prietenă Credincioasă | 1 | 1/1 | sorbire |
| Dansatoare de Rouă | 2 | 3/2 | — |
| Gardiană a Florii | 2 | 3/2 | — |
| Cântăreață de Argint | 3 | 4/2 | zbor |
| Războinică Cu Panglică | 3 | 3/3 | elan |
| Prințesă Din Cer | 4 | 4/5 | — |
| Regina Cristalului | 4 | 4/4 | armură |
| Vrăjitoare Neagră | 5 | 5/6 | gardă |
| Sfera Lunii Pline | 6 | 5/7 | zbor |

| Vrajă | Cost | Efect |
|---|---|---|
| Dans de Primăvară | 3 | vindecare 2 |
| Rază de Lună | 2 | lovitură 3 |
| Petale Tămăduitoare | 5 | binecuvântare 3 |
| Chemarea Prietenelor | 2 | trezire 0 |
| Strălucire | 3 | întărire 3 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
