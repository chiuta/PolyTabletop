# Arena: meciuri AI contra AI

Trei comenzi în CLI-ul `polytabletop` (cod: `polytabletop/src/arena.rs`).

## `polytabletop arena ID`

```
polytabletop arena ID [--games N] [--ms A:B[:C..]] [--level easy:hard] [--threads T] [--seed S] [--max-plies P]
```

* `--games N` (implicit 10): partide; părțile se alternează (la partidele pare A joacă `Player::One`, la cele impare `Player::Two`).
* `--ms A:B` buget de timp (ms) per loc; `--level` primește nume (`easy`=10, `normal`=40, `hard`=160, `max`=640) sau ms. Implicit 50:50.
* Mai mult de două bugete (`--ms 10:50:200`): locul 1 este echipa A, locurile 2.. sunt echipa B, iar mutările lui `Player::Two`/ale mesei se rotesc între ele. Motorul are doar două părți, deci „locurile” sunt doar bugete diferite în aceeași echipă; se tipăresc timpii pe fiecare loc. Jocurile cu mai mulți jucători (hruba, lupii) sunt modelate ca tu contra mesei, deci acolo A = jucătorul, B = roboții mesei.
* `--threads T` (implicit min(nuclee, 4)): partide în paralel (`std::thread`). Jocurile se creează în ordinea indicelui, deci repartizările cu seed sunt aceleași indiferent de T; cu T=1 rezultatul este complet reproductibil, cu T>1 timpii (și deci rezultatele jocurilor cronometrate) variază. Sub 1 fir pe nucleu timpii sunt mai fideli.
* `--seed S` (implicit 1): seed global (`Rng::set_global_seed`) + seed AI per partidă.
* Afișează victorii A / remize / victorii B, lungimea medie, ms medii/mutare, cea mai lentă mutare. Partidele oprite la `--max-plies` (600) se numără ca remiză.

Rezultatele sunt reproductibile (același seed, 1 fir) doar pentru AI fără buget de timp (ex. belote); cu căutare cronometrată numărul de noduri depinde de ceas.

Bugetul se aplică doar jocurilor cu `AiStyle::AlphaBetaTime`; cele cu adâncime sau iterații fixe îl ignoră (ms/mutare rămâne informativ).

## `polytabletop bench [--threads T] [--seed S] [--only id,id]`

Pentru fiecare joc: 8 mutări aleatoare (seed), apoi 3 mutări AI la 20 ms, cronometrate. Tabel: joc, categorie, mutări, ms medii, ms maxim. Implicit 1 fir (măsurători curate).

## `polytabletop ladder ID [--games N] [--threads T] [--seed S]`

Bugete 10, 20, 40, 80, 160, 320 ms, fiecare contra fiecăruia (N partide/pereche, implicit 6). Elo aproximativ prin potrivire logistică (prior de o jumătate de partidă pentru a evita valori infinite), 10 ms = 0. Pentru jocuri cu AI fix toate valorile ies apropiate de 0.

## Mecanism

`polytabletop_core::ai::set_thread_budget_ms(Some(ms))` suprascrie bugetul pe firul curent (are prioritate față de `set_time_budget_override_ms` și `POLYTABLETOP_AI_MS`).
