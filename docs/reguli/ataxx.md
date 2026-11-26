# Ataxx (7×7) (`ataxx`)

Două piese în colțuri opuse. La mutare clonezi o piesă pe o groapă liberă la distanță 1 (a1-b2) sau o sari la distanță 2 (a1-c3); toate piesele adverse vecine cu destinația devin ale tale. Fără mutări: pas. Câștigă cine are mai multe piese când tabla se umple sau nimeni nu mai poate muta.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/ataxx.rs`.

