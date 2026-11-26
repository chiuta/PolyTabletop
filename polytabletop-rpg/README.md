# polytabletop-rpg

Trusă generică, fără dependențe externe, pentru jocuri de rol de masă. Nu conține regulile, textele sau datele
vreunui editor: sunt piese configurabile din care se pot obține multe stiluri de joc.

## Module

| modul | ce face |
|---|---|
| `dice` | notație `NdM+K`, `kh/kl/dh/dl`, explodare `!`, pool-uri de succese `s>=T`, `d%`, `dF` |
| `sheet` | fișă generică (atribute, abilități, resurse, inventar, condiții), format text, `ModStyle` |
| `check` | sisteme de teste interschimbabile (`CheckSystem`) + presetări (`presets()`) |
| `initiative` | ordine de inițiativă, runde, intrări/ieșiri în timpul luptei |
| `tables` | tabele aleatorii cu intervale, referințe imbricate `{Alt tabel}` |
| `dungeon` | generator pe grilă (camere, coridoare, uși, scări), conectivitate garantată |
| `data` | parser TOML-like propriu (`[creature.id]`, `[item.id]`) + bestiar/obiecte tipizate |
| `journal` | jurnal de sesiune cu format text + export Markdown (`log md`) |
| `combat` | simulator de bătălie/încăierare: unități, moral, teren (`battle`) |
| `chargen` | generator de personaje cu „sisteme” definite în date (`data/system-*.toml`: percentile, d20-heroic, grit-pool, star-narrative) |
| `gen` | generatoare de vrăji/abilități/obiecte cu ponderi de raritate (`data/generators.toml`) |
| `encounter` | calculator de echilibru (buget tip CR, `data/foes.toml`) și împărțirea comorii (`loot`) |
| `overland` | hartă hex (odd-r), vreme pe anotimp, timp de drum, randare ASCII (`hexmap`, `travel`, `weather`) |
| `cli` / `polytabletop-rpg` | interpretor interactiv (`cargo run --release -p polytabletop-rpg`) |

## Sistemele de teste

* `RollOver` — zar(uri) + rating + modificator față de un prag; naturale critice, marjă critică, avantaj/dezavantaj (`edge`).
* `RollUnder` — d100 sau d20 sub valoare; trepte de dificultate (normal / greu / extrem), grade de succes, zar bonus/penalizare.
* `SuccessPool` — grămadă de d6/d10 cu prag, număr de succese necesare, 1 care anulează, complicații.
* `HighestPool` — cel mai bun zar din grămadă: curat / cu complicație / eșec.
* `NarrativePool` — zaruri pozitive și negative cu succese, avantaje și amenințări (complicații).
* `TripleUnder` — 3d20 față de trei atribute, diferențele se plătesc din puncte de abilitate.

## Cum se obține fiecare stil (presetări generice, fără regulile editorilor)

Cheile sunt cele din catalogul PolyTabletop (categoria „Rol (RPG)”). Presetarea se alege cu `use ID` în CLI sau `preset(id)` în cod;
fișele și tabelele se completează cu propriile cifre. Obținem *stilul* de mecanică, nu conținutul.

| cheie catalog | presetare | cum se configurează |
|---|---|---|
| dungeons & dragons | `d20-dc` | 1d20 + modificator vs prag; `ModStyle::Step{base:10,step:2}` pentru a deriva modificatorul din scorul atributului; avantaj = `edge=1`; initiativă `1d20`; PV ca resursă |
| pathfinder | `d20-dc` | la fel; grade de succes din `crit_margin` (setat în `RollOver`) ca +10/-10; condiții cu durată în fișă |
| starfinder | `d20-dc` | ca mai sus; adaugă resurse suplimentare (scut, energie) ca `res` în fișă |
| tormenta | `d20-dc` | ca mai sus; tabele proprii pentru efecte și bestiar TOML propriu |
| mörk borg | `d20-dr` | 1d20 + modificator vs prag fix (implicit 12); naturale 1/20 critice; resurse puține, tabele aleatorii pentru tot (`tables`) |
| call of cthulhu | `d100-under` | d100 sub valoarea abilității; `difficulty` 0/1/2 = normal/greu/extrem (jumătate, o cincime); zar bonus/penalizare = `edge`; resursa „Rațiune” ca `res` |
| insane | `d100-under` | percentile sub abilitate, cu aceeași scară de trepte; resursă de stres/rațiune în fișă și condiții cu durată |
| warhammer fantasy roleplay | `d100-under` | d100 sub caracteristică + modificator; `successes` din rezultat = gradele de succes (zeci de marjă); iniția­tivă `1d10` + bonus |
| drakar och demoner | `d20-under` | d20 sub valoarea abilității; 1 critic, 20 ratare gravă; `difficulty` 1 = jumătate |
| vampire: the masquerade | `d10-pool-cancel` | grămadă de d10 (atribut + abilitate), 6+ succes, 1 anulează, eșec dezastruos fără succese; `difficulty` = succese necesare |
| shadowrun | `d6-pool-hits` | grămadă de d6, 5-6 lovituri, prag de lovituri; jumătate sau mai multe de 1 = complicație (critică fără lovituri); `edge` adaugă zaruri |
| blades in the dark | `d6-highest` | cel mai bun d6 din grămadă: 6 curat, 4-5 cu complicație, 1-3 eșec; 2+ de 6 critic; zero zaruri = cel mai mic din două; `difficulty` scade zaruri |
| das schwarze auge (the dark eye) | `triple-d20` | 3d20 față de trei atribute (`attrs=a,b,c`), diferențele se scad din punctele de abilitate (`rating`); două de 1 = critic, două de 20 = ratare |
| sword world rpg | `2d6-target` | 2d6 + rating vs țintă; 12 natural critic, 2 natural eșec automat; `ModStyle::Div(6)` pentru bonusuri |
| cyberpunk red | `d10-explode-dc` | 1d10 explozibil + atribut + abilitate vs prag (`1d10!`); tabele de loot și fișe cu resurse de echipament |
| the witcher trpg | `d10-explode-dc` | la fel: d10 explozibil + atribut + abilitate vs prag; inventar și condiții cu durată |

Observație: presetările `d20-dc`, `d20-dr`, `2d6-target` și `d10-explode-dc` sunt același motor `RollOver` cu zaruri
și criticale diferite; oricine își poate construi altele cu `RollOver::new("expresie")` și câmpurile publice.
`narrative-pool` nu e legat de o cheie din catalog, dar e disponibil pentru jocuri narative cu complicații.

## Formate

Fișă (`sheet save/load`):

```
name: Aria
attr: Vigor | 14
skill: Stealth | 3
res: Health | 10 | 12
item: Rope | 2 | 15 m
cond: Poisoned | 3
note: ...
```

Tabele:

```
table: Encounter
dice: 1d6
1-2: Goblins carrying {Loot}
3-6: Nothing

table: Loot
1: a coin
2: a gem
```

Bestiar / obiecte (subset TOML):

```
[creature.rat]
name = "Rat"
hp = 2
defense = 10
attack = 1
damage = "1d3"
tags = ["beast"]

[item.rope]
weight = 2.5
value = 10
```

## Rulare

```
cargo run --release -p polytabletop-rpg
rpg> roll 4d6kh3
rpg> use d100-under
rpg> check 60 1
rpg> dungeon 60 22 8
```

## Extinderi (comenzi noi)

```
rpg> battle demo rounds=10          # sau battle FIȘIER cu [battle] + [unit.ID]
rpg> chargen list | chargen percentile Ana | chargen mysys.toml
rpg> gen spell 3 min=rare | gen item 5 | gen load tabele.toml
rpg> encounter budget 4 3 | encounter build 4 3 hard max=6 | encounter rate 4 3 1 1 3
rpg> loot 1500 Ana Bo Cy
rpg> season winter | hexmap 24 14 | travel 0 0 10 8 pace=4 | weather
rpg> log md sesiune.md Titlu       # `-` = tipărește
```

Toate sunt deterministe cu `seed N`. Format sistem: secțiunea `[system]` cu `attributes`, `attr_method`
(`roll` + `attr_roll`/`attr_mult` sau `spread` + `attr_points/min/max`), `skills` (`"Nume:bază"`), `skill_points/step/cap/picks`,
`resources` (`"Nume:zar+@Atribut"`), `res_attr_offset/div`, `preset` (sistemul de teste recomandat).
Teste: `cargo test --release -p polytabletop-rpg` (`tests/z7_expansion.rs`).
