# Mituri și zei (`mituri_si_zei`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mituri_si_zei --seed 1`. Sursa: `polytabletop-games/src/mituri_si_zei.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Cetele Soarelui contra Cetele Trăsnetului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Resurse: 12 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 8); energia se reface la fiecare tură. Fiecare are 5 sigilii: o lovitură directă rupe 1 sigiliu (2 dacă atacul e cel puțin 4); fără sigilii, următoarea lovitură directă câștigă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (gardă, zbor, primul, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, binecuvântare, îngheț, nimicire, vindecare. Răsucire: toate unitățile au elan (atacă imediat). Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Cetele Soarelui, J2 Cetele Trăsnetului; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 12 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Păstor de Munte | 1 | 1/2 | — |
| Cântăreț de Șoapte | 1 | 1/1 | zbor |
| Războinic Tânăr | 2 | 2/3 | — |
| Sătean Curajos | 2 | 2/2 | zbor |
| Lăncier Zeificat | 3 | 4/3 | — |
| Vânător de Dragoni | 3 | 2/3 | venin |
| Semizeu Brav | 4 | 4/4 | armură |
| Demon al Prăpastiei | 4 | 4/5 | — |
| Zeul Furtunii | 5 | 6/4 | primul |
| Titanul Veșnic | 6 | 4/9 | gardă |

| Vrajă | Cost | Efect |
|---|---|---|
| Rază de Soare | 1 | lovitură 2 |
| Binecuvântare Zeiască | 2 | binecuvântare 1 |
| Capcană de Ceață | 2 | îngheț 0 |
| Pedeapsa Cerului | 4 | nimicire 9 |
| Jertfă Curată | 1 | vindecare 1 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
