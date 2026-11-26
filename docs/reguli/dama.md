# Dama (dame turcești) (`dama`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play dama --seed 1`. Sursa implementării: `polytabletop-games/src/dama.rs`.

> Rezumat din `polytabletop list`: 8 × 8, 16 piese pe jucător pe rândurile 2–3. Piesele merg înainte sau lateral, capturează ortogonal; dama zboară pe orice distanță. Captura e obligatorie, cu cele mai multe piese; piesele luate se scot la finalul secvenței. Mutări: a3-a4, a3xa5xc5.

## Scop

Capturezi toate piesele adversarului sau îl blochezi.

## Pregătire

Tablă 8 × 8, 16 piese pe jucător pe rândurile 2–3 (Jucătorul 1) și 6–7 (Jucătorul 2).

## Desfășurarea turei

Piesele obișnuite se mută un pas înainte sau lateral și capturează ortogonal (înainte sau lateral) prin salt. Piesa care ajunge pe ultimul rând devine dama (la sfârșitul mutării); dama se mută oricâte pătrate libere ortogonal și captură prin salt de la orice distanță. Captura este obligatorie și trebuie aleasă secvența cu cele mai multe piese capturate; piesele luate se scot abia la sfârșitul secvenței.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Pas: `a3-a4`.
* Captură: `a3xa5xc5` (pătratele de aterizare, în ordine).

## Reguli speciale și variante alese

* O piesă capturată nu poate fi sărită de două ori în aceeași secvență.

## Punctaj

Nu există punctaj.

## Sfârșitul jocului

Câștigă cine capturează toate piesele adversarului sau îl lasă fără mutări; remiză după 80 de semi-mutări fără captură și fără mutarea unei piese obișnuite.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 6.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Regula remizei (80 de semi-mutări) este proprie.
