# Caravanele de nisip (`caravane_nisip`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play caravane_nisip --seed 1`. Sursa: `polytabletop-games/src/caravane_nisip.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Casa Fântânii contra Casa Șacalului), mână ascunsă. Start: 5 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8, și se reface la fiecare tură. Câștigi la 15 puncte de glorie: o unitate pregătită poate pleca în misiune (miss k: obosește și aduce glorie egală cu costul ei); atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, armură, venin); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: îngheț, pescuit, întoarcere în mână, lovitură, vindecare. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Casa Fântânii, J2 Casa Șacalului; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Cămilar Grăbit | 1 | 1/2 | — |
| Negustor de Apă | 2 | 2/2 | zbor |
| Păzitor de Oază | 2 | 1/4 | — |
| Călăreț Mirajelor | 3 | 2/5 | — |
| Scorpion Uriaș | 3 | 1/5 | zbor |
| Gardă Cu Voal | 4 | 2/6 | armură |
| Maestru de Caravană | 4 | 4/5 | — |
| Vierme de Dună | 5 | 3/6 | venin |
| Vizir Lacom | 6 | 6/6 | armură |
| Marele Emir | 7 | 6/8 | elan |

| Vrajă | Cost | Efect |
|---|---|---|
| Furtună de Praf | 2 | îngheț 0 |
| Fântână Ascunsă | 1 | pescuit 1 |
| Drum Întors | 1 | întoarcere în mână 0 |
| Lovitură de Iatagan | 2 | lovitură 3 |
| Odihnă la Umbră | 1 | vindecare 1 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
