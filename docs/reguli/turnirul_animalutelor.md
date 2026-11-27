# Turnirul animăluțelor (`turnirul_animalutelor`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play turnirul_animalutelor --seed 1`. Sursa: `polytabletop-games/src/turnirul_animalutelor.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Echipa Fulgerilor contra Echipa Frunzelor), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Resurse: 10 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 8); energia se reface la fiecare tură. Câștigi la 14 puncte de glorie: o unitate pregătită poate pleca în misiune (miss k: obosește și aduce glorie egală cu costul ei); atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, armură); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: pescuit, vindecare, lovitură, binecuvântare, trezire. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Echipa Fulgerilor, J2 Echipa Frunzelor; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 10 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Pisoi Jucăuș | 1 | 1/2 | — |
| Iepuraș Iute | 1 | 1/1 | elan |
| Căței Zglobiu | 2 | 3/1 | zbor |
| Pasăre Cântătoare | 2 | 1/3 | zbor |
| Vulpiță Isteață | 3 | 4/2 | primul |
| Dragonel de Buzunar | 3 | 4/3 | — |
| Urs de Pluș | 4 | 4/4 | elan |
| Fluture Uriaș | 4 | 5/4 | — |
| Grifon Tânăr | 5 | 5/5 | armură |
| Marele Campion | 6 | 7/6 | — |

| Vrajă | Cost | Efect |
|---|---|---|
| Rația Dublă | 3 | pescuit 3 |
| Masaj Bun | 1 | vindecare 1 |
| Săritură Ușoară | 2 | lovitură 3 |
| Premiul Mare | 2 | binecuvântare 1 |
| Dorm Puțin | 3 | trezire 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
