# Șoapte din abis (`soapte_din_abis`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play soapte_din_abis --seed 1`. Sursa: `polytabletop-games/src/soapte_din_abis.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Veghea Clopotului contra Ucenicii Mării), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9, și se reface la fiecare tură. Câștigi când viața adversarului ajunge la 0 (start 24); lovitura directă scade viața cu atacul unității. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (gardă, zbor, armură, venin, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: nimicire, întoarcere în mână, lovitură, îngheț, pescuit. Răsucire: daunele nu se vindecă niciodată. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Veghea Clopotului, J2 Ucenicii Mării; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Pescar Rătăcit | 1 | 1/2 | — |
| Cărturar Neliniștit | 2 | 3/2 | — |
| Locuitor de Far | 2 | 2/2 | sorbire |
| Prunc cu Ochi Mari | 3 | 3/3 | zbor |
| Anchetator Obosit | 3 | 2/5 | gardă |
| Tentacul Plutitor | 4 | 3/6 | — |
| Preot al Valurilor | 4 | 2/5 | venin |
| Dormitor din Adânc | 5 | 4/6 | armură |
| Chip Fără Nume | 6 | 6/7 | — |
| Marele Dinafară | 7 | 4/9 | venin |

| Vrajă | Cost | Efect |
|---|---|---|
| Ruinarea Minții | 3 | nimicire 5 |
| Maree Neagră | 2 | întoarcere în mână 0 |
| Șoaptă Tulburătoare | 1 | lovitură 2 |
| Cântec de Adormire | 2 | îngheț 0 |
| Rupere de Ceas | 2 | pescuit 2 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
