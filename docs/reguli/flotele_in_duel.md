# Flotele în duel (`flotele_in_duel`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play flotele_in_duel --seed 1`. Sursa: `polytabletop-games/src/flotele_in_duel.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Armada Albastră contra Flota Răzvrătită), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 15 ture pe jucător decide scorul. Resurse: 12 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 8); energia se reface la fiecare tură. Fiecare are 3 zone cu 6 viață; o unitate lovește o zonă (atac k zN); cu două zone distruse pierzi. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: măturare, lovitură, nimicire, îngheț, pescuit. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Armada Albastră, J2 Flota Răzvrătită; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 12 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Navă de Marfă | 1 | 2/1 | — |
| Vânător Ușor | 1 | 1/2 | gardă |
| Corvetă Iute | 2 | 4/1 | — |
| Navă de Asalt | 2 | 1/3 | zbor |
| Bombardier Greoi | 3 | 5/1 | elan |
| Fregată Blindată | 3 | 3/4 | — |
| Distrugător Greu | 4 | 6/2 | zbor |
| Nava Comandă | 4 | 7/2 | — |
| Gigant de Război | 5 | 6/4 | armură |
| Stația Mobilă | 6 | 7/4 | venin |

| Vrajă | Cost | Efect |
|---|---|---|
| Bombardament Stelar | 3 | măturare 1 |
| Salvă Unită | 2 | lovitură 3 |
| Zid de Scuturi | 4 | nimicire 9 |
| Capcană Magnetică | 1 | îngheț 0 |
| Alarmă Generală | 2 | pescuit 2 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
