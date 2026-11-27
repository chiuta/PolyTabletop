# Zeii de Lut (`zeii_de_lut`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zeii_de_lut --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 9×8 hexagonală cu dealuri (înălțimi). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Zeu al furtunii 6/4/4/4/5; Erou de lut 5/4/1/3/5; Zeiță a vânătorii 3/3/6/3/3; Monstru de piatră 7/3/1/4/5; Preoteasă 3/3/1/1/3. J1: Zeu al furtunii, Erou de lut, Zeiță a vânătorii, Preoteasă. J2: Zeu al furtunii, Erou de lut, Monstru de piatră, Zeiță a vânătorii. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: împingere (2+ pagube în corp la corp împing ținta); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri; atacuri în arie (1 pagubă în jurul țintei, și aliaților); vindecători (h); armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «mythic battles: pantheon».

## Reguli

Variantă originală pentru doi pe o tablă 9×8 hexagonală cu dealuri (înălțimi). La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Zeu al furtunii 6/4/4/4/5; Erou de lut 5/4/1/3/5; Zeiță a vânătorii 3/3/6/3/3; Monstru de piatră 7/3/1/4/5; Preoteasă 3/3/1/1/3. J1: Zeu al furtunii, Erou de lut, Zeiță a vânătorii, Preoteasă. J2: Zeu al furtunii, Erou de lut, Monstru de piatră, Zeiță a vânătorii. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: împingere (2+ pagube în corp la corp împing ținta); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri; atacuri în arie (1 pagubă în jurul țintei, și aliaților); vindecători (h); armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
