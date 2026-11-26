# Aventuri cooperative de rol (motorul `aventura`)

Șaisprezece jocuri „Rol (RPG)” din catalog sunt jucabile ca **aventuri cooperative în variantă proprie**, toate pe același motor
(`polytabletop-games/src/aventura.rs`, conținutul în `aventura_pachete.rs`). Titlurile, lumile, numele, monștrii și textele sunt
originale; din fiecare stil de rol se păstrează doar *mecanica de test* (zaruri și prag), care e generică.

## Scop

Voi doi (J1 și J2; AI-ul poate fi partenerul) străbateți un graf de 7–8 scene — explorare, test, luptă, odihnă — și învingeți
**stăpânul** din ultima scenă, contra **Maestrului-AI**. Victorie = „Câștigă Jucătorul 1” (victorie comună). Dacă ambii eroi sunt
doborâți (sau lupta durează peste 24 de runde, sau aventura trece de limita de timp), rezultatul este remiză: nu există adversar uman.

## Pregătire

Mai întâi fiecare jucător alege un arhetip: `alege 1`, `alege 2` sau `alege 3` (J2 nu poate lua același arhetip ca J1). Un arhetip are PV,
trei abilități (niveluri 1–6), daune, apărare și o **abilitate specială** de trei feluri: lovitură dublă (*Strike*), vindecare de 4 PV
(*Heal*) sau scut (*Ward*: dușmanii lovesc cu o treaptă mai greu în runda curentă). Specialul se poate folosi o dată pe scenă.
Echipa pornește cu o poțiune.

## Desfășurarea

Eroii acționează pe rând, câte o acțiune. În **explorare** (prima intrare în scenă) se aruncă un zar pe tabelul de evenimente al pachetului
(`eveniment`): poțiune, vindecare, cursă, presiune, ambuscadă, răgaz. În **test** trebuie adunate 1–2 succese cu abilitatea scenei (`incearca`);
un eșec costă PV, iar după 4 încercări echipa trece cu forța, șifonată. În **luptă** fiecare erou face o acțiune pe rundă (`atac N`, `parez`,
`special`, `bea N`), apoi joacă Maestrul-AI (`maestrul`). **Stăpânul** are la fiecare a treia rundă o mișcare specială (lovește pe toți,
adaugă presiune). În **odihnă** (`odihna`) eroii se refac. Oricând, în afara luptei: `cauta` (o dată pe scenă; poate da o poțiune),
`cedez` (treci inițiativa colegului), `bea N` (poțiune pentru eroul N, repornește și un erou doborât) și `ir N` (următoarea scenă, când a
fost depășită cea curentă).

## Zaruri, resursă, Maestrul-AI

* Toate zarurile sunt **mutări forțate publice** (`zaruri`, `eveniment`, `maestrul`) și rezultatul apare în jurnal. Nu există informație ascunsă.
* **Maestrul-AI nu trișează**: dușmanii urmează reguli fixe (ținte alese după PV și rundă), aruncă aceleași zaruri ca voi și nu văd zarurile viitoare;
  AI-ul eroilor alege acțiunea din starea publică și din probabilități, nu din zarurile care urmează (verificat în test).
* **Resursa** pachetului. *Rezervă* (Curaj, Soarta, Șansa…): o cheltuiești cu `refac` ca să reiei un zar ratat și o recuperezi la odihnă sau la un
  critic. *Presiune* (Zdruncinare, Foamea, Stresul…): `refac` o crește cu 1; teroarea dușmanilor și evenimentele o cresc; la maxim eroul se frânge
  (pierde 3 PV, presiunea scade la jumătate); odihna o reduce. După un zar ratat alegi `refac` sau `accept`.
* Rezultatele unui test: critic, succes, parțial (reușești cu un preț de 1 PV), eșec, eșec grav.

## Mecanicile de test

| mecanică | cum se rezolvă |
|---|---|
| d20 + modificator | d20 + nivel + 1 ≥ prag (8/11/14/18); 20 = critic, 1 = ratare gravă |
| d20 cu grade | ca mai sus; marjă ≥ +10 = critic, ≤ −10 = ratare gravă |
| d100 sub abilitate | d100 ≤ 30 + 10·nivel; greu = jumătate, extrem = o cincime; 1 = critic, 96+ = ratare gravă |
| d100 cu grade | d100 față de caracteristică ± 20/40; gradele = zecile de marjă (0 = parțial, 3+ = critic) |
| d20 sub abilitate | d20 ≤ 6 + 2·nivel (greu = jumătate, extrem = pătrime); 1 critic, 20 ratare gravă |
| grămadă de d10 | (nivel+2) d10, 6+ = succes, fiecare 1 anulează un succes; necesar 1–4 după dificultate |
| grămadă de d6 | (nivel+3) d6, 5–6 = lovitură; jumătate de 1 = complicație |
| cel mai mare d6 | (nivel+3)/2 zaruri (± dificultate); 6 curat, 4–5 parțial, 1–3 eșec; două 6 = critic |
| 3d20 față de trei atribute | trei d20 față de atribut; excesul se plătește din punctele de abilitate; două 1 = critic, două 20 = ratare gravă |
| 2d6 + rating | 2d6 + nivel ≥ țintă (8/10/12/14); 12 natural critic, 2 ratare gravă |
| d10 exploziv | d10 (10 adaugă, 1 scade) + nivel + 5 ≥ prag (9/12/14/16) |

Dificultate: 0 ușor, 1 normal, 2 greu, 3 extrem. Dușmanii folosesc aceeași mecanică; un erou are „apărare” 1–2 (+1 în gardă, +1 sub scut).

## Cele 16 pachete

| stil de rol (catalog) | id | titlu | cadru | mecanică | resursă | arhetipuri | monștri |
|---|---|---|---|---|---|---|---|
| Dungeons & Dragons | `turnul_zorilor` | Turnul Zorilor Frânte | fantasy eroic | d20 + modificator ≥ prag | Curaj (rezervă) | Paladin al Cununii; Vraci al Ierburilor; Iscoadă de Pădure | 7 |
| Call of Cthulhu | `arhiva_abisala` | Arhiva Abisală | groază cosmică de investigator | d100 sub abilitate | Zdruncinare (presiune) | Detectiv de Port; Arhivistă Ocultă; Medic Legist | 7 |
| Pathfinder | `ruinele_cartografilor` | Ruinele Cartografilor | expediție în junglă | d20 cu grade de succes | Avânt (rezervă) | Gardian al Taberei; Cartograf Vagabond; Trăgător de Elită | 7 |
| Vampire: The Masquerade | `noapte_de_catifea` | Noapte de Catifea | urban fantasy cu vampiri | grămadă de d10 | Foamea (presiune) | Gardă de Seară; Seducătoare de Salon; Umbră de Culise | 7 |
| Das Schwarze Auge | `padurea_sticlei` | Pădurea Ochilor de Sticlă | fantasy de basm sumbru | 3d20 față de trei atribute | Soarta (rezervă) | Ucenic-Călăreț; Cunoscătoare de Plante; Trubadur Rătăcit | 7 |
| Shadowrun | `neon_si_rune` | Neon și Rune | cyber-urban cu magie | grămadă de d6 cu lovituri | Șansa (rezervă) | Samuraiul Străzii; Șamană Digitală; Decker de Acoperiș | 7 |
| Warhammer Fantasy Roleplay | `targul_ciumat` | Târgul Ciumat | fantasy sumbru | d100 cu grade de succes | Corupția (presiune) | Sergent de Târg; Frate Spitalicesc; Negustor Ambulant | 7 |
| Sword World RPG | `insula_celor_doua_luni` | Insula celor Două Luni | fantasy clasic | 2d6 + rating față de țintă | Binecuvântare (rezervă) | Spadasin al Farurilor; Preoteasă de Maree; Meșter de Talismane | 7 |
| Blades in the Dark | `hotii_din_ceata` | Hoții din Ceață | jaf într-un oraș cu ceață | cel mai mare d6 | Stresul (presiune) | Pumn de Cartier; Doctor de Noapte; Pândar de Acoperiș | 7 |
| Mörk Borg | `crucea_de_cenusa` | Crucea de Cenușă | apocalipsă sumbră | d20 față de prag fix | Semnele (rezervă) | Călău Fugar; Preot Fără Zei; Pungaș al Morții | 7 |
| Cyberpunk RED | `strada_de_otel` | Strada de Oțel | cyberpunk de cartier | d10 exploziv | Ciberpsihoza (presiune) | Solo Blindat; Medtech de Cartier; Netrunner Stradal | 7 |
| Tormenta | `tinutul_furtunilor` | Ținutul Furtunilor Vii | fantasy colorat | d20 + modificator ≥ prag | Mana (rezervă) | Războinic cu Zmeu; Vrăjitor de Nori; Tâlhar cu Pelerină | 7 |
| Starfinder | `epava_din_inelul_sapte` | Epava din Inelul Șapte | operă spațială | d20 + modificator ≥ prag | Energia (rezervă) | Pușcaș de Ring; Medic de Bord; Inginer-Șmecher | 7 |
| Drakar och Demoner | `fiordul_gheatii_negre` | Fiordul Gheții Negre | fantasy nordic | d20 sub abilitate | Onoarea (rezervă) | Berserker Ulvor; Skald Vindecător; Vânătoare de Fiorduri | 7 |
| The Witcher TRPG | `mlastina_strigoilor` | Mlaștina Strigoilor | vânătoare de monștri | d10 exploziv | Toxicitatea (presiune) | Vânător cu Două Săbii; Alchimistă de Mlaștină; Vrăjitor de Semne | 7 |
| InSANe | `satul_lanternelor` | Satul Lanternelor Stinse | groază de sat cu spirite | d100 sub abilitate | Teroarea (presiune) | Sportiv de Liceu; Ceremonierul Templului; Cititoare de Legende | 7 |

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează): `alege N`, `ir N`, `atac N`, `parez`, `special` / `special N`,
`bea N`, `cauta`, `incearca`, `odihna`, `cedez`, plus mutările forțate `zaruri`, `accept`, `refac`, `eveniment`, `maestrul`, `sfârșit`.

## Ce rămâne de îmbogățit

Pachetele sunt compacte (7–8 scene, 7 monștri, 6 evenimente fiecare): se pot adăuga scene alternative, obiecte, arbori de abilități și
traducerea textelor din scene (jurnalul de joc este în română).
