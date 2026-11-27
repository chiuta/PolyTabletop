# Corsari și amirali (`corsari_si_amirali`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play corsari_si_amirali --seed 1`. Sursa: `polytabletop-games/src/corsari_si_amirali.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Frăția Pânzelor Negre contra Flota Coroanei), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 16 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui una (sursa i): energia +1 (maximum 8); energia se reface la fiecare tură. Câștigi la 15 puncte de glorie: o unitate pregătită poate pleca în misiune (miss k: obosește și aduce glorie egală cu costul ei); atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, armură, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, întoarcere în mână, întărire, pescuit, vindecare. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Frăția Pânzelor Negre, J2 Flota Coroanei; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți, plus 11 cărți-sursă).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Mus Șmecher | 1 | 1/2 | — |
| Marinar Cu Cuțit | 1 | 1/2 | — |
| Tunar Surd | 2 | 1/4 | — |
| Cercetător din Gabie | 2 | 1/3 | sorbire |
| Spadasin de Punte | 3 | 3/3 | zbor |
| Căpitan de Briz | 3 | 3/3 | primul |
| Navigator Vrăjit | 4 | 4/4 | elan |
| Fregată Rapidă | 4 | 5/3 | armură |
| Amiral Încrâncenat | 5 | 5/6 | — |
| Vasul-Fantomă | 6 | 5/7 | elan |

| Vrajă | Cost | Efect |
|---|---|---|
| Salvă de Tun | 1 | lovitură 2 |
| Hartă Furată | 2 | întoarcere în mână 0 |
| Vânt din Pupa | 3 | întărire 3 |
| Cală Spartă | 1 | pescuit 1 |
| Rom și Prăjituri | 1 | vindecare 1 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
