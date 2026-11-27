# Pădure cu Mușchete (`padure_cu_muschete`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play padure_cu_muschete --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 9×8 pătrată (8 direcții) cu dărâmături (adăpost) și câțiva pereți. La fiecare tură un zar de comandă dă 2–4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pușcaș de graniță 2/4/5/2/4; Luptător cu topor 3/5/1/2/3; Locotenent 3/4/3/2/4; Gealat de pădure 3/4/1/2/4. J1: Locotenent, 2× Pușcaș de graniță, Luptător cu topor, Gealat de pădure. J2: Locotenent, 2× Pușcaș de graniță, Luptător cu topor, Gealat de pădure. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); imobilizare (focul la distanță încetinește ținta). Unități speciale: lideri. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 7 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «muskets & tomahawks».

## Reguli

Variantă originală pentru doi pe o tablă 9×8 pătrată (8 direcții) cu dărâmături (adăpost) și câțiva pereți. La fiecare tură un zar de comandă dă 2–4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pușcaș de graniță 2/4/5/2/4; Luptător cu topor 3/5/1/2/3; Locotenent 3/4/3/2/4; Gealat de pădure 3/4/1/2/4. J1: Locotenent, 2× Pușcaș de graniță, Luptător cu topor, Gealat de pădure. J2: Locotenent, 2× Pușcaș de graniță, Luptător cu topor, Gealat de pădure. Atac: arunci zarurile unității; fiecare zar cel puțin egal cu ținta apărătorului lovește. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); imobilizare (focul la distanță încetinește ținta). Unități speciale: lideri. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 7 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
