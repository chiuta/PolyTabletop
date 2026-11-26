# Sheepshead (`sheepshead`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sheepshead --seed 1`. Sursa implementării: `polytabletop-games/src/sheepshead.rs`.

> Rezumat din `polytabletop list`: Levate cu 32 de cărți și atuuri fixe (Q, J, caro); adaptare pentru doi, cu blind și cărți îngropate. Mutări: iau, pas, ingrop 7c, apoi cartea jucată (ex. Qc).

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare pentru doi jucători (jocul clasic e pentru 3–5): blind și cărți îngropate.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Adaptare pentru doi jucători (jocul clasic e pentru 3–5): blind și cărți îngropate.
