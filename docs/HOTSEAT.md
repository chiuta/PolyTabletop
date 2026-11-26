# Hot-seat și vizualizare (set_viewer)

Convenția: `set_viewer(None)` = hot-seat (se arată informația privată a jucătorului la mutare); `set_viewer(Some(p))` = vederea fixă a lui `p`. Lista `HOTSEAT_SAFE` din `polytabletop-games/src/lib.rs` conține jocurile care respectă convenția.

Fiecare joc jucabil are exact una dintre cele trei stări de mai jos. Testul `polytabletop-games/tests/a1_viewer_audit.rs` verifică acest lucru: un joc nou care nu e în `HOTSEAT_SAFE`, `SOLO_GOD` sau `NO_HIDDEN` (listele din test) face testul să pice.

* **SAFE**: informație privată reală; `render` respectă `set_viewer` (în `HOTSEAT_SAFE`).
* **GOD**: un om (J1) contra roboților/șansei, cu informație ascunsă; `set_viewer(Some(Two))` adaugă la sfârșitul `render` o secțiune `--- VEDERE TOTALĂ (J2) ---` cu tot ce e ascuns (roluri, mâini, soluție), ca un al doilea om să poată inspecta; celelalte vederi nu o conțin. Jocurile GOD nu sunt în `HOTSEAT_SAFE` (nu au al doilea jucător uman real).
* **NO_HIDDEN**: nimic privat unui loc (informație perfectă, zaruri/cărți publice, mâini deschise, sau ascundere simetrică).

| id | stare | observații |
|---|---|---|
| litere | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| curieri | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| fitil | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| codru | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| conac | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| insula | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| magistrale | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| bresla | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| catun | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| solzi | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| salina | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| reverdire | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| rezervatia | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| epoci | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| hotar | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| capcana | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| cristale | SAFE | mâna/cărțile private ale locului vizualizat; adversarul apare doar ca număr (adăugat în a1) |
| lupii | GOD | vedere totală pentru `Some(Two)` |
| santinela | GOD | vedere totală pentru `Some(Two)` |
| consiliul | GOD | vedere totală pentru `Some(Two)` |
| valea | GOD | vedere totală pentru `Some(Two)` |
| hruba | GOD | vedere totală pentru `Some(Two)` |
| roade | GOD | vedere totală pentru `Some(Two)` |
| ceasornic | GOD | vedere totală pentru `Some(Two)` |
| farul | GOD | vedere totală pentru `Some(Two)` |
| chess | NO_HIDDEN | informație perfectă sau publică |
| draughts | NO_HIDDEN | informație perfectă sau publică |
| reversi | NO_HIDDEN | informație perfectă sau publică |
| connect4 | NO_HIDDEN | informație perfectă sau publică |
| gomoku | NO_HIDDEN | informație perfectă sau publică |
| hex | NO_HIDDEN | informație perfectă sau publică |
| amazoane | NO_HIDDEN | informație perfectă sau publică |
| linii | NO_HIDDEN | informație perfectă sau publică |
| nim | NO_HIDDEN | informație perfectă sau publică |
| morris | NO_HIDDEN | informație perfectă sau publică |
| mancala | NO_HIDDEN | informație perfectă sau publică |
| go | NO_HIDDEN | informație perfectă sau publică |
| xiangqi | NO_HIDDEN | informație perfectă sau publică |
| shogi | NO_HIDDEN | informație perfectă sau publică |
| dara | NO_HIDDEN | informație perfectă sau publică |
| morabaraba | NO_HIDDEN | informație perfectă sau publică |
| sungka | NO_HIDDEN | informație perfectă sau publică |
| makruk | NO_HIDDEN | informație perfectă sau publică |
| janggi | NO_HIDDEN | informație perfectă sau publică |
| backgammon | NO_HIDDEN | informație perfectă sau publică |
| ludo | NO_HIDDEN | informație perfectă sau publică |
| snakes | NO_HIDDEN | informație perfectă sau publică |
| miniaturi | NO_HIDDEN | informație perfectă sau publică |
| yut | NO_HIDDEN | informație perfectă sau publică |
| sugoroku | NO_HIDDEN | informație perfectă sau publică |
| chaupar | NO_HIDDEN | informație perfectă sau publică |
| mosii | NO_HIDDEN | informație perfectă sau publică |
| mozaic | NO_HIDDEN | informație perfectă sau publică |
| faianta | NO_HIDDEN | informație perfectă sau publică |
| orasel | NO_HIDDEN | informație perfectă sau publică |
| congkak | NO_HIDDEN | informație perfectă sau publică |
| pallanguzhi | NO_HIDDEN | informație perfectă sau publică |
| ayo | NO_HIDDEN | informație perfectă sau publică |
| memory | NO_HIDDEN | cărțile cu fața în jos sunt ascunse la fel pentru ambii; nu există mână privată |
| pachisi | NO_HIDDEN | informație perfectă sau publică |
| carrom | NO_HIDDEN | informație perfectă sau publică |
| arhipelag | NO_HIDDEN | informație perfectă sau publică |
| roata | NO_HIDDEN | informație perfectă sau publică |
| zaruri5 | NO_HIDDEN | informație perfectă sau publică |
| turnul | NO_HIDDEN | informație perfectă sau publică |
| cunoastere | NO_HIDDEN | întrebarea o pune calculatorul; răspunsul corect nu e afișat nimănui înainte de verificare |
| zidurile | NO_HIDDEN | informație perfectă sau publică |
| prajituri | NO_HIDDEN | cartea trasă e publică; ordinea pachetului e simetrică |
| patrula | NO_HIDDEN | informație publică (zaruri, hartă deschisă) |
| reactii | NO_HIDDEN | informație publică |
| ordine | NO_HIDDEN | informație publică |
| gridiron | NO_HIDDEN | informație perfectă sau publică |
| focare | NO_HIDDEN | cooperativ: ambele mâini sunt afișate (partenerul AI e deschis) |
| strajile | NO_HIDDEN | cooperativ: ambele mâini afișate |
| cenusii | NO_HIDDEN | cooperativ, fără mână privată; doar ordinea pachetului e ascunsă |
| cripta | NO_HIDDEN | cooperativ: cărțile ambilor eroi afișate |
| apus | NO_HIDDEN | informație perfectă sau publică |
| fulger | NO_HIDDEN | informație perfectă sau publică |
| spionii | NO_HIDDEN | maestrul-spion e calculatorul; cheia nu apare în render decât după descoperire/final |
| mosia | NO_HIDDEN | informație perfectă sau publică |
| gospodaria | NO_HIDDEN | informație perfectă sau publică |
| cetatuia | NO_HIDDEN | informație perfectă sau publică |
| taramuri | NO_HIDDEN | informație perfectă sau publică |
| furnale | NO_HIDDEN | informație perfectă sau publică |
| poieni | NO_HIDDEN | informație perfectă sau publică |
| tinuturi | NO_HIDDEN | informație perfectă sau publică |
| sideral | NO_HIDDEN | informație perfectă sau publică |
| coroana | NO_HIDDEN | informație perfectă sau publică |
| coridor | NO_HIDDEN | informație perfectă sau publică |
| drum | NO_HIDDEN | informație perfectă sau publică |
| tripierd | NO_HIDDEN | informație perfectă sau publică |
| konane | NO_HIDDEN | informație perfectă sau publică |
| fanorona | NO_HIDDEN | informație perfectă sau publică |
| tigri | NO_HIDDEN | informație perfectă sau publică |
| surakarta | NO_HIDDEN | informație perfectă sau publică |
| hasami | NO_HIDDEN | informație perfectă sau publică |
| puncte | NO_HIDDEN | informație perfectă sau publică |
| strapungere | NO_HIDDEN | informație perfectă sau publică |
| izolare | NO_HIDDEN | informație perfectă sau publică |
| rotire | NO_HIDDEN | informație perfectă sau publică |
| coloane | NO_HIDDEN | informație perfectă sau publică |
| bile | NO_HIDDEN | informație perfectă sau publică |
| patru | NO_HIDDEN | informație perfectă sau publică |
| cupe | NO_HIDDEN | informație perfectă sau publică |
| poliedre | NO_HIDDEN | informație perfectă sau publică |

Jocurile deja din `HOTSEAT_SAFE` înainte de a1 (cărți, TCG etc.) nu sunt repetate aici; testul `z4_hotseat_viewer` din `z4_qa.rs` le verifică.

Notă: în `cristale`, cărțile rezervate ale adversarului apar acum doar ca număr ("N ascunse"), conform regulii.
