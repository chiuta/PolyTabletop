# Cărți Prăfuite (`carti_prafuite`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play carti_prafuite --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 9×8 pătrată (8 direcții) cu stâlpi și adăposturi împrăștiate. La fiecare tură un zar de comandă dă 2–4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pistolar de noroc 2/4/5/2/3; Căpitanul bandei 3/4/4/3/4; Bătăuș de bar 3/3/1/3/4; Hoț de cai 2/6/1/2/3. J1: Căpitanul bandei, 2× Pistolar de noroc, Bătăuș de bar, Hoț de cai. J2: Căpitanul bandei, 2× Pistolar de noroc, Bătăuș de bar, Hoț de cai. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: împingere (2+ pagube în corp la corp împing ținta). Unități speciale: lideri; trăgători mobili. Adu jetoanele $ înapoi în primele tale două rânduri; primul la 3 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «dead man's hand».

## Reguli

Variantă originală pentru doi pe o tablă 9×8 pătrată (8 direcții) cu stâlpi și adăposturi împrăștiate. La fiecare tură un zar de comandă dă 2–4 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Pistolar de noroc 2/4/5/2/3; Căpitanul bandei 3/4/4/3/4; Bătăuș de bar 3/3/1/3/4; Hoț de cai 2/6/1/2/3. J1: Căpitanul bandei, 2× Pistolar de noroc, Bătăuș de bar, Hoț de cai. J2: Căpitanul bandei, 2× Pistolar de noroc, Bătăuș de bar, Hoț de cai. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: împingere (2+ pagube în corp la corp împing ținta). Unități speciale: lideri; trăgători mobili. Adu jetoanele $ înapoi în primele tale două rânduri; primul la 3 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
