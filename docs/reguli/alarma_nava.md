# Alarma pe Navă (`alarma_nava`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play alarma_nava --seed 1`. Sursa implementării: `polytabletop-games/src/c2_siege.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de apărare: 4 culoare (punte, motoare, scuturi, arme); echipa (J1: Pilotul și Inginera, câte 2 acțiuni pe rând) apără o bază cu 12 viață. Fiecare acțiune joacă o carte (1–5) din mână: atac L k lovește amenințarea cea mai apropiată de pe culoarul L cu valoarea cărții k (pe care personajul o poate spori cu un bonus propriu). J2 este Jocul: mutarea forțată eveniment înaintează amenințările, cele care ajung rup baza și apar altele noi (a cincea parte din ele mai tari pe parcurs). Zidul (zid) absorb lovituri pe culoar. Cura (cur) dă +2 bazei. Câștigă echipa (J1) dacă elimină 12 amenințări; câștigă Jocul (J2) dacă baza cade sau timpul se depășește. Mutări: atac L k, zid L k, cur k, pas.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
