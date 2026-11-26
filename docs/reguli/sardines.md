# Sardines (`sardines`)

Ascunsul inversat, două runde cu roluri schimbate, într-o casă cu 9 camere (grilă 3×3). Ascunsul alege în secret camera (`ascund N`); căutătorul pornește din camera 1 și face câte un pas (`ir N`), apoi `asculta` arată dacă l-a găsit sau un indiciu public (respirații aproape / liniște). Camerele încă posibile sunt marcate cu «?». Ascunsul poate sta (`sta`) sau se poate muta (`ma N`). Ascunsul primește atâtea puncte câți pași a făcut căutătorul (13 dacă nu e găsit în 10).

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: da (vezi doar mâna ta; în hot-seat se arată mâna jucătorului la mutare). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/sardines.rs`.

