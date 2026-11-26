# Yote (`yote`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play yote --seed 1`. Sursa implementării: `polytabletop-games/src/yote.rs`.

> Rezumat din `polytabletop list`: Tablă 5 × 6, 12 piese pe jucător, inițial în mână. La rând: pui o piesă, muți un pas ortogonal sau sari peste o piesă adversă (o captezi și mai scoți încă una la alegere; săritura se poate lanțui). Câștigă cine rămâne singur cu piese. Mutări: c3, c3-c4, c3xc5+a1, c3xc5+m, opreste.

## Scop

Rămâi singurul jucător cu piese.

## Pregătire

Tablă 5 × 6, 12 piese pe jucător, inițial în mână.

## Desfășurarea turei

O acțiune pe tură: pui o piesă din mână pe un pătrat liber; muți o piesă un pas ortogonal; sau sari ortogonal peste o piesă adversă adiacentă, pe pătratul liber de după ea, o captezi și scoți în plus încă o piesă adversă la alegere (de pe tablă; dacă n-are alta pe tablă, din mâna lui). După un salt poți continua cu aceeași piesă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Plasare: `c3`.
* Pas: `c3-c4`.
* Salt cu piesă în plus de pe tablă: `c3xc5+a1`; din mâna adversarului: `c3xc5+m`; fără piesă în plus: `c3xc5`.
* `opreste` încheie un lanț de sărituri; `pas` doar dacă nu ai nicio mutare.

## Reguli speciale și variante alese

* Piesa în plus se alege dintre piesele adverse de pe tablă (nu cea sărită).

## Punctaj

Numărul de piese (tablă + mână).

## Sfârșitul jocului

Câștigă cine rămâne singur cu piese; după 100 de mutări fără captură câștigă cine are mai multe piese (egal = remiză).

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 4.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Limita de 100 de mutări fără captură este regulă proprie.
* Nu se aplică restricții suplimentare privind piesa luată în plus.
