# Mikado (Pick-up sticks) (`mikado`)

Un morman de 21 de bețișoare (Mikado 20, Mandarini 10, Bonzi 5, Samurai 3, Kuli 2). Ridici bețișorul `ia N`: dacă nu are altele deasupra reușești sigur și continui; dacă este acoperit riști (`incerc`: șanse 50% pentru un bețișor acoperit, 25% pentru două …), iar la eșec pierzi rândul. `pas` încheie rândul după ce ai ridicat ceva. Când mormanul e gol câștigă cine are mai multe puncte.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/mikado.rs`.

