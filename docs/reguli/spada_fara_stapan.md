# Sabia fără Stăpân (`spada_fara_stapan`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play spada_fara_stapan --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 7×7 pătrată (4 direcții) cu stâlpi și adăposturi împrăștiate. La fiecare tură ai 2 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Samurai fără stăpân 4/4/1/4/5; Lăncier rătăcit 3/4/1/3/4; Mercenar 3/3/1/3/4; Arcaș de drum 2/3/4/2/3. J1: Samurai fără stăpân, Lăncier rătăcit, Arcaș de drum. J2: 2× Mercenar, 2× Arcaș de drum. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: contraatac (supraviețuitorii din corp la corp lovesc înapoi); împingere (2+ pagube în corp la corp împing ținta). Unități speciale: lideri. Câștigi distrugând toate unitățile inamice. Cel mult 16 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «ronin».

## Reguli

Variantă originală pentru doi pe o tablă 7×7 pătrată (4 direcții) cu stâlpi și adăposturi împrăștiate. La fiecare tură ai 2 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Samurai fără stăpân 4/4/1/4/5; Lăncier rătăcit 3/4/1/3/4; Mercenar 3/3/1/3/4; Arcaș de drum 2/3/4/2/3. J1: Samurai fără stăpân, Lăncier rătăcit, Arcaș de drum. J2: 2× Mercenar, 2× Arcaș de drum. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: contraatac (supraviețuitorii din corp la corp lovesc înapoi); împingere (2+ pagube în corp la corp împing ținta). Unități speciale: lideri. Câștigi distrugând toate unitățile inamice. Cel mult 16 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
