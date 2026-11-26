# Turnurile din Hanoi (Tower of Hanoi) (`hanoi`)

Cursă: fiecare are propriul turn cu 5 discuri pe tija a și îl mută pe tija c (a-c = discul de sus de pe a trece pe c), niciodată un disc mare peste unul mic. Mutați pe rând; câștigă cine termină cu mai puține mutări (minim 31; egal = remiză).

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/hanoi.rs`.

