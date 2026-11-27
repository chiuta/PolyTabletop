# Hrube și Comori (`hrube_si_comori`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play hrube_si_comori --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 8×8 pătrată (4 direcții) cu coridoare cu pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Cavaler de peșteră 4/3/1/3/5; Preot al luminii 3/3/2/2/3; Mag al scânteii 3/3/5/2/3; Hoț cu arc 2/4/5/2/3; Orc de hrubă 4/3/1/2/4; Scheletar 1/3/1/2/3; Trol de peșteră 6/2/1/3/4. J1: Cavaler de peșteră, Preot al luminii, Mag al scânteii, Hoț cu arc. J2: 2× Orc de hrubă, 2× Scheletar, Trol de peșteră. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: contraatac (supraviețuitorii din corp la corp lovesc înapoi). Unități speciale: atacuri în arie (1 pagubă în jurul țintei, și aliaților); vindecători (h). Adu jetoanele $ înapoi în primele tale două rânduri; primul la 3 câștigă. Cel mult 16 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «dungeons & dragons miniatures».

## Reguli

Variantă originală pentru doi pe o tablă 8×8 pătrată (4 direcții) cu coridoare cu pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Cavaler de peșteră 4/3/1/3/5; Preot al luminii 3/3/2/2/3; Mag al scânteii 3/3/5/2/3; Hoț cu arc 2/4/5/2/3; Orc de hrubă 4/3/1/2/4; Scheletar 1/3/1/2/3; Trol de peșteră 6/2/1/3/4. J1: Cavaler de peșteră, Preot al luminii, Mag al scânteii, Hoț cu arc. J2: 2× Orc de hrubă, 2× Scheletar, Trol de peșteră. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: contraatac (supraviețuitorii din corp la corp lovesc înapoi). Unități speciale: atacuri în arie (1 pagubă în jurul țintei, și aliaților); vindecători (h). Adu jetoanele $ înapoi în primele tale două rânduri; primul la 3 câștigă. Cel mult 16 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), h uN uM, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
