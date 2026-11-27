# Creaturi de pixeli (`creaturi_de_pixeli`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play creaturi_de_pixeli --seed 1`. Sursa: `polytabletop-games/src/creaturi_de_pixeli.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Echipa Verde contra Echipa Violet), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână drept sursă (sursa i): energia +1 (maximum 8), dar cartea se pierde; energia se reface la fiecare tură. Câștigi la 14 puncte de glorie: o unitate pregătită poate pleca în misiune (miss k: obosește și aduce glorie egală cu costul ei); atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: întărire, binecuvântare, lovitură, pescuit, trezire. Răsucire: toate unitățile au elan (atacă imediat). Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Echipa Verde, J2 Echipa Violet; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Boț Pufos | 1 | 2/1 | — |
| Pui de Pixel | 1 | 2/1 | — |
| Sprinter Cubic | 1 | 2/1 | — |
| Dragonel de Octeți | 2 | 2/2 | zbor |
| Cățel de Cod | 2 | 2/1 | venin |
| Șarpe Cu Ecran | 3 | 5/2 | — |
| Gigant de Biți | 3 | 3/3 | primul |
| Zburător Holografic | 4 | 3/5 | elan |
| Gazda Cuantică | 5 | 6/4 | armură |
| Dragonul Suprem | 6 | 7/5 | elan |

| Vrajă | Cost | Efect |
|---|---|---|
| Evoluție Rapidă | 2 | întărire 2 |
| Upgrade Total | 2 | binecuvântare 1 |
| Atac de Pixeli | 1 | lovitură 2 |
| Cartela Norocoasă | 1 | pescuit 1 |
| Reîncărcare | 3 | trezire 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
