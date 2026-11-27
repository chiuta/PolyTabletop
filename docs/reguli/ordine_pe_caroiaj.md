# Ordine pe Caroiaj (`ordine_pe_caroiaj`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ordine_pe_caroiaj --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 12×8 pătrată (4 direcții) cu teren deschis. La fiecare tură un zar de comandă dă 2–4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Unitate de linie 3/3/1/3/4; Unitate de arcași 2/3/4/2/3; Unitate de care 3/5/1/3/4; General 3/4/1/3/4. J1: 2× Unitate de linie, 2× Unitate de arcași, Unitate de care, General. J2: 2× Unitate de linie, 2× Unitate de arcași, Unitate de care, General. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 8 câștigă. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «to the strongest!».

## Reguli

Variantă originală pentru doi pe o tablă 12×8 pătrată (4 direcții) cu teren deschis. La fiecare tură un zar de comandă dă 2–4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Unitate de linie 3/3/1/3/4; Unitate de arcași 2/3/4/2/3; Unitate de care 3/5/1/3/4; General 3/4/1/3/4. J1: 2× Unitate de linie, 2× Unitate de arcași, Unitate de care, General. J2: 2× Unitate de linie, 2× Unitate de arcași, Unitate de care, General. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); zone de control (mutarea lângă un inamic te oprește). Unități speciale: lideri. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 8 câștigă. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
