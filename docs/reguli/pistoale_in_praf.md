# Pistoale în Praf (`pistoale_in_praf`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pistoale_in_praf --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu dărâmături (adăpost) și câțiva pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pistolar 3/4/5/2/4; Pușcaș cu carabină 2/3/7/2/4; Șerif 4/4/4/3/4; Tâlhar 2/5/3/2/3; Doctor de vagon 2/3/1/1/3. J1: Pistolar, Pușcaș cu carabină, Șerif, Doctor de vagon. J2: 2× Tâlhar, Pistolar, Pușcaș cu carabină, Doctor de vagon. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: contraatac (supraviețuitorii din corp la corp lovesc înapoi). Unități speciale: lideri; vindecători (h); trăgători mobili. Adu jetoanele $ înapoi în primele tale două rânduri; primul la 3 câștigă. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «a fistful of lead».

## Reguli

Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu dărâmături (adăpost) și câțiva pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pistolar 3/4/5/2/4; Pușcaș cu carabină 2/3/7/2/4; Șerif 4/4/4/3/4; Tâlhar 2/5/3/2/3; Doctor de vagon 2/3/1/1/3. J1: Pistolar, Pușcaș cu carabină, Șerif, Doctor de vagon. J2: 2× Tâlhar, Pistolar, Pușcaș cu carabină, Doctor de vagon. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: contraatac (supraviețuitorii din corp la corp lovesc înapoi). Unități speciale: lideri; vindecători (h); trăgători mobili. Adu jetoanele $ înapoi în primele tale două rânduri; primul la 3 câștigă. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
