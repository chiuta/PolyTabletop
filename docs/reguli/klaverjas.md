# Klaverjas (`klaverjas`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play klaverjas --seed 1`. Sursa implementării: `polytabletop-games/src/q5_trick.rs, klaverjas.rs`.

> Rezumat din `polytabletop list`: Klaverjas olandez (Amsterdam) pentru doi, 32 de cărți, 8 fiecare, o carte întoarsă dă atuul, 4 mâini. Valetul de atu 20, noua 14; ordine simplă A > 10 > K > Q > J; ultima levată +10, pit +100. Răspunzi la culoare, bați atuul dacă poți. Mutări: Jh, 10s, Ac.

## Scop

Aduni mai multe puncte pe cele 4 mâini (152 + 10 pe mână).

## Pregătire

Pachet de 32 de cărți (7–A), 8 cărți fiecare; următoarea carte se întoarce și dă atuul.

## Desfășurarea turei

Joci o carte (`Jh`, `10s`); răspunzi la culoare; la atu cerut bați atuul dacă poți; fără culoare tai dacă ai atu.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `Jh`
* `10s`

## Simplificări și note

Adaptare pentru doi: fără roem (secvențe, stuk). AI-ul nu vede mâna adversarului; hot-seat arată doar mâna celui la mutare.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
