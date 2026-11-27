# Culoare de Oțel (`culoare_de_otel`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play culoare_de_otel --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 7×11 pătrată (4 direcții) cu coridoare cu pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Gardian în armură 4/3/5/3/5; Sergent în armură 4/3/5/3/5; Roi de ghiare 2/6/1/2/3; Patriarh al roiului 5/5/1/3/4. J1: 4× Gardian în armură, Sergent în armură. J2: 6× Roi de ghiare, Patriarh al roiului. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; armură (cu 1 pagubă mai puțin). Jucătorul 1 trebuie să ducă 2 unități în ultimul rând; Jucătorul 2 câștigă distrugându-le sau ținând toate rundele. Cel mult 16 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «space hulk».

## Reguli

Variantă originală pentru doi pe o tablă 7×11 pătrată (4 direcții) cu coridoare cu pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Gardian în armură 4/3/5/3/5; Sergent în armură 4/3/5/3/5; Roi de ghiare 2/6/1/2/3; Patriarh al roiului 5/5/1/3/4. J1: 4× Gardian în armură, Sergent în armură. J2: 6× Roi de ghiare, Patriarh al roiului. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; armură (cu 1 pagubă mai puțin). Jucătorul 1 trebuie să ducă 2 unități în ultimul rând; Jucătorul 2 câștigă distrugându-le sau ținând toate rundele. Cel mult 16 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
