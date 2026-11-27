# Clanurile de Orhidee (`clanuri_de_orhidee`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play clanuri_de_orhidee --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu stâlpi și adăposturi împrăștiate. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Maestru de clan 4/4/1/4/5; Spadasin 3/4/1/3/4; Arcaș de ceremonie 2/3/5/2/3; Călugăr de templu 3/3/1/2/3; Ucenic 2/4/1/2/3. J1: Maestru de clan, 2× Spadasin, Arcaș de ceremonie, Călugăr de templu. J2: Maestru de clan, 2× Spadasin, Arcaș de ceremonie, 2× Ucenic. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: contraatac (supraviețuitorii din corp la corp lovesc înapoi); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri; vindecători (h). Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «bushido».

## Reguli

Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu stâlpi și adăposturi împrăștiate. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Maestru de clan 4/4/1/4/5; Spadasin 3/4/1/3/4; Arcaș de ceremonie 2/3/5/2/3; Călugăr de templu 3/3/1/2/3; Ucenic 2/4/1/2/3. J1: Maestru de clan, 2× Spadasin, Arcaș de ceremonie, Călugăr de templu. J2: Maestru de clan, 2× Spadasin, Arcaș de ceremonie, 2× Ucenic. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: contraatac (supraviețuitorii din corp la corp lovesc înapoi); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri; vindecători (h). Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
