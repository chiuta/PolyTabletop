# Lame și Gheare (`lame_si_ghiare`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play lame_si_ghiare --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu stâlpi și adăposturi împrăștiate. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Mare Războinic 5/4/1/4/5; Gheară de pădure 3/5/1/3/3; Gardă de fier 5/2/1/3/5; Haiduc 3/3/1/3/4; Cercetaș cu arc 2/3/4/2/3. J1: Mare Războinic, Gheară de pădure, Gardă de fier, Haiduc, Cercetaș cu arc. J2: Mare Războinic, Gheară de pădure, Gardă de fier, Haiduc, Cercetaș cu arc. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: contraatac (supraviețuitorii din corp la corp lovesc înapoi); flancare (+1 zar dacă un aliat e lângă țintă); împingere (2+ pagube în corp la corp împing ținta). Unități speciale: lideri; armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «confrontation».

## Reguli

Variantă originală pentru doi pe o tablă 8×8 pătrată (8 direcții) cu stâlpi și adăposturi împrăștiate. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Mare Războinic 5/4/1/4/5; Gheară de pădure 3/5/1/3/3; Gardă de fier 5/2/1/3/5; Haiduc 3/3/1/3/4; Cercetaș cu arc 2/3/4/2/3. J1: Mare Războinic, Gheară de pădure, Gardă de fier, Haiduc, Cercetaș cu arc. J2: Mare Războinic, Gheară de pădure, Gardă de fier, Haiduc, Cercetaș cu arc. Atac: cel mai bun zar al atacatorului contra celui mai bun al apărătorului (țintă−1 zaruri); câștigă cel mai mare, la diferență de 3 paguba se dublează. Reguli speciale: contraatac (supraviețuitorii din corp la corp lovesc înapoi); flancare (+1 zar dacă un aliat e lângă țintă); împingere (2+ pagube în corp la corp împing ținta). Unități speciale: lideri; armură (cu 1 pagubă mai puțin). Câștigi doborând liderul inamic. Cel mult 14 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
