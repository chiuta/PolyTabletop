# Zaruri colorate (Kismet) (`kismet`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play kismet --seed 1`. Sursa implementării: `polytabletop-games/src/kismet.rs`.

> Rezumat din `polytabletop list`: 15 ture; până la 3 aruncări cu 5 zaruri (tine 3 3 5), apoi scor într-o categorie: unu…sase (bonus 35/55/75), 2perechi (aceeași culoare), 3la-fel, scara (30), culoare (35), casa (+15), casa-culoare (+20), 4la-fel (+25), sansa, kismet (+50). Zaruri impare = roșii, pare = albastre.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Fără regula jokerului.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Fără regula jokerului.
