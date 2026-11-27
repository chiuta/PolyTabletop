# Vânătoarea de noapte (`vanatoarea_de_noapte`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play vanatoarea_de_noapte --seed 1`. Sursa: `polytabletop-games/src/vanatoarea_de_noapte.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Vânătorii din Liceu contra Frăția Colților), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. Rezerva de energie primește +2 la fiecare tură proprie (maximum 9); ce nu cheltui se păstrează. Fiecare are 4 sigilii: o lovitură directă rupe 1 sigiliu (2 dacă atacul e cel puțin 5); fără sigilii, următoarea lovitură directă câștigă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, nimicire, îngheț, pescuit, vindecare. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Vânătorii din Liceu, J2 Frăția Colților; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Elevă Curajoasă | 1 | 2/1 | — |
| Bibliotecar Șmecher | 1 | 1/1 | elan |
| Vrăjitor Mărunt | 2 | 2/2 | elan |
| Atlet de Seară | 2 | 4/1 | — |
| Vânătoare de Stafii | 3 | 4/3 | — |
| Vampir Tânăr | 3 | 3/2 | venin |
| Căpitan de Echipă | 4 | 5/2 | venin |
| Demon Grațios | 4 | 5/3 | zbor |
| Lider al Cavoului | 5 | 7/4 | — |
| Stăpânul Nopții | 6 | 7/5 | primul |

| Vrajă | Cost | Efect |
|---|---|---|
| Țăruș de Lemn | 3 | lovitură 4 |
| Argint în Buzunar | 2 | nimicire 3 |
| Beție Neagră | 1 | îngheț 0 |
| Ritual de Vindecare | 2 | pescuit 2 |
| Slujbă Rapidă | 3 | vindecare 2 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
