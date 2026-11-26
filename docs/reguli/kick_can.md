# Kick the Can (`kick_can`)

Joc de ascuns și prins pe un teren 7×7. J1 este Căutătorul (S), J2 conduce trei ascunși (1–3); cutia O stă în centru (d4). Fiecare face două pași pe tură în 8 direcții (`ne`, `s`…; ascunșii `h2 ne`). Căutătorul prinde un ascuns călcând pe el; un ascuns liber care pășește pe cutie îi eliberează pe toți prinșii. Căutătorul câștigă când toți 3 sunt prinși; ascunșii câștigă dacă rezistă 20 de ture.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/kick_can.rs`.

