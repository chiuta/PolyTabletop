# Cercurile nopții (`cercurile_noptii`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cercurile_noptii --seed 1`. Sursa: `polytabletop-games/src/cercurile_noptii.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Cercul Trandafirului contra Cercul Lanțului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână drept sursă (sursa i): energia +1 (maximum 8), dar cartea se pierde; energia se reface la fiecare tură. Fiecare are 4 sigilii: o lovitură directă rupe 1 sigiliu (2 dacă atacul e cel puțin 4); fără sigilii, următoarea lovitură directă câștigă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: nimicire, lovitură, pescuit, întoarcere în mână, îngheț. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Cercul Trandafirului, J2 Cercul Lanțului; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Servitor de Noapte | 1 | 1/2 | — |
| Muzician Palid | 1 | 1/1 | zbor |
| Păstrător de Taine | 2 | 3/2 | — |
| Curtezan Pătat | 2 | 2/3 | — |
| Duelist Elegant | 3 | 3/2 | venin |
| Lup-Chip | 3 | 4/2 | elan |
| Erudit Sângeriu | 4 | 7/2 | — |
| Prinț al Mahalalei | 4 | 5/3 | primul |
| Bătrân de Cavou | 5 | 5/4 | venin |
| Arhonte Ascuns | 6 | 7/5 | zbor |

| Vrajă | Cost | Efect |
|---|---|---|
| Pecete Roșie | 2 | nimicire 3 |
| Mască Falsă | 1 | lovitură 2 |
| Dar al Sângelui | 1 | pescuit 1 |
| Cădere Sub Voal | 1 | întoarcere în mână 0 |
| Plecăciune Mortală | 2 | îngheț 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
