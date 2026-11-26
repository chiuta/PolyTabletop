# Brandubh (`brandubh`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play brandubh --seed 1`. Sursa implementării: `polytabletop-games/src/tafl.rs`.

> Rezumat din `polytabletop list`: Brandubh 7×7 (Irlanda): 8 atacatori contra 4 apărători și rege; regele câștigă ajungând într-un colț sau pe tron cu apărare liberă, e capturat prin încadrare. Piesele mută ca turnul. Mutări: d1-d4.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Motor comun pentru jocurile tafl: Hnefatafl (11×11, varianta „Copenhaga”) și Brandubh (7×7).


Reguli implementate (clean-room, din regulile publice reconstituite):

* Atacatorii (J1, `X`) mută primii; apărătorii (J2, `O`) au și un rege (`K`).

* Toate piesele mută ca turnul (orizontal/vertical, oricâte câmpuri libere), fără salt.

  Doar regele poate ajunge pe tron (centru) sau în colțuri; orice piesă poate trece peste

  tronul gol.

* Captură prin încadrare (custodial): piesa mutată încadrează o piesă adversă între ea și o

  altă piesă proprie sau un câmp ostil (colțurile; tronul – ostil atacatorilor mereu, apărătorilor

  doar gol). Regele ajută la capturi. Nu se capturează activ intrând între două piese adverse.

* Regele: Hnefatafl = capturat când e încercuit pe patru laturi de atacatori sau de tronul gol;

  Brandubh = ca o piesă obișnuită (încadrare pe două laturi) când nu e pe tron sau lângă el,

  altfel încercuit pe patru laturi.

* Apărătorii câștigă când regele ajunge într-un colț; atacatorii când capturează regele. Cine

  nu mai are mutări pierde. Remiză după 400 de mutări.

* Nu sunt implementate: zidul de scuturi (shieldwall), încercuirea totală, interzicerea

  repetițiilor (înlocuită de limita de mutări).


Mutări: `d1-d4` (coloană a.., rând 1.. de sus în jos).
