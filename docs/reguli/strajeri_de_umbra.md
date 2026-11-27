# Străjerii din Umbră (`strajeri_de_umbra`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play strajeri_de_umbra --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 8×10 pătrată (4 direcții) cu coridoare cu pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Străjer 3/4/4/3/4; Căpitan de străji 4/4/3/3/4; Ghicitor 2/4/1/1/3; Ghoul de umbră 2/4/1/2/3; Cioban al umbrei 4/3/1/3/5; Spectru 3/5/1/2/4. J1: 3× Străjer, Căpitan de străji, Ghicitor. J2: 4× Ghoul de umbră, Cioban al umbrei, Spectru. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; zburători (ignoră terenul); vindecători (h). Jucătorul 1 trebuie să ducă 3 unități în ultimul rând; Jucătorul 2 câștigă distrugându-le sau ținând toate rundele. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «rangers of shadow deep».

## Reguli

Variantă originală pentru doi pe o tablă 8×10 pătrată (4 direcții) cu coridoare cu pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Străjer 3/4/4/3/4; Căpitan de străji 4/4/3/3/4; Ghicitor 2/4/1/1/3; Ghoul de umbră 2/4/1/2/3; Cioban al umbrei 4/3/1/3/5; Spectru 3/5/1/2/4. J1: 3× Străjer, Căpitan de străji, Ghicitor. J2: 4× Ghoul de umbră, Cioban al umbrei, Spectru. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri; zburători (ignoră terenul); vindecători (h). Jucătorul 1 trebuie să ducă 3 unități în ultimul rând; Jucătorul 2 câștigă distrugându-le sau ținând toate rundele. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
