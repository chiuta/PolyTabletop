# Snap (`snap`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play snap --seed 1`. Sursa implementării: `polytabletop-games/src/snap.rs`.

> Rezumat din `polytabletop list`: Pachet de 52 împărțit în două; pe rând întorci (intorc) câte o carte în mijloc. Când ultimele două cărți au aceeași valoare, cine e la mutare poate striga snap și ia grămada; un snap fals costă o carte. Cine rămâne fără cărți pierde.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare: strigătul „snap” e o mutare disponibilă celui la mutare.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Adaptare: strigătul „snap” e o mutare disponibilă celui la mutare.
