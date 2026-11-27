# Imperii de stele (`imperii_de_stele`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play imperii_de_stele --seed 1`. Sursa: `polytabletop-games/src/imperii_de_stele.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Imperiul Zorilor contra Liga Neagră), mână ascunsă. Start: 5 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9, și se reface la fiecare tură. Fiecare are 4 sigilii: o lovitură directă rupe 1 sigiliu (2 dacă atacul e cel puțin 5); fără sigilii, următoarea lovitură directă câștigă. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, gardă, zbor, primul, armură); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: măturare, lovitură, binecuvântare, vindecare, trezire. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Imperiul Zorilor, J2 Liga Neagră; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Pilot de Patrulă | 1 | 2/1 | — |
| Dronă de Recunoaștere | 1 | 2/1 | — |
| Fregată Ușoară | 2 | 2/2 | zbor |
| Nava-Spital | 2 | 2/3 | — |
| Crucișător Mic | 3 | 3/3 | primul |
| Distrugător | 3 | 2/4 | elan |
| Cuirasat Pitic | 4 | 5/4 | — |
| Nava Amiral | 4 | 3/6 | gardă |
| Stație de Luptă | 5 | 4/6 | armură |
| Planeta-Fortăreață | 6 | 5/8 | gardă |

| Vrajă | Cost | Efect |
|---|---|---|
| Salvă Orbitală | 3 | măturare 1 |
| Scut Reînnoit | 2 | lovitură 3 |
| Armistițiu Rupt | 2 | binecuvântare 1 |
| Reparații | 2 | vindecare 1 |
| Mobilizare | 2 | trezire 0 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
