# Chowka Bara (`chowka_bara`)

Joc de cursă indian pe tablă 5×5 cu patru piese pe jucător și patru scoici ca zaruri (4 și 8 dau o tură în plus). O piesă intră în joc doar cu 4 sau 8; piesele fac un tur pe marginea tablei, apoi intră în spirala interioară până la centru. Capturezi piese adverse pe pătratele neprotejate; câștigă cine aduce toate piesele în centru.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/chowka_bara.rs`.

