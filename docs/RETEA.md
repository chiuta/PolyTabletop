# Joc în rețea (doi oameni)

```
polytabletop host ID [--port P] [--seed N] [--side 1|2|random] [--timeout SECUNDE]
polytabletop join HOST:PORT
```

`host` ascultă pe `0.0.0.0:P` (implicit 7878), joacă el însuși ca Jucătorul 1 (sau 2 / aleatoriu cu `--side`)
și așteaptă un client. `join` se conectează și primește cealaltă latură. În terminal: o mutare, `?` (mutările legale),
`state`, `chat TEXT`, `q`. Fiecare parte vede doar informația ei privată (`set_viewer(Some(latură))`).
Atenție: jocurile care nu implementează `set_viewer` (vezi `HOTSEAT_SAFE`) pot arăta informație privată în STATE.

## Protocol `polytabletop/1`

Linii UTF-8 terminate cu `\n`; comenzile nu țin cont de majuscule; linie de la client maxim 1024 octeți.

Client → server: `HELLO polytabletop/1`, `MOVE text`, `LEGAL` (doar la rândul tău), `STATE` (retrimite starea), `chat TEXT`
(max 200 caractere, retransmis adversarului), `QUIT`.

Server → client:

| Mesaj | Sens |
|---|---|
| `HELLO polytabletop/1` | răspuns la handshake |
| `START id seed latură` | jocul începe; latură = 1 sau 2 |
| `STATE tură n` + `n` linii | randarea pentru acest jucător; tură = 1/2 (cine mută) sau 0 (terminat) |
| `OK mutare` / `ERR motiv` | rezultatul unui `MOVE` / orice eroare |
| `LEGAL m1<TAB>m2...` | mutările legale ale celui la rând |
| `CHAT latură text` | mesaj de la adversar |
| `END win1\|win2\|draw\|aborted motiv` | sfârșit; apoi conexiunea se închide |

Serverul este autoritar: singurul joc (`GameObj`) e la el, orice `MOVE` trece prin `GameObj::play`; mutarea adversarului
nu se retransmite (ar putea fi privată), clientul primește noul `STATE`. Handshake-ul are timeout (10 s); cine nu mută în
`--timeout` secunde (600 implicit) pierde; deconectarea sau excluderea (30 de linii greșite) dă `END aborted`; linii prea
lungi, UTF-8 invalid, comenzi necunoscute primesc `ERR`; conexiunile în plus primesc `ERR jocul este plin`.
Codul: `polytabletop/src/net.rs`, `polytabletop/src/proto.rs`; teste: `polytabletop/tests/net_loopback.rs`.
