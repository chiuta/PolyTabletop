# Cristale (variantă originală) (`cristale`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cristale --seed 1`. Sursa implementării: `polytabletop-games/src/cristale.rs`.

> Rezumat din `polytabletop list`: Jetoane de minerale și dezvoltări, 2 jucători: o acțiune pe tură — iau G A T (3 culori diferite), iau2 G (2 la fel, dacă sunt ≥4 în bancă), rezerv a1 (+ chihlimbar-joker), cumpar a1 / cumpar rez1. Cărțile cumpărate dau reduceri permanente și puncte; peste 10 jetoane arunci (arunc G). Patronii (3 p) vizitează automat. La 15 puncte se mai joacă până la capătul rundei. Cărțile sunt generate de noi.

## Scop

Ajungi primul la 15 puncte (se termină runda) și ai cele mai multe puncte la final.

## Pregătire

Cinci minerale (Granat G, Ametist A, Topaz T, Malachit M, Cuarț Q) și Chihlimbarul C (joker). Cărți de dezvoltare pe niveluri a, b, c, locuri 1–4; cărțile sunt generate de noi.

## Desfășurarea turei

O singură acțiune pe tură. Cu peste 10 jetoane arunci excedentul.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `iau G A T` (3 jetoane de culori diferite).
* `iau2 G` (2 de aceeași culoare, doar dacă în grămadă sunt cel puțin 4).
* `rezerv a1` (rezervi o carte vizibilă, max. 3, și primești un Chihlimbar).
* `cumpar a1` / `cumpar rez2`; `arunc G`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (27): `iau G A T`, `iau G A M`, `iau G A Q`, `iau G T M`, `iau G T Q`, `iau G M Q`, `iau A T M`, `iau A T Q`, `iau A M Q`, `iau T M Q` …. Introduci `iau G A T`; răspunsul calculatorului (în ordine): `iau A T M`.
2. Mutări legale (23): `iau G A T`, `iau G A M`, `iau G A Q`, `iau G T M`, `iau G T Q`, `iau G M Q`, `iau A T M`, `iau A T Q`, `iau A M Q`, `iau T M Q` …. Introduci `rezerv a1`; răspunsul calculatorului (în ordine): `iau A T M`.
3. Mutări legale (25): `iau G A T`, `iau G A M`, `iau G A Q`, `iau G T M`, `iau G T Q`, `iau G M Q`, `iau A T M`, `iau A T Q`, `iau A M Q`, `iau T M Q` …. Introduci `rezerv c4`; răspunsul calculatorului (în ordine): `iau A T M`.

## Reguli speciale și variante alese

* Fiecare carte cumpărată dă o reducere permanentă pe culoarea ei și, uneori, puncte.
* Patronii (3 puncte) te vizitează automat când îndeplinești cerința; vizitează primul îndeplinit.

## Punctaj

Puncte din cărți și patroni.

## Sfârșitul jocului

La 15 puncte se mai joacă până după tura Jucătorului 2. Câștigă cine are mai multe puncte; egal: mai puține cărți cumpărate. Plafon de 120 de runde.

## Note despre AI

Informație publică; pachetele (ordinea cărților) sunt necunoscute. AI-ul alege după o euristică; locul reumplut de pe piață nu intră în evaluare.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Variantă originală; nu se rezervă din vârful pachetului.
