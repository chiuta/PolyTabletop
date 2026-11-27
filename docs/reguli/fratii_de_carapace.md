# Frații de carapace (`fratii_de_carapace`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fratii_de_carapace --seed 1`. Sursa: `polytabletop-games/src/fratii_de_carapace.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Frații din Canal contra Clanul Labei), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9, și se reface la fiecare tură. Câștigi la 15 puncte de glorie: o unitate pregătită poate pleca în misiune (miss k: obosește și aduce glorie egală cu costul ei); atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, armură); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, binecuvântare, trezire, întărire, îngheț. Răsucire: daunele nu se vindecă niciodată. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Frații din Canal, J2 Clanul Labei; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Ucenic de Canal | 1 | 1/2 | — |
| Ninja Mărunt | 1 | 1/1 | primul |
| Pizzar Grăbit | 2 | 2/3 | — |
| Savant Ciudat | 2 | 3/1 | elan |
| Țestoasă Cu Băț | 3 | 4/3 | — |
| Țestoasă Cu Sabie | 3 | 2/4 | zbor |
| Mutant Cu Gheare | 4 | 4/4 | armură |
| Maestru Șobolan | 4 | 3/5 | zbor |
| Robot Cu Creier | 5 | 5/6 | — |
| Stăpânul Din Turn | 6 | 7/5 | armură |

| Vrajă | Cost | Efect |
|---|---|---|
| Lovitură Cu Bățul | 1 | lovitură 2 |
| Pizza Caldă | 5 | binecuvântare 3 |
| Salt de Pe Acoperiș | 3 | trezire 0 |
| Cutie Misterioasă | 2 | întărire 2 |
| Ceață Fumigenă | 2 | îngheț 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
