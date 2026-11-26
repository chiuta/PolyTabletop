# Reguli detaliate: jocurile 1–120 (prima parte din `polytabletop list`)

Fiecare pagină descrie, pentru implementarea din acest depozit: scopul, pregătirea, structura turei, sintaxa mutărilor (cu exemple rulate efectiv în CLI cu `--seed 1`), regulile speciale, punctajul, sfârșitul jocului, notele despre AI și informația ascunsă și diferențele față de regulile tradiționale.

Ordinea este cea din `polytabletop list`; partea a doua (jocurile 121–240) este în [INDEX2.md](INDEX2.md). Regulile scurte și sintaxa generală sunt și în `docs/JOCURI.md`.

Observații comune:

* Mutările se scriu exact ca în lista afișată de `?`; majusculele nu contează, spațiile se ignoră. `hint` cere sugestia AI-ului, `q` iese.
* Exemplele au fost produse rulând `polytabletop play <id> --seed 1` și introducând mutări legale; acolo unde AI-ul folosește un buget de timp sau simulări, răspunsul calculat poate diferi de la o rulare la alta.
* Jocurile cu informație ascunsă separă pașii care ar dezvălui informația (tragere, zaruri, revelare, verdict) în mutări forțate; AI-ul nu citește informația privată a adversarului.
* Coloana `HOTSEAT_SAFE` indică jocurile al căror afișaj respectă modul „la masă” (`--you both`): informația privată se arată doar celui la mutare. Jocurile cu „nu” fie nu au informație privată, fie nu ascund informația Jucătorului 1 în acest mod (vezi nota din fiecare pagină).
* „(în curs)” marchează jocurile a căror pagină de reguli încă nu a fost scrisă.

| Nr. | Id | Titlu | Categorie | `HOTSEAT_SAFE` | AI |
|---|---|---|---|---|---|
| 1 | [`chess`](chess.md) | Șah | Abstract | nu | alpha-beta cu timp |
| 2 | [`draughts`](draughts.md) | Dame (8×8) | Abstract | nu | alpha-beta cu timp |
| 3 | [`reversi`](reversi.md) | Reversi | Abstract | nu | alpha-beta cu timp |
| 4 | [`connect4`](connect4.md) | Patru în linie | Abstract | nu | alpha-beta cu timp |
| 5 | [`gomoku`](gomoku.md) | Gomoku (15×15) | Abstract | nu | alpha-beta cu timp |
| 6 | [`hex`](hex.md) | Hex (11×11) | Abstract | nu | alpha-beta cu timp |
| 7 | [`amazoane`](amazoane.md) | Amazoane (10×10) | Abstract | nu | alpha-beta cu timp |
| 8 | [`linii`](linii.md) | Linii de acțiune (8×8) | Abstract | nu | alpha-beta cu timp |
| 9 | [`nim`](nim.md) | Nim cu grămezi | Abstract | nu | alpha-beta cu timp |
| 10 | [`morris`](morris.md) | Moara (Țintar) | Abstract | nu | alpha-beta cu timp |
| 11 | [`mancala`](mancala.md) | Mancala (Kalah) | Abstract | nu | alpha-beta |
| 12 | [`go`](go.md) | Go (9×9) | Abstract | nu | MCTS |
| 13 | [`xiangqi`](xiangqi.md) | Xiangqi (șah chinezesc) | Abstract | nu | alpha-beta cu timp |
| 14 | [`shogi`](shogi.md) | Shogi (șah japonez) | Abstract | nu | alpha-beta cu timp |
| 15 | [`dara`](dara.md) | Dara (Nigeria) | Abstract | nu | alpha-beta |
| 16 | [`morabaraba`](morabaraba.md) | Morabaraba (Africa de Sud) | Abstract | nu | alpha-beta |
| 17 | [`sungka`](sungka.md) | Sungka (Filipine) | Abstract | nu | alpha-beta |
| 18 | [`makruk`](makruk.md) | Makruk (șah thailandez) | Abstract | nu | alpha-beta cu timp |
| 19 | [`janggi`](janggi.md) | Janggi (șah coreean) | Abstract | nu | alpha-beta cu timp |
| 20 | [`popa`](popa.md) | Popa prostul | Cărți | da | alpha-beta |
| 21 | [`pescuit`](pescuit.md) | Pescuit (Go Fish) | Cărți | da | alpha-beta |
| 22 | [`treizeci1`](treizeci1.md) | Treizeci și unu (31) | Cărți | da | alpha-beta |
| 23 | [`casino`](casino.md) | Cassino (capturi) | Cărți | da | alpha-beta |
| 24 | [`zarpoker`](zarpoker.md) | Poker de zaruri | Zaruri | da | alpha-beta |
| 25 | [`septica`](septica.md) | Septica | Cărți | da | alpha-beta |
| 26 | [`macao`](macao.md) | Macao | Cărți | da | alpha-beta |
| 27 | [`domino`](domino.md) | Domino (dublu-6) | Clasic | da | alpha-beta |
| 28 | [`backgammon`](backgammon.md) | Table (Backgammon) | Zaruri | nu | alpha-beta |
| 29 | [`ludo`](ludo.md) | Nu te supăra, frate (Ludo) | Zaruri | nu | alpha-beta |
| 30 | [`snakes`](snakes.md) | Șerpi și scări | Zaruri | nu | alpha-beta |
| 31 | [`barbut`](barbut.md) | Barbut (duel simplificat) | Zaruri | da | alpha-beta |
| 32 | [`66`](66.md) | Șase-zeci și șase (Santase) | Cărți | da | alpha-beta |
| 33 | [`blackjack`](blackjack.md) | Blackjack (21) | Cărți | da | alpha-beta |
| 34 | [`rummy`](rummy.md) | Remi (Rummy, 13 cărți) | Cărți | da | alpha-beta |
| 35 | [`poker`](poker.md) | Poker (Texas hold'em, heads-up) | Cărți | da | alpha-beta |
| 36 | [`tcg`](tcg.md) | Cărți colecționabile (Ținutul Cenușiu) | TCG | da | alpha-beta |
| 37 | [`miniaturi`](miniaturi.md) | Miniaturi hexagonale (Trecătoarea Cenușie) | Miniaturi | nu | alpha-beta |
| 38 | [`mahjong`](mahjong.md) | Mahjong (Hong Kong, 2 jucători) | Clasic | da | alpha-beta |
| 39 | [`hanafuda`](hanafuda.md) | Hanafuda (Koi-Koi) | Cărți | da | alpha-beta |
| 40 | [`hwatu`](hwatu.md) | Hwatu (Go-Stop) | Cărți | da | — |
| 41 | [`karuta`](karuta.md) | Karuta (recitare vs AI, proverbe) | Clasic | da | alpha-beta |
| 42 | [`doudizhu`](doudizhu.md) | Dou dizhu (2 jucători) | Cărți | da | alpha-beta |
| 43 | [`paigow`](paigow.md) | Pai Gow (domino chinezesc) | Clasic | da | alpha-beta |
| 44 | [`teenpatti`](teenpatti.md) | Teen Patti (heads-up) | Cărți | da | alpha-beta |
| 45 | [`riichi`](riichi.md) | Riichi Mahjong (2 jucători) | Clasic | da | — |
| 46 | [`sicbo`](sicbo.md) | Sic Bo | Zaruri | nu | — |
| 47 | [`ashtachamma`](ashtachamma.md) | Ashta Chamma | Familie | nu | — |
| 48 | [`daifugo`](daifugo.md) | Daifugo (Daihinmin) | Cărți | da | — |
| 49 | [`tienlen`](tienlen.md) | Tiến lên | Cărți | da | — |
| 50 | [`bigtwo`](bigtwo.md) | Big Two (Pusoy dos) | Cărți | da | — |
| 51 | [`tongits`](tongits.md) | Tong-its | Cărți | da | — |
| 52 | [`pasur`](pasur.md) | Pasur | Cărți | da | — |
| 53 | [`hokm`](hokm.md) | Hokm | Cărți | da | — |
| 54 | [`pisti`](pisti.md) | Piști | Cărți | da | — |
| 55 | [`courtpiece`](courtpiece.md) | Court Piece (Rang, Mendikot) | Cărți | da | — |
| 56 | [`callbreak`](callbreak.md) | Call Break | Cărți | da | — |
| 57 | [`ganjifa`](ganjifa.md) | Ganjifa (levate) | Cărți | da | alpha-beta |
| 58 | [`yut`](yut.md) | Yut (Yunnori) | Zaruri | nu | alpha-beta |
| 59 | [`sugoroku`](sugoroku.md) | Sugoroku (cursă cu casete) | Zaruri | nu | alpha-beta |
| 60 | [`chaupar`](chaupar.md) | Chaupar (cauri, 2 jucători) | Zaruri | nu | alpha-beta |
| 61 | [`culori`](culori.md) | Culori (potrivit cărți colorate) | Cărți | da | alpha-beta |
| 62 | [`mosii`](mosii.md) | Moșii (afaceri imobiliare cu zaruri) | Zaruri | nu | alpha-beta |
| 63 | [`itinerar`](itinerar.md) | Itinerar (turism pe tablă, rezervi destinații) | Zaruri | nu | — |
| 64 | [`magistrale`](magistrale.md) | Magistrale (rute cu cărți de vagon) | Strategie | da | alpha-beta |
| 65 | [`conac`](conac.md) | Misterul din Conac (deducție) | Deducție | da | alpha-beta |
| 66 | [`insula`](insula.md) | Insula (colonizare cu resurse) | Strategie | da | alpha-beta |
| 67 | [`mozaic`](mozaic.md) | Mozaic (plăcuțe cu drumuri, cetăți, mănăstiri) | Strategie | nu | alpha-beta |
| 68 | [`litere`](litere.md) | Litere (cuvinte pe tablă) | Cuvinte | da | alpha-beta |
| 69 | [`faianta`](faianta.md) | Faianța (variantă originală) | Strategie | nu | alpha-beta |
| 70 | [`bresla`](bresla.md) | Bresla (variantă originală) | Strategie | da | alpha-beta |
| 71 | [`cristale`](cristale.md) | Cristale (variantă originală) | Strategie | da | alpha-beta |
| 72 | [`curieri`](curieri.md) | Curieri (variantă originală) | Cărți | da | alpha-beta |
| 73 | [`fitil`](fitil.md) | Fitil (variantă originală) | Cărți | da | alpha-beta |
| 74 | [`bufet`](bufet.md) | Bufet (variantă originală) | Cărți | da | alpha-beta |
| 75 | [`expeditii`](expeditii.md) | Expediții | Cărți | da | alpha-beta |
| 76 | [`artificii`](artificii.md) | Artificii (cooperativ, partener AI) | Cooperativ | da | alpha-beta |
| 77 | [`orasel`](orasel.md) | Orășel (variantă originală) | Familie | nu | alpha-beta |
| 78 | [`cursa_probelor`](cursa_probelor.md) | Cursa probelor (joc de familie cu mini-probe, 2–4 jucători) | Familie | nu | — |
| 79 | [`capcana`](capcana.md) | Capcana (variantă originală) | Party | da | alpha-beta |
| 80 | [`congkak`](congkak.md) | Congkak (Malaezia) | Abstract | nu | alpha-beta |
| 81 | [`pallanguzhi`](pallanguzhi.md) | Pallanguzhi (Tamil Nadu) | Abstract | nu | alpha-beta |
| 82 | [`ayo`](ayo.md) | Ayo (Ayoayo, Yoruba) | Abstract | nu | alpha-beta |
| 83 | [`oware`](oware.md) | Oware (Ghana) | Abstract | nu | — |
| 84 | [`bao`](bao.md) | Bao (Bao la Kiswahili) | Abstract | nu | — |
| 85 | [`seega`](seega.md) | Seega | Abstract | nu | — |
| 86 | [`yote`](yote.md) | Yote | Abstract | nu | — |
| 87 | [`hus`](hus.md) | Hus (Namibia) | Abstract | nu | — |
| 88 | [`tab`](tab.md) | Tâb (Egipt) | Clasic | nu | — |
| 89 | [`senet`](senet.md) | Senet | Clasic | nu | — |
| 90 | [`whot`](whot.md) | Whot (cărți nigeriene) | Cărți | da | — |
| 91 | [`chkobba`](chkobba.md) | Chkobba (Tunisia) | Cărți | da | — |
| 92 | [`ronda`](ronda.md) | Ronda (Maroc / Spania) | Cărți | da | — |
| 93 | [`shologuti`](shologuti.md) | Sholo Guti (Șaisprezece soldați) | Abstract | nu | — |
| 94 | [`aadupuli`](aadupuli.md) | Aadu Puli Attam (Oi și tigri) | Abstract | nu | — |
| 95 | [`mangala`](mangala.md) | Mangala (mancala turcească) | Abstract | nu | — |
| 96 | [`dama`](dama.md) | Dama (dame turcești) | Abstract | nu | — |
| 97 | [`okey`](okey.md) | Okey (2 jucători) | Clasic | da | — |
| 98 | [`battleship`](battleship.md) | Bătălia navală (10×10) | Clasic | da | alpha-beta |
| 99 | [`mastermind`](mastermind.md) | Spargerea codului (cu schimb de roluri) | Clasic | da | alpha-beta |
| 100 | [`memory`](memory.md) | Perechi (joc de memorie) | Clasic | nu | alpha-beta |
| 101 | [`bingo`](bingo.md) | Bingo (75 de bile) | Clasic | da | alpha-beta |
| 102 | [`loteria`](loteria.md) | Lotería (imagini) | Clasic | da | alpha-beta |
| 103 | [`pachisi`](pachisi.md) | Pachisi (cursă cu cauri) | Zaruri | nu | alpha-beta |
| 104 | [`carrom`](carrom.md) | Carrom (discret, fără fizică) | Abstract | nu | alpha-beta |
| 105 | [`arhipelag`](arhipelag.md) | Arhipelagul în Flăcări (cucerire cu zaruri) | Strategie | nu | alpha-beta |
| 106 | [`roata`](roata.md) | Roata Destinului (parcurs de viață) | Familie | nu | alpha-beta |
| 107 | [`zaruri5`](zaruri5.md) | Cinci Zaruri (13 categorii) | Zaruri | nu | alpha-beta |
| 108 | [`placute`](placute.md) | Plăcuțe (seturi și serii) | Cărți | da | alpha-beta |
| 109 | [`turnul`](turnul.md) | Turnul Șovăitor (turn de blocuri) | Familie | nu | alpha-beta |
| 110 | [`cunoastere`](cunoastere.md) | Cupa Cunoașterii (întrebări, 6 felii) | Party | nu | alpha-beta |
| 111 | [`ceata`](ceata.md) | Câmpul Ceții (piese ascunse, 10×10) | Strategie | da | alpha-beta |
| 112 | [`umbra`](umbra.md) | Umbra Urbei (urmărire cu bilete) | Familie | da | alpha-beta |
| 113 | [`zidurile`](zidurile.md) | Zidurile Mișcătoare (labirint cu plăci culisante) | Familie | nu | alpha-beta |
| 114 | [`prajituri`](prajituri.md) | Drumul Prăjiturilor (cursă de culori) | Familie | nu | alpha-beta |
| 115 | [`portret`](portret.md) | Portretul Misterios (ghicește personajul) | Familie | da | alpha-beta |
| 116 | [`briscola`](briscola.md) | Briscola (Italia) | Cărți | da | — |
| 117 | [`scopa`](scopa.md) | Scopa (Italia) | Cărți | da | — |
| 118 | [`durak`](durak.md) | Durak (Rusia) | Cărți | da | — |
| 119 | [`belote`](belote.md) | Belote (Franța, 2 jucători) | Cărți | da | — |
| 120 | [`skat`](skat.md) | Skat (2 jucători) | Cărți | da | — |
