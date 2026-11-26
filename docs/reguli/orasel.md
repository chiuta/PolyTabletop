# Orășel (variantă originală) (`orasel`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play orasel --seed 1`. Sursa implementării: `polytabletop-games/src/orasel.rs`.

> Rezumat din `polytabletop list`: Zaruri și construit orașe: arunci 1 sau 2 zaruri (după Gară), clădirile albastre plătesc în orice tură, verzi doar în a ta, roșii (cafenea, grătar) iau de la cel care aruncă; cumperi o clădire (cumpar lan) sau un reper (construiesc gara/mall/parc/turn). Câștigă cine are primul cele 4 repere. Mutări: arunc 1, reiau, pastrez, nimic.

## Scop

Primul care construiește toate cele 4 repere câștigă.

## Pregătire

Fiecare începe cu un Lan, o Brutărie și 3 monede. Rezerva comună are 6 din fiecare clădire.

## Desfășurarea turei

Arunci 1 zar (sau 2, după ce ai construit Gara), apoi cumperi cel mult o clădire din rezervă sau construiești un reper.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunc 1` (sau `arunc 2`), `reiau`, `pastrez`.
* `cumpar lan`, `construiesc gara` (sau `mall`, `parc`, `turn`), `nimic`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `arunc 1`. Introduci `arunc 1`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (11): `nimic`, `cumpar lan`, `cumpar padure`, `cumpar livada`, `cumpar brutarie`, `cumpar magazin`, `cumpar fabrica`, `cumpar moara`, `cumpar cafenea`, `cumpar gratar` …. Introduci `cumpar magazin`; răspunsul calculatorului (în ordine): `arunc 1`; `cumpar lan`.
3. Mutări legale (1): `arunc 1`. Introduci `arunc 1`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Clădiri albastre (în orice tură): Lan 1: +1; Pădure 5: +1; Livadă 8: +2; Mină 11–12 (cost 6): +5.
* Verzi (doar tura ta): Brutărie 2–3: +1; Magazin 4: +3; Fabrică 6: +3 per Pădure/Mină; Moară 7: +2 per Lan/Livadă.
* Roșii (în tura adversarului, pe banii celui care aruncă): Cafenea 3: ia 1; Grătar 9–10: ia 2.
* Repere: Gară (4): 2 zaruri; Centru comercial (10): +1 la Brutărie, Magazin, Cafenea, Grătar; Parc (16): la dublă, încă o tură; Turn (22): poți arunca din nou o dată.

## Punctaj

Monede; condiția de victorie este numărul de repere.

## Sfârșitul jocului

Primul cu cele 4 repere. Plafon de 150 de runde: câștigă cine are mai multe repere, apoi monede.

## Note despre AI

Zarurile sunt publice. AI-ul alege 1 sau 2 zaruri după valoarea așteptată, nu după rezultatul real.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Variantă originală, mecanica „arunci zarurile, colectezi din cărțile cu numărul ieșit”.
