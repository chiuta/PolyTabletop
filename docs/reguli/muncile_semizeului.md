# Muncile semizeului (`muncile_semizeului`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play muncile_semizeului --seed 1`. Sursa: `polytabletop-games/src/muncile_semizeului.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Eroii Olimpului contra Monștrii Labirintului), mână ascunsă. Start: 5 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8, și se reface la fiecare tură. Fiecare are 5 sigilii: o lovitură directă rupe 1 sigiliu (2 dacă atacul e cel puțin 4); fără sigilii, următoarea lovitură directă câștigă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (gardă, zbor, primul, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: binecuvântare, lovitură, vindecare, nimicire, îngheț. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Eroii Olimpului, J2 Monștrii Labirintului; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Păstor de Capre | 1 | 1/2 | — |
| Atlet Tânăr | 2 | 2/2 | primul |
| Satir Glumeț | 2 | 2/2 | zbor |
| Nimfă a Izvorului | 3 | 4/3 | — |
| Centaur Arcaș | 3 | 1/4 | venin |
| Hidră Mică | 4 | 3/6 | — |
| Leul de Piatră | 4 | 2/6 | armură |
| Taur de Bronz | 5 | 2/9 | gardă |
| Gorgonă Mohorâtă | 6 | 5/8 | — |
| Titanul Încătușat | 7 | 5/10 | gardă |

| Vrajă | Cost | Efect |
|---|---|---|
| Binecuvântarea Muzelor | 5 | binecuvântare 3 |
| Lovitură de Trăsnet | 3 | lovitură 4 |
| Cântec de Liră | 1 | vindecare 1 |
| Daruri de Nuntă | 3 | nimicire 5 |
| Pietrificare | 1 | îngheț 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
