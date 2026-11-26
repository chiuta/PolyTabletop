# Chinchirorin (zaruri în bol) (`chinchirorin`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chinchirorin --seed 1`. Sursa implementării: `polytabletop-games/src/chinchirorin.rs`.

> Rezumat din `polytabletop list`: Trei zaruri; oya (banca) și ko alternează. Ko pariază 1–3 (pariez 2), apoi se aruncă (arunc, până la 3 încercări): Pinzoro 1-1-1 = 5×, Arashi (triplet) 3×, Shigoro 4-5-6 2×, punct = zarul singur dintr-o pereche; 1-2-3 pierde 2×. 12 runde sau până la faliment.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Variantă cu mize 1–3 și până la 3 încercări de aruncare.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Variantă cu mize 1–3 și până la 3 încercări de aruncare.
