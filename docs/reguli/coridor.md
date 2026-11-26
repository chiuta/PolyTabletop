# Coridor (cursă de pioni cu ziduri) (`coridor`)

Coridor: cursă de pioni pe tablă 9×9 cu ziduri. Reguli publice, implementare originală.

## Scop

Ajungi primul cu pionul pe rândul opus: J1 (X) pe rândul 9, J2 (O) pe rândul 1.

## Pregătire

J1 pornește din e1, J2 din e9. Fiecare are 10 ziduri.

## Desfășurarea unei ture

Pe rând: muți pionul un pas ortogonal (liber de ziduri) SAU pui un zid lung cât două pătrate.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `e2` — muți pionul pe e2
* `d4h` — zid orizontal peste d4–e4, între rândurile 4 și 5
* `d4v` — zid vertical între coloanele d și e, rândurile 4–5

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play coridor --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `e2` — pionul avansează de la e1 la e2.
2. `e3` — apoi la e3.
3. `e4` — apoi la e4.

## Reguli speciale și variante alese

Pionul adversar vecin se sare în linie dreaptă; dacă în spatele lui e zid, margine sau nimic, se poate sări în diagonală. Zidurile nu se pot suprapune, încrucișa în cruce sau lăsa un jucător fără nicio cale către rândul său țintă.

## Punctaj

Nu există scor.

## Sfârșitul jocului

Primul pe rândul țintă câștigă; remiză la 200 de semi-mutări (protecție împotriva buclelor).

## Note despre AI și informația ascunsă

Informație completă; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp.

## Diferențe față de regulile tradiționale

Reguli publice; remiza la 200 de semi-mutări este o protecție a implementării.
