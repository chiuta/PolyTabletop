# Sueca (`sueca`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sueca --seed 1`. Sursa implementării: `polytabletop-games/src/q5_trick.rs, sueca.rs`.

> Rezumat din `polytabletop list`: Sueca portugheză pentru doi, pachet de 40 de cărți (fără 8, 9, 10), 10 fiecare, o carte întoarsă dă atuul, 4 mâini. Ordine A > 7 > K > J > Q > 6 > 5 > 4 > 3 > 2, puncte A 11, 7 10, K 4, J 3, Q 2; răspunzi la culoare dacă poți. Mutări: Ah, 7s, Qc.

## Scop

Aduni mai multe puncte de carte pe cele 4 mâini.

## Pregătire

Pachet de 40 de cărți (fără 8, 9, 10), 10 cărți fiecare; următoarea carte se întoarce și dă atuul.

## Desfășurarea turei

Joci o carte (`Ah`, `7s`); trebuie să răspunzi la culoare dacă poți; fără culoare arunci orice (nu există obligația de a tăia).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `Ah`
* `7s`

## Simplificări și note

Adaptare pentru doi (jocul se joacă de obicei în patru). AI-ul nu vede mâna adversarului; hot-seat arată doar mâna celui la mutare.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
