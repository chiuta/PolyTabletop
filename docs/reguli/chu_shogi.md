# Chu Shogi (12×12) (`chu_shogi`)

Șahul japonez mare pe tablă 12×12 cu 46 de piese pe parte (Leu, Regină, Tigru, Elefant ebriu, Căpcăun etc.). Mutări scrise ca traseu: g6-g7, leul poate face doi pași (g6-g7-g8), iar `+` promovează la intrarea în zona adversă. Captura prințului sau a regelui câștigă (versiune simplificată, fără regula de repetare).

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/chu_shogi.rs`.

