# Planeta cu Spini (`planeta_cu_spini`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play planeta_cu_spini --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 9×9 hexagonală cu ruine (pereți și adăpost). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Infanterist colonial 3/4/5/3/4; Sergent colonial 4/4/4/3/4; Flăcărar 3/3/3/2/3; Vierme de spini 2/5/1/2/4; Zburător de spini 2/6/1/2/3; Colos de spini 7/3/1/4/5. J1: 2× Infanterist colonial, Sergent colonial, Flăcărar. J2: 3× Vierme de spini, 2× Zburător de spini, Colos de spini. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; zburători (ignoră terenul); atacuri în arie (1 pagubă în jurul țintei, și aliaților); armură (cu 1 pagubă mai puțin). Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 7 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «project pandora».

## Reguli

Variantă originală pentru doi pe o tablă 9×9 hexagonală cu ruine (pereți și adăpost). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Infanterist colonial 3/4/5/3/4; Sergent colonial 4/4/4/3/4; Flăcărar 3/3/3/2/3; Vierme de spini 2/5/1/2/4; Zburător de spini 2/6/1/2/3; Colos de spini 7/3/1/4/5. J1: 2× Infanterist colonial, Sergent colonial, Flăcărar. J2: 3× Vierme de spini, 2× Zburător de spini, Colos de spini. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; zburători (ignoră terenul); atacuri în arie (1 pagubă în jurul țintei, și aliaților); armură (cu 1 pagubă mai puțin). Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 7 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
