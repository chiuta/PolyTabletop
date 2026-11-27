# Clanurile oazei (`clanuri_de_oaza`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play clanuri_de_oaza --seed 1`. Sursa: `polytabletop-games/src/clanuri_de_oaza.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Clanul Palmierului contra Clanul Scorpionului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Rezerva de energie primește +3 la fiecare tură proprie (maximum 10); ce nu cheltui se păstrează. Fiecare are 5 sigilii: o lovitură directă rupe 1 sigiliu (2 dacă atacul e cel puțin 4); fără sigilii, următoarea lovitură directă câștigă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: binecuvântare, lovitură, întărire, nimicire, vindecare. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Clanul Palmierului, J2 Clanul Scorpionului; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Culegător de Curmale | 1 | 1/2 | — |
| Fântânar | 1 | 2/1 | — |
| Tânăr Săgetător | 2 | 1/4 | gardă |
| Cămilă Încărcată | 2 | 3/1 | zbor |
| Mag de Nisip | 3 | 4/2 | armură |
| Garda Oazei | 3 | 3/4 | — |
| Șef de Trib | 4 | 3/6 | gardă |
| Vânător de Vipere | 4 | 4/3 | venin |
| Marele Înțelept | 5 | 5/5 | elan |
| Gigantul Dunelor | 6 | 5/8 | — |

| Vrajă | Cost | Efect |
|---|---|---|
| Ploaie Rară | 4 | binecuvântare 2 |
| Săgeată de Vânt | 2 | lovitură 3 |
| Dans de Foc | 1 | întărire 1 |
| Pecete de Clan | 2 | nimicire 3 |
| Boală de Praf | 1 | vindecare 1 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
