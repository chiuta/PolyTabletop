# Mingea de Piele (`mingea_de_piele`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mingea_de_piele --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 7×9 pătrată (8 direcții) cu teren deschis. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Atacant 2/6/1/2/3; Mijlocaș 3/4/1/2/4; Fundaș 4/3/1/2/5; Portar 3/3/1/2/5. J1: 2× Atacant, 2× Mijlocaș, Fundaș, Portar. J2: 2× Atacant, 2× Mijlocaș, Fundaș, Portar. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: împingere (2+ pagube în corp la corp împing ținta); zone de control (mutarea lângă un inamic te oprește). Unități speciale: trăgători mobili. Du mingea (o) în rândul din spate al adversarului ca să marchezi, pasează cu p; primul la 3 câștigă. Cel mult 16 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), p uN uM, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «guild ball».

## Reguli

Variantă originală pentru doi pe o tablă 7×9 pătrată (8 direcții) cu teren deschis. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Atacant 2/6/1/2/3; Mijlocaș 3/4/1/2/4; Fundaș 4/3/1/2/5; Portar 3/3/1/2/5. J1: 2× Atacant, 2× Mijlocaș, Fundaș, Portar. J2: 2× Atacant, 2× Mijlocaș, Fundaș, Portar. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: împingere (2+ pagube în corp la corp împing ținta); zone de control (mutarea lângă un inamic te oprește). Unități speciale: trăgători mobili. Du mingea (o) în rândul din spate al adversarului ca să marchezi, pasează cu p; primul la 3 câștigă. Cel mult 16 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), p uN uM, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
