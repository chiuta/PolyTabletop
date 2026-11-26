# Dame rusești (Shashki) (`shashki`)

8×8: piesele simple mută înainte, dar captură și înapoi; dama zboară pe diagonală și capturează de la distanță. Captura e obligatorie, salturile continuă; piesa care atinge ultimul rând în captură devine damă și continuă. Mutări: c3-d4, c3xe5xg7. Pierde cine nu mai poate muta.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/shashki.rs`.

