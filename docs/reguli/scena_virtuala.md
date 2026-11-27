# Scena virtuală (`scena_virtuala`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play scena_virtuala --seed 1`. Sursa: `polytabletop-games/src/scena_virtuala.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Trupa Aurorei contra Trupa Mării), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. Rezerva de energie primește +3 la fiecare tură proprie (maximum 10); ce nu cheltui se păstrează. Câștigi la 13 puncte de glorie: o unitate pregătită poate pleca în misiune (miss k: obosește și aduce glorie egală cu costul ei); atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: vindecare, pescuit, întărire, trezire, lovitură. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Trupa Aurorei, J2 Trupa Mării; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Fan Entuziast | 1 | 2/1 | — |
| Cântăreață de Început | 1 | 1/1 | zbor |
| Dansator de Fundal | 2 | 3/1 | sorbire |
| Chitaristă Zâmbitoare | 2 | 3/1 | elan |
| Prezentatoare Glumeață | 3 | 2/4 | zbor |
| Regizoare Fantezistă | 3 | 5/2 | — |
| Vedetă în Ascensiune | 4 | 3/5 | sorbire |
| Idol Strălucitor | 4 | 4/5 | — |
| Dirijor de Efecte | 5 | 7/4 | — |
| Marea Divă | 6 | 6/6 | primul |

| Vrajă | Cost | Efect |
|---|---|---|
| Duet Surpriză | 2 | vindecare 1 |
| Transmisie Live | 3 | pescuit 3 |
| Aplauze | 3 | întărire 3 |
| Bis | 2 | trezire 0 |
| Reflector Orb | 2 | lovitură 3 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
