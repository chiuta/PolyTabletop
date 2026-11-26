# Cursa cămilelor (pariuri) (`camile`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Camel Up”. Pornire: `polytabletop play camile --seed 1`. Sursa implementării: `polytabletop-games/src/camile.rs`.

> Reguli: Cinci cămile (A–E) pe o pistă de 16 celule, mutate cu zaruri extrase din pahar. O tură: `zar` (cămila lui avansează 1–3 celule cu cele de pe ea; +1 monedă), `pariu A` (iei cea mai de sus bilă de pariu a etapei: 5, 3 sau 2), `castig A` / `pierd A` (cartea cămilei pe câștigătoarea cursei / pe ultima). Etapa se termină când toate zarurile au fost folosite: bila aduce valoarea ei dacă cămila e prima, 1 monedă dacă e a doua, altfel −1. Cursa se termină când o cămilă trece de ultima celulă; cărțile se plătesc în ordinea punerii: 8, 5, 3, 2, 1 pentru corecte, −1 pentru greșite. Câștigă cine are mai multe monede. Mutări: zar, pariu A, castig A, pierd A.
