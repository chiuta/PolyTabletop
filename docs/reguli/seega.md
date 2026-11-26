# Seega (`seega`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play seega --seed 1`. Sursa implementării: `polytabletop-games/src/seega.rs`.

> Rezumat din `polytabletop list`: Tablă 5 × 5, 12 piese pe jucător. Plasare: două piese pe tură, fără centru; apoi mută cine a plasat ultima, un pas ortogonal. Captură custodială a unui șir de piese adverse între piesa mutată și o piesă proprie; cine capturează mută din nou. Blocaj: adversarul scoate o piesă proprie. Mutări: b2 (plasare), b2-b3, scoate-b2.

## Scop

Capturezi toate piesele adversarului.

## Pregătire

Tablă 5 × 5, 12 piese pe jucător, în mână; pătratul central (c3) rămâne gol la plasare.

## Desfășurarea turei

Plasare: jucătorii pun alternativ câte două piese (J1 două, J2 două …) pe pătrate libere, fără centru. Mutare: începe cine a plasat ultima piesă; o piesă se mută un pas ortogonal pe un pătrat liber (prima mutare intră în centru). Dacă piesa mutată prinde între ea și o piesă proprie, pe aceeași linie, un șir de piese adverse, toate sunt luate, iar jucătorul mută din nou.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Plasare: `b2`.
* Mutare: `b2-b3`.
* Scoatere după blocaj: `scoate-b2`.

## Reguli speciale și variante alese

* Blocaj: dacă jucătorul la mutare nu poate muta, adversarul își scoate una dintre piese (`scoate-<pătrat>`) și blocatul încearcă din nou; dacă nici adversarul nu poate muta, jocul se oprește.

## Punctaj

Numărul de piese rămase.

## Sfârșitul jocului

Câștigă cine rămâne cu piese; după 100 de mutări fără captură câștigă cine are mai multe piese (egal = remiză).

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 4.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Regula blocajului și limita de 100 de mutări fără captură sunt alegeri documentate (regulile istorice variază).
* Captura unui șir de piese (nu doar a uneia) este varianta aleasă.
