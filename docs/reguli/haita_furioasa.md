# Haita furioasă (`haita_furioasa`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play haita_furioasa --seed 1`. Sursa: `polytabletop-games/src/haita_furioasa.rs` (motor comun `polytabletop-games/src/t6_tcg.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_T6.tsv`.

> Variantă originală pentru doi: pachete fixe de 30 de cărți construite de joc (Haita Lunii Roșii contra Stejarii Bătrâni), mână ascunsă. Start: 6 cărți (J2 primește una în plus); la începutul turei tragi o carte (cine nu mai poate trage pierde); după 14 ture pe jucător decide scorul. Rezerva de energie primește +2 la fiecare tură proprie (maximum 9); ce nu cheltui se păstrează. Câștigi când viața adversarului ajunge la 0 (start 24); lovitura directă scade viața cu atacul unității. Până la 6 unități pe câmp, cu cost, atac/viață și cuvinte-cheie (elan, zbor, primul, venin, sorbire); unitățile noi nu pot ataca (decât cu elan); în luptă ambele lovesc, daunele se șterg la sfârșitul turei; lovitura directă e oprită de o gardă neobosită; zburătorii pot fi loviți doar de zburători. Vrăji: lovitură, întărire, măturare, îngheț, pescuit. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: sursa i, joc i [țintă eN|mN|f|zN], atac k țintă, miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Cărțile

Pachetele: J1 Haita Lunii Roșii, J2 Stejarii Bătrâni; fiecare are 30 de cărți fixe (două compoziții diferite din aceleași 15 cărți).

| Unitate | Cost | Atac/viață | Cuvânt-cheie |
|---|---|---|---|
| Pui de Lup | 1 | 2/1 | — |
| Vânător de Hățiș | 1 | 1/1 | elan |
| Urs Cenușiu | 1 | 1/1 | zbor |
| Pază de Pădure | 2 | 2/3 | — |
| Mistreț Sălbatic | 2 | 3/1 | sorbire |
| Vulpe Fantomă | 3 | 3/4 | — |
| Alfa Furios | 3 | 3/4 | — |
| Căprioară de Foc | 4 | 3/5 | primul |
| Gigant al Stâncii | 5 | 5/5 | elan |
| Fiara Primordială | 6 | 5/6 | venin |

| Vrajă | Cost | Efect |
|---|---|---|
| Mușcătură | 1 | lovitură 2 |
| Chemarea Haitei | 2 | întărire 2 |
| Urlet Cumplit | 4 | măturare 2 |
| Capcană de Ramuri | 2 | îngheț 0 |
| Furie Oarbă | 1 | pescuit 1 |

Informația ascunsă (mâinile, ordinea pachetelor) este protejată prin `set_viewer` (HOTSEAT_SAFE); AI-ul are adâncime fixă 0, nu citește mâna sau pachetul adversarului (doar numărul cărților) și nu trișează.
