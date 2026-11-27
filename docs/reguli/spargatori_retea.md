# Spărgătorii de rețea (`spargatori_retea`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play spargatori_retea --seed 1`. Sursa: `polytabletop-games/src/spargatori_retea.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Echipa Furișată contra Paznicii Nodului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână drept sursă (sursa i): energia +1 (maximum 8), dar cartea se pierde; energia se reface la fiecare tură. Câștigi la 14 puncte de glorie: o unitate pregătită poate pleca în misiune (miss k: obosește și aduce glorie egală cu costul ei); atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, armură); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, pescuit, îngheț, întoarcere în mână, întărire. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Echipa Furișată, J2 Paznicii Nodului; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Curier Fantomă | 1 | 2/1 | — |
| Sondă Mută | 1 | 1/1 | primul |
| Spărgător de Parole | 2 | 1/3 | zbor |
| Păianjen de Cod | 2 | 2/3 | — |
| Sentinelă Digitală | 3 | 2/4 | armură |
| Vierme de Releu | 3 | 2/4 | zbor |
| Gardian de Pod | 4 | 4/4 | elan |
| Arhitect de Breșe | 4 | 4/4 | elan |
| Ceață Neagră | 5 | 5/6 | — |
| Nucleul Orb | 6 | 6/7 | — |

| Vrajă | Cost | Efect |
|---|---|---|
| Impuls Invers | 2 | lovitură 3 |
| Descărcare Rapidă | 1 | pescuit 1 |
| Îngheț de Port | 1 | îngheț 0 |
| Retragere Forțată | 2 | întoarcere în mână 0 |
| Ghid de Spargere | 3 | întărire 3 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
