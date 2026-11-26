# Tâb (Egipt) (`tab`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tab --seed 1`. Sursa implementării: `polytabletop-games/src/tab.rs`.

> Rezumat din `polytabletop list`: Cursă cu 4 bețe-zaruri pe tablă 4 × 7 (variantă simplificată): 7 piese pe jucător; intri cu 1; 1, 4 și 6 dau aruncare în plus; drum în zigzag pe cele 4 rânduri; aterizarea pe o piesă adversă (în afara rândului ei de plecare) o captură. Câștigă primul care scoate toate piesele. Mutări: arunca, intra-a1, a1-a3, a1-iese, pas.

## Scop

Scoți primul toate cele 7 piese.

## Pregătire

Tablă 4 × 7; 7 piese pe jucător, în rezervă. Se aruncă 4 bețe cu două fețe.

## Desfășurarea turei

Aruncarea (`arunca`) dă 1, 2, 3 sau 4 (câte fețe plate în sus); niciuna = 6. 1, 4 și 6 dau o nouă aruncare. O piesă intră în joc doar cu un 1, pe primul pătrat al drumului ei. Drumul parcurge rândurile în zigzag (rândul propriu stânga → dreapta, apoi dreapta → stânga etc.); Jucătorul 2 pornește din rândul opus (rotit cu 180°). Aterizarea pe o piesă adversă o captură (se întoarce în rezervă), cu excepția pieselor din rândul de plecare al proprietarului.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunca`.
* `intra-a1`: intri în joc (pătratul de start al drumului tău).
* `a1-a3`: muți piesa de pe a1 pe a3; `a1-iese`: piesa iese.
* `pas` dacă nu ai nicio mutare.

## Reguli speciale și variante alese

* Piesele din rândul de plecare al proprietarului nu pot fi capturate.
* Ieșirea se face cu orice aruncare care depășește sfârșitul drumului.

## Punctaj

Piesele scoase.

## Sfârșitul jocului

Câștigă primul care scoate toate cele 7 piese.

## Note despre AI

Joc cu zaruri: AI-ul alege mutarea după aruncare, cu adâncime 1 (nu „vede” aruncările viitoare).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Regulile istorice (dimensiunea tablei, rândurile suplimentare, regula ultimului rând) variază mult; aceasta este o simplificare.
* Aruncările sunt fără informație ascunsă; generatorul e propriu jocului (seed determinist cu `--seed`).
