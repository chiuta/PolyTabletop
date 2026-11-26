# Shatranj (Persia) (`shatranj`)

Strămoșul șahului: ferz (un pas pe diagonală), alfil (sare exact 2 pe diagonală), cal, turn, rege; pionul avansează cu un pas și se promovează în ferz; fără rocadă. Câștigi prin mat, pat sau rege gol (cu excepția contra-capturii). Mutări: e2e3, b1c3.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/shatranj.rs`.

