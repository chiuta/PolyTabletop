# Manille (`manille`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play manille --seed 1`. Sursa implementării: `polytabletop-games/src/q5_trick.rs, manille.rs`.

> Rezumat din `polytabletop list`: Manille franceză pentru doi, 32 de cărți, 8 fiecare, o carte întoarsă dă atuul, 4 mâini. Ordine 10 > A > K > Q > J > 9 > 8 > 7, puncte 5-4-3-2-1; răspunzi la culoare și, dacă poți, mai sus, altfel tai. Mutări: 10h, As, 7c.

## Scop

Aduni mai multe puncte pe cele 4 mâini (60 pe mână + 10 pentru ultima levată).

## Pregătire

Pachet de 32 de cărți (7–A), 8 cărți fiecare; următoarea carte se întoarce și dă atuul.

## Desfășurarea turei

Joci o carte (`10h`, `As`); obligații: răspunzi la culoare și mai sus dacă poți; fără culoare tai cu atu dacă ai.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `10h`
* `As`

## Simplificări și note

Adaptare pentru doi, fără anunțuri (manille muette). AI-ul nu vede mâna adversarului; hot-seat arată doar mâna celui la mutare.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
