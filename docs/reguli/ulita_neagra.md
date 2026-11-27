# Ulița Neagră (`ulita_neagra`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ulita_neagra --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu ruine (pereți și adăpost). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Vigilent 4/5/1/3/4; Acrobat 2/6/1/2/3; Aruncător de cuțite 2/3/4/2/3; Căpetenie de bandă 4/4/1/2/4; Gealat 3/4/1/2/4; Pungaș 2/5/1/1/3. J1: Vigilent, Acrobat, Aruncător de cuțite. J2: Căpetenie de bandă, 2× Gealat, 2× Pungaș. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: flancare (+1 zar dacă un aliat e lângă țintă); contraatac (supraviețuitorii din corp la corp lovesc înapoi); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; trăgători mobili. Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «batman miniature game».

## Reguli

Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu ruine (pereți și adăpost). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Vigilent 4/5/1/3/4; Acrobat 2/6/1/2/3; Aruncător de cuțite 2/3/4/2/3; Căpetenie de bandă 4/4/1/2/4; Gealat 3/4/1/2/4; Pungaș 2/5/1/1/3. J1: Vigilent, Acrobat, Aruncător de cuțite. J2: Căpetenie de bandă, 2× Gealat, 2× Pungaș. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: flancare (+1 zar dacă un aliat e lângă țintă); contraatac (supraviețuitorii din corp la corp lovesc înapoi); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; trăgători mobili. Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
