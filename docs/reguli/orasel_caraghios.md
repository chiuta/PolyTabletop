# Orășelul caraghios (`orasel_caraghios`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play orasel_caraghios --seed 1`. Sursa: `polytabletop-games/src/orasel_caraghios.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Familia Din Colț contra Vecinii Zgomotoși), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Rezerva de energie primește +2 la fiecare tură proprie (maximum 9); ce nu cheltui se păstrează. Câștigi la 12 puncte de glorie: o unitate pregătită poate pleca în misiune (miss k: obosește și aduce glorie egală cu costul ei); atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, armură); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: întoarcere în mână, pescuit, întărire, lovitură, îngheț. Răsucire: toate unitățile au elan (atacă imediat). Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Familia Din Colț, J2 Vecinii Zgomotoși; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Pisică de Canapea | 1 | 2/1 | — |
| Bunică Cu Mătură | 1 | 1/1 | zbor |
| Copil Cu Praștie | 1 | 2/1 | — |
| Vânzător de Gogoși | 2 | 2/2 | elan |
| Primar Pomădat | 2 | 2/2 | armură |
| Paznic Somnoros | 3 | 3/3 | primul |
| Profesor Cu Cretă | 3 | 5/2 | — |
| Hoț de Vinerea | 4 | 3/5 | zbor |
| Șeful Poliției | 5 | 7/3 | elan |
| Starul Televiziunii | 6 | 7/6 | — |

| Vrajă | Cost | Efect |
|---|---|---|
| Tort în Față | 2 | întoarcere în mână 0 |
| Zvon Răspândit | 3 | pescuit 3 |
| Glumă Veche | 2 | întărire 2 |
| Petrecere Mare | 1 | lovitură 2 |
| Pană de Curent | 2 | îngheț 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
