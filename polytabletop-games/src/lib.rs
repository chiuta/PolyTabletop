//! PolyTabletop games.
//!
//! Clean-room policy: every game here is written only from its publicly known
//! rules. No code, text, art or data of any commercial product is used.
//! Trademarked product names (Monopoly, Othello, Connect Four, …) are never
//! used for our own games: we use the generic names ("Reversi",
//! "Patru în linie", …).

// B6-mods-begin
pub mod aruncari_cupe;
pub mod b6_ask;
pub mod b6_crowd;
pub mod b6_deduce;
pub mod b6_mask;
pub mod b6_pick;
pub mod b6_slap;
pub mod b6_util;
pub mod bal_masti;
pub mod banda_nebuna;
pub mod bentita;
pub mod camere_fitil;
pub mod categoria_indrazneata;
pub mod ciocnirea_literelor;
pub mod crima_din_port;
pub mod definitii_false;
pub mod duel_categorii;
pub mod duel_gloante;
pub mod duelul_argumentelor;
pub mod favoritul_judecatorului;
pub mod fitilul_cuvintelor;
pub mod gandaci_blufati;
pub mod ganduri_impartasite;
pub mod gesturi_mute;
pub mod grunturi;
pub mod indicii_cascada;
pub mod intrusul_tacut;
pub mod legenda_potrivita;
pub mod nuanta_ghicita;
pub mod obiect_fulger;
pub mod palat_intrigi;
pub mod palmuta_pe_cuvant;
pub mod pariu_pe_cifre;
pub mod povesti_negre;
pub mod primarul_si_lupul;
pub mod provocarea_mintii;
pub mod provocari_trasnite;
pub mod reguli_schimbatoare;
pub mod schite_rapide;
pub mod semne_icoane;
pub mod totem_grabit;
pub mod trei_runde;
pub mod valul_ideilor;
pub mod vama_bacsis;
pub mod vorbe_scurte;
pub mod zaruri_povestit;
pub mod zece_usi;
// B6-mods-end
// B2-mods-begin
pub mod agenti;
pub mod ambuteiaj;
pub mod animale;
pub mod armonia;
pub mod asalt;
pub mod b1_race;
pub mod b1_reg;
pub mod b1_util;
pub mod balans;
pub mod bambuzeria;
pub mod banane;
pub mod banchiza;
pub mod bilinele;
pub mod brand_util;
pub mod camile;
pub mod caravana;
pub mod cariera;
pub mod cartimut;
pub mod cazanul;
pub mod cetate_intrigi;
pub mod colturi;
pub mod cub2;
pub mod cuburi;
pub mod culori_pas;
pub mod curierii_vraji;
pub mod dar4;
pub mod detectiv;
pub mod discuri;
pub mod drumul_stivelor;
pub mod evadare;
pub mod firul;
pub mod flipper_duel;
pub mod forme6;
pub mod genial;
pub mod hochei_masa;
pub mod iepuri;
pub mod impingere;
pub mod inele;
pub mod izvoare;
pub mod kilometri;
pub mod lanturi;
pub mod lumina;
pub mod marele_bazar;
pub mod margine;
pub mod menajerie;
pub mod morcovi;
pub mod morpiuri9;
pub mod palate;
pub mod peisaj;
pub mod pelerin;
pub mod perechi_duel;
pub mod pietre_puzzle;
pub mod piramida;
pub mod pisici;
pub mod plapuma;
pub mod postalioni;
pub mod poteca;
pub mod rapide;
pub mod roiul;
pub mod salariu;
pub mod santier;
pub mod sase;
pub mod sferturi;
pub mod sir5;
pub mod tablou;
pub mod templele;
pub mod terase;
pub mod tinut_zane;
pub mod tinuturi_negociate;
pub mod trei_tipuri;
pub mod tren_jaf;
pub mod trenuri;
pub mod triunghiuri;
pub mod turnuri_litere;
pub mod turnuri_piatra;
pub mod vamesul;
pub mod vapor;
pub mod zgarie_nori;
// B2-mods-end
pub mod aadupuli;
pub mod aceydeucey;
pub mod alquerque;
pub mod amazoane;
pub mod apus;
pub mod arhipelag;
pub mod artificii;
pub mod ashtachamma;
pub mod aventura;
pub mod aventura_pachete;
pub mod ayo;
pub mod babaoarba;
pub mod baccarat;
pub mod backgammon;
pub mod baloot;
pub mod bao;
pub mod barbilliards;
pub mod barbut;
pub mod battleship;
pub mod beetle;
pub mod belote;
pub mod bezique;
pub mod bgfam;
pub mod bigtwo;
pub mod bile;
pub mod biliard;
pub mod bingo;
pub mod blackjack;
pub mod brand_grid;
pub mod brand_market;
pub mod brand_race;
pub mod brand_rail;
pub mod brand_war;
pub mod bresla;
pub mod bridge;
pub mod briscola;
pub mod bufet;
pub mod callbreak;
pub mod camelot;
pub mod canasta;
pub mod capcana;
pub mod capturetheflag;
pub mod cards;
pub mod cards52;
pub mod cardsb_util;
pub mod carrom;
pub mod carti_cascada;
pub mod casino;
pub mod catalog;
pub mod catun;
pub mod ceasornic;
pub mod ceata;
pub mod ceelo;
pub mod cenusii;
pub mod cetatuia;
pub mod chaturanga;
pub mod chaupar;
pub mod chess;
pub mod chess_san;
pub mod chinchirorin;
pub mod chinesewhispers;
pub mod cinci_pietricele;
pub mod codru;
pub mod coganh;
pub mod coloane;
pub mod comori;
pub mod conac;
pub mod congkak;
pub mod conkers;
pub mod connect4;
pub mod consiliul;
pub mod coridor;
pub mod coroana;
pub mod courtpiece;
pub mod craps;
pub mod crazyhouse;
pub mod cribbage;
pub mod crima;
pub mod cripta;
pub mod cristale;
pub mod culori;
pub mod cunoastere;
pub mod cunoastere_bank;
pub mod cupe;
pub mod curieri;
pub mod cursa_probelor;
pub mod cuvantunic;
pub mod cuvantunic_date;
pub mod daifugo;
pub mod dama;
pub mod dame10;
pub mod dara;
pub mod darurile_serii;
pub mod deadmansdice;
pub mod debarcader;
pub mod dicebaseball;
pub mod dictionar;
pub mod dictionar_date;
pub mod dog;
pub mod dogkit;
pub mod domino;
pub mod domino9;
pub mod doppelkopf;
pub mod dou_dizhu;
pub mod douazeci;
pub mod draughts;
pub mod dreidel;
pub mod dropdead;
pub mod drum;
pub mod duckduckgoose;
pub mod durak;
pub mod epoci;
pub mod euchre;
pub mod expeditii;
pub mod faianta;
pub mod fanorona;
pub mod farul;
pub mod fia;
pub mod fitil;
pub mod flipcup;
pub mod focare;
pub mod fotbalmasa;
pub mod foxgeese;
pub mod fulger;
pub mod furnale;
pub mod ganjifa;
pub mod gebeta;
pub mod go;
pub mod gomoku;
pub mod gospodaria;
pub mod gridiron;
pub mod gulbara;
pub mod gura;
pub mod gura_date;
pub mod halma;
pub mod hanafuda;
pub mod hangman;
pub mod hasami;
pub mod hazard;
pub mod hex;
pub mod hokm;
pub mod hotar;
pub mod howzat;
pub mod hruba;
pub mod hus;
pub mod hwatu;
pub mod i18n;
pub mod impletitura;
pub mod impostor;
pub mod impostor_date;
pub mod inimi;
pub mod insula;
pub mod insule_punti;
pub mod ispy;
pub mod itinerar;
pub mod izolare;
pub mod janggi;
pub mod joc15;
pub mod jocuri_ata;
pub mod jungle;
pub mod kalah;
pub mod karuta;
pub mod kismet;
pub mod konane;
pub mod latrunculi;
pub mod linegraph;
pub mod linii;
pub mod litere;
pub mod ludo;
pub mod ludofam;
pub mod lupii;
pub mod macao;
pub mod madgab;
pub mod magistrale;
pub mod mahjong;
pub mod mahjong_am;
pub mod mahjong_hk;
pub mod makruk;
pub mod mancala;
pub mod mancala4;
pub mod mangala;
pub mod mariash;
pub mod mastermind;
pub mod mehen;
pub mod memory;
pub mod mexico;
pub mod mima;
pub mod miniaturi;
pub mod morabaraba;
pub mod morra;
pub mod morris;
pub mod mosia;
pub mod mosii;
pub mod mozaic;
pub mod mus;
pub mod namethattune;
pub mod narde;
pub mod nave;
pub mod nim;
pub mod okey;
pub mod oldmaid;
pub mod ombre;
pub mod omweso;
pub mod oprite;
pub mod oprite_date;
pub mod orasel;
pub mod ordine;
pub mod oversevens;
pub mod oware;
pub mod p2_util;
pub mod pachisi;
pub mod pai_gow;
pub mod pallanguzhi;
pub mod parimpar;
pub mod pasari_sarma;
pub mod passedix;
pub mod pasur;
pub mod patru;
pub mod patrula;
pub mod pegsol;
pub mod pescuit;
pub mod petitschevaux;
pub mod petteia;
pub mod pica;
pub mod pisti;
pub mod pit;
pub mod placute;
pub mod planete_verzi;
pub mod poieni;
pub mod poker;
pub mod poliedre;
pub mod popa;
pub mod portret;
pub mod poteca_turmelor;
pub mod povestitor;
pub mod povestitor_date;
pub mod prajituri;
pub mod prinde;
pub mod pubquiz;
pub mod puncte;
pub mod q2_trick;
pub mod q2_util;
pub mod q4_secret;
pub mod q4_util;
pub mod reactii;
pub mod reteaua_luminii;
pub mod reverdire;
pub mod reversi;
pub mod rezervatia;
pub mod riichi;
pub mod roade;
pub mod roata;
pub mod ronda;
pub mod rotire;
pub mod rps;
pub mod rummy;
pub mod salina;
pub mod sanma;
pub mod santinela;
pub mod schita;
pub mod schita_date;
pub mod schocken;
pub mod scopa;
pub mod seega;
pub mod senet;
pub mod septica;
pub mod settemezzo;
pub mod sevens;
pub mod sheepshead;
pub mod shipcrew;
pub mod shogi;
pub mod shologuti;
pub mod shuffleboard;
pub mod sicbo;
pub mod sideral;
pub mod simon;
pub mod sittuyin;
pub mod sixtysix;
pub mod skat;
pub mod skitgubbe;
pub mod skittles;
pub mod skunk;
pub mod snakes;
pub mod snap;
pub mod solzi;
pub mod sotron;
pub mod spionii;
pub mod spionii_date;
pub mod sprouts;
pub mod statuile;
pub mod strajile;
pub mod strapungere;
pub mod sudoku;
pub mod sugoroku;
pub mod sungka;
pub mod surakarta;
pub mod tab;
pub mod tablanet;
pub mod tabula;
pub mod tangram;
pub mod taramuri;
pub mod targul_marii;
pub mod tarneeb;
pub mod tarock;
pub mod tarot;
pub mod tavli;
pub mod tcg;
pub mod tcg_cerneala;
pub mod tcg_evo;
pub mod tcg_lant;
pub mod tcg_zone;
pub mod teen_patti;
pub mod texas42;
pub mod tictactoe;
pub mod tiddlywinks;
pub mod tiengow;
pub mod tienlen;
pub mod tigri;
pub mod tinuturi;
pub mod toc;
pub mod toguz;
pub mod tongits;
pub mod treicai;
pub mod treicai_date;
pub mod treizeci1;
pub mod trickkit;
pub mod trictrac;
pub mod tripierd;
pub mod trix;
pub mod truco;
pub mod turnul;
pub mod tute;
pub mod twentynine;
pub mod twixt;
pub mod tysiac;
pub mod umbra;
pub mod ur;
pub mod valea;
pub mod w4_comun;
pub mod w4_ghici;
pub mod w5_util;
pub mod w7_util;
pub mod whist;
pub mod whot;
pub mod wouldyourather;
pub mod xiangqi;
pub mod yatzy;
pub mod yote;
pub mod yut;
pub mod zarpoker;
pub mod zaruri5;
pub mod zidurile;
pub mod zipzapzop;
// --- P4 (jocuri tradiționale din America Latină / Oceania) ---
pub mod ancora;
pub mod baraja_es;
pub mod burako;
pub mod chinchon;
pub mod cuarenta;
pub mod escoba;
pub mod fivehundred;
pub mod generala;
pub mod mutorere;
pub mod parques;
pub mod pontoon;
pub mod twoup;

// Valul P3
pub mod all_fours;
pub mod bid_whist;
pub mod bunco;
pub mod chicken_foot;
pub mod conquian;
pub mod crazy_eights;
pub mod crokinole;
pub mod farkle;
pub mod gin_rummy;
pub mod liars_dice;
pub mod p3_comun;
pub mod pinochle;
pub mod pitch;

// Q3-BEGIN
pub mod ataxx;
pub mod bar_dice;
pub mod brag;
pub mod chowka_bara;
pub mod chu_shogi;
pub mod chuck;
pub mod cosmic;
pub mod crossword;
pub mod fan_tan;
pub mod hanoi;
pub mod hounds;
pub mod hypergammon;
pub mod kick_can;
pub mod ligretto;
pub mod mau_mau;
pub mod mikado;
pub mod mingea;
pub mod muggins;
pub mod nardy;
pub mod o_an_quan;
pub mod pachinko;
pub mod perquackey;
pub mod pingpong;
pub mod sardines;
pub mod scaune_muzicale;
pub mod shashki;
pub mod shatranj;
pub mod shut_box;
pub mod sticla;
pub mod tombola;
pub mod trick32;
pub mod truth_dare;
pub mod uckers;
// Q3-END
// Valul Q5 (jocuri tradiționale rămase)
pub mod bagatelle;
pub mod balut;
pub mod bobing;
pub mod capra;
pub mod chaturaji;
pub mod chohan;
pub mod coarda;
pub mod crag;
pub mod damas;
pub mod jass;
pub mod kim;
pub mod klaverjas;
pub mod klondike;
pub mod lasca;
pub mod mahjong_tw;
pub mod manille;
pub mod marbles;
pub mod matador;
pub mod nhie;
pub mod nonogram;
pub mod parcel;
pub mod q5_dice;
pub mod q5_trick;
pub mod q5_util;
pub mod renju;
pub mod ruleta;
pub mod salta;
pub mod scattergories;
pub mod sjoelen;
pub mod slapjack;
pub mod snooker;
pub mod spoons;
pub mod sueca;
pub mod tablut;
pub mod tali;

// --- Val 17 / Q6 ---
pub mod ascunselea;
pub mod batak;
pub mod bau_cua;
pub mod boggle;
pub mod brandubh;
pub mod celebrity;
pub mod chicago;
pub mod chinese_checkers;
pub mod daldos;
pub mod de_a_prinselea;
pub mod dice_race;
pub mod duodecim_scripta;
pub mod elasticul;
pub mod goose;
pub mod guobiao;
pub mod havannah;
pub mod hnefatafl;
pub mod jigsaw;
pub mod keno;
pub mod kings;
pub mod long_nard;
pub mod loto_imagini;
pub mod mj2;
pub mod oina;
pub mod patolli;
pub mod pente;
pub mod pin_the_tail;
pub mod piquet;
pub mod pong_hau_kiki;
pub mod q6_reg;
pub mod q6_util;
pub mod rithmomachia;
pub mod shove_ha_penny;
pub mod sichuan_mahjong;
pub mod tafl;
pub mod tressette;
pub mod war;
pub mod yacht;
pub mod yaniv;
pub mod zara;
use polytabletop_core::GameObj;

/// How a catalog entry will be (or is) handled.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// Playable in PolyTabletop today.
    Implemented,
    /// Traditional / public-domain game: planned with its standard rules.
    Traditional,
    /// Modern branded game: the mechanics are free, the name, art and text are
    /// not, so it is planned as an original variant under our own name.
    Branded,
    /// Role-playing, miniature or collectible-card game: needs a generic
    /// toolkit (dice, sheets, decks), not a copy of any publisher's rules.
    Toolkit,
}

impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Implemented => "implementat",
            Kind::Traditional => "planificat (domeniu public)",
            Kind::Branded => "planificat (variantă proprie)",
            Kind::Toolkit => {
                "toolkit generic (RPG: vezi polytabletop-rpg; TCG/miniaturi: planificat)"
            }
        }
    }
}

/// One row of the catalog of target games.
pub struct E {
    pub name: &'static str,
    pub category: &'static str,
    pub origin: &'static str,
    pub kind: Kind,
    pub impl_id: &'static str,
}

/// A game that can be played now.
pub struct Playable {
    pub id: &'static str,
    pub title: &'static str,
    pub category: &'static str,
    pub rules: &'static str,
    pub make: fn() -> Box<dyn GameObj>,
}

/// Games whose `render` honours `set_viewer` (hot-seat shows only the mover's private info).
pub const HOTSEAT_SAFE: &[&str] = &[
    "trenuri",
    "forme6",
    "sir5",
    "kilometri",
    "palate",
    "tren_jaf",
    "rps",
    "morra",
    "mexico",
    "statuile",
    "pubquiz",
    "oldmaid",
    "mus",
    "bezique",
    "twentynine",
    "tute",
    "tiengow",
    "mahjong_hk",
    "sanma",
    // Q2-HOT-BEGIN
    "parimpar",
    "doppelkopf",
    "tarock",
    "sevens",
    "skitgubbe",
    "ombre",
    "domino9",
    "dog",
    "toc",
    "wouldyourather",
    "chinesewhispers",
    "babaoarba",
    // Q2-HOT-END
    // Q3-HOT-BEGIN
    "mau_mau",
    "muggins",
    "schafkopf",
    "preferans",
    "ulti",
    "brag",
    "sardines",
    // Q3-HOT-END
    "mahjong_am",
    "pit",
    "texas42",
    "mariash",
    "tysiac",
    "sheepshead",
    "crima",
    "mima",
    "comori",
    "ispy",
    "douazeci",
    "hangman",
    "settemezzo",
    "marbles",
    "jass",
    "manille",
    "klaverjas",
    "sueca",
    "matador",
    "spoons",
    "mahjong_tw",
    "scattergories",
    "nhie",
    "sichuan_mahjong",
    "guobiao",
    "celebrity",
    "tressette",
    "piquet",
    "batak",
    "yaniv",
    "ascunselea",
    // B2-hs-begin
    "vamesul",
    "agenti",
    "tablou",
    "turnuri_litere",
    // B2-hs-end
    // B6-hs-begin
    "vorbe_scurte",
    "semne_icoane",
    "grunturi",
    "gesturi_mute",
    "trei_runde",
    "schite_rapide",
    "crima_din_port",
    "primarul_si_lupul",
    "bentita",
    "totem_grabit",
    "palmuta_pe_cuvant",
    "duel_categorii",
    "duelul_argumentelor",
    "legenda_potrivita",
    "banda_nebuna",
    "favoritul_judecatorului",
    "ganduri_impartasite",
    "obiect_fulger",
    "pariu_pe_cifre",
    "definitii_false",
    "nuanta_ghicita",
    "camere_fitil",
    "duel_gloante",
    "vama_bacsis",
    "gandaci_blufati",
    "bal_masti",
    "reguli_schimbatoare",
    "palat_intrigi",
    // B6-hs-end
    "darurile_serii",
    "burako",
    "chinchon",
    "cuarenta",
    "escoba",
    "fivehundred",
    "pontoon",
    "pinochle",
    "gin_rummy",
    "crazy_eights",
    "pitch",
    "bid_whist",
    "all_fours",
    "conquian",
    "chicken_foot",
    "liars_dice",
    "litere",
    "curieri",
    "fitil",
    "codru",
    "conac",
    "insula",
    "magistrale",
    "bresla",
    "catun",
    "solzi",
    "salina",
    "reverdire",
    "rezervatia",
    "epoci",
    "hotar",
    "capcana",
    "cristale",
    "septica",
    "popa",
    "pescuit",
    "treizeci1",
    "casino",
    "zarpoker",
    "domino",
    "macao",
    "gura",
    "culori",
    "cuvantunic",
    "impostor",
    "povestitor",
    "schita",
    "treicai",
    "oprite",
    "dictionar",
    "belote",
    "bridge",
    "tarot",
    "skat",
    "poker",
    "rummy",
    "canasta",
    "cribbage",
    "euchre",
    "inimi",
    "pica",
    "prinde",
    "whist",
    "tarneeb",
    "baloot",
    "tablanet",
    "truco",
    "trix",
    "briscola",
    "scopa",
    "durak",
    "66",
    "doudizhu",
    "teenpatti",
    "riichi",
    "daifugo",
    "tienlen",
    "bigtwo",
    "tongits",
    "pasur",
    "hokm",
    "pisti",
    "courtpiece",
    "callbreak",
    "paigow",
    "ganjifa",
    "hanafuda",
    "hwatu",
    "barbut",
    "karuta",
    "blackjack",
    "battleship",
    "mastermind",
    "bingo",
    "loteria",
    "mahjong",
    "placute",
    "ceata",
    "umbra",
    "portret",
    "tcg",
    "tcg_evo",
    "tcg_zone",
    "tcg_lant",
    "tcg_cerneala",
    "nave",
    "expeditii",
    "artificii",
    "bufet",
    "whot",
    "chkobba",
    "ronda",
    "okey",
];

fn playable_base() -> Vec<Playable> {
    vec![
        Playable {
            id: "chess",
            title: "Șah",
            category: "Abstract",
            rules: "Reguli complete: rocadă (mută regele două pătrate: e1g1 sau e1c1), en passant, promovare (obligatorie; e7e8q, sufixul q/r/b/n alege piesa), remiză prin pat, repetiție de trei ori, 50 de mutări fără captură sau pion, material insuficient. Mutări: e2e4, e7e8q.",
            make: || Box::new(chess::Chess::new()),
        },
        Playable {
            id: "draughts",
            title: "Dame (8×8)",
            category: "Abstract",
            rules: "Dame engleze 8×8: piesele simple mută și capturează doar înainte, regele (piesa care ajunge pe ultimul rând; mutarea se oprește acolo) un pas în toate cele patru diagonale. Captura e obligatorie și salturile continuă cât se poate. Mutări: c3-d4, c3xe5xg7.",
            make: || Box::new(draughts::Draughts::new()),
        },
        Playable {
            id: "reversi",
            title: "Reversi",
            category: "Abstract",
            rules: "Pui o piesă care încadrează în linie dreaptă piese ale adversarului și le întoarce; negrul (J1) începe; câștigă cine are mai multe piese la final. Mutări: d3 (pătratul ales), pass (doar când nu ai nicio mutare).",
            make: || Box::new(reversi::Reversi::new()),
        },
        Playable {
            id: "connect4",
            title: "Patru în linie",
            category: "Abstract",
            rules: "Lasă discuri în coloane; primul cu patru în linie (orizontal, vertical sau diagonal) câștigă. Mutări: 1–7 (numărul coloanei).",
            make: || Box::new(connect4::Connect4::new()),
        },
        Playable {
            id: "gomoku",
            title: "Gomoku (15×15)",
            category: "Abstract",
            rules: "Cinci sau mai multe în linie câștigă; negrul începe în centru. Mutări: h8 (coloane a–o, rânduri 1–15).",
            make: || Box::new(gomoku::Gomoku::new()),
        },
        Playable {
            id: "hex",
            title: "Hex (11×11)",
            category: "Abstract",
            rules: "X unește sus și jos, O unește stânga și dreapta printr-un lanț de hexagoane vecine; nu există remiză, X începe. Mutări: f6 (coloane a–k, rânduri 1–11).",
            make: || Box::new(hex::Hex::new()),
        },
        Playable {
            id: "amazoane",
            title: "Amazoane (10×10)",
            category: "Abstract",
            rules: "Patru amazoane pe parte; muți una ca o regină de șah, apoi tragi o săgeată (tot ca o regină) care blochează pătratul; pierde cine nu mai poate muta. Mutări: d1-d5/d8 (de la–la/săgeată).",
            make: || Box::new(amazoane::Amazoane::new()),
        },
        Playable {
            id: "linii",
            title: "Linii de acțiune (8×8)",
            category: "Abstract",
            rules: "Muți o piesă pe linie atâtea pătrate câte piese sunt pe acea linie; poți sări piese proprii, nu ale adversarului, și captezi aterizând pe una; câștigă cine își unește toate piesele într-un grup (remiză după 300 de mutări). Mutări: b1-b3, b1xd3.",
            make: || Box::new(linii::Linii::new()),
        },
        Playable {
            id: "nim",
            title: "Nim cu grămezi",
            category: "Abstract",
            rules: "Patru grămezi (3, 4, 5, 6 bețe); iei oricâte bețe dintr-o singură grămadă; cine ia ultimul băț câștigă. Mutări: b3 (litera grămezii + număr).",
            make: || Box::new(nim::Nim::new()),
        },
        Playable {
            id: "morris",
            title: "Moara (Țintar)",
            category: "Abstract",
            rules: "9 piese: plasare, apoi mutare pe linii; o moară (trei pe o linie) scoate o piesă adversă; cu 3 piese poți „zbura”. Mutări: d1 (plasare), a1-a4 (mutare); cu moară se adaugă piesa scoasă: g1xb2, a1-a4xb2.",
            make: || Box::new(morris::Morris::new()),
        },
        Playable {
            id: "mancala",
            title: "Mancala (Kalah)",
            category: "Abstract",
            rules: "6 gropi și un depozit; semănatul ajunge în depozit = încă o mutare; groapă goală proprie = captură. Mutări: 1–6 (groapa proprie aleasă).",
            make: || Box::new(mancala::Mancala::new()),
        },
        Playable {
            id: "go",
            title: "Go (9×9)",
            category: "Abstract",
            rules: "Scor pe arie (pietre + teritoriu), komi 7,5 pentru alb, ko simplu, fără sinucidere; două pass-uri consecutive încheie jocul. Mutări: e5 (coloane a–h și j, fără i; rânduri 1–9), pass.",
            make: || Box::new(go::Go::new()),
        },
        Playable {
            id: "xiangqi",
            title: "Xiangqi (șah chinezesc)",
            category: "Abstract",
            rules: "9×10, palat, râu, tun cu ecran, cal blocabil, generalii nu se pot privi. Mat sau pat = pierdere. Mutări: h2e2 (fișiere a–i, rânduri 0–9).",
            make: || Box::new(xiangqi::Xiangqi::new()),
        },
        Playable {
            id: "shogi",
            title: "Shogi (șah japonez)",
            category: "Abstract",
            rules: "9×9; piesele capturate se pun înapoi (drop). Promovare în zona adversă (obligatorie dacă piesa n-ar mai avea mutări). Drop interzis: nifu (doi pioni pe fișier), pion/lance/cal fără mutări, pion care dă mat. Mat = pierdere; a 4-a repetiție sau 320 de semi-mutări = remiză. Mutări: 7g7f, 2b8h+ (promovare), P*5e (drop).",
            make: || Box::new(shogi::Shogi::new()),
        },
        Playable {
            id: "dara",
            title: "Dara (Nigeria)",
            category: "Abstract",
            rules: "Tablă 5×6, 12 piese pe jucător. Faza 1: plasare pe rând (fără capturi). Faza 2: mutare ortogonală cu un pas; o moară de exact 3 (rând sau coloană, nu 4) scoate o piesă adversă oarecare. Pierzi cu sub 3 piese sau blocat; remiză după 100 de mutări fără captură. Mutări: b3, b3-b4, b3-b4xd2 (fișiere a–f, rânduri 1–5).",
            make: || Box::new(dara::Dara::new()),
        },
        Playable {
            id: "morabaraba",
            title: "Morabaraba (Africa de Sud)",
            category: "Abstract",
            rules: "Tablă ca la Moară plus diagonalele prin colțuri (20 de „vaci”), 12 piese. Plasare, apoi mutare pe linii (și diagonale); vaca scoate o piesă adversă (nu din vacă, dacă are alta); cu 3 piese zbori; sub 3 piese sau blocat = pierdere; tablă plină după plasare = remiză. Mutări: d1, a1-b2, c3xg7, a1-d5xg7.",
            make: || Box::new(morabaraba::Morabaraba::new()),
        },
        Playable {
            id: "sungka",
            title: "Sungka (Filipine)",
            category: "Abstract",
            rules: "7 gropi și un depozit pe jucător, 7 semințe în fiecare groapă. Semănat în sens trigonometric prin depozitul propriu, sărind depozitul adversarului. Ultima sămânță în depozit = încă o mutare; în groapă ocupată = ridici și continui; în groapă goală proprie = capturezi groapa opusă; în groapă goală adversă = tura se termină. O singură rundă, fără gropi „arse”, mutări pe rând; câștigă cine are peste 49 în depozit sau mai mult la final. Mutări: 1–7 (groapa proprie, de la stânga la dreapta).",
            make: || Box::new(sungka::Sungka::new()),
        },
        Playable {
            id: "makruk",
            title: "Makruk (șah thailandez)",
            category: "Abstract",
            rules: "8×8: Khun (rege), Met (un pas diagonal), Khon (un pas diagonal sau înainte), cal, turn; pionii mută și capturează cu un pas și se promovează în Met pe rândul 6. Fără rocadă. Mat = victorie; pat = remiză; fără pioni, numărare limitată (8–64 de mutări) = remiză. Mutări: e3e4.",
            make: || Box::new(makruk::Makruk::new()),
        },
        Playable {
            id: "janggi",
            title: "Janggi (șah coreean)",
            category: "Abstract",
            rules: "9×10, palate cu diagonale, Cho (jos) începe. Car pe diagonalele palatului, tun care sare exact o piesă (nu tun), cal blocabil, elefant (1 ortogonal + 2 diagonale) blocabil, soldați înainte/lateral. Pass permis când nu ești în șah; mat = pierdere; repetiție, 2 pass-uri sau 150 de semi-mutări fără captură = remiză. Mutări: b2e2, pass.",
            make: || Box::new(janggi::Janggi::new()),
        },
        Playable {
            id: "popa",
            title: "Popa prostul",
            category: "Cărți",
            rules: "Old Maid românesc: 51 de cărți (fără Dama de treflă), perechile se aruncă automat. Tragi pe rând o carte din mâna adversarului, la alegere după poziție; cine rămâne cu popa (Dama nepereche) pierde. Informație ascunsă: vezi doar mâna ta. Mutări: trage 3, sfarsit.",
            make: || Box::new(popa::Popa::new()),
        },
        Playable {
            id: "pescuit",
            title: "Pescuit (Go Fish)",
            category: "Cărți",
            rules: "52 de cărți, 7 în mână. Ceri o valoare pe care o ai (cer 7); adversarul răspunde forțat da (îți dă toate cărțile) sau pescuieste (tragi din stoc; dacă prinzi valoarea cerută mai ceri). Patru la fel formează un set; cele mai multe seturi câștigă. Informație ascunsă: vezi doar mâna ta. Mutări: cer K, da, pescuieste.",
            make: || Box::new(pescuit::Pescuit::new()),
        },
        Playable {
            id: "treizeci1",
            title: "Treizeci și unu (31)",
            category: "Cărți",
            rules: "3 cărți în mână, A=11, figuri=10; valoarea = suma celei mai bune culori (trei la fel = 30). Tragi (trage) sau iei din aruncate (ia), apoi arunci (arunca 5h), sau bați (bat): adversarul mai are o tură, apoi cea mai mică mână pierde o viață (bătătorul singur cel mai mic: două). 31 = adversarul pierde o viață. 3 vieți. Informație ascunsă: vezi doar mâna ta. Mutări: trage, ia, bat, arunca Kd, sfarsit.",
            make: || Box::new(treizeci1::Treizeci1::new()),
        },
        Playable {
            id: "casino",
            title: "Cassino (capturi)",
            category: "Cărți",
            rules: "52 de cărți, 4 pe masă, 4 în mână. Lași o carte (5h) sau capturezi (5h x 2c 3d): grupuri de cărți identice sau cu suma egală cu a ta (A=1, J/Q/K doar identice). Puncte pe împărțire: cărți 3, pici 1, 10♦ 2, 2♠ 1, ași 1, mături 1; se joacă până la 11. Fără construiri. Informație ascunsă: vezi doar mâna ta.",
            make: || Box::new(casino::Casino::new()),
        },
        Playable {
            id: "zarpoker",
            title: "Poker de zaruri",
            category: "Zaruri",
            rules: "Variantă originală: 7 runde, 5 zaruri, până la 3 aruncări (arunc; tine 3 3 5 sau tine -; fin). Mâini de poker: cinci la fel > careu > full > scară > trei > două perechi > pereche > nimic. Mâna terminată a primului jucător rămâne acoperită până termină al doilea; cine are mâna mai bună ia punctul rundei.",
            make: || Box::new(zarpoker::ZarPoker::new()),
        },
        Playable {
            id: "septica",
            title: "Septica",
            category: "Cărți",
            rules: "Pachet de 32 de cărți, 4 în mână. Preiei masa cu aceeași valoare ca prima carte sau cu 7 / 8; poți tăia la rândul tău; cine dă pas pierde masa. 10 și A = 1 punct (8 în total), peste 4 câștigi. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 9h, 7s, pas.",
            make: || Box::new(septica::Septica::new()),
        },
        Playable {
            id: "macao",
            title: "Macao",
            category: "Cărți",
            rules: "52 de cărți, 5 în mână: culoare sau valoare. 2 / 3 se adună (taie cu alt 2 / 3, altfel tragi); 4 sare peste rând; 7 merge peste orice și alege culoarea (7h:s); As cere culoarea (Ah:d). Fără carte bună tragi una. Câștigă cine rămâne fără cărți (nu cu carte specială). Mutări: 9h, trage, sar, pas.",
            make: || Box::new(macao::Macao::new()),
        },
        Playable {
            id: "domino",
            title: "Domino (dublu-6)",
            category: "Clasic",
            rules: "28 de piese, 7 fiecare, restul stoc. Începe cel mai mare dublu. Așază la stânga (S) sau dreapta (D); fără piesă pescuiești (pesc), cu stocul gol dai pas. Câștigă cine termină piesele (ia punctele adversarului) sau, la blocaj, cine are mai puține puncte. Mutări: 3-4:S, 3-4:D, pesc, pas.",
            make: || Box::new(domino::Domino::new()),
        },
        Playable {
            id: "backgammon",
            title: "Table (Backgammon)",
            category: "Zaruri",
            rules: "Două zaruri aruncate automat; 15 piese, bară, bear-off, dubla se joacă de 4 ori, fără cub de dublare. Un pas = un zar; trebuie jucate cât mai multe zaruri. Numerotarea e a jucătorului la mutare. Mutări: 13/7, bar/22, 3/off.",
            make: || Box::new(backgammon::Backgammon::new()),
        },
        Playable {
            id: "ludo",
            title: "Nu te supăra, frate (Ludo)",
            category: "Zaruri",
            rules: "4 piese, un zar. Cu 6 scoți o piesă din bază; 6 = încă o aruncare; aterizarea pe adversar îl trimite în bază; ținta se atinge exact. Mutări: intră, sau progresul piesei (ex. 12).",
            make: || Box::new(ludo::Ludo::new()),
        },
        Playable {
            id: "snakes",
            title: "Șerpi și scări",
            category: "Zaruri",
            rules: "Tablă 1–100; zarul e aruncat automat și afișat; scările urcă, șerpii coboară; ținta se atinge exact. Singura mutare: arunc.",
            make: || Box::new(snakes::Snakes::new()),
        },
        Playable {
            id: "barbut",
            title: "Barbut (duel simplificat)",
            category: "Zaruri",
            rules: "10 jetoane fiecare. Pariu 1–3; prima aruncare: 7/11 câștigă, 2/3/12 pierde, altfel punct; adversarul plătește (dublează) sau lasă; apoi punctul bate 7. Mutări: pariez 1, arunc, plătesc, las.",
            make: || Box::new(barbut::Barbut::new()),
        },
        Playable {
            id: "66",
            title: "Șase-zeci și șase (Santase)",
            category: "Cărți",
            rules: "2 jucători, 24 de cărți, atu, anunțuri 20/40, schimbul nouarului de atu, închiderea jocului; 66 de puncte încheie mâna, meci până la 7 puncte de joc. Informație ascunsă: vezi doar mâna ta (J1) și numărul de cărți ale adversarului. Mutări: 9h, Qs, 10d, Ac, Qh+ (anunț), schimb, inchide, urmatoarea.",
            make: || Box::new(sixtysix::SixtySix::new()),
        },
        Playable {
            id: "blackjack",
            title: "Blackjack (21)",
            category: "Cărți",
            rules: "Tu (J1) contra dealerului (J2, trage până la 17): carte / stai / dublu; blackjack plătește 3:2. Set de 21 de mâini cu pariu de 10 din 100 de jetoane: câștigi dacă termini peste 100. Pașii dealerului sunt mutări forțate: dealer-trage, dealer-sta, decontare.",
            make: || Box::new(blackjack::Blackjack::new()),
        },
        Playable {
            id: "rummy",
            title: "Remi (Rummy, 13 cărți)",
            category: "Cărți",
            rules: "2 jucători, 13 cărți, un pachet: tragi (stoc sau talon) și arunci; închizi când cele 13 cărți rămase formează secvențe și seturi (cel puțin o secvență; aranjare automată). Penalizare = punctele libere; meci până la 101. Mutări: stoc, talon, Ah, 10c, inchide-Ah, urmatoarea.",
            make: || Box::new(rummy::Rummy::new()),
        },
        Playable {
            id: "poker",
            title: "Poker (Texas hold'em, heads-up)",
            category: "Cărți",
            rules: "20 de mâini, blinduri fixe 10/20, 1000 de jetoane, raise fix (20 / 40 de la turn), maxim 3 raise-uri pe rundă. Câștigă cine are mai multe jetoane. Mutări: fold, call/check, raise; pașii de împărțire sunt forțați: flop, turn, river, showdown, decontare.",
            make: || Box::new(poker::Poker::new()),
        },
        Playable {
            id: "tcg",
            title: "Cărți colecționabile (Ținutul Cenușiu)",
            category: "TCG",
            rules: "Motor generic de cărți colecționabile cu 60 de cărți originale (resurse, creaturi, vrăji); pachete pregătite de 30 de cărți (J1 Cetatea, J2 Furtuna), 20 de viață, 5 cărți în mână. Tură: pescuire (forțată), faza principală, fin. O resursă pe tură (+1 mană). Creaturile noi nu atacă (Grabă = da); cuvinte-cheie: Zbor, Grabă, Armură, Venin, Primul, Străjer. Atac: ataca 1,3; apărătorul alege pe rând bloc N sau fara. Pachet gol la pescuit = pierdere. Informație ascunsă: vezi doar mâna J1. Mutări: pesca, joaca 2, joaca 3 e1 (țintă: e=inamic, m=al meu, fata), ataca 1,2, bloc 1, fara, fin.",
            make: || Box::new(tcg::Tcg::new()),
        },
        Playable {
            id: "miniaturi",
            title: "Miniaturi hexagonale (Trecătoarea Cenușie)",
            category: "Miniaturi",
            rules: "Motor generic de miniaturi pe hartă hexagonală: 7 unități originale pe parte (Lăncieri, Arcași, Călăreți, Colos, Catapulte, Comandant), teren (stâncă blochează vederea, pădure = acoperire), zaruri de lovire și salvare, moral (șovăit, apoi fugă). 6 runde cu activări alternante; 3 obiective (cine are mai multe figuri la ≤1 hex ia 1 punct pe rundă); eliminarea armatei adverse câștigă. Informație completă; zarurile se aruncă la mutarea forțată arunca. Mutări: A pas, A>d4, A>d4 xK (mută și atacă), A xK, arunca.",
            make: || Box::new(miniaturi::Miniaturi::new()),
        },
        Playable {
            id: "mahjong",
            title: "Mahjong (Hong Kong, 2 jucători)",
            category: "Clasic",
            rules: "136 de piese, 13 în mână, zid viu de 96 și zid mort de 14 (ascuns; vezi doar câte au rămas). Tragi și arunci; adversarul poate lua aruncarea cu pung / kong / chi (de la oricare aruncare) sau pe câștig. Mână câștigătoare = 4 seturi + pereche și minimum 1 fan (tragere proprie, mână închisă, toate secvențele, toate pungurile, o culoare, dragoni, vânturi). Fără șapte perechi / mâini speciale. Zid epuizat = remiză. Mutări: trage, d5m, d Dr, mahjong, pung, kong, chi2m3m, kong5m, pas, trage-moarta.",
            make: || Box::new(mahjong::Mahjong::new()),
        },
        Playable {
            id: "hanafuda",
            title: "Hanafuda (Koi-Koi)",
            category: "Cărți",
            rules: "48 de cărți de flori (12 luni × 4), 8 în mână și 8 pe masă; potrivești luna: obligatoriu capturezi dacă ai pereche (doi candidați = alegi: 3a:3c, trei = iei toate). Apoi întorci o carte din stoc (intoarce) și o joci la fel. Yaku nou (luminoase, mistreț-cerb-fluturi, panglici, animale, simple, hanami, tsukimi): koi-koi (continui) sau shobu (încasezi; ×2 la 7+ și ×2 dacă adversarul a spus koi-koi). 3 runde. Mutări: 3b, 3a:3c, intoarce, ia 3c, koi-koi, shobu.",
            make: || Box::new(hanafuda::Hanafuda::new()),
        },
        Playable {
            id: "hwatu",
            title: "Hwatu (Go-Stop)",
            category: "Cărți",
            rules: "Aceleași 48 de cărți de flori; 10 în mână, 8 pe masă, 20 în stoc. Captură ca la Hanafuda (3c, 3a:3c, intoarce, ia 3c). Scor: gwang, godori, hongdan/cheongdan/chodan, tti, yeolkkeut, pi. De la 7 puncte alegi stop (încasezi) sau go (continui: +1 la primele două, apoi ×2; ×2 gwang-bak, pi-bak și go-bak). Măturarea mesei ia o simplă de la adversar. 3 runde. Fără ppeok/jjok/ttadak.",
            make: || Box::new(hwatu::new()),
        },
        Playable {
            id: "karuta",
            title: "Karuta (recitare vs AI, proverbe)",
            category: "Clasic",
            rules: "Karuta cu 40 de proverbe românești: 16 cărți pe masă (8 pe parte, cu sfârșitul proverbului), 24 de citiri (8 false). Începutul se dezvăluie cuvânt cu cuvânt; pe rând: asteapta sau ia N (atingi cartea). Corect: cartea dispare (ținta de pe partea adversarului: îi trimiți una de-a ta). Greșit: adversarul îți trimite o carte. Câștigă cine rămâne primul fără cărți (sau are mai puține după toate citirile). Mutări: asteapta, ia 12.",
            make: || Box::new(karuta::Karuta::new()),
        },
        Playable {
            id: "doudizhu",
            title: "Dou dizhu (2 jucători)",
            category: "Cărți",
            rules: "54 de cărți (3…K, A, 2, jokeri X/Y); 17 fiecare, 20 ascunse nefolosite. Combinații: single, pereche, trio (+1 sau +pereche), scară de 5+ (34567), scară de perechi 3+ (334455), bombă (5555), rachetă (XY). Răspunzi cu același tip și lungime, mai mare, sau bombă/rachetă, ori pas. Cine rămâne fără cărți câștigă runda; cel mai bun din 3. Fără avioane și careu+2. Mutări: 5, 55, 555+3, 34567, 5555, XY, pas.",
            make: || Box::new(dou_dizhu::DouDizhu::new()),
        },
        Playable {
            id: "paigow",
            title: "Pai Gow (domino chinezesc)",
            category: "Clasic",
            rules: "32 de piese; 4 piese fiecare, împărțite în față (mâna mică) și spate (mâna mare); față vs față și spate vs spate contra dealerului (rolul alternează; copiile și 0–0 merg la dealer). Perechi (Gee Joon > Teen > Day > … > Chop-Ng) > Wong > Gong > puncte modulo 10. Ambele câștigate = +10, una și una = remiză. 12 runde, 100 de jetoane. Mutări: aranj 1/2/3, arata, urmatoarea.",
            make: || Box::new(pai_gow::PaiGow::new()),
        },
        Playable {
            id: "teenpatti",
            title: "Teen Patti (heads-up)",
            category: "Cărți",
            rules: "3 cărți fiecare, 12 mâini, 500 de jetoane, boot 10. Ierarhie: trio > scară de culoare > scară (AKQ, A23, KQJ…) > culoare > pereche > carte mare. Mutări: pack (abandon), chaal (plătești miza), raise (dublezi, max 80), show (după cel puțin o acțiune; la egalitate pierde cel care cere show); arata și urmatoarea sunt forțate. Maxim 6 acțiuni pe mână.",
            make: || Box::new(teen_patti::TeenPatti::new()),
        },
        Playable {
            id: "riichi",
            title: "Riichi Mahjong (2 jucători)",
            category: "Clasic",
            rules: "Mahjong japonez pentru doi: 136 de piese (1m–9m, 1p–9p, 1s–9s, 1z–7z = vânturi și dragoni), 13 piese, 4 mâini (Est 1–2, Sud 1–2), 25 000 de puncte. Tura: trage, apoi arunci (5m), riichi 5m, tsumo, ankan 5m, kakan 5m (după careu: trage-moarta); pe aruncarea adversarului: ron, pon, kan, chi 4m 6m sau pas. Mână = 4 seturi + pereche (sau 7 perechi / 13 orfani) cu cel puțin un yaku; furiten, dora/ura-dora, fu și han până la yakuman. Zidul e ascuns. Fără piese roșii.",
            make: || Box::new(riichi::Riichi::new()),
        },
        Playable {
            id: "sicbo",
            title: "Sic Bo",
            category: "Zaruri",
            rules: "Trei zaruri; duel contra „casei” cu 100 de jetoane, 12 runde (ordinea de pariere alternează). Pariuri (plăți X:1): mic 1, mare 1 (pierd la triplu), cifra N 1/2/3, dublu N 10, triplu N 180, oricare-triplu 30, suma S (4 și 17: 60 … 9–12: 6), pereche a-b 5. Cel mult 6 pariuri pe rundă, mize 1/5/10/25. Mutări: mic:5, cifra:3:5, suma:9:5, pereche:1-2:5, fin, arunc, urmatoarea. Câștigă cine are mai multe jetoane.",
            make: || Box::new(sicbo::SicBo::new()),
        },
        Playable {
            id: "ashtachamma",
            title: "Ashta Chamma",
            category: "Familie",
            rules: "Tablă 5×5, 4 piese pe jucător, 4 cauri (1–4; zero în sus = 8; 4 și 8 dau încă o aruncare). Piesele merg în sens trigonometric pe inelul exterior (16 pătrate) și intră în inelul interior și în centru doar după o captură; centrul se atinge exact. Pătrate sigure (X): mijloace de latură, colțuri, centru; două piese ale aceluiași jucător formează un bloc. Captura dă încă o aruncare. Câștigă cine aduce toate piesele în centru. Mutări: arunc, 7 sau 7* (piesa de pe pătratul 7 al căii; * = a capturat), pas.",
            make: || Box::new(ashtachamma::AshtaChamma::new()),
        },
        Playable {
            id: "daifugo",
            title: "Daifugo (Daihinmin)",
            category: "Cărți",
            rules: "54 de cărți (cu 2 jokeri), 27 fiecare, 5 runde. Ordine 3…K, A, 2, joker. Joci un grup de același rang (33, 777), adversarul același număr de cărți mai mari sau pas; după pas conduce cine a jucat ultimul. Joker singur (X) sau wild (5X). 4 cărți = revoluție (ordinea se inversează), orice grup cu 8 oprește jocul (conduci din nou). Câștigi runda când rămâi fără cărți; perdantul îți dă cele mai bune 2 cărți, tu dai 2 înapoi (da 4 5). Mutări: 3, 77, JJJ, 55X, X, pas, da 4 5.",
            make: || Box::new(daifugo::Daifugo::new()),
        },
        Playable {
            id: "tienlen",
            title: "Tiến lên",
            category: "Cărți",
            rules: "52 de cărți, 13 fiecare (restul ascuns). Rang 3…A, 2; culoare ♠ < ♣ < ♦ < ♥. Combinații: carte, pereche, treime, careu, scară (3+, fără 2), 3+ perechi consecutive; aceeași formă și mai mare, sau pas. Tăieri pe 2: careu sau 3 perechi consecutive pe un 2, careu sau 4 perechi consecutive pe o pereche de 2; 4 perechi consecutive taie și careul. Victorie instantanee: patru de 2, șase perechi, scară 3…A. Primul la 3 mâini. Mutări: 3s, 9h 9d, 5c 6d 7h, pas.",
            make: || Box::new(tienlen::TienLen::new()),
        },
        Playable {
            id: "bigtwo",
            title: "Big Two (Pusoy dos)",
            category: "Cărți",
            rules: "52 de cărți, 13 fiecare (restul ascuns). Rang 3…A, 2; culoare ♦ < ♣ < ♥ < ♠. Carte, pereche, treime sau mână de 5: scară < culoare < full < careu+1 < scară de culoare. Dai aceeași formă mai mare ori pas; cine are cea mai mică carte începe cu ea. Mâna se termină la golirea cărților: perdantul primește cărțile rămase ca penalizare (dublu de la 10 cărți); la 30 de puncte pierde meciul (cel mult 8 mâini). Mutări: 3d, 9h 9s, 3d 4c 5h 6s 7d, pas.",
            make: || Box::new(bigtwo::BigTwo::new()),
        },
        Playable {
            id: "tongits",
            title: "Tong-its",
            category: "Cărți",
            rules: "52 de cărți, 12 fiecare (dealerul 13 și începe fără să tragă). Tura: trage din stoc sau ia aruncata dacă o folosești imediat (ia 5d 5h 5c / ia sapaw 2), apoi pune seturi sau scări (pune 3h 4h 5h), adaugă la orice combinație (sapaw 6h 2) și aruncă (arunca Kd). Mână goală = Tongits, 3 p.; cheama (cu o combinație pusă) → adversarul renunt (1 p.) sau lupta: mai puține puncte în mână câștigă (egalitate: cel care a luptat; dacă pierde cel care a chemat, adversarul ia 2 p.); stoc epuizat: mai puține puncte. Primul la 6 p.",
            make: || Box::new(tongits::Tongits::new()),
        },
        Playable {
            id: "pasur",
            title: "Pasur",
            category: "Cărți",
            rules: "52 de cărți, 4 pe masă și 4 în mână (se dau din nou câte 4). Un număr (A=1…10) capturează orice grupuri de cărți de pe masă cu suma 11 împreună cu el; Valetul ia tot în afară de Dame și Regi; Dama ia o Damă, Regele un Rege. Sur (masa golită, nu cu Valet) 5 p. Puncte: cele mai multe cărți 3, ≥7 trefle 7, 10♦ 3, 2♣ 2, Ași 1, Valeți 1. Joc până la 62. Mutări: 7h (pui), 5s x 2c 4d 6h (capturezi), imparte.",
            make: || Box::new(pasur::Pasur::new()),
        },
        Playable {
            id: "hokm",
            title: "Hokm",
            category: "Cărți",
            rules: "52 de cărți, câte 13 (restul ascuns). Hakem vede mai întâi 5 cărți și alege atuul (hokm h); conduce prima levată. Răspunzi la culoare; atuul bate tot. 7 din 13 levate = 1 p., 13–0 = 2 p., hakem cu zero levate = 3 p.; hakem rămâne dacă câștigă. Primul la 7 puncte. Mutări: hokm h (c/d/h/s), 10h, As.",
            make: || Box::new(hokm::Hokm::new()),
        },
        Playable {
            id: "pisti",
            title: "Piști",
            category: "Cărți",
            rules: "52 de cărți: 4 pe masă (3 cu fața în jos), 4 în mână. Cartea ta ia toată grămada dacă are același rang ca cea de deasupra sau e Valet. Piști (grămadă de o carte luată cu același rang) 10 p., Valet peste Valet 20 p. Puncte: Ași și Valeți 1, 2♣ 2, 10♦ 3, cele mai multe cărți 3. Se joacă până la 101. Mutări: 7h, Js, imparte.",
            make: || Box::new(pisti::Pisti::new()),
        },
        Playable {
            id: "courtpiece",
            title: "Court Piece (Rang, Mendikot)",
            category: "Cărți",
            rules: "52 de cărți, câte 26. Cine nu e dealer vede primele 5 cărți și alege atuul (rang h), apoi se împarte restul și el conduce. Răspunzi la culoare; atuul bate tot. Mâna o câștigă cine ia mai multe zeci (mendi): 1 p.; toate 4 zecile (Mendikot) 2 p.; toate levatele (Court) 3 p.; la 2–2 decid levatele. Primul la 5 puncte. Mutări: rang h, 10h, As.",
            make: || Box::new(courtpiece::CourtPiece::new()),
        },
        Playable {
            id: "callbreak",
            title: "Call Break",
            category: "Cărți",
            rules: "52 de cărți, câte 13 (restul ascuns); pica e mereu atu. Cine nu e dealer licitează primul (licit 4) numărul de levate și conduce. Răspunzi la culoare și, dacă poți, cu o carte mai mare; fără culoare tai cu pică. Scor: licitația + 0,1 pe levată în plus, sau − licitația dacă ai luat mai puțin. 5 mâini, scorul total mai mare câștigă. Mutări: licit 4, 10h, As.",
            make: || Box::new(callbreak::CallBreak::new()),
        },
        Playable {
            id: "ganjifa",
            title: "Ganjifa (levate)",
            category: "Cărți",
            rules: "48 de cărți (4 culori: Taj T, Shamsher S, Surkh R, Chang C; Mir M, Vazir V, 1–10). La T și S 10 e cel mai mare număr, la R și C ordinea e inversată (1 cel mai mare). 12 cărți fiecare; cartea de sus din stoc arată atu. 12 levate, culoare obligatorie, atu bate. Mai multe levate câștigă. Mutări: TM, S10, R1, CV.",
            make: || Box::new(ganjifa::Ganjifa::new()),
        },
        Playable {
            id: "yut",
            title: "Yut (Yunnori)",
            category: "Zaruri",
            rules: "4 bețe-zar (arunc): 1 plat = do, 2 = gae, 3 = geol, 4 = yut (încă o aruncare), 0 = mo (5, încă o aruncare). 4 piese pe jucător; rezultatele se folosesc în orice ordine; grupurile de pe același nod merg împreună; capturarea trimite la start și dă o aruncare. Scurtături din colțurile 5 și 10 (centrul 22). Câștigă cine scoate toate piesele. Mutări: arunc, do:start, gae:7, mo:22.",
            make: || Box::new(yut::Yut::new()),
        },
        Playable {
            id: "sugoroku",
            title: "Sugoroku (cursă cu casete)",
            category: "Zaruri",
            rules: "Drum 0–40, două zaruri (arunc), alegi un zar sau suma (avansez N). Atingi exact 40 ca să câștigi (surplusul te întoarce). Casete: 4 +3, 9 −3, 12 și 28 pierzi tura, 15 și 36 încă o aruncare, 18 +5, 21 schimbi locul cu adversarul, 25 −6, 30 +4, 33 −8, 38 −10 (fără lanț). Mutări: arunc, avansez 7.",
            make: || Box::new(sugoroku::Sugoroku::new()),
        },
        Playable {
            id: "chaupar",
            title: "Chaupar (cauri, 2 jucători)",
            category: "Zaruri",
            rules: "4 piese, circuit de 48 de pătrate (start 0 / 24), 4 cauri: 1–4 = pași, 0 = 8; 4 și 8 dau încă o aruncare; scoți din curte cu 1 sau 8 (intra). Castelele (multipli de 6) sunt sigure; captura trimite în curte și dă o aruncare. Coloană de acasă și centru exact. Mutări: arunc, intra, mut:12, mut:casa2, pas.",
            make: || Box::new(chaupar::Chaupar::new()),
        },
        Playable {
            id: "culori",
            title: "Culori (potrivit cărți colorate)",
            category: "Cărți",
            rules: "Joc original: 108 cărți (4 culori cu cifre 0–9, sari, intoarce, plus2, plus joker și joker4). Joci o carte de aceeași culoare sau cu aceeași cifră/simbol; în doi, sari/intoarce = adversarul pierde rândul, plus2 = trage 2, joker4 = trage 4 (jokerii aleg culoarea: joker:r). Fără carte jucabilă tragi (trage), apoi o joci sau dai pas. Câștigă cine rămâne primul fără cărți. Informație ascunsă: vezi doar mâna ta. Mutări: r5, gsari, vplus2, joker:a, trage, pas.",
            make: || Box::new(culori::Culori::new()),
        },
        Playable {
            id: "mosii",
            title: "Moșii (afaceri imobiliare cu zaruri)",
            category: "Zaruri",
            rules: "Joc original, 32 de câmpuri: arunci două zaruri (automat), cumperi terenuri și gări (cumpar / las), încasezi chirii (grup complet = chirie dublă, case: construiesc 3), taxe, evenimente Noroc și închisoare (platesc 50 sau dublă); dublă = încă o aruncare. Faliment = pierdere; după 1500 de mutări câștigă averea mai mare. Fără licitații și ipoteci.",
            make: || Box::new(mosii::Mosii::new()),
        },
        Playable {
            id: "itinerar",
            title: "Itinerar (turism pe tablă, rezervi destinații)",
            category: "Zaruri",
            rules: "Joc original, 28 de câmpuri (7 regiuni × 3 destinații + aeroporturi, tombolă, vamă, bac, hotel de lux, taxă de turist): arunc, apoi mut (zarurile 2d6 sunt automate); pe o destinație liberă: rezerv (taxa de intrare) sau las; pe una ocupată plătești taxa de cazare. La final: construiesc N (cazare, 3 niveluri), rezerv N (la distanță, preț ×1,5, o dată pe tură), zbor (între aeroporturi, 60) sau gata. Colecție completă de regiune = taxe ×2; dublă = încă o aruncare, a treia dublă = vamă (pierzi tura); lux (nivel gratuit la Hotelul de lux). Faliment = eliminare; după 40 de runde câștigă averea mai mare. Informație publică.",
            make: || Box::new(itinerar::Itinerar::new()),
        },
        Playable {
            id: "magistrale",
            title: "Magistrale (rute cu cărți de vagon)",
            category: "Strategie",
            rules: "Joc original: hartă cu 19 orașe și 42 de rute; strângi cărți de vagon (r a v g m, j = locomotivă) și ocupi rute (ocupa 12 r), tragi 2 cărți (trage / trage r) sau bilete de destinație secrete (bilete, pastrez 1,3). La final bilet îndeplinit +valoarea, neîndeplinit −valoarea, drum lung +10. În doi, rutele duble se ocupă o singură dată. Informație ascunsă: vezi doar cărțile și biletele tale.",
            make: || Box::new(magistrale::Magistrale::new()),
        },
        Playable {
            id: "conac",
            title: "Misterul din Conac (deducție)",
            category: "Deducție",
            rules: "Joc original: o crimă cu soluție secretă (suspect s1–s6, armă a1–a6, cameră c1–c9); 18 cărți împărțite 9–9. Sugerezi (sugerez s2 a3 c5), adversarul îți arată în secret o carte pe care o are (arat s2) sau spune nu-am; deduci și acuzi (acuz s2 a3 c5, apoi verific): corect câștigi, greșit pierzi. Fără tablă de mers. Informație ascunsă: vezi doar mâna ta și cărțile arătate ție.",
            make: || Box::new(conac::Conac::new()),
        },
        Playable {
            id: "insula",
            title: "Insula (colonizare cu resurse)",
            category: "Strategie",
            rules: "Joc original pe 19 hexagoane (pădure l, argilă a, pășune p, lan g, munte m) cu numere 2–12: pui colonii și drumuri (colonie v12, drum e7), arunci zarurile (arunc) ca să primești resurse, la 7 mută tâlharul (bandit h5), construiești colonii/orașe/drumuri, schimbi 4:1 cu banca (schimb l g) și închei (sfarsit). Colonie 1 punct, oraș 2, cel mai lung drum (≥5) 2; primul la 10 câștigă. Fără cărți de dezvoltare, porturi sau schimb între jucători. Informație ascunsă: vezi doar mâna ta.",
            make: || Box::new(insula::Insula::new()),
        },
        Playable {
            id: "mozaic",
            title: "Mozaic (plăcuțe cu drumuri, cetăți, mănăstiri)",
            category: "Strategie",
            rules: "Joc original, 65 de plăcuțe: tragi o plăcuță și o lipești cu muchiile potrivite (F câmp, R drum, C cetate; pune(x,y)r1), apoi poți pune un pion pe un drum, o cetate sau o mănăstire (drum:N, oras:E, manastire, sau fara). Cetate închisă 2/plăcuță (+2 per scut), drum 1/plăcuță, mănăstire 9; majoritatea ia punctele, la final se numără și ce-i neterminat. Fără câmpuri/fermieri. Dacă plăcuța nu se potrivește: arunca.",
            make: || Box::new(mozaic::Mozaic::new()),
        },
        Playable {
            id: "litere",
            title: "Litere (cuvinte pe tablă)",
            category: "Cuvinte",
            rules: "Joc original pe tablă 15×15 cu 97 de plăcuțe (2 blank): formezi cuvinte legate de cele existente, cu pătrate premiate (# cuvânt ×3, + cuvânt ×2, x literă ×3, - literă ×2); +50 cu toate cele 7 plăcuțe. Dicționar mic românesc încorporat (~4000 de cuvinte uzuale, fără diacritice; cuvintele absente sunt respinse). Mutări: casa@h8> (orizontal), casa@h8v (vertical), ca(s)a@h8> (blank între paranteze), schimb (toată mâna), pas. Primul cuvânt acoperă h8. Informație ascunsă: vezi doar mâna ta.",
            make: || Box::new(litere::Litere::new()),
        },
        Playable {
            id: "faianta",
            title: "Faianța (variantă originală)",
            category: "Strategie",
            rules: "Draft de plăcuțe pe modele, 2 jucători: iei toate plăcuțele de o culoare dintr-o manufactură (restul trec în centru) sau din centru (primul ia jetonul de start, −1) și le pui pe o linie de model (1–5, o singură culoare, nu dacă peretele are deja culoarea) sau pe podea (−1…−3). La sfârșitul rundei (mutarea forțată decontare) liniile pline urcă pe perete 5×5 și punctează adiacența; jocul se oprește la un rând complet; bonus rând +2, coloană +7, culoare +10. Mutări: f3 T 2, c O p, decontare.",
            make: || Box::new(faianta::Faianta::new()),
        },
        Playable {
            id: "bresla",
            title: "Bresla (variantă originală)",
            category: "Strategie",
            rules: "Construire de pachet pentru doi: 7 Bani + 3 Parcele, mână de 5. Tură: acțiune (joc sat), cumparaturi (comorile se numără automat), cumpar <carte>, gata. 14 cărți: bani/argint/aur, parcelă/moșie/domeniu (1/3/6 p), acțiuni Sat, Meșteșug, Târg, Sărbătoare, Tăietor, Laborator, Atelier (iau <carte>), Cămătar (adversarul aruncă la 3 cărți: dau <carte>). Sfârșit: Domeniile epuizate sau 3 rezerve goale; câștigă cine are mai multe puncte. Informație ascunsă: ordinea pachetului și mâna adversarului.",
            make: || Box::new(bresla::Bresla::new()),
        },
        Playable {
            id: "cristale",
            title: "Cristale (variantă originală)",
            category: "Strategie",
            rules: "Jetoane de minerale și dezvoltări, 2 jucători: o acțiune pe tură — iau G A T (3 culori diferite), iau2 G (2 la fel, dacă sunt ≥4 în bancă), rezerv a1 (+ chihlimbar-joker), cumpar a1 / cumpar rez1. Cărțile cumpărate dau reduceri permanente și puncte; peste 10 jetoane arunci (arunc G). Patronii (3 p) vizitează automat. La 15 puncte se mai joacă până la capătul rundei. Cărțile sunt generate de noi.",
            make: || Box::new(cristale::Cristale::new()),
        },
        Playable {
            id: "curieri",
            title: "Curieri (variantă originală)",
            category: "Cărți",
            rules: "Deducție cu 16 cărți: tragi, joci una din două. 1 Iscoadă ghicește (iscoada:5), 2 Crainic se uită, 3 Judecător compară, 4 Străjer protejează, 5 Mag (mag:eu/mag:el) schimbă cartea, 6 Cămăraș schimbă mâinile, 7 Contesă (obligatorie cu 5/6), 8 Prințesă (aruncată = ieși). Ultimul rămas sau cartea cea mai mare câștigă runda; primul cu 3 runde câștigă. Informație ascunsă: vezi doar cartea ta.",
            make: || Box::new(curieri::Curieri::new()),
        },
        Playable {
            id: "fitil",
            title: "Fitil (variantă originală)",
            category: "Cărți",
            rules: "O Bombă în pachet: joci orice cărți, apoi tragi (trag). Fără Dezamorsare, bomba te scoate; cu ea o repui unde vrei (dezamorsez 0..N). Cărți: atac (adversarul trage de 2 ori), sari, amesteca, privire (primele 3), cere (adversarul îți dă o carte: dau X), pereche A|B|C (furi o carte la întâmplare). Informație ascunsă: vezi doar mâna ta, privirea ta și locul bombei puse de tine.",
            make: || Box::new(fitil::Fitil::new()),
        },
        Playable {
            id: "bufet",
            title: "Bufet (variantă originală)",
            category: "Cărți",
            rules: "Draft rapid de 10 cărți, 3 runde: alegi o carte (alege mici), mâinile se schimbă după revelare (revelare). Mici (perechi 5), salată (triplete 10), colțunași (1/3/6/10/15), cârnați 1-2-3 (muștarul îi triplează), pateuri (majoritate 6/3), papanași (la final cei mai mulți +6, cei mai puțini −6). Alegerea adversarului rămâne ascunsă până la revelare.",
            make: || Box::new(bufet::Bufet::new()),
        },
        Playable {
            id: "expeditii",
            title: "Expediții",
            category: "Cărți",
            rules: "Cinci masive (B F R D C), cărți 2–10 și 3 angajamente (x) pe culoare; mână de 8. Tură: joci o carte pe expediția ta (valori crescătoare) sau o arunci (juc B5, arunc B5), apoi tragi din pachet sau iei de pe o grămadă (trag, ia F). Scor pe expediție: (suma − 20) × (1 + angajamente), +20 cu ≥8 cărți. Se termină la ultima carte din pachet.",
            make: || Box::new(expeditii::Expeditii::new()),
        },
        Playable {
            id: "artificii",
            title: "Artificii (cooperativ, partener AI)",
            category: "Cooperativ",
            rules: "Tu și AI-ul construiți 5 șiruri de artificii (R G V A W) de la 1 la 5; vezi mâna partenerului, nu pe a ta. Tură: juc N, arunc N (+1 indiciu) sau indiciu R / indiciu 3 (cost 1 din 8 jetoane). 3 fitile arse = pierdeți. Obiectiv: 20 de puncte (echipa reușește = „Câștigă Jucătorul 1”; altfel remiză). Încheierea e o mutare forțată (sfârșit).",
            make: || Box::new(artificii::Artificii::new()),
        },
        Playable {
            id: "orasel",
            title: "Orășel (variantă originală)",
            category: "Familie",
            rules: "Zaruri și construit orașe: arunci 1 sau 2 zaruri (după Gară), clădirile albastre plătesc în orice tură, verzi doar în a ta, roșii (cafenea, grătar) iau de la cel care aruncă; cumperi o clădire (cumpar lan) sau un reper (construiesc gara/mall/parc/turn). Câștigă cine are primul cele 4 repere. Mutări: arunc 1, reiau, pastrez, nimic.",
            make: || Box::new(orasel::Orasel::new()),
        },
        Playable {
            id: "cursa_probelor",
            title: "Cursa probelor (joc de familie cu mini-probe, 2–4 jucători)",
            category: "Familie",
            rules: "Joc de familie propriu: pistă circulară de 30 de câmpuri și zar d6 (zar). Fiecare câmp declanșează o mini-probă: N numărare (raspuns 1|2|3), M memorie scurtă (gata, apoi literele A–D pe rând), D duel de zaruri (duel 2), R pariu de risc (arunc / opresc: până la 3 zaruri, un 1 pierde, țintă 9), L logică/aritmetică generată procedural (raspuns 1|2|3), O odihnă (a doua șansă). Începi cu jucatori 2|3|4; ceilalți sunt roboți (Easy–Expert; răspund doar din ce vezi și tu, cu zgomot). Primul cu 5 medalii câștigă; după 3 ture, cele mai multe medalii; egalitate la vârf = remiză. Mutări automate: rezolv, robot.",
            make: || Box::new(cursa_probelor::CursaProbelor::new()),
        },
        Playable {
            id: "capcana",
            title: "Capcana (variantă originală)",
            category: "Party",
            rules: "Bluf și licitație cu discuri ascunse: 3 flori + 1 capcană. Pui discuri (pun floare / pun capcana), apoi licitezi (licitez N) câte discuri poți întoarce fără capcană; celălalt ridică sau dă pas. Provocatorul întoarce întâi ale lui, apoi ale adversarului (intorc). 2 reușite câștigă; capcana te costă un disc.",
            make: || Box::new(capcana::Capcana::new()),
        },
        Playable {
            id: "congkak",
            title: "Congkak (Malaezia)",
            category: "Abstract",
            rules: "2 × 7 case, 7 semințe în fiecare, depozit pe jucător; semănat trigonometric, depozitul adversarului se sare. Ultima în depozit = încă o mutare; în casă ocupată = ridici și continui; în casă goală proprie = capturezi casa opusă plus sămânța ultimă (opusa goală: doar sămânța ultimă); în casă goală adversă = tura se termină. Runda se încheie când jucătorul la mutare nu mai are semințe; apoi casele se reumplu din depozite (7 fiecare), cele nerefăcute sunt arse. Cine nu mai poate umple nicio casă pierde. Mutări pe rând, nu simultan. Mutări: 1–7.",
            make: || Box::new(congkak::Congkak::new()),
        },
        Playable {
            id: "pallanguzhi",
            title: "Pallanguzhi (Tamil Nadu)",
            category: "Abstract",
            rules: "2 × 7 cupe, 5 semințe în fiecare. Ridici o cupă proprie și semeni trigonometric; apoi: cupa următoare ocupată = o ridici și continui; goală = capturezi cupa de după ea și tura se termină. Captură suplimentară: orice cupă care ajunge la exact 4 semințe e capturată. Fără semințe la mutare = jocul se termină, fiecare ia ce are pe partea lui; câștigă cine are peste 35. O singură rundă. Mutări: 1–7.",
            make: || Box::new(pallanguzhi::Pallanguzhi::new()),
        },
        Playable {
            id: "ayo",
            title: "Ayo (Ayoayo, Yoruba)",
            category: "Abstract",
            rules: "2 × 6 gropi, 4 semințe în fiecare; semănat trigonometric cu semănat în lanț (ultima într-o groapă ocupată = continui) și groapa de start sărită. Captură: ultima sămânță într-o groapă goală proprie ia groapa opusă (fără sămânța ultimă). Dacă adversarul nu mai are semințe, ești obligat să-l hrănești; dacă nu poți, jocul se termină. Mutări: 1–6.",
            make: || Box::new(ayo::Ayo::new()),
        },
        Playable {
            id: "oware",
            title: "Oware (Ghana)",
            category: "Abstract",
            rules: "2 × 6 gropi, 4 semințe în fiecare; semănat trigonometric (groapa de start e sărită la 12+ semințe), fără lanț. Captură: dacă ultima sămânță cade într-o groapă adversă ajunsă la 2 sau 3 semințe, le iei și continui înapoi cât timp grupele au 2 sau 3; mutarea care ar goli complet partea adversă nu capturează (grand slam). Dacă adversarul nu are semințe, trebuie să-l hrănești. Câștigă primul cu 25. Mutări: 1–6.",
            make: || Box::new(oware::Oware::new()),
        },
        Playable {
            id: "bao",
            title: "Bao (Bao la Kiswahili)",
            category: "Abstract",
            rules: "Tablă 4 × 8, două rânduri pe jucător, 22 de semințe în rezervă și nyumba cu 6. Namua: captura e obligatorie (pui o sămânță într-o groapă din față, iei groapa opusă și o semeni de la capătul rândului); altfel pui o sămânță și semeni sensul ales. Mtaji: muți dintr-o groapă cu ≥ 2 semințe; captură din prima aterizare obligatorie. Câștigi golind rândul din față al adversarului. Mutări: 3r, 3l (sens), b3l (rândul din spate).",
            make: || Box::new(bao::Bao::new()),
        },
        Playable {
            id: "seega",
            title: "Seega",
            category: "Abstract",
            rules: "Tablă 5 × 5, 12 piese pe jucător. Plasare: două piese pe tură, fără centru; apoi mută cine a plasat ultima, un pas ortogonal. Captură custodială a unui șir de piese adverse între piesa mutată și o piesă proprie; cine capturează mută din nou. Blocaj: adversarul scoate o piesă proprie. Mutări: b2 (plasare), b2-b3, scoate-b2.",
            make: || Box::new(seega::Seega::new()),
        },
        Playable {
            id: "yote",
            title: "Yote",
            category: "Abstract",
            rules: "Tablă 5 × 6, 12 piese pe jucător, inițial în mână. La rând: pui o piesă, muți un pas ortogonal sau sari peste o piesă adversă (o captezi și mai scoți încă una la alegere; săritura se poate lanțui). Câștigă cine rămâne singur cu piese. Mutări: c3, c3-c4, c3xc5+a1, c3xc5+m, opreste.",
            make: || Box::new(yote::Yote::new()),
        },
        Playable {
            id: "hus",
            title: "Hus (Namibia)",
            category: "Abstract",
            rules: "Mancala 4 × 8, 2 semințe în fiecare groapă. Muți dintr-o groapă proprie cu ≥ 2 semințe; ultima sămânță într-o groapă ocupată: din rândul din față cu groapa opusă ocupată captezi coloana adversă (față + spate) și o semeni mai departe; altfel continui lanțul. Pierde cine nu mai are nicio groapă cu ≥ 2. Mutări: 3 (față), b3 (spate).",
            make: || Box::new(hus::Hus::new()),
        },
        Playable {
            id: "tab",
            title: "Tâb (Egipt)",
            category: "Clasic",
            rules: "Cursă cu 4 bețe-zaruri pe tablă 4 × 7 (variantă simplificată): 7 piese pe jucător; intri cu 1; 1, 4 și 6 dau aruncare în plus; drum în zigzag pe cele 4 rânduri; aterizarea pe o piesă adversă (în afara rândului ei de plecare) o captură. Câștigă primul care scoate toate piesele. Mutări: arunca, intra-a1, a1-a3, a1-iese, pas.",
            make: || Box::new(tab::Tab::new()),
        },
        Playable {
            id: "senet",
            title: "Senet",
            category: "Clasic",
            rules: "Cursă egipteană pe 30 de pătrate (reconstituirea lui Kendall), 5 piese pe jucător, 4 bețe (1–4, zero = 5; 1, 4, 5 dau aruncare în plus). Schimbi locul cu o piesă adversă neprotejată; nu treci de 26; 27 (apa) te trimite pe 15; ieșire din 26 cu 5, din 28 cu 3, din 29 cu 2, din 30 cu 1. Mutări: arunca, 7-9, 28-iese, pas.",
            make: || Box::new(senet::Senet::new()),
        },
        Playable {
            id: "whot",
            title: "Whot (cărți nigeriene)",
            category: "Cărți",
            rules: "Pachet de 54 de cărți (forme o t x p s, plus 5 Whot), 6 în mână. Joci aceeași formă sau același număr; Whot cere o formă. 1 și 8: mai joci o dată; 14: adversarul trage o carte; 2: trage 2, 5: trage 3 (se pot înlănțui). Câștigi golindu-ți mâna. Informație ascunsă: vezi mâna Jucătorului 1. Mutări: o5, t14, w:s, trage, pas.",
            make: || Box::new(whot::Whot::new()),
        },
        Playable {
            id: "chkobba",
            title: "Chkobba (Tunisia)",
            category: "Cărți",
            rules: "Pachet de 40 de cărți (1–7, damă, valet, rege), 3 în mână, 4 pe masă. Iei o carte de aceeași valoare, altfel un grup cu aceeași sumă; golești masa = chkobba (nu la ultima carte). Punctaj: carte, denari, bermila (cei mai mulți de 7, apoi de 6), haya (7 de denari), chkobba; meci până la 11. Informație ascunsă: vezi mâna Jucătorului 1. Mutări: 7d, 7d:3c+4b, imparte, urmatoarea.",
            make: || Box::new(scopa::Scopa::chkobba()),
        },
        Playable {
            id: "ronda",
            title: "Ronda (Maroc / Spania)",
            category: "Cărți",
            rules: "Pachet spaniol de 40 de cărți, 4 pe masă, 3 în mână. Joci o carte: dacă pe masă e același rang, iei toate cărțile acelui rang și apoi, în șir, rangurile următoare prezente. Puncte: ronda (pereche în mână) 1, trio 5, missa (masă golită) 1, peste 20 de cărți 1; meci până la 12. Informație ascunsă: vezi mâna Jucătorului 1. Mutări: 5o, Rb, imparte, urmatoarea.",
            make: || Box::new(ronda::Ronda::new()),
        },
        Playable {
            id: "shologuti",
            title: "Sholo Guti (Șaisprezece soldați)",
            category: "Abstract",
            rules: "16 piese pe jucător pe o tablă de 37 de puncte (grilă 5 × 5 cu diagonale plus două triunghiuri), rândul din mijloc gol. Mută un pas pe un punct liber vecin sau sare peste o piesă adversă (captură obligatorie, lanț opțional). Câștigă cine capturează tot sau blochează adversarul. Mutări: c2-c3, c1xc3, opreste.",
            make: || Box::new(shologuti::SholoGuti::new()),
        },
        Playable {
            id: "aadupuli",
            title: "Aadu Puli Attam (Oi și tigri)",
            category: "Abstract",
            rules: "3 tigri (Jucătorul 2) contra a 15 oi (Jucătorul 1) pe o tablă de 23 de puncte în formă de triunghi. Oile se plasează pe rând, apoi se mută un pas; tigrii se mută sau sar peste o oaie (captură, lanț opțional). Tigrii câștigă cu 5 oi capturate; oile câștigă imobilizându-i. Mutări: b3, b3-b4, vxb2, opreste.",
            make: || Box::new(aadupuli::AaduPuli::new()),
        },
        Playable {
            id: "mangala",
            title: "Mangala (mancala turcească)",
            category: "Abstract",
            rules: "2 × 6 gropi, 4 semințe, câte un depozit. Prima sămânță rămâne în groapa de start. Ultima în depozit: mai muți; ultima într-o groapă adversă ajunsă la număr par: o iei; ultima într-o groapă proprie goală: iei groapa opusă. Câștigă depozitul mai mare. Mutări: 1–6.",
            make: || Box::new(mangala::Mangala::new()),
        },
        Playable {
            id: "dama",
            title: "Dama (dame turcești)",
            category: "Abstract",
            rules: "8 × 8, 16 piese pe jucător pe rândurile 2–3. Piesele merg înainte sau lateral, capturează ortogonal; dama zboară pe orice distanță. Captura e obligatorie, cu cele mai multe piese; piesele luate se scot la finalul secvenței. Mutări: a3-a4, a3xa5xc5.",
            make: || Box::new(dama::Dama::new()),
        },
        Playable {
            id: "okey",
            title: "Okey (2 jucători)",
            category: "Clasic",
            rules: "106 plăci (4 culori × 1–13 × 2 + 2 okey false), indicator și joker. 15/14 plăci; tragi din teanc (trage) sau iei ultima aruncată (ia), apoi arunci (ata-r5). Câștigi închizând 14 plăci în serii (≥ 3 consecutive, aceeași culoare) și grupe (3–4 cu același număr și culori diferite): bitir-r5. Informație ascunsă: vezi mâna Jucătorului 1.",
            make: || Box::new(okey::Okey::new()),
        },
        Playable {
            id: "battleship",
            title: "Bătălia navală (10×10)",
            category: "Clasic",
            rules: "Flotă de 5 nave (5, 4, 3, 3, 2). Îți așezi navele (a1h = orizontal, a1v = vertical, sau aleator); flota AI-ului se așază printr-o mutare forțată (plasez) și rămâne ascunsă. Tragi pe rând (e5): apă, lovit sau scufundat. AI-ul vânează cu hartă de densitate și țintește, fără să citească flota ta. Final cu mutarea forțată recunosc. Mutări: a1h, aleator, plasez, e5, recunosc.",
            make: || Box::new(battleship::Battleship::new()),
        },
        Playable {
            id: "mastermind",
            title: "Spargerea codului (cu schimb de roluri)",
            category: "Clasic",
            rules: "Cod de 4 cifre 1–6 (repetiții permise), cel mult 10 încercări; răspuns N = poziție exactă, A = culoare corectă pe altă poziție. Runda 1: spargi codul ascuns al AI-ului. Runda 2: îți alegi un cod (cod:1234) și AI-ul îl sparge. Câștigă cine a avut nevoie de mai puține încercări (nespart = 11). Mutări: 1234, cod:1234, confirm.",
            make: || Box::new(mastermind::Mastermind::new()),
        },
        Playable {
            id: "memory",
            title: "Perechi (joc de memorie)",
            category: "Clasic",
            rules: "24 de cărți (12 perechi A–L), 6 coloane × 4 rânduri. O tură = două întoarceri (mutări separate): pereche = punct și mută din nou, altfel cărțile se întorc și mută adversarul. AI-ul are memorie limitată (memorează 70% din cărțile văzute, uită cu timpul). Mutări: a1–f4.",
            make: || Box::new(memory::Memory::new()),
        },
        Playable {
            id: "bingo",
            title: "Bingo (75 de bile)",
            category: "Clasic",
            rules: "Fișă 5 × 5 (B 1–15, I 16–30, N 31–45 cu centru liber, G 46–60, O 61–75). Tragerea e o mutare (trag), marcarea o alta (marchez); cu o combinație (rând, coloană, diagonală) strigi sau continui. Cine strigă singur câștigă; dacă strigă amândoi = remiză. Fișa AI-ului e ascunsă. Mutări: trag, marchez, strig, continui.",
            make: || Box::new(bingo::CallGame::bingo()),
        },
        Playable {
            id: "loteria",
            title: "Lotería (imagini)",
            category: "Clasic",
            rules: "Tablă 4 × 4 cu 16 din 54 de imagini (set propriu, nu cel comercial). Tragi o imagine (trag), o marchezi dacă o ai (marchez); combinații: rând, coloană, diagonală, cele patru colțuri sau pătratul din centru; strigi sau continui. Fișa AI-ului e ascunsă. Mutări: trag, marchez, strig, continui.",
            make: || Box::new(bingo::CallGame::loteria()),
        },
        Playable {
            id: "pachisi",
            title: "Pachisi (cursă cu cauri)",
            category: "Zaruri",
            rules: "6 cauri: niciunul deschis = 25, 1 deschis = 10, 2 = 2, 3 = 3, 4 = 4, 5 = 5, 6 = 6; 6, 10 și 25 sunt grațioase (încă o aruncare și poți scoate o piesă din bază). Pistă de 48 de pătrate, castele sigure din 6 în 6, două piese proprii = bloc, aterizarea pe o piesă singură o captură (+ aruncare). Ținta exactă de pe pistă; din culoarul de acasă orice aruncare ≥ distanța. Mutări: intră, sau progresul piesei (ex. 12).",
            make: || Box::new(pachisi::Pachisi::new()),
        },
        Playable {
            id: "carrom",
            title: "Carrom (discret, fără fizică)",
            category: "Abstract",
            rules: "Tablă 9 × 9, 12 monede pe jucător, damă, patru buzunare în colțuri. Lovitura = coloană + direcție (L/S/R) + forță 1–3, traseu deterministic cu ricoșeu; monedă proprie băgată = +1 și mai tragi; moneda adversarului = punct pentru el; dama se acoperă la lovitura următoare (+3) sau revine; strikerul în buzunar = greșeală. Mutări: eS3, aL1.",
            make: || Box::new(carrom::Carrom::new()),
        },
        Playable {
            id: "arhipelag",
            title: "Arhipelagul în Flăcări (cucerire cu zaruri)",
            category: "Strategie",
            rules: "Variantă originală: 14 insule în 4 regiuni cu bonus, 7 insule și 3 armate fiecare la start. Tura: întărire (pune Fiord / pune Fiord*), atac (ataca Fiord>Prag, apoi mutarea forțată arunc: 3 zaruri contra 2, egalitatea o câștigă apărătorul; gata atac), o mutare de armate (muta A>B n) sau gata. Câștigă cine rămâne singur; după 70 de ture, cine are mai multe insule.",
            make: || Box::new(arhipelag::Arhipelag::new()),
        },
        Playable {
            id: "roata",
            title: "Roata Destinului (parcurs de viață)",
            category: "Familie",
            rules: "Variantă originală: roata (1–6) e afișată; mutarea avansez alege ramura la bifurcații (facultate/ucenicie, afacere/salariu) și, pe pătratul Casă, cumpar/nu. Plată la fiecare trecere, copii și casă valorează la final. Scor = bani + 30 casă + 8 per copil. Mutări: avansez, avansez facultate, avansez ucenicie, avansez afacere, avansez salariu, avansez cumpar.",
            make: || Box::new(roata::Roata::new()),
        },
        Playable {
            id: "zaruri5",
            title: "Cinci Zaruri (13 categorii)",
            category: "Zaruri",
            rules: "Variantă originală: 13 runde, 5 zaruri, până la 3 aruncări. Aruncarea e mutare forțată (arunc); între aruncări ții zaruri (tine 3 3 5, tine -) sau scrii o categorie (scor casa). Bonus 35 la 63 sus; fără regula joker. Câștigă totalul mai mare.",
            make: || Box::new(zaruri5::Zaruri5::new()),
        },
        Playable {
            id: "placute",
            title: "Plăcuțe (seturi și serii)",
            category: "Cărți",
            rules: "Variantă originală: 106 plăci (4 culori ×1–13 ×2 + 2 jokeri), 14 în mână. Pune serii/grupuri (pune r3 r4 r5), adaugă la masă (adauga r6 la 2), schimbă un joker (schimb J din 2 cu r4); prima ieșire ≥ 30. Terminare: gata; altfel trag / pas; anuleaza. Fără reorganizarea liberă a mesei. Informație ascunsă: vezi doar mâna ta.",
            make: || Box::new(placute::Placute::new()),
        },
        Playable {
            id: "turnul",
            title: "Turnul Șovăitor (turn de blocuri)",
            category: "Familie",
            rules: "Simulare discretă, deterministă: 12 straturi de 3 blocuri; iei un bloc de sub stratul complet de sus și îl pui deasupra. Turnul stă dacă centrul de masă de deasupra fiecărui strat cade în învelișul blocurilor lui rămase; cine îl prăbușește pierde. Mutări: ia 4b pune a.",
            make: || Box::new(turnul::Turnul::new()),
        },
        Playable {
            id: "cunoastere",
            title: "Cupa Cunoașterii (întrebări, 6 felii)",
            category: "Party",
            rules: "Banca proprie de 126 de întrebări în 6 categorii; alege o categorie (alege istorie), răspunde a–d (raspund b), apoi verific. Corect = felia și încă o întrebare; greșit = trece tura. Prima 6/6 câștigă. AI-ul nu are cunoștințe (ghicește); pentru meci echilibrat jucați în doi oameni.",
            make: || Box::new(cunoastere::Cunoastere::new()),
        },
        Playable {
            id: "ceata",
            title: "Câmpul Ceții (piese ascunse, 10×10)",
            category: "Strategie",
            rules: "Variantă originală: 40 de piese pe parte, ranguri 1–X, mine (B), steag (F), lacuri. Vezi doar piesele tale și pe ale adversarului după bătălie. Mutări: e4-e5, e4xe5 (atac), apoi mutarea forțată bataie. Săpătorul dezamorsează minele, Iscoada bate Mareșalul. 100 de mutări fără bătălie = remiză.",
            make: || Box::new(ceata::Ceata::new()),
        },
        Playable {
            id: "umbra",
            title: "Umbra Urbei (urmărire cu bilete)",
            category: "Familie",
            rules: "Variantă originală, J1 = 4 detectivi, J2 = Umbra (ascunsă; apare după mutările 4, 9, 14, 19). Hartă de 30 de stații pe jos/tram/metro cu bilete limitate; Umbra are 3 bilete negre. Mutări: d2 17 tram, x 12 jos. Detectivii câștigă prinzând sau blocând Umbra; Umbra, rezistând 22 de mutări. Se afișează stațiile posibile.",
            make: || Box::new(umbra::Umbra::new()),
        },
        Playable {
            id: "zidurile",
            title: "Zidurile Mișcătoare (labirint cu plăci culisante)",
            category: "Familie",
            rules: "Variantă originală: tablă 7×7 cu plăci mobile; introduci placa de rezervă (mete N2 r1) apoi muți pionul pe un drum deschis (mergi c3) ca să iei comorile în ordine, apoi acasă. Nu poți anula inserția adversarului. După 100 de ture, cine are mai multe comori.",
            make: || Box::new(zidurile::Zidurile::new()),
        },
        Playable {
            id: "prajituri",
            title: "Drumul Prăjiturilor (cursă de culori)",
            category: "Familie",
            rules: "Variantă originală: 58 de pătrate colorate, pachet de cărți (simple, duble, locuri), scurtături și pătrate lipicioase. Cartea e afișată; singura mutare este trag. Primul în Palatul de zahăr câștigă.",
            make: || Box::new(prajituri::Prajituri::new()),
        },
        Playable {
            id: "portret",
            title: "Portretul Misterios (ghicește personajul)",
            category: "Familie",
            rules: "Variantă originală: 24 de personaje inventate cu 14 trăsături. Întrebi (intreb ochelari), adversarul răspunde corect prin mutare forțată (raspund da/nu); ghicești (ghicesc Matei), iar adversarul dă verdictul printr-o mutare forțată (verdict corect / verdict gresit). Ghicit greșit = pierzi.",
            make: || Box::new(portret::Portret::new()),
        },
        Playable {
            id: "briscola",
            title: "Briscola (Italia)",
            category: "Cărți",
            rules: "Pachet italian de 40 de cărți (culori d, c, s, b; rang 2 4 5 6 7 F C R 3 A), 3 în mână, atuul întors rămâne jos. Nu e obligatoriu să răspunzi la culoare; levata o ia cartea mai mare de aceeași culoare sau un atu; câștigătorul trage primul. Puncte: A 11, 3 10, R 4, C 3, F 2; peste 60 câștigi, 60–60 remiză. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: Ad, 3s, Rb, 7c.",
            make: || Box::new(briscola::Briscola::new()),
        },
        Playable {
            id: "scopa",
            title: "Scopa (Italia)",
            category: "Cărți",
            rules: "Pachet italian de 40 de cărți (d c s b; rang 1–7 F C R, valori 1–10), 3 în mână, 4 pe masă. Joci o carte: dacă pe masă e aceeași valoare, iei acea carte; altfel iei un grup cu aceeași sumă; altfel o lași. Golești masa = scopa (1 punct, nu la ultima carte). Punctaj: carte, denari, settebello (7d), primiera, scope; meci până la 11. Informație ascunsă: vezi doar mâna Jucătorului 1; împărțirea următoare e mutare forțată. Mutări: 7d, 7d:3c+4b, 7d:7s, imparte, urmatoarea.",
            make: || Box::new(scopa::Scopa::new()),
        },
        Playable {
            id: "durak",
            title: "Durak (Rusia)",
            category: "Cărți",
            rules: "36 de cărți (6..A, culori c d h s), 6 în mână, atu întors jos. Atacatorul pune o carte; apărătorul o bate (mai mare de aceeași culoare sau atu) ori ia. Atacatorul adaugă cărți cu valori deja pe masă (cel mult 6, cel mult câte are apărătorul; 5 la început); gata = rundă încheiată, apărătorul devine atacator. După fiecare rundă se trage până la 6 (mutare forțată umple). Fără pachet, cine rămâne primul fără cărți câștigă, celălalt e durak; ambii = remiză. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 6h, 9h>6h (bate 6h cu 9h), iau, gata, umple.",
            make: || Box::new(durak::Durak::new()),
        },
        Playable {
            id: "belote",
            title: "Belote (Franța, 2 jucători)",
            category: "Cărți",
            rules: "32 de cărți (7..A; c d h s), 5 în mână + o carte întoarsă. Licitație în două tururi: prends (atu = culoarea cărții întoarse) / pas, apoi prends-c|d|h|s; toți pas = redistribuire. Cine ia primește cartea și se împarte până la 8 (imparte); 16 cărți rămân nejucate. Obligatoriu culoarea, altfel tai cu atu, la atu urci. Atu: J 20, 9 14, A 11, 10 10, K 4, Q 3; altfel A 11, 10 10, K 4, Q 3, J 2. Belote 20, dix de der 10, capot 90; contractul cade dacă ai mai puțin decât adversarul. Meci până la 301. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: prends, pas, prends-h, imparte, Jh, 10s, urmatoarea.",
            make: || Box::new(belote::Belote::new()),
        },
        Playable {
            id: "skat",
            title: "Skat (2 jucători)",
            category: "Cărți",
            rules: "32 de cărți (7..A; c ♣, s ♠, h ♥, d ♦), 10 în mână, 2 în skat, 10 ascunse nejucate. Licitația: cine a împărțit spune 18 20 22 23 24 27 … (licit N), celălalt ține (tin) sau pas; declarantul ia skat-ul (skat) și pune 2 cărți (pune 10h Jc) sau joacă din mână (hand), apoi alege jocul: trefla, pica, cupa, caro (atu = juniorii + culoarea), grand (doar juniorii), null (declarantul nu ia nicio levată). Valoare = bază × (matadori + 1 + mână + schneider + schwarz); câștig +valoarea, pierdere −2×, supralicitare = pierdere. 61 din 120 de puncte. Meci de 6 mâini. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: licit 18, tin, pas, skat, trefla, grand, Jh, reimparte, urmatoarea.",
            make: || Box::new(skat::Skat::new()),
        },
        Playable {
            id: "tarot",
            title: "Tarot francez (78 de cărți, 2 jucători)",
            category: "Cărți",
            rules: "78 de cărți (c d h s: 1–10 V C D R; atuuri T1–T21; Scuza Ex), 36 în mână și 6 în chien. Licitație într-un singur tur: pas, petite, garde, garde-sans, garde-contre (multiplicatori 1/2/4/6); cine ia primește chien-ul (chien) și pune jos 6 cărți (ecart Vh). Culoare obligatorie, altfel taie cu atu, la atu urci; Scuza nu câștigă. 91 de puncte; cerut 56/51/41/36 cu 0/1/2/3 oudlers; scor (25 + diferența + 10 petit au bout) × multiplicator, zero-sum; meci de 3 mâini. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: petite, pas, chien, ecart 2h, T21, Rs, Ex, reimparte, urmatoarea.",
            make: || Box::new(tarot::Tarot::new()),
        },
        Playable {
            id: "cribbage",
            title: "Cribbage",
            category: "Cărți",
            rules: "2 jucători, 6 cărți: dai 2 în crib (al dealerului), se taie startul (valet = 2 pentru dealer), pegging până la 31 (15, 31, perechi, serii, go) și numărarea mâinilor și a cribului (15-uri, perechi, serii, culoare, valet). Tabla de colac de 121 de găuri e desenată în text. Primul la 121 câștigă. Mutări: d 5h 6s, taie, 5h, go, numara, imparte.",
            make: || Box::new(cribbage::Cribbage::new()),
        },
        Playable {
            id: "euchre",
            title: "Euchre (2 jucători)",
            category: "Cărți",
            rules: "24 de cărți (9–A), 5 fiecare, o carte întoarsă. Runda 1: pas / up (dealerul ia cartea și dă una); runda 2: numești altă culoare (dealerul e obligat). Valetul de atu (bower dreapta) și cel de aceeași culoare (stânga, tot atu) sunt cele mai mari. Maker: 3–4 levate = 1 punct, 5 = 2, euchre = 2 pentru adversar; până la 10. Mutări: pas, up, nume h, dis 9h, Jh, imparte.",
            make: || Box::new(euchre::Euchre::new()),
        },
        Playable {
            id: "inimi",
            title: "Inimi (evitarea punctelor, 2 jucători)",
            category: "Cărți",
            rules: "13 cărți fiecăruia din 52 (restul nevăzut). Răspunzi obligatoriu la culoare, ia cartea cea mai mare din culoarea condusă. Fiecare inimă = 1 punct, Dama de pică = 13; nu conduci inimă până nu s-a jucat una. La 40 de puncte pierde cine are mai multe. Informație ascunsă: vezi doar mâna ta. Mutări: imparte, 7c, Qs, 10h.",
            make: || Box::new(inimi::Inimi::new()),
        },
        Playable {
            id: "pica",
            title: "Pică (licitație și atu pică, 2 jucători)",
            category: "Cărți",
            rules: "13 cărți fiecăruia din 52; atu = pica. Licitezi câte levate iei (licit 1…13) sau licit 0 = nil. Reușit: 10×licitație + 1 pe levată în plus (bag; 10 bag-uri = −100), picat: −10×licitație; nil ±50. Nu conduci pică până nu s-a jucat una. Până la 150 sau 12 mâini. Informație ascunsă: vezi doar mâna ta. Mutări: imparte, licit 4, As, 7h.",
            make: || Box::new(pica::Pica::new()),
        },
        Playable {
            id: "prinde",
            title: "Prinde (licitație exactă, 2 jucători)",
            category: "Cărți",
            rules: "13 runde cu mâini de 1…7…1 cărți; o carte întoarsă dă atuul. Spui câte levate iei (licit N); dealerul nu poate aduce suma licitațiilor la numărul de levate. Exact = 10 + licitația, altfel 0. Culoare obligatorie, atu bate tot. Informație ascunsă: vezi doar mâna ta. Mutări: imparte, licit 2, Kd, 4s.",
            make: || Box::new(prinde::Prinde::new()),
        },
        Playable {
            id: "whist",
            title: "Whist (2 jucători)",
            category: "Cărți",
            rules: "13 cărți fiecăruia din 52; ultima carte a dealerului se arată și dă atuul. Culoare obligatorie, atu bate tot. Cine ia peste 6 levate primește câte un punct pentru fiecare levată peste a șasea; jocul se încheie la 5 puncte (fără onoruri). Informație ascunsă: vezi doar mâna ta. Mutări: imparte, Ah, 9c.",
            make: || Box::new(whist::Whist::new()),
        },
        Playable {
            id: "tarneeb",
            title: "Tarneeb (2 jucători)",
            category: "Cărți",
            rules: "13 cărți fiecare din 52 (celelalte 26 ascunse). Licitezi levate 7–13 (urci cu 1 sau 2), câștigătorul numește atuul și conduce; făcut = punctele pe levatele luate (16 la 13), pierdut = minus cât ai licitat; adversarul ia levatele lui (+3 dacă ai căzut); până la 31. Mutări: pas, 7…13, atu h, Ah, imparte.",
            make: || Box::new(tarneeb::Tarneeb::new()),
        },
        Playable {
            id: "baloot",
            title: "Baloot (2 jucători)",
            category: "Cărți",
            rules: "32 de cărți, 8 fiecare, o carte întoarsă. Licitație în două runde: hokm (atu), sun (fără atu) sau pas. Atu: J 20, 9 14, A 11, 10 10, K 4, Q 3; ultima levată +10; la hokm tai obligatoriu cu atu și urci peste atu. Cumpărătorul trebuie să aibă mai multe puncte decât adversarul; baloot (K+Q de atu) = +2; până la 50. Mutări: pas, hokm, hokm h, sun, Jh, imparte.",
            make: || Box::new(baloot::Baloot::new()),
        },
        Playable {
            id: "tablanet",
            title: "Tablanet",
            category: "Cărți",
            rules: "52 de cărți: 4 pe masă, câte 6 fiecare, de patru ori. Capturezi cărți egale sau grupuri care se adună la valoarea ta (A = 1 sau 11; J, Q, K doar egale); captura e obligatorie și maximă. Punctaj: cele mai multe cărți 3, cele mai multe treflă 1, 10♦ 2, 2♣ 1, fiecare As 1, fiecare tablă 1. Mutări: 7h, 7h x 3c 4d, imparte.",
            make: || Box::new(tablanet::Tablanet::new()),
        },
        Playable {
            id: "truco",
            title: "Truco (Argentina)",
            category: "Cărți",
            rules: "Pachet spaniol de 40, 3 cărți fiecare, ierarhie specială (1 de spadă cea mai mare), 3 runde, până la 15. Cânți truco / retruco / vale4 (cel provocat: quiero, noquiero sau urcă) și envido / real / falta în prima rundă; mazo = te retragi. Fără flor. Mutări: 7e, truco, envido, quiero, noquiero, cuenta, mazo, reparte.",
            make: || Box::new(truco::Truco::new()),
        },
        Playable {
            id: "trix",
            title: "Trix (2 jucători)",
            category: "Cărți",
            rules: "32 de cărți, 16 fiecare, 10 mâini (5 „împărăția” fiecăruia). Proprietarul alege contractul: king (K♥ −75), queens (−25 fiecare), diamonds (−10 fiecare), slaps (−15 pe levată) sau trex (aranjament pornit din J, primul care termină +200). Mutări: king, queens, diamonds, slaps, trex, Jh, pas, imparte.",
            make: || Box::new(trix::Trix::new()),
        },
        Playable {
            id: "bridge",
            title: "Bridge duplicat (2 jucători)",
            category: "Cărți",
            rules: "Două tabele cu aceeași donă: J1 conduce N–S la tabla 1 și E–V la tabla 2 (J2 invers); fiecare vede ambele mâini ale cuplului lui. Licitație (1c…7n, x, xx, pass), declarant Sud / Vest, mâna moartă vizibilă după prima carte; scor duplicat (nimeni vulnerabil). Câștigă scorul combinat mai bun. Mutări: 1h, 3n, x, xx, pass, Ah.",
            make: || Box::new(bridge::Bridge::new()),
        },
        Playable {
            id: "canasta",
            title: "Canasta (2 jucători, o mână)",
            category: "Cărți",
            rules: "108 de cărți, 15 fiecare; 2 și jokerii sunt wild, 3 roșii = bonus, 3 negre doar se aruncă. Tura: roba (2 cărți) sau ia aruncarea, combinații de 3+ cărți de același rang (cu cel puțin 2 naturale), arunci o carte. Canastă (7 cărți) = 500 naturală / 300 mixtă; ieși doar cu canastă. O mână, fără minim de coborâre; câștigă scorul mai mare. Mutări: roba, ia, m 7, mw 7, w K, d 9c.",
            make: || Box::new(canasta::Canasta::new()),
        },
        Playable {
            id: "nave",
            title: "Escadrile din Rift (manevre secrete)",
            category: "Miniaturi",
            rules: "Manevre planificate simultan: 3 nave pe parte (Bivol, Șoim, Vrabie) pe o hartă 12×12 cu asteroizi. Fiecare navă primește în secret o manevră (cadran): drept dN, deviere blN/brN, viraj tlN/trN, întoarcere u, oprire s; roșii (viraj 3, u, s) dau stres, iar o navă stresată nu poate alege roșu. Apoi revelarea mută forțat, în ordinea îndemânării pilotului; ciocnire = oprire și fără foc; asteroid = 1 daună. Foc în arcul din față (Bivolul: turelă) la distanță 1–3, zaruri de atac contra zaruri de apărare, scut apoi coca. 10 runde. Informație ascunsă (AI-ul nu vede planurile tale). Mutări: A d2, A bl1, A tr2, A u, A s, revela, A>X, A pas, arunca.",
            make: || Box::new(nave::Nave::new()),
        },
        Playable {
            id: "patrula",
            title: "Patrula (skirmish cu echipă mică)",
            category: "Miniaturi",
            rules: "5 miniaturi individuale pe parte (Lider, 2 Pușcași, Greu, Asalt) pe o hartă 10×10 cu ziduri, acoperiri și 4 obiective; fiecare are răni proprii, armă de foc și armă de luptă. Părțile alternează activări (fiecare miniatură o dată pe rundă) cu ordine: Avans (mută și poate trage), Țintă (pe loc, +1 la lovire; lipit = luptă), Asalt, Retragere (doar lipit), Gardă. 6 runde; obiectivele dau puncte la finalul rundei. Mutări: A>d4, A>d4 xV, A xV, A!d4 xV, A<d4, A garda, arunca.",
            make: || Box::new(patrula::Patrula::new()),
        },
        Playable {
            id: "reactii",
            title: "Ordine și reacții (skirmish cu reacții)",
            category: "Miniaturi",
            rules: "4 soldați pe parte (Lider, Pușcaș, Lunetist, Asalt), hartă 10×10, zaruri d20. Partea activă are un ordin per soldat (+1 pentru Lider); fiecare ordin mută și/sau trage. În tura adversarului, inamicii care văd soldatul activ reacționează pe rând: pas, tira (reacție) sau esiva (doar ținta declarată); față în față câștigă cel mai mare rezultat reușit, egalul la critic. 8 runde; obiective. Mutări: A>d4 xV, A xV, A>d4, fin, V pas, V tira, V esiva, arunca.",
            make: || Box::new(reactii::Reactii::new()),
        },
        Playable {
            id: "ordine",
            title: "Ordine de bătaie (infanterie, zaruri de ordine)",
            category: "Miniaturi",
            rules: "Bătălie de infanterie pe hartă 12×8: Ofițer, 2 Grupe, Mitralieră și Mortier pe parte. Un zar de ordine pe unitate într-un săculeț; mutarea forțată trage extrage un zar la întâmplare și partea respectivă dă un ordin: fire, adv, run (asalt), amb, rally, down. Suprimare: marcajele de suprimare scad moralul și trebuie testate pentru a executa ordinul. 6 ture; obiective. Mutări: trage, B fire xW, B adv d4 xW, B run d4, B amb, B rally, B down, arunca.",
            make: || Box::new(ordine::Ordine::new()),
        },
        Playable {
            id: "gridiron",
            title: "Gridiron (sport fantastic pe ture)",
            category: "Miniaturi",
            rules: "Echipe de 5 pe un teren 12×7; 6 ture pe repriză, 2 reprize. O acțiune per jucător: mutare (escapare, ridicare minge, mutări în plus cu zar), tăvăleală cu zaruri de forță, asalt (mutare + tăvăleală), pasă, predare. O cădere sau o minge scăpată pierde tura. Purtătorul mingii în zona adversă = punct. Mutări: A>d4, A>d4 xV, A xV, A pass B, A hand B, A sus, fin, lovitura, arunca.",
            make: || Box::new(gridiron::Gridiron::new()),
        },
        Playable {
            id: "focare",
            title: "Focare (cooperativ, boli pe hartă, partener AI)",
            category: "Cooperativ",
            rules: "Variantă originală: voi doi, agronomi, opriți 4 boli ale culturilor pe o hartă cu 24 de așezări. Tura: 4 acțiuni (mers X, zbor X, charter X, naveta X, statie, tratez r, dau/iau, vindec r, pas), apoi tragi 2 cărți (trage; 3 epidemii în pachet; max 7 cărți: descarta X) și se infectează (infecteaza). Al 4-lea spor dintr-o boală produce focar în lanț. Vindecare: la stație, 3 cărți de aceeași culoare. Victorie (toate 4 vindecate) = „Câștigă Jucătorul 1”; înfrângere (6 focare, spori epuizați, pachet golit) = remiză. Încheierea e mutare forțată (sfârșit). Informație: mâinile sunt publice.",
            make: || Box::new(focare::Focare::new()),
        },
        Playable {
            id: "strajile",
            title: "Străjile Codrului (cooperativ de apărare, cărți de putere)",
            category: "Cooperativ",
            rules: "Variantă originală: voi, două duhuri (J1 Rădăcinilor, J2 Furtunii, AI), apărați o vale cu 8 ținuturi de invadatori. Tura: creștere (creste N / aduna / reia), apoi până la 2 cărți de putere (joc spini 3; plătești energia, ținut în raza cărții) și gata. Apoi runda invadatorilor, mutări forțate: devastare, constructie, explorare. Colonie ucisă = frică. Victorie: frică 28 sau valea curată după ultima carte de invadatori (= „Câștigă Jucătorul 1”); 10 blesteme sau invadatori rămași = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(strajile::Strajile::new()),
        },
        Playable {
            id: "farul",
            title: "Farul Fantomei (deducție cooperativă cu indicii vizuale)",
            category: "Cooperativ",
            rules: "Variantă originală: tu ești mediumul (J1), AI-ul este fantoma (J2) care îți trimite „viziuni” de câte 3 simboluri (viziune N). Patru etape: portretul, camera, obiectul (6 candidați cu câte 5 simboluri, ghic N sau asteapt; max 10 viziuni) și scena finală (3 candidați, o singură ghicire). Ghicirea se verifică într-o mutare forțată (verific). Victorie = „Câștigă Jucătorul 1”; eșec = remiză. Informație ascunsă: nu vezi soluția, nici mâna fantomei.",
            make: || Box::new(farul::Farul::new()),
        },
        Playable {
            id: "turnul_zorilor",
            title: "Turnul Zorilor Frânte (aventură cooperativă, fantasy eroic)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 + modificator ≥ prag. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("turnul_zorilor")),
        },
        Playable {
            id: "arhiva_abisala",
            title: "Arhiva Abisală (aventură cooperativă, groază cosmică de investigator)",
            category: "Cooperativ",
            rules: "Mecanica de test: d100 sub abilitate. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("arhiva_abisala")),
        },
        Playable {
            id: "ruinele_cartografilor",
            title: "Ruinele Cartografilor (aventură cooperativă, expediție în junglă)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 cu grade de succes. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("ruinele_cartografilor")),
        },
        Playable {
            id: "noapte_de_catifea",
            title: "Noapte de Catifea (aventură cooperativă, urban fantasy cu vampiri)",
            category: "Cooperativ",
            rules: "Mecanica de test: grămadă de d10. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("noapte_de_catifea")),
        },
        Playable {
            id: "padurea_sticlei",
            title: "Pădurea Ochilor de Sticlă (aventură cooperativă, fantasy de basm sumbru)",
            category: "Cooperativ",
            rules: "Mecanica de test: 3d20 față de trei atribute. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("padurea_sticlei")),
        },
        Playable {
            id: "neon_si_rune",
            title: "Neon și Rune (aventură cooperativă, cyber-urban cu magie)",
            category: "Cooperativ",
            rules: "Mecanica de test: grămadă de d6 cu lovituri. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("neon_si_rune")),
        },
        Playable {
            id: "targul_ciumat",
            title: "Târgul Ciumat (aventură cooperativă, fantasy sumbru)",
            category: "Cooperativ",
            rules: "Mecanica de test: d100 cu grade de succes. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("targul_ciumat")),
        },
        Playable {
            id: "insula_celor_doua_luni",
            title: "Insula celor Două Luni (aventură cooperativă, fantasy clasic)",
            category: "Cooperativ",
            rules: "Mecanica de test: 2d6 + rating față de țintă. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("insula_celor_doua_luni")),
        },
        Playable {
            id: "hotii_din_ceata",
            title: "Hoții din Ceață (aventură cooperativă, jaf într-un oraș cu ceață)",
            category: "Cooperativ",
            rules: "Mecanica de test: cel mai mare d6. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("hotii_din_ceata")),
        },
        Playable {
            id: "crucea_de_cenusa",
            title: "Crucea de Cenușă (aventură cooperativă, apocalipsă sumbră)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 față de prag fix. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("crucea_de_cenusa")),
        },
        Playable {
            id: "strada_de_otel",
            title: "Strada de Oțel (aventură cooperativă, cyberpunk de cartier)",
            category: "Cooperativ",
            rules: "Mecanica de test: d10 exploziv. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("strada_de_otel")),
        },
        Playable {
            id: "tinutul_furtunilor",
            title: "Ținutul Furtunilor Vii (aventură cooperativă, fantasy colorat)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 + modificator ≥ prag. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("tinutul_furtunilor")),
        },
        Playable {
            id: "epava_din_inelul_sapte",
            title: "Epava din Inelul Șapte (aventură cooperativă, operă spațială)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 + modificator ≥ prag. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("epava_din_inelul_sapte")),
        },
        Playable {
            id: "fiordul_gheatii_negre",
            title: "Fiordul Gheții Negre (aventură cooperativă, fantasy nordic)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 sub abilitate. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("fiordul_gheatii_negre")),
        },
        Playable {
            id: "mlastina_strigoilor",
            title: "Mlaștina Strigoilor (aventură cooperativă, vânătoare de monștri)",
            category: "Cooperativ",
            rules: "Mecanica de test: d10 exploziv. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("mlastina_strigoilor")),
        },
        Playable {
            id: "satul_lanternelor",
            title: "Satul Lanternelor Stinse (aventură cooperativă, groază de sat cu spirite)",
            category: "Cooperativ",
            rules: "Mecanica de test: d100 sub abilitate. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("satul_lanternelor")),
        },
        Playable {
            id: "cenusii",
            title: "Noapte Cenușie (cooperativ de supraviețuire pe grilă)",
            category: "Cooperativ",
            rules: "Variantă originală: Ilinca (J1) și Radu (AI) strâng 3 baterii (B) și ajung amândoi la ieșire (E) pe o grilă 7×6, ținând piept umbrelor. 3 acțiuni pe tură: mers n|s|e|v, atac 1 c3 (zaruri într-o mutare forțată: zaruri), cauta (extrage), ia, pansez, gata; apoi umbre și aparitie (forțate). Rătăciți, alergători și colosi (necesită armă cu forță 2). 4 răni = mort. Victorie = „Câștigă Jucătorul 1”; moartea, roi sau 40 de ture = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(cenusii::Cenusii::new()),
        },
        Playable {
            id: "cripta",
            title: "Cripta Clopotelor (aventură tactică cu cărți de acțiune)",
            category: "Cooperativ",
            rules: "Variantă originală: Paznicul de Far (J1) și Vânătoarea de Umbre (AI) coboară într-o criptă; 3 scenarii (omorâți toți monștrii / ajungeți la ieșire / omorâți Regele Clopot). Rundă: fiecare alege 2 cărți (joc 3 5) sau odihna; revela; apoi în ordinea inițiativei jucați sus 3 jos 5: mers c4, atac m2 / atac toti, aplic, sari; atacul trage modificatorul într-o mutare forțată (trage). Monștrii (monstri) trag cărți de comportament. Victorie = „Câștigă Jucătorul 1”; ambii epuizați sau 25 de runde = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(cripta::Cripta::new()),
        },
        Playable {
            id: "lupii",
            title: "Lupii din Hârtop (roluri ascunse, masa = roboți)",
            category: "Party",
            rules: "Tu ești locul 1 la o masă de 8–9; ceilalți sunt roboți care văd doar informația publică. Noaptea acționează lupii și rolurile speciale (văzător, paznic/doctor, vrăjitoare, vânător); ziua toți votează cine pleacă, rolul celui eliminat se arată. Satul câștigă fără lupi; lupii când sunt cât restul. Alegi presetarea la început: preset mafie (clasic), preset sat (stil francez), preset stepa (stil chinezesc). Mutări: preset sat, ucide 3, cerceteaza 3, pazeste 3, salveaza, otraveste 3, dormi, vot 3, impusca 3, masa.",
            make: || Box::new(lupii::Lupii::new()),
        },
        Playable {
            id: "santinela",
            title: "Străjerii Farului (misiuni cu echipe, roluri ascunse)",
            category: "Party",
            rules: "Cinci locuri (tu + 4 roboți): trei paznici (unul e Ochiul, care știe sabotorii) și doi sabotori (unul e Vulpea). Cinci misiuni cu echipe de 2, 3, 2, 3, 3: conducătorul propune, toți votează, cei din echipă aleg în secret succes sau sabotaj (doar sabotorii pot sabota). 3 reușite = paznicii, dar Vulpea poate ghici Ochiul; 3 ratate sau 5 respingeri = sabotorii. Mutări: echipa 1 3, da, nu, succes, sabotaj, tinta 4, masa.",
            make: || Box::new(santinela::Farul::new()),
        },
        Playable {
            id: "consiliul",
            title: "Sfatul Negustorilor (alegeri și legi, roluri ascunse)",
            category: "Party",
            rules: "Cinci locuri (tu + 4 roboți): trei negustori, un complice și Cămătarul (se cunosc între ei). Președintele nominalizează un cancelar, toți votează; guvernul ales trage 3 legi, una e aruncată, una promulgată (6 P Prosperitate, 11 M Monopol în pachet). 5 P sau Cămătarul executat = negustorii; 6 M sau Cămătarul cancelar după 3 M = conspiratorii. A 3-a lege M: vezi pachetul; a 4-a și a 5-a: execuție. Mutări: cancelar 3, da, nu, arunca M, promulga P, executa 4, masa.",
            make: || Box::new(consiliul::Consiliul::new()),
        },
        Playable {
            id: "valea",
            title: "Valea Uscată (duel western, roluri ascunse)",
            category: "Cărți",
            rules: "Cinci pistolari (tu + 4 roboți): Șerif (public), Adjunct, doi Haiduci, Trădător. Legea câștigă dacă mor haiducii și trădătorul, haiducii dacă moare Șeriful, trădătorul rămânând singur. Pe tură tragi 2 cărți: foc (o dată, distanță 1; 2 cu pușcă), ocolire, leac, salon, mitralieră, pușcă, panică; arunci la câte vieți ai. Secetă după runda 30. Mutări: foc 3, leac, salon, mitraliera, pusca, panica 2, incheie, scarta foc, ocolire, accept, masa.",
            make: || Box::new(valea::Valea::new()),
        },
        Playable {
            id: "hruba",
            title: "Hoinari prin Hrubă (monștri și comori, masa = roboți)",
            category: "Cărți",
            rules: "Trei hoinari (tu + 2 roboți); primul la nivelul 10 omorând un monstru câștigă. Deschizi ușa: monstru (lupți: forța ta trebuie să DEPĂȘEASCĂ nivelul lui; poți folosi poțiuni +3; sau fugi cu zar 5+), blestem sau carte în mână; fără monstru poți căuta sau vâna un monstru din mână. Roboții pot îngreuna lupta. Max 5 cărți la final de tură; limită 90 de ture. Mutări: deschide, cauta, echipeaza arma3, vaneaza monstru4, crestere, forta, lupta, fugi, scapa blestem, masa.",
            make: || Box::new(hruba::Hruba::new()),
        },
        Playable {
            id: "apus",
            title: "Imperii în Apus (cucerire cu popoare în declin)",
            category: "Strategie",
            rules: "Hartă de 12 ținuturi, trei jucători (tu + 2 roboți), 8 ture. Alegi o combinație popor + dar din șase (cea din față e gratis, celelalte costă monede lăsate pe cele sărite). Un ținut costă 2 + 1 (munte) + jetoanele ocupantului; trebuie să fie vecin cu ale tale; ultima cucerire cu zarul de întărire. Câștigi 1 monedă pe ținut + bonusuri; poporul vlăguit intră în declin și alegi altul. Mai multe monede câștigă. Mutări: alege 2, continua, cucereste 5, zar 7, gata, declin.",
            make: || Box::new(apus::Apus::new()),
        },
        Playable {
            id: "fulger",
            title: "Fulger (simbolul comun, cronometru simulat)",
            category: "Party",
            rules: "31 de cărți cu 6 simboluri; oricare două au exact un simbol comun. Numești simbolul comun dintre cartea ta și cea din mijloc; masa (robot) are un timp de reacție tras la zar și greșește uneori. Cine e corect și mai rapid ia cartea din mijloc (+1 punct). Simbol greșit = runda masei. Mutări: numele simbolului (ex. luna, stea), cronometru.",
            make: || Box::new(fulger::Fulger::new()),
        },
        Playable {
            id: "roade",
            title: "Roade de Toamnă (legume și schimburi, masa = roboți)",
            category: "Cărți",
            rules: "Trei fermieri (tu + 2 roboți), 8 legume, 2 ogoare (al treilea 3 monede), mână cu ordine fixă: plantezi mereu prima carte. Tura: plantezi 1–2 cărți, întorci 2 cărți (le plantezi sau le oferi mesei în schimbul unei cărți orbe), iei 3 cărți. Poți recolta oricând (cărți → monede după tabel). Roboții îți pot oferi cărți. Final la a doua golire a pachetului. Mutări: planteaza 1, recolta 2, compra, treci, ofera, accepta ceapa, refuza.",
            make: || Box::new(roade::Roade::new()),
        },
        Playable {
            id: "tcg_evo",
            title: "Cărți colecționabile: evoluție și energie (Făpturi de Câmp)",
            category: "TCG",
            rules: "Set original de 40+ cărți (făpturi evolutive pe elemente, energii, ajutoare); pachete pregătite de 30 (J1 Pădurea, J2 Vulcanul). Pregătire: activ N, banca N, gata; 3 cărți-premiu. Tură: pesca (forțat), apoi banca N, evolueaza N T (nu pe făptură nouă, nu în prima ta tură), energie N T (una pe tură), joaca N [T] (un singur «sprijin»), retragere bK, apoi atac 1|2 (încheie tura) sau fin. Țintă: a (activa), b1..b5 (banca mea), e1..e5 (a adversarului). Slăbiciunea dublează daunele; efecte: vindecare, daune la bancă, recul, aruncare de energie, amețire. Făptura activă căzută = adversarul ia un premiu, tu alegi promova bK. Câștigi cu toate premiile, cu adversarul fără făpturi sau dacă el rămâne fără pachet. Informație ascunsă: vezi doar mâna J1.",
            make: || Box::new(tcg_evo::TcgEvo::new()),
        },
        Playable {
            id: "tcg_zone",
            title: "Cărți colecționabile: monștri în zone și capcane (Hotarele Ceții)",
            category: "TCG",
            rules: "Set original de 44 de cărți (monștri, vrăji, capcane); pachete pregătite de 30 (J1 Hotarul, J2 Mlaștina), 8000 PV. Tură: pesca (forțat), apoi o invocare pe tură (nivel 1-4 liber, 5-6 un tribut, 7+ două): invoca 2, seteaza 2 (întors, în apărare), invoca 2 tribut 1,3; intoarce I, schimba I, joaca N [e2|m1|g1] (vrăji), seteaza N (capcană), ataca 1 e2 / ataca 1 direct, fin. ATK vs ATK / ATK vs apărare (Pătrunde, Vânător). După fiecare atac apărătorul primește continua sau activa T (capcane puse într-o tură anterioară). J1 nu atacă în tura 1; pachet gol = pierdere. Informație ascunsă: cărțile întoarse ale adversarului nu se văd.",
            make: || Box::new(tcg_zone::TcgZone::new()),
        },
        Playable {
            id: "tcg_lant",
            title: "Cărți colecționabile: pitch, lanț și echipament (Lanțuri de Oțel)",
            category: "TCG",
            rules: "Set original de 40+ cărți; pachete pregătite de 30 (J1 Gardiana Zorilor, J2 Vânătorul de Furtuni), 30 viață, mână de 4, patru piese de echipament și o armă. Resursele vin din pitch: atac 2 pitch 1,3 (cărțile aruncate în pitch merg la fundul pachetului la sfârșitul turei). Un punct de acțiune pe tură: atac H, arma, joaca H (acțiuni, nu costă punctul), fin; «continuă» îl returnează. Lanț: apărătorul alege bloc 1,3 echip 2 (sau bloc fara), apoi reacții de atac / de apărare (reactie H pitch ..., pasa). Daune = putere - apărare. După 25 de ture pe jucător câștigă cine are mai multă viață. Informație ascunsă: vezi doar mâna J1.",
            make: || Box::new(tcg_lant::TcgLant::new()),
        },
        Playable {
            id: "tcg_cerneala",
            title: "Cărți colecționabile: cerneală, misiuni și provocări (Povestiri din Cerneală)",
            category: "TCG",
            rules: "Set original de 42 de cărți (personaje, acțiuni); pachete pregătite de 30 (J1 Pădurea Cântătoare, J2 Muntele de Fier), 7 cărți în mână; primul la 15 faimă câștigă. Tură: pesca (mutare forțată la începutul fiecărei ture; în prima tură J1 o scrie tot, dar nu trage nicio carte), cerneala N (o carte cu cerneală pe tură în călimară = 1 cerneală pe tură), joaca N [e2|m1] (personajele se usucă o tură), misiune P (aduce faimă), provocare P e Q (doar contra personajelor ÎNCLINATE), fin. Cuvinte-cheie: Elan, Gardian, Zburător, Provocator, Rezistență. Daunele rămân până la eliminare. Informație ascunsă: vezi doar mâna J1.",
            make: || Box::new(tcg_cerneala::TcgCerneala::new()),
        },
        Playable {
            id: "spionii",
            title: "Spionii Cuvintelor (grilă 5×5)",
            category: "Party",
            rules: "Variantă proprie de joc de spionaj cu cuvinte: grilă de 25 de cuvinte, 9 agenți roșii (J1), 8 albaștri (J2), 7 neutre, 1 asasin. Maestrul-spion al fiecărei echipe este calculatorul: dă ca indiciu numele unei teme din tabela proprie (MARE, CIRC…) și un număr. Tu ești ghicitorul: ghici cuvânt (fără diacritice), apoi dezvaluie întoarce cartea; cel mult număr+1 ghiciri, stop încheie tura. Asasinul pierde pe loc. AI-ul ghicește după aceeași tabelă de teme, fără să vadă cheia.",
            make: || Box::new(spionii::Spionii::new()),
        },
        Playable {
            id: "povestitor",
            title: "Povestitorul (cărți-simbol, 2 jucători)",
            category: "Party",
            rules: "Variantă proprie de joc de povestit cu cărți: 231 de descrieri-simbol. Povestitorul alege o carte și unul din indiciile ei (spune 3a); celălalt pune o momeală (momeala 2); amesteca adaugă 3 cărți-fantomă; ghicitorul votează (vot N), Publicul votează după potrivirea cuvintelor-cheie, apoi dezvaluie punctează (3/3 dacă exact un votant găsește cartea; altfel 0 și 2; +1 pentru momeală votată de Public). Până la 12 puncte sau 14 runde.",
            make: || Box::new(povestitor::Povestitor::new()),
        },
        Playable {
            id: "gura",
            title: "Gura Satului (completează fraza)",
            category: "Party",
            rules: "Variantă proprie de joc de completat fraze: 205 fraze cu gol și 231 de răspunsuri originale. Judecătorul alternează; celălalt joacă un răspuns (joaca N), amesteca adaugă 2 răspunsuri ale casei, judecătorul alege cea mai nostimă completare (alege N), numara dă punctul: jucătorului dacă a ales cartea lui, judecătorului dacă a ales una a casei. Primul la 6 puncte sau după 12 runde. AI-ul potrivește etichetele frazei cu ale răspunsului.",
            make: || Box::new(gura::Gura::new()),
        },
        Playable {
            id: "treicai",
            title: "Trei Căi (spune, mimează, desenează)",
            category: "Party",
            rules: "Variantă proprie de joc de explicat și ghicit, în terminal: 218 acțiuni și meserii cu aspecte predefinite redate pe rând vorbit, mimă sau schiță. Descriptorul alege indicii (arata N), adversarul poate fura ghicind (ghici N, pas), apoi robot lasă colegul-robot să ghicească dintre 6 candidați. Cu cât mai puține indicii, cu atât mai multe puncte (4…1); ghicire greșită a adversarului −1. 12 runde, rolurile alternează.",
            make: || Box::new(treicai::new()),
        },
        Playable {
            id: "oprite",
            title: "Cuvinte Oprite (descrie fără cuvinte interzise)",
            category: "Party",
            rules: "Variantă proprie de joc cu cuvinte interzise: 233 de cărți cu cuvânt-țintă, 4 interdicții și indicii-propoziție predefinite; indiciile care calcă o interdicție sunt fauluri (−1, detectate automat). Descriptorul alege indicii (arata N), adversarul poate fura ghicind (ghici N, pas), colegul-robot ghicește la pasul robot. 10 runde, cu cât mai puține indicii cu atât mai multe puncte.",
            make: || Box::new(oprite::new()),
        },
        Playable {
            id: "schita",
            title: "Schița (desen simbolic)",
            category: "Party",
            rules: "Variantă proprie de joc de desenat și ghicit: 224 de cuvinte, fiecare cu 6 tușe textuale (cerc, coadă, urechi ascuțite…). Desenatorul arată tușele una câte una (arata N), adversarul poate fura ghicind (ghici N, pas), apoi colegul-robot ghicește dintre 6 candidați (robot). Cu cât mai puține tușe, cu atât mai multe puncte (5…1). 10 runde, rolurile alternează.",
            make: || Box::new(schita::new()),
        },
        Playable {
            id: "dictionar",
            title: "Dicționar Viu (explică în rafală)",
            category: "Party",
            rules: "Variantă proprie de joc de explicat cuvinte: 215 cuvinte cu trei explicații predefinite. Fiecare tură are 9 acțiuni (arata N sau treci); fiecare cuvânt ghicit de colegul-robot (robot) sau furat de adversar (ghici N) valorează 1 punct; rădăcina cuvântului în explicație este fault (−1). 6 ture, rolurile alternează.",
            make: || Box::new(dictionar::new()),
        },
        Playable {
            id: "impostor",
            title: "Impostorul din Grilă (4 locuri)",
            category: "Party",
            rules: "Variantă proprie de joc cu impostor: 14 subiecte × 16 cuvinte; un cuvânt secret pe care îl știu toți mai puțin impostorul (J1, J2 sau unul din cei doi roboți). Fiecare dă un indiciu dintr-un cuvânt (indiciu cuvânt), apoi J1 și J2 votează (vot N), roboții votează la numara. Impostorul prins ghicește cuvântul (ghici cuvant, verifica). Puncte: vot corect +1, impostor neprins +2, prins fără să ghicească: nevinovații +2, prins care ghicește +1. 8 runde.",
            make: || Box::new(impostor::Impostor::new()),
        },
        Playable {
            id: "cuvantunic",
            title: "Cuvânt Unic (indicii care se anulează)",
            category: "Party",
            rules: "Variantă proprie de joc cu indicii de un cuvânt, în duel: 224 de cuvinte cu lexicon de indicii. Cel care dă indiciul (indiciu N) are doi martori robotici; indiciile identice se anulează (dezvaluie). Ghicitorul alege dintre 6 candidați (ghici N) sau pas; verifica: ghicit +1 ghicitorului, greșit +1 celui care a dat indiciul. 12 runde, rolurile alternează.",
            make: || Box::new(cuvantunic::CuvantUnic::new()),
        },
        Playable {
            id: "mosia",
            title: "Moșia (roluri, plantații și clădiri)",
            category: "Strategie",
            rules: "Joc original: 7 roluri (rol cultivator/constructor/producator/negustor/capitan/vistier/zidar); fiecare alege 3 pe rând, toți execută acțiunea, cine a ales are un privilegiu. Cultivator: planta 2 sau cariera; constructor: construieste targ (costul scade cu carierele); negustor: vinde z; capitan: expediaza z 1 (nave 4 și 6, un fel de marfă pe navă). Culturile p i z t c (prețuri 0–4). Câștigă cine are mai multe puncte (expediere + clădiri + monede/4). Fără colonii, ci doar plantații care produc direct.",
            make: || Box::new(mosia::Mosia::new()),
        },
        Playable {
            id: "gospodaria",
            title: "Gospodăria (plasare de muncitori și fermă)",
            category: "Strategie",
            rules: "Joc original, 6 runde, recoltă după rundele 2, 4, 6: pui pe rând un muncitor pe o acțiune liberă (pune lemn / lut / peste / oi / cereale / legume / zilier / arat / semanat 2 / camera / tarc / familie / inceput); fiecare acțiune ține un singur muncitor pe rundă, lemn, lut, peste și oi se acumulează dacă nu sunt luate. La recoltă câmpurile dau cereale, se hrănește familia (2 pe membru, altfel cerșetorii −3) și oile se înmulțesc. Punctaj pe categorii (câmpuri, țarcuri, cereale, oi, legume), camere, familie. Ferma e doar contoare, fără grilă.",
            make: || Box::new(gospodaria::Gospodaria::new()),
        },
        Playable {
            id: "cetatuia",
            title: "Cetățuia (muncitori pe drum de clădiri, castel)",
            category: "Strategie",
            rules: "Joc original, 9 runde: 5 muncitori pe jucător, plasați pe rând pe locurile unui drum (pune ferma / padure / cariera / piata / atelier / arhitect / castel / pod), primele 3 plasări costă 1 ban, apoi 2; pas dă 1 ban primului care pasează. La pod muncitorul mută jalonerul (pune pod +2): se activează doar locurile până la jaloner. Atelierul și arhitectul construiesc clădiri proprii (construieste gater) care cresc drumul și dau puncte când le folosește altcineva; la castel livrezi piese (castel l2 p1). Final: cetate completă sau runda 9; majoritatea pieselor +4.",
            make: || Box::new(cetatuia::Cetatuia::new()),
        },
        Playable {
            id: "taramuri",
            title: "Tărâmuri (transformarea terenului și piste de ritual)",
            category: "Strategie",
            rules: "Joc original pe hartă 7×6 cu 7 tipuri de teren pe o roată: J1 e de tip 0, J2 de tip 3. Pe rând o acțiune: colonie c3 (transformi terenul vecin în al tău, plătind lopeți, și pui o locuință), oras c3, templu c3 (îmbunătățiri), preot foc|apa|pamant|aer (+2 trepte pe pista de cult; la 5 privilegiu), pas. Început: inicial c3. 6 runde cu venit și un bonus de rundă; la final puncte pe piste, cel mai mare grup de clădiri și monede. Fără mană, fără facțiuni distincte, vecinătate pe 4 direcții.",
            make: || Box::new(taramuri::Taramuri::new()),
        },
        Playable {
            id: "furnale",
            title: "Furnale (rețea de legături și industrii)",
            category: "Strategie",
            rules: "Joc original, 8 runde, 2 acțiuni pe tură: pe o hartă cu 8 orașe construiești industrii (fabrica 3 t; c cărbune, f fier, t textil, p port) și legături (link 2-4) lipite de rețeaua ta; cărbunele se ia din cea mai apropiată mină legată sau din piață (2 bani), fierul din orice fierărie sau piață. Plăcuța se întoarce când se golește sau se vinde (vinde 4, prin legături până la un port) și dă venit și puncte; la final legăturile punctează după plăcuțele întoarse din orașele lor. imprumut = +8 bani, −2 puncte. Fără cărți, o singură eră.",
            make: || Box::new(furnale::Furnale::new()),
        },
        Playable {
            id: "reverdire",
            title: "Reverdire (variantă originală)",
            category: "Strategie",
            rules: "Motor de resurse și proiecte de cărți: ridici Umiditatea, Aerul și Lacurile (RR). O generație începe cu mutarea forțată generatie, apoi Cercetare (compra N, 3 L; gata), acțiuni una pe tură (joc N, vinde N, pompa, irigare, plantare, asezare, conv sem, conv caldura, pas; apoi pune c3), producție. Cărămizile plătesc proiectele B, Cristalele pe cele S. Scor: RR + PV + arbori + orașe. Informație ascunsă: mâna, oferta și pachetul.",
            make: || Box::new(reverdire::Reverdire::new()),
        },
        Playable {
            id: "salina",
            title: "Salina (variantă originală)",
            category: "Strategie",
            rules: "Plasare de agenți și construire de pachet, 10 runde sau 10 PV. agent 3 e (carte + locație a–l cu pictograma potrivită; la locațiile de luptă e–h: agent 3 e 2 = desfășori 2 trupe), revela apoi compra N / apa (1 Apă = +1 persuasiune) / sare (1 Sare = +1 sabie) / fin. Conflictul rundei: forța = 2 × trupe + săbii; premiul I/II; runda pregătește runda următoare. Informație ascunsă: mâna adversarului și pachetele.",
            make: || Box::new(salina::Salina::new()),
        },
        Playable {
            id: "catun",
            title: "Cătun (variantă originală)",
            category: "Strategie",
            rules: "Plasare de muncitori și cătun de cărți, 4 anotimpuri (2,3,4,6 muncitori). lucru a (locație a–h; lucru g L la Han), joc N / piata N (carte din mână / piață, max 15 în cătun), sezon (pregătire: muncitorii se întorc, producătoarele lucrează; în Toamnă ieși din joc). Scor: puncte de pe cărți, jetoane, prosperitate. Informație ascunsă: mâna adversarului și pachetul.",
            make: || Box::new(catun::Catun::new()),
        },
        Playable {
            id: "solzi",
            title: "Solzi (variantă originală)",
            category: "Cărți",
            rules: "Motor de cărți-reptile și amfibieni cu habitate, hrană și ouă, 4 runde (8,7,6,5 acțiuni). joc N p|s|b (vietate în Pădure/Stâncă/Baltă, plătești hrană și ouă), padure I V (hrană), stanca p1 s2 (ouă), balta (cărți). Puterile cărților se declanșează doar în habitatul acțiunii. Obiective de rundă; scor: puncte de pe cărți + ouă + obiective. Informație ascunsă: mâna adversarului și pachetul.",
            make: || Box::new(solzi::Solzi::new()),
        },
        Playable {
            id: "poieni",
            title: "Poieni (variantă originală)",
            category: "Strategie",
            rules: "Plăcuțe hexagonale de habitat și jetoane de animale. Tura: ia N (pereche din piață), pune q,r rN (plăcuța lipită de ținutul tău, rotire 0–5), animal q,r (jetonul pe o plăcuță liberă care îl acceptă) sau arunca. Jeton-natură: nat N M (orice plăcuță cu orice jeton), nat reset. 20 de ture. Scor: animale Urs/Vidră/Zimbru/Gaiță/Vulpe cu reguli proprii, cel mai mare coridor al fiecărui habitat (+3 pentru cel mai mare), jetoane-natură.",
            make: || Box::new(poieni::Poieni::new()),
        },
        Playable {
            id: "epoci",
            title: "Epoci (variantă originală)",
            category: "Cărți",
            rules: "Draft de cărți în 3 epoci cu comerț între vecini, pentru J1, J2 și Orașul Oarb (automat, al treilea oraș din inel, vecin cu amândoi). Fiecare epocă: 7 cărți pe oraș, 6 ture; alegi o carte și pasezi mâna. joc N (construiești, cumpărând resursele lipsă de la vecini cu 2 monede, 1 cu reducere), etapa N (cartea ține loc de etapă a minunii), vinde N (+3 monede). Scor: civice, minune, victorii militare (1/3/5, înfrângere −1), 1 p la 3 monede, știință, bresle. Împărțirea epocii următoare: imparte.",
            make: || Box::new(epoci::Epoci::new()),
        },
        Playable {
            id: "tinuturi",
            title: "Ținuturi (variantă originală)",
            category: "Strategie",
            rules: "Domino de teritorii cu coroane, 2 jucători: regat 9×9 cu castel în e5, încape în 7×7. 24 de piese-domino trase din setul nostru de 48 (generate de noi) în rânduri de câte 4: proprietarul piesei cu numărul mic joacă primul. Tura: pune e6-e7 (prima jumătate pe e6) lipit de castel sau de același teritoriu, apoi alege 2 din rândul următor; descarta doar dacă nu ai nicio așezare. Primul rând: alege N (J1, J2, J2, J1); revelare dezvăluie rândul. Scor: regiune = pătrate × coroane, +5 fără aruncări, +10 castel central.",
            make: || Box::new(tinuturi::Tinuturi::new()),
        },
        Playable {
            id: "rezervatia",
            title: "Rezervația (variantă originală)",
            category: "Strategie",
            rules: "Rezervație naturală cu cărți-animale și sponsori. Cinci acțiuni (Faună, Amenajare, Sponsori, Cărți, Conservare) într-un rând: puterea = locul (1–5); cea jucată trece pe locul 1. animal 2 e1 (la putere ≥ 4 încă un animal sau gata), construieste padure 3 / chiosc / pavilion, sponsor 3 / finantare, carti / carti v2, proiect 1 / donatie. Pauză la ritm 9: venit. Scor = apel + 2×conservare; la 50 jocul se încheie la sfârșitul rundei. Informație ascunsă: mâna adversarului și pachetul.",
            make: || Box::new(rezervatia::Rezervatia::new()),
        },
        Playable {
            id: "hotar",
            title: "Hotarul (variantă originală)",
            category: "Strategie",
            rules: "Motor de resurse și teritorii pe hartă hexagonală (a1–g4), facțiuni cu avantaje diferite (alege luncenii/munteanii/padurenii/campenii). Tura: o secțiune a tablei tale (nu cea de data trecută): acțiunea de sus (s1 mars h:g1>f1 / bani, s2 putere / carti, s3 produce a1 b2 / muncitor, s4 comert grane / faima, sau sari), apoi cea de jos (mobilizeaza f1, imbunatateste mars, construieste moara f1, inrolare 2, sau gata). Luptă: lupta P C (putere și carte angajate pe ascuns), apoi rezolva; egal câștigă apărătorul. 5 stele sau 32 de ture încheie jocul; scor = monede + stele + hexuri + resurse (după popularitate). Informație ascunsă: cărțile de luptă.",
            make: || Box::new(hotar::Hotar::new()),
        },
        Playable {
            id: "codru",
            title: "Codrul Nesupus (război asimetric, variantă originală)",
            category: "Strategie",
            rules: "Pădure cu 8 poieni și drumuri; J1/J2 primesc la sorți Regența (gărzi, Cherestea/Atelier/Cazarmă, lemn) sau Frunzișul (luptători, simpatie, suporteri, baze). Regența: trag, 3 acțiuni (mars A>B n, lupta P, recruteaza, construieste c|a|k P — în poiană stăpânită, 1 lemn), seara fabrica P, gata. Frunzișul: trag, 3 + baze acțiuni (mobilizeaza fel, propaga P, revolta P, recruteaza, mars, lupta). Lupta: ambuscada/fara, contra/lasa, apoi zar (gherila: apărând, Frunzișul lovește cu zarul mare). Mâna max 5 (arunca fel). 18 puncte sau 30 de ture. Informație ascunsă: mâna și suporterii adversarului.",
            make: || Box::new(codru::Codru::new()),
        },
        Playable {
            id: "sideral",
            title: "Coroana Siderală (4X compact, variantă originală)",
            category: "Strategie",
            rules: "9 sisteme (Citadela în centru, inel, două capete), planete cu resurse/influență, nave, 5 tehnologii. Runda: alegi pe rând 2 din 6 cărți de strategie (alege Comert), apoi acțiuni în ordinea inițiativei: taktic A>B n (activezi B, muți n nave; adauga C>B n; gata), carte Nume (efect principal; adversarul poate lua secundarul: sec / nu), pas. Lupta: zar până rămâne o singură parte; ocupi planeta cu ≥ r nave (r+1 dacă e a adversarului; capitalele nu se cuceresc). În sub-faze: nava Sistem, tech Nume, gata. Status: obiectiv N. 8 puncte sau 12 runde.",
            make: || Box::new(sideral::Sideral::new()),
        },
        Playable {
            id: "ceasornic",
            title: "Ceasornicul din Valea Ceții (deducție cu povestitor, masa = roboți)",
            category: "Party",
            rules: "Variantă originală: 12 locuri (tu + 11 roboți), 14 roluri în scenariu (7 localnici din 9, Bețivul, Sfântul, Otrăvitorul, Moștenitoarea, Strigoiul). Povestitorul e automat; Bețivul și cei otrăviți primesc informații false. Noaptea rolurile cu informație (Ghicitoarea, Cronicarul, Veghea, Cercetașul, Groparul) află ceva, Călugărul ocrotește, Strigoiul ucide din noaptea 2. Ziua roboții își spun rolul (cer N îi întreabă), tu declari adevărul (declar) sau minți (mint rol), nominalizezi și votezi; cine strânge jumătate din vii și cel mai mult e executat. Morții au un vot fantomă. Binele câștigă dacă moare Strigoiul; răul la 2 vii, Sfânt executat sau 20 de zile. Mutări: ghicesc 3 7, otravesc 3, ocrotesc 3, ucid 3, declar, mint veghea, cer 4, vanez 5, treci, nominez 6, pas, da, nu, masa.",
            make: || Box::new(ceasornic::Ceasornic::new()),
        },
        Playable {
            id: "coroana",
            title: "Drumul spre Coroană (aventură pe inele, om contra roboți)",
            category: "Strategie",
            rules: "Variantă originală: tu și doi roboți, șase personaje cu avantaje (alege haiducul, vrajitoarea, negustorul, padurarul, pelerinul, calauza). Pornești din Câmpie (16 câmpuri), treci Podul Gri (plată de 2 Aur, Cheia sau luptă), în Țara Cenușie câștigi Pecetea în Turn (asalt forta/iscusinta), urci Scara Neagră în Inelul de Jar și intri pe Ușa Coroanei. Tura: zar, apoi dreapta sau stanga; cărți de aventură (monștri, obiecte, descântece, întâmplări), han, capelă, târg (cumpar sabie, plec). Lupta: Forța + d6 trebuie să depășească forța monstrului (lupt, lupt fulger, fug, ceata). Cine rămâne fără Viață cade la Han. Cine e în camera Coroanei dă Porunca: ceilalți pierd 1 Viață pe tură (la 0 sunt scoși). Mutări: zar, bea, leac, urc, urc plata, urc cheie, urc lupta, intru, mantin, masa.",
            make: || Box::new(coroana::Coroana::new()),
        },
        Playable {
            id: "coridor",
            title: "Coridor (cursă de pioni cu ziduri)",
            category: "Abstract",
            rules: "Tablă 9×9; J1 (X) pornește din e1 și ajunge pe rândul 9, J2 (O) din e9 spre rândul 1. Pe rând: un pas ortogonal (se sare peste pionul adversar vecin, în diagonală dacă în spate e zid/margine) SAU un zid (10 de jucător, lung cât două pătrate) care nu se suprapune, nu se încrucișează și nu închide complet calea nimănui. Remiză la 200 de semi-mutări. Mutări: e2 (pion), d4h (zid orizontal peste d4–e4), d4v (zid vertical la dreapta lui d4–d5).",
            make: || Box::new(coridor::Coridor::new()),
        },
        Playable {
            id: "drum",
            title: "Drum de pietre (5×5, stive)",
            category: "Abstract",
            rules: "21 de pietre plate și o căpetenie de jucător. Prima mutare: pui o piatră plată a adversarului. Apoi: pui o piatră pe un pătrat gol (c3 plată, sc3 zid, cc3 căpetenie) sau muți o stivă proprie: iei de sus 1–5 pietre și le lași pe o linie (+ sus, - jos, < stânga, > dreapta), cel puțin una pe pătrat; ex. c3> sau 3c3>21. Zidul și căpetenia blochează stivele; doar căpetenia lăsată singură pe zid îl turtește. Câștigă drumul (lanț de pietre plate/căpetenii proprii între margini opuse); la tablă plină, rezervă goală sau 150 de semi-mutări câștigă cine are mai multe pietre plate la vârf.",
            make: || Box::new(drum::Drum::new()),
        },
        Playable {
            id: "tripierd",
            title: "Trei-pierde (hexagonal)",
            category: "Abstract",
            rules: "Tablă hexagonală cu latura 5 (61 de celule, rânduri a–i). Pe rând pui o piesă pe o celulă goală: patru (sau mai multe) în linie câștigă, exact trei în linie (fără patru) pierde; tabla plină = remiză. Mutări: e5 (litera = rândul, cifra = poziția în rând).",
            make: || Box::new(tripierd::TriPierd::new()),
        },
        Playable {
            id: "konane",
            title: "Konane (dame hawaiiene)",
            category: "Abstract",
            rules: "8×8 plină în șah (negrul = J1 pe a1, c1, …). Deschidere: J1 ridică d4, e5, a1 sau h8 (remove d4), J2 ridică o piesă proprie vecină cu golul. Apoi sari ortogonal peste o piesă adversă într-un pătrat gol (o scoți), poți continua în aceeași direcție. Cine nu mai poate muta pierde. Mutări: remove d4, c3-e3, c3-g3 (salt dublu).",
            make: || Box::new(konane::Konane::new()),
        },
        Playable {
            id: "fanorona",
            title: "Fanorona (Madagascar)",
            category: "Abstract",
            rules: "Tablă 5×9 de puncte (diagonale doar pe punctele pare), 22 piese pe jucător. Mutare cu un pas pe un punct liber; dacă pasul continuă spre o piesă adversă o captură pe ea și șirul din spatele ei (abordare, a), iar dacă te îndepărtezi de o piesă adversă o captură la fel (retragere, w). Captura e obligatorie; după ea aceeași piesă poate continua (opțional) cu alte capturi, fără să repete direcția și fără să revină pe un punct deja ocupat în tură. Câștigi scoțând toate piesele adverse; remiză după 40 de semi-mutări fără captură. Mutări: e2-e3, c3-d3a, c3-d3w, c3-d3a-d4w-e5a (lanț; fiecare prefix al unui lanț apare în listă ca mutare separată, iar mutarea aleasă încheie tura: așa alegi unde te oprești).",
            make: || Box::new(fanorona::Fanorona::new()),
        },
        Playable {
            id: "tigri",
            title: "Tigri și capre (Nepal)",
            category: "Abstract",
            rules: "Tablă 5×5 de puncte (diagonale pe punctele pare). Tu ești caprele (20, în mână) și muți primul; calculatorul are 4 tigri în colțuri. Caprele se plasează pe rând pe puncte libere, apoi se mută cu un pas. Tigrii se mută cu un pas sau sar peste o capră vecină pe punctul liber de după ea, capturând-o (neobligatoriu). Tigrii câștigă cu 5 capre capturate; caprele câștigă imobilizând toți tigrii; remiză după 100 de semi-mutări fără captură în faza de mutare. Mutări: c3 (plasare), a1-b2 (pas), a1xc1 (săritură cu captură).",
            make: || Box::new(tigri::Tigri::new()),
        },
        Playable {
            id: "surakarta",
            title: "Surakarta (Java)",
            category: "Abstract",
            rules: "Tablă 6×6, 12 piese pe jucător. Mutare simplă pe un punct vecin liber (și diagonal). Captura: piesa aleargă pe unul din cele două circuite (liniile 2 și 5 = mic; 3 și 4 = mare, legate prin arcuri în colțuri), în oricare sens, peste puncte libere, trece prin cel puțin un arc și ia prima piesă întâlnită dacă e a adversarului; propriile piese blochează, iar locul de plecare contează ca liber. Câștigi scoțând toate piesele adverse; remiză după 60 de semi-mutări fără captură. Mutări: b2-b3 (pas), a2xb6 (captură).",
            make: || Box::new(surakarta::Surakarta::new()),
        },
        Playable {
            id: "hasami",
            title: "Hasami Shogi (Japonia)",
            category: "Abstract",
            rules: "Tablă 9×9, 9 piese pe jucător pe rândul din spate. Piesele se mută ca turnul (fără sărituri). Captură prin încadrare: un șir de piese adverse consecutive, pe rând sau coloană, aflat între piesa mutată și o altă piesă a ta, este scos; poți intra liber între două piese adverse. Câștigi lăsând adversarul cu cel mult o piesă sau blocat; remiză după 100 de semi-mutări fără captură (colțurile nu au regulă specială). Mutări: a1-a5 (fișiere a–i, rânduri 1–9).",
            make: || Box::new(hasami::Hasami::new()),
        },
        Playable {
            id: "puncte",
            title: "Puncte și căsuțe (5×5)",
            category: "Abstract",
            rules: "Rețea de 6×6 puncte (5×5 căsuțe). Pe rând tragi o linie între două puncte vecine; cine închide a patra latură a unei căsuțe o câștigă și mută din nou (o linie poate închide două căsuțe). Când s-au tras toate cele 60 de linii, câștigă cine are mai multe căsuțe (25, deci fără remiză). Mutări: oa1 (linie orizontală, de la punctul a1 spre dreapta), va1 (linie verticală, de la punctul a1 în sus); coloane a–f, rânduri 1–6.",
            make: || Box::new(puncte::Puncte::new()),
        },
        Playable {
            id: "strapungere",
            title: "Străpungere (8×8)",
            category: "Abstract",
            rules: "Fiecare jucător are 16 piese pe primele două rânduri. O piesă înaintează un pas drept pe un pătrat gol sau un pas în diagonală pe un pătrat gol ori ocupat de adversar (captură); înainte nu se capturează. Câștigă cine ajunge primul pe rândul de capăt al adversarului sau cine capturează toate piesele adverse; cine nu mai are nicio mutare pierde. J1 (X) urcă, J2 (O) coboară. Mutări: b2-b3, b2-c3, b2xc3.",
            make: || Box::new(strapungere::Strapungere::new()),
        },
        Playable {
            id: "izolare",
            title: "Izolare (7×7)",
            category: "Abstract",
            rules: "Fiecare jucător are un pion (X la d1, O la d7). O tură: muți pionul ca regina la șah (orice distanță în linie dreaptă sau diagonală, doar peste pătrate libere), apoi scoți definitiv de pe tablă un pătrat liber (nu cel cu pionul adversarului). Cine nu mai poate muta la începutul turei pierde; nu există remiză. Mutări: d1-d3/e5 (de la, la, pătratul scos).",
            make: || Box::new(izolare::Izolare::new()),
        },
        Playable {
            id: "rotire",
            title: "Rotire în cinci (6×6)",
            category: "Abstract",
            rules: "Tablă 6×6 din patru cadrane 3×3 (1 stânga-sus, 2 dreapta-sus, 3 stânga-jos, 4 dreapta-jos). O tură: pui o bilă pe un pătrat gol, apoi rotești un cadran cu un sfert de tură (d = în sensul acelor de ceasornic, s = invers). Cinci bile în linie (orizontal, vertical, diagonal) după rotire câștigă; dacă ambii jucători au cinci deodată sau tabla se umple fără cinci, e remiză. Mutări: c3/1d (bila pe c3, cadranul 1 spre dreapta).",
            make: || Box::new(rotire::Rotire::new()),
        },
        Playable {
            id: "coloane",
            title: "Coloane (dame în turnuri)",
            category: "Abstract",
            rules: "Tablă 7×7 pe pătratele închise, 11 piese pe jucător. Soldatul merge un pas în diagonală înainte, ofițerul (soldat ajuns pe ultimul rând) în toate diagonalele. Captura e obligatorie: sari peste o coloană adversă vecină pe un pătrat liber și iei doar piesa ei de sus, care ajunge sub coloana ta; săriturile se înlănțuie, o coloană nu poate fi sărită de două ori, promovarea încheie tura. Coloana e condusă de piesa de sus. Pierde cine nu mai poate muta; remiză după 80 de semi-mutări fără captură. Mutări: c3-d4 (pas), c3xe5xg3 (salturi).",
            make: || Box::new(coloane::Coloane::new()),
        },
        Playable {
            id: "bile",
            title: "Bile pe hexagoane (5 pe latură)",
            category: "Abstract",
            rules: "Tablă hexagonală cu 61 de celule, 14 bile pe jucător. Muți 1-3 bile proprii vecine pe aceeași linie cu un pas: pe lungime (poți împinge 1 sau 2 bile adverse dacă ai mai multe bile decât ele și locul de dincolo e liber sau în afara tablei) ori lateral (toate destinațiile libere). Bilele împinse în afară sunt capturate; câștigă cine scoate 6; remiză după 120 de semi-mutări fără captură. Celule: rândul a (jos) – i (sus) și coloana 1-9. Mutări: e5>ne (o bilă), c3-c5>se (grup, de la prima la ultima bilă); direcții ne, e, se, sw, w, nw.",
            make: || Box::new(bile::Bile::new()),
        },
        Playable {
            id: "patru",
            title: "Patru atribute (piesa dată)",
            category: "Abstract",
            rules: "16 piese distincte cu patru însușiri binare (culoare, înălțime, formă, vârf), tablă 4×4. În tura ta pui pe tablă piesa primită de la adversar, apoi alegi una dintre cele rămase și i-o dai lui; prima tură doar dă o piesă. Câștigă cine completează un rând, o coloană sau o diagonală de patru piese cu o însușire comună; tabla plină = remiză. Piesele se numesc 0-9, a-f (biții = însușirile). Mutări: >5 (prima tură), c3>7 (pui pe c3, dai piesa 7), c3 (mutare câștigătoare sau ultima).",
            make: || Box::new(patru::Patru::new()),
        },
        Playable {
            id: "cupe",
            title: "Cupe încuibate (4×4)",
            category: "Abstract",
            rules: "Tablă 4×4; fiecare jucător are 3 stive de câte 4 cupe (mărimi 1-4) încuibate, se ia doar cea de sus. Pui o cupă din rezervă pe o celulă goală sau peste una mai mică, sau muți o cupă proprie vizibilă pe una goală ori mai mică. Din rezervă poți acoperi o cupă adversă doar dacă face parte dintr-un șir de trei piese adverse vizibile pe o linie. Câștigă patru cupe vizibile pe o linie; dacă ridici o cupă și descoperi șirul de patru al adversarului, el câștigă. Remiză după 50 de semi-mutări fără punere din rezervă. Mutări: 3@b2 (din rezervă, mărimea 3), b2-c3 (mutare pe tablă).",
            make: || Box::new(cupe::Cupe::new()),
        },
        Playable {
            id: "poliedre",
            title: "Poliedre (duo, 14×14)",
            category: "Abstract",
            rules: "Tablă 14×14, 2 jucători, fiecare cu 21 de poliomino (1-5 pătrate, notate a-u). Prima piesă acoperă pătratul de start (e5 pentru J1, j10 pentru J2); următoarele trebuie să atingă o piesă proprie prin colț și niciodată pe latură (piesele adverse se pot atinge). Cine nu poate pune trece; la două treceri la rând jocul se termină și câștigă cine are mai puține pătrate în piesele nefolosite. Mutări: k3@e5 (piesa k, orientarea 3, colțul stânga-sus al dreptunghiului piesei la e5), pas.",
            make: || Box::new(poliedre::Poliedre::new()),
        },
        Playable {
            id: "impletitura",
            title: "Împletitura (echilibru pe covor cu cercuri colorate)",
            category: "Party",
            rules: "Covor de 4 rânduri (a–d) × 6 coloane (1–6) cu cercuri în 4 culori (Roșu, Albastru, Verde, Galben; câte 6, după seed), 2 jucători, fiecare cu 4 membre (ms, md, ps, pd) care încep în afara covorului. Tura începe cu rotița (gira), care dă un membru și o culoare; muți membrul pe un cerc liber de acea culoare (b3) la distanță cel mult 3 (Chebyshev) de celelalte membre ale tale de pe covor; un cerc nu poate avea două membre; poți rămâne pe cercul curent dacă are culoarea dată. Fără cerc valid, singura mutare este cad: cazi și pierzi; ultimul rămas câștigă (remiză după 300 de mutări fără cădere). Fără informație ascunsă; AI: Easy aleator, Normal păstrează flexibilitatea, Hard/Expert îți blochează cercurile. Mutări: gira, b3, cad.",
            make: || Box::new(impletitura::Impletitura::new()),
        },
        Playable {
            id: "pinochle",
            title: "Pinochle",
            category: "Cărți",
            rules: "48 de cărți (două de fiecare 9–A pe 4 culori), 12 fiecare, 2 jucători, primul în stoc. Se joacă 24 de levate; cât timp stocul nu e gol nu ești obligat să urmezi culoarea, după aceea trebuie să urmezi și să învingi când poți. Câștigătorul unei levate poate anunța o combinație (pereche, familie, pinochle, cuplu etc.) care conține cel puțin o carte nouă; la sfârșit se numără A, 10, K din levate. Primul la 1000. Fără schimbul lui 9 de atu. Mutări: 9h, anunt <combinatie>, fara, urmatoarea.",
            make: || Box::new(pinochle::Pinochle::new()),
        },
        Playable {
            id: "gin_rummy",
            title: "Gin Rummy",
            category: "Cărți",
            rules: "52 de cărți, 10 fiecare. La rând: iei din talon (talon) sau stoc (stoc), apoi arunci (dis X). Cu deadwood de cel mult 10 poți bate (bat X) sau da gin (gin X, 0 deadwood); adversarul poate adăuga cărți la combinațiile tale (adauga X la N). Se punctează diferența de deadwood, undercut pentru apărător; până la 100. Fără big gin. Mutări: talon, pas, stoc, dis 9h, bat 9h, gin 9h, adauga 5h la 1, gata, arata, urmatoarea.",
            make: || Box::new(gin_rummy::GinRummy::new()),
        },
        Playable {
            id: "crazy_eights",
            title: "Crazy Eights (Optarii)",
            category: "Cărți",
            rules: "52 de cărți, 7 fiecare. Joci o carte de aceeași culoare sau valoare cu cea de deasupra; optarul (8) se joacă oricând și alege culoarea (8h:s). Dacă nu poți, tragi (trage) și poți juca ori pasa. Cine rămâne fără cărți ia punctele din mâna adversarului (8 = 50, figuri = 10, A = 1); până la 100. Mutări: 7h, 8h:s, trage, pas, urmatoarea.",
            make: || Box::new(crazy_eights::CrazyEights::new()),
        },
        Playable {
            id: "pitch",
            title: "Pitch (Setback)",
            category: "Cărți",
            rules: "52 de cărți, 6 fiecare, 2 jucători. Licitezi 2–4 (licit N) sau pas; licitatorul conduce, iar culoarea primei cărți devine atu. Răspunzi la culoare sau tai cu atu. Patru puncte: High, Low, Jack, Game; licitatorul care nu atinge contractul pierde cât a licitat. Primul la 11. Mutări: pas, licit 3, 9h, urmatoarea.",
            make: || Box::new(pitch::Pitch::new()),
        },
        Playable {
            id: "bid_whist",
            title: "Bid Whist (2 jucători)",
            category: "Cărți",
            rules: "Adaptare pentru doi jucători: 54 de cărți (cu doi jokeri), 12 fiecare și un kitty de 6. Licitezi 3–6 (licit N) sau pas; câștigătorul ia kitty-ul și aruncă 6 cărți, apoi alege Uptown (A mare), Downtown (A mic) sau Fără atu. Punctele: levate peste 6 (sau contractul), dublu la fără atu; primul la ±7. Mutări: pas, licit 4, dis 9h, sus h, jos h, fara sus, fara jos, 9h, reimparte, urmatoarea.",
            make: || Box::new(bid_whist::BidWhist::new()),
        },
        Playable {
            id: "all_fours",
            title: "All Fours",
            category: "Cărți",
            rules: "52 de cărți, 6 fiecare, o carte de atu întoarsă (valet = 3 puncte dealerului). Non-dealerul cere (cer) sau joacă (joc); dealerul dă un punct (dau) sau schimbă (schimb, apoi imparte, aruncări dis X). Patru puncte: High, Low, Jack, Game; primul la 14. Mutări: cer, joc, dau, schimb, imparte, dis 9h, 9h, urmatoarea.",
            make: || Box::new(all_fours::AllFours::new()),
        },
        Playable {
            id: "conquian",
            title: "Conquián",
            category: "Cărți",
            rules: "Pachet spaniol de 40 de cărți (A–7, S, C, R; patru culori), 2 jucători, 10 cărți. Tragi din stoc (stoc) sau iei cartea de sus a talonului (ia ...) doar ca s-o legi imediat într-o combinație (set de 3–4 sau serie de cel puțin 3 din aceeași culoare); apoi cobori combinații (pune ...), lipești cărți (adauga X la N) și arunci (descarta X). Cine rămâne fără cărți în mână câștigă mâna și un punct; primul la 3. Mutări: stoc, ia ..., pune ..., adauga 5o la 1, descarta 5o, pas, urmatoarea.",
            make: || Box::new(conquian::Conquian::new()),
        },
        Playable {
            id: "chicken_foot",
            title: "Chicken Foot (domino)",
            category: "Clasic",
            rules: "Set dublu-12, 15 piese fiecare, 13 runde (de la dublu-12 la dublu-0). Se pornește de la dublul de rundă; un dublu trebuie acoperit (pui piesa pe el) înainte de a continua. Runda se termină când cineva rămâne fără piese sau la blocaj; fiecare adaugă la scor punctele pieselor rămase în mână. După 13 runde câștigă cel mai mic total. Mutări: 5-7@2, trage, pas, urmatoarea.",
            make: || Box::new(chicken_foot::ChickenFoot::new()),
        },
        Playable {
            id: "bunco",
            title: "Cele șase ținte (zaruri)",
            category: "Clasic",
            rules: "Joc de zaruri după Bunco (nume comercial): 6 runde, în runda N ținta e N. Arunci 3 zaruri cât timp ai puncte: fiecare zar egal cu ținta = 1 punct, trei egale cu ținta = 21 (bunco, încheie runda), trei la fel altfel = 5. Runda se încheie la 21 de puncte; după 6 runde câștigă cine a luat mai multe runde. Mutări: arunc.",
            make: || Box::new(bunco::Bunco::new()),
        },
        Playable {
            id: "farkle",
            title: "Zece mii de puncte (zaruri)",
            category: "Clasic",
            rules: "Joc de zaruri după Farkle / 10 000 (nume comercial): arunci 6 zaruri, păstrezi combinații care punctează (1 = 100, 5 = 50, trei la fel, serii, perechi), apoi arunci din nou sau bancrezi. Fără combinație pierzi tot ce ai strâns în tură. Cu 0 puncte în total banca doar de la 500 în tură; țintă 10 000 cu rundă finală. Mutări: arunc, pastrez 1 5, banca, farkle.",
            make: || Box::new(farkle::Farkle::new()),
        },
        Playable {
            id: "liars_dice",
            title: "Zarurile mincinosului",
            category: "Clasic",
            rules: "Variantă Perudo/Dudo/Cacho: fiecare are 5 zaruri ascunse sub pahar. Licitezi cantitatea unei fețe pentru toate zarurile de pe masă (aștii sunt jokeri), mereu mai sus, sau spui dudo; cine greșește pierde un zar. Ultimul cu zaruri câștigă. Fără calza și fără palifico. Mutări: 3x4, dudo, arata.",
            make: || Box::new(liars_dice::LiarsDice::new()),
        },
        Playable {
            id: "crokinole",
            title: "Crokinole",
            category: "Familie",
            rules: "Joc de dexteritate pe tablă rotundă, modelat ca simulare deterministă cu zgomot din seed: alegi poziția, unghiul și puterea tragerii (x-1 u6 p3), apoi lasi discul. Trebuie să lovești un disc adversar dacă există. 12 discuri pe jucător; inelele valorează 5, 10 și 15, gaura centrală 20. Câștigătorul rundei ia diferența de puncte; primul la 100 (cel mult 6 runde). Mutări: x-1 u6 p3, lasa.",
            make: || Box::new(crokinole::Crokinole::new()),
        },
        // --- P4 ---
        Playable {
            id: "chinchon",
            title: "Chinchón (Spania)",
            category: "Cărți",
            rules: "Baraja spaniolă de 40 de cărți (o c e b; 1–7, 10, 11, 12), 7 cărți fiecare. Tragi din stoc sau talon și arunci; combinații: seturi și secvențe (7 nu se leagă de 10). Închizi cu cel puțin 6 cărți potrivite și cel mult 5 puncte libere (−10 dacă toate 7 se potrivesc; chinchón = 1–7 din aceeași culoare câștigă jocul); cine ajunge la 100 pierde. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: stoc, talon, 5o, inchide-5o, urmatoarea.",
            make: || Box::new(chinchon::Chinchon::new()),
        },
        Playable {
            id: "burako",
            title: "Burako (Buraco)",
            category: "Cărți",
            rules: "106 cărți (2 pachete + 2 jokeri), 11 în mână și un muerto de 11. Tragi din stoc (mazo) sau iei tot pozo, lași jos scări (≥3 consecutive, aceeași culoare) și piernas (≥3 de același rang), cel mult un comodin (joker sau 2) per combinație, apoi arunci. Canastă = 7+ cărți: pură 200, impură 100; închizi cu muertoul luat și o canastă (+100). Meci până la 1500 sau 4 mâini. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: mazo, pozo, esc:4h+5h+6h, set:7h+7c+Jk, +1:8h, descarta 5h, muerto, urmatoarea.",
            make: || Box::new(burako::Burako::new()),
        },
        Playable {
            id: "escoba",
            title: "Escoba (Escoba de 15)",
            category: "Cărți",
            rules: "Baraja spaniolă de 40 de cărți (valori 1–7, 10=8, 11=9, 12=10), 3 în mână și 4 pe masă. Joci o carte și poți lua cărți de pe masă care, adunate cu ea, dau 15; golești masa = escoba (1 punct, nu la ultima carte). Punctaj la capăt de mână: cartas, oros, siete de velo (7o), setenta și escobe; primul la 21. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 7o, 7o:4c+4e, imparte, urmatoarea.",
            make: || Box::new(escoba::Escoba::new()),
        },
        Playable {
            id: "cuarenta",
            title: "Cuarenta (Ecuador)",
            category: "Cărți",
            rules: "Baraja spaniolă de 40 de cărți, câte 5 în mână, masa începe goală. Joci o carte și poți lua: o carte de același rang (potrivire) sau cărți 1–7 cu aceeași sumă (adunare), plus secvența ascendentă care urmează. Caída 2, limpia 2, ronda 4 puncte; la capăt de mână puncte după numărul de cărți luate; primul la 40. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 5o, 5o:5c, 6o:2c+4e, imparte, urmatoarea.",
            make: || Box::new(cuarenta::Cuarenta::new()),
        },
        Playable {
            id: "generala",
            title: "Generala (zaruri)",
            category: "Zaruri",
            rules: "5 zaruri, până la 3 aruncări pe tură, 10 categorii: unu…sase (suma zarurilor), escalera 20, full 30, poker 40, generala 50. Servida (din prima aruncare) +5 la escalera/full/poker; generala servida câștigă jocul pe loc. Categoriile nepotrivite se taie (0). 10 ture pe jucător, câștigă totalul mai mare. Mutări: arunc, tine 3 3 5, tine -, scor escalera.",
            make: || Box::new(generala::Generala::new()),
        },
        Playable {
            id: "parques",
            title: "Parqués (Colombia)",
            category: "Zaruri",
            rules: "Pistă de 68 de pătrate, 4 piese pe jucător, două zaruri. Din cárcel ieși doar cu pereche (toate piesele deodată); pereche = încă o aruncare, trei perechi = o piesă direct în corona. Captura (obligatorie) trimite piesa adversă în cárcel, pe pătratele sigure nu se capturează; corona se atinge exact. Primul cu 4 piese în corona câștigă. Mutări: 12:5 (piesa de la progresul 12 avansează 5), sale, pasa.",
            make: || Box::new(parques::Parques::new()),
        },
        Playable {
            id: "fivehundred",
            title: "Five Hundred (500, doi jucători)",
            category: "Cărți",
            rules: "500 pentru 2 jucători: 43 de cărți (cu joker), 10 în mână, 5 grămezi de câte 2 (jos + sus) și un kitty de 3. Licitezi 6–10 levate cu atu sau fără atu (fără misere); contractantul ia kitty-ul și aruncă 3 cărți. Levate de 4 cărți (câte una din mână și una cu fața în sus); jokerul și valeții (bower) sunt atuurile cele mai mari. Contract reușit = valoarea lui, eșuat = −valoarea; adversarul ia 10 pe levată; 500 / −500. Informație ascunsă. Mutări: licit 7h, licit 8nt, pas, descarta As, 10h, Jk:h, urmatoarea.",
            make: || Box::new(fivehundred::FiveHundred::new()),
        },
        Playable {
            id: "pontoon",
            title: "Pontoon",
            category: "Cărți",
            rules: "Banca (rolul alternează la fiecare mână) contra jucătorului: Pontoon (as + carte de 10) > Five Card Trick (5 cărți ≤ 21) > 21 > cât mai aproape de 21; egalitatea o câștigă banca. Jucătorul: carte (twist), cumpar (carte ascunsă, miza +10, doar înainte de carte), stai (de la 15); banca stă de la 17. Pontoon și Five Card Trick plătesc dublu. 20 de mâini cu miza 10, câștigă cine are mai multe jetoane. Informație ascunsă. Mutări: carte, cumpar, stai, decontare.",
            make: || Box::new(pontoon::Pontoon::new()),
        },
        Playable {
            id: "twoup",
            title: "Two-up (Australia)",
            category: "Zaruri",
            rules: "Spinnerul mizează pe stemă (5, 10 sau 20 de jetoane); celălalt acoperă. Două monede: cap-cap = spinnerul câștigă și continuă; pajura-pajura = pierde și rolul trece; cap-pajura (odds) = se aruncă din nou, cinci odds la rând = spinnerul pierde. 24 de mize, start 50 de jetoane; câștigă cine are mai multe. Mutări: miza 5, miza 10, miza 20, arunc.",
            make: || Box::new(twoup::TwoUp::new()),
        },
        Playable {
            id: "ancora",
            title: "Ancora și coroana",
            category: "Zaruri",
            rules: "Trei zaruri cu simboluri (coroana, ancora, inima, romb, trefla, pica) — joc britanic tradițional «Crown and Anchor». Mizezi 5, 10 sau 20 pe unul sau mai multe simboluri; pentru fiecare zar cu simbolul tău primești miza înapoi plus câștig (1:1, 2:1, 3:1), altfel pierzi miza. 12 runde împotriva băncii, câștigă cine are mai multe jetoane. Mutări: pun ancora 10, gata, arunc.",
            make: || Box::new(ancora::Ancora::new()),
        },
        Playable {
            id: "mutorere",
            title: "Mu Torere (Noua Zeelandă)",
            category: "Abstract",
            rules: "8 puncte pe cerc (kewai 1–8) și centrul (0); fiecare are 4 piese pe puncte vecine, centrul gol. Muți o piesă pe un punct vecin gol; în centru intri doar cu o piesă vecină cu o piesă adversă. Pierde cine nu mai poate muta; remiză la a treia repetare sau după 200 de mutări. Mutări: 3-4 (de pe 3 pe 4), 4-0 (în centru), 0-6 (din centru).",
            make: || Box::new(mutorere::MuTorere::new()),
        },
        Playable {
            id: "tictactoe",
            title: "X și 0 (Tic-tac-toe)",
            category: "Abstract",
            rules: "Tablă 3×3; jucătorii pun pe rând X și 0. Trei la rând (linie, coloană sau diagonală) câștigă; tabla plină fără linie e remiză. Mutări: a1…c3.",
            make: || Box::new(tictactoe::TicTacToe::new()),
        },
        Playable {
            id: "ur",
            title: "Jocul Regal din Ur",
            category: "Abstract",
            rules: "Cursă din Mesopotamia: 7 piese fiecare, patru zaruri 0–4, drum de 14 pași. Rozetele (pașii 4, 8, 14) dau o nouă aruncare; rozeta centrală comună (8) e sigură; aterizarea pe o piesă adversă din zona comună o trimite la start; ieșirea cere aruncare exactă. Mutări: intra sau numărul pasului piesei (ex. 7).",
            make: || Box::new(ur::Ur::new()),
        },
        Playable {
            id: "crazyhouse",
            title: "Crazyhouse (șah cu reintroducere)",
            category: "Abstract",
            rules: "Șah în care piesele capturate trec în rezerva ta și pot fi introduse pe orice câmp liber în loc de mutare (pionii nu pe prima/ultima linie); o piesă promovată capturată revine pion. Reguli complete de șah; remiză prin repetiție triplă sau după 200 de mutări. Mutări: e2e4, e7e8q, N@f3.",
            make: || Box::new(crazyhouse::Crazyhouse::new()),
        },
        Playable {
            id: "omweso",
            title: "Omweso",
            category: "Abstract",
            rules: "Mancala din Uganda: 4×8 gropi, două rânduri fiecare, câte 2 semințe (64). Alegi o groapă cu cel puțin 2 semințe și semeni în sens trigonometric prin cele 16 gropi ale tale; dacă ultima sămânță cade într-o groapă nevidă din rândul interior, cu ambele gropi adverse din coloană nevide, le capturezi; altfel continui să semeni (releu). Pierzi când nu mai ai nicio groapă cu 2 semințe. Mutări: i1…i8 (rândul interior), o1…o8 (exterior).",
            make: || Box::new(omweso::Omweso::new()),
        },
        Playable {
            id: "twixt",
            title: "TwixT",
            category: "Abstract",
            rules: "Joc de conectare pe tabla 24×24: J1 leagă marginea de sus de cea de jos, J2 pe cea din stânga de cea din dreapta; nu poți pune cuie pe marginile adversarului sau în colțuri. Cuiele aflate la mutare de cal se leagă automat dacă puntea nu taie alta. Câștigă cine închide primul lanțul. Mutări: e12 (coloana a–x, rândul 1–24; rândul 24 e sus).",
            make: || Box::new(twixt::Twixt::new()),
        },
        Playable {
            id: "halma",
            title: "Halma",
            category: "Clasic",
            rules: "Halma pe 16×16 pentru doi: 19 piese fiecare în colțuri opuse. Muți un pas (și pe diagonală) sau un lanț de sărituri peste piese vecine; nu se captează nimic. Câștigă primul care aduce toate cele 19 piese în tabăra adversă; după 150 de mutări pe jucător, cine are mai multe acolo. Mutări: b2d4 (de la, la).",
            make: || Box::new(halma::Halma::new()),
        },
        Playable {
            id: "mahjong_hk",
            title: "Mahjong Hong Kong (minimum 3 fan)",
            category: "Clasic",
            rules: "Mahjong în stil Hong Kong pentru 2 jucători, 136 de piese fără flori; o mână câștigă doar cu cel puțin 3 fan (tragere proprie 1, închisă 1, toate secvențele 1, toate pungurile 3, o culoare + onoruri 3, culoare pură 6, doar onoruri 10, pung de dragon/vânt propriu/vântul rundei câte 1). Mutări: trage, d5m, mahjong, kong5m, trage-moarta, pung, kong, chi2m3m, pas.",
            make: || Box::new(mahjong_hk::MahjongHk::new()),
        },
        Playable {
            id: "fia",
            title: "Fia-med-knuff",
            category: "Clasic",
            rules: "Ludo suedez: 4 piese fiecare, pistă de 48 de câmpuri. 1 sau 6 intră o piesă; 6 dă încă o aruncare; aterizând pe o piesă adversă o trimiți acasă. Finalul cere aruncare exactă; câștigă primul cu toate cele patru acasă. Mutări: intra sau numărul progresului piesei (ex. 12).",
            make: || Box::new(fia::new()),
        },
        Playable {
            id: "tiengow",
            title: "Tien Gow (Cer și nouă)",
            category: "Clasic",
            rules: "Joc cantonez de levate cu 32 de piese de domino, adaptat pentru doi: 8 piese fiecare. Conduci cu o piesă sau o pereche; celălalt bate cu o piesă mai mare de aceeași culoare (civil/militar) sau o pereche mai mare de același tip, ori aruncă cu fața în jos. Ultima levată câștigă mâna; primul la 10 puncte. Mutări: 66, 66+66, d64 (d = aruncare ascunsă).",
            make: || Box::new(tiengow::TienGow::new()),
        },
        Playable {
            id: "aceydeucey",
            title: "Acey-deucey",
            category: "Clasic",
            rules: "Variantă americană de table: toate cele 15 piese pornesc din afara tablei și intră în casa adversarului; zarul 1+2 (acey-deucey) se joacă, apoi alegi orice dublă și mai arunci; dublele dau patru mutări. Scoaterea e exactă când toate sunt acasă. Mutări: ent-3, 5-9, 20-off, dublet 4.",
            make: || Box::new(aceydeucey::new()),
        },
        Playable {
            id: "barbilliards",
            title: "Bar billiards",
            category: "Clasic",
            rules: "Masa de crâșmă cu găuri și popici, simulată pe lovituri, pentru doi: găurile valorează 10, 20, 30, 50, 100, 200; bila roșie dublează punctele. O vizită continuă cât timp loviturile punctează; ratarea o încheie (punctele rămân); un fault (popică doborâtă) o încheie și anulează punctele vizitei; popica neagră anulează tot scorul. 8 vizite fiecare; lovitura se hotărăște la pasul forțat lovitura. Mutări: 10 alba, 100 rosie, lovitura, stop.",
            make: || Box::new(barbilliards::BarBilliards::new()),
        },
        Playable {
            id: "tabula",
            title: "Tabula",
            category: "Clasic",
            rules: "Strămoșul roman al tablelor: 15 piese fiecare pornesc din afara tablei, 3 zaruri, iar ambii jucători merg în același sens pe cele 24 de puncte. Intri, muți, apoi scoți exact; câștigă cine scoate primul toate piesele. Mutări: ent-3, 5-9, 20-off.",
            make: || Box::new(tabula::new()),
        },
        Playable {
            id: "petitschevaux",
            title: "Jeu des petits chevaux",
            category: "Clasic",
            rules: "Ludo francez cu cai: 4 cai fiecare, pistă de 56 de câmpuri și 6 câmpuri de final. Un 6 scoate un cal din grajd și dă încă o aruncare; câmpul tău de start e sigur; aterizând pe un cal adversar îl trimiți înapoi; sosirea cere aruncare exactă. Mutări: intra sau numărul progresului calului (ex. 12).",
            make: || Box::new(petitschevaux::new()),
        },
        Playable {
            id: "coganh",
            title: "Cờ gánh",
            category: "Clasic",
            rules: "Joc vietnamez de capturi pe tabla 5×5 cu diagonale; 8 piese fiecare. Muți un pas pe o linie către un punct liber. Capturile schimbă culoarea pieselor: gánh (piesa mutată încadrează o piesă adversă pe o linie) și vây (un grup advers fără niciun punct liber în jur). Câștigi lăsând adversarul fără piese sau mutări. Mutări: a1b2 (de la, la).",
            make: || Box::new(coganh::CoGanh::new()),
        },
        Playable {
            id: "petteia",
            title: "Petteia",
            category: "Clasic",
            rules: "Reconstituire a jocului grecesc antic pe 8×8: 16 piese fiecare, orice piesă merge ca un turn; o piesă e capturată când mutarea ta o prinde între două ale tale pe un rând sau o coloană (mai multe capturi într-o mutare). Pierzi cu o singură piesă sau fără mutări; după 60 de mutări fără captură câștigă cine are mai multe piese. Mutări: a2a4 (de la, la).",
            make: || Box::new(petteia::Petteia::new()),
        },
        Playable {
            id: "morra",
            title: "Morra (Mora)",
            category: "Clasic",
            rules: "Joc italian cu degete: ambii arată pe ascuns 1–5 degete și strigă suma presupusă; dacă exact unul ghicește, ia un punct, altfel runda se reia. Alegerea e simultană (ascunsă până la dezvăluire); primul la 7. Mutări: 3/7 (3 degete, suma presupusă 7), dezvaluie.",
            make: || Box::new(morra::Morra::new()),
        },
        Playable {
            id: "sanma",
            title: "Sanma (mahjong în trei, adaptat la doi)",
            category: "Clasic",
            rules: "Mahjong riichi cu schimbările sanma: 108 piese (fără 2m–8m), fără chi, piesele Nord pot fi declarate kita (dora bonus, tragere de înlocuire), indicatorul de dora face cerc 1m↔9m. Riichi, ron, tsumo, furiten și punctaj ca la riichi. Mutări: trage, 5m, riichi 5p, tsumo, ankan, kakan, kita, trage-moarta, ron, pon, kan, pas, urmatoarea.",
            make: || Box::new(sanma::Sanma::new()),
        },
        Playable {
            id: "flipcup",
            title: "Flip Cup",
            category: "Party",
            rules: "Duel de relee de petrecere, fără băutură: două echipe de câte 5 trebuie să termine fiecare 5 răsturnări de pahar. Alegi repede (55% reușită, costă 2 timp) sau atent (80%, costă 3); la ratare se repetă, timpul se pierde. Joacă echipa cu ceasul mai mic; câștigă echipa care termină cele 5 răsturnări cu cel mai mic timp. Rezultatul se hotărăște la pasul forțat rastoarna. Mutări: repede, atent, rastoarna.",
            make: || Box::new(flipcup::FlipCup::new()),
        },
        Playable {
            id: "madgab",
            title: "Mad Gab",
            category: "Party",
            rules: "Puzzle fonetic cu cuvinte, în română, pentru doi: un șir de silabe fără sens sună, citit repede cu voce tare, ca o expresie obișnuită. Alegi expresia corectă dintre 4 (ghici 1…ghici 4), eventual după ajutor (se vede primul cuvânt, puncte mai puține). Corect = 2 puncte (1 cu ajutor); greșit = celălalt are o singură încercare pentru 1 punct. 8 runde; verific la final. Mutări: ghici N, ajutor, verific.",
            make: || Box::new(madgab::MadGab::new()),
        },
        Playable {
            id: "pubquiz",
            title: "Pub quiz",
            category: "Party",
            rules: "Seară de întrebări pentru două echipe: 4 runde de câte 5 întrebări cu 4 variante, o altă categorie în fiecare rundă; un răspuns corect = 1 punct. Înaintea fiecărei runde o echipă poate juca jokerul (o dată pe meci; dublează punctele rundei) sau refuza. Răspunsurile rămân secrete până la verific; scor egal = remiză. Mutări: joker, fara joker, raspund a…raspund d, verific.",
            make: || Box::new(pubquiz::PubQuiz::new()),
        },
        Playable {
            id: "oldmaid",
            title: "Old Maid (Bătrâna fată)",
            category: "Cărți",
            rules: "Bătrâna fată pentru doi: pachet de 52 de cărți din care lipsește o damă, deci o damă rămâne fără pereche. Se aruncă perechile de aceeași valoare; apoi tragi pe nevăzute din mâna celuilalt (trag N = a N-a carte) și arunci perechile. Când rămâne o carte, cine o ține (dama) pierde. Mâini ascunse; rezultatul apare la pasul final. Mutări: trag N, final.",
            make: || Box::new(oldmaid::OldMaid::new()),
        },
        Playable {
            id: "mus",
            title: "Mus",
            category: "Cărți",
            rules: "Joc spaniol cu 40 de cărți, adaptat pentru doi: 4 cărți fiecare; 3 valorează ca regii, 2 ca asul. Mus opțional (arunci și tragi), apoi patru licitații: grande, chica, pares, juego; paso sau envido (miza 2), celălalt răspunde quiero / no quiero. Primul la 20. Mutări: mus, corto, descarta 0101, reparte, declara, paso, envido, quiero, no quiero, dezvaluie.",
            make: || Box::new(mus::Mus::new()),
        },
        Playable {
            id: "bezique",
            title: "Bézique",
            category: "Cărți",
            rules: "Joc francez pentru doi cu levate și figuri, pachet de 64 de cărți (7–A dublu), 8 cărți fiecare, atu întors. Cât timp mai e pachet, câștigătorul levatei poate declara o figură (căsătorie 20/40, secvență 250, careu 40–100, bézique 40, dublu bézique 500; dix = schimbi 7 de atu pentru 10), apoi ambii trag; după aceea răspunzi la culoare și bați. Brisques (ași și zeci) și ultima levată dau 10. Primul la 1000 (cel mult 4 mâini). Mutări: Qs, mariaj s, secventa, patru K, bezic, dublubezic, dix, fara, trag.",
            make: || Box::new(bezique::Bezique::new()),
        },
        Playable {
            id: "twentynine",
            title: "Twenty-Nine (29)",
            category: "Cărți",
            rules: "Joc sud-asiatic de levate cu licitație, adaptat pentru doi: 32 de cărți, 8 fiecare; valet 3, nouă 2, as 1, zece 1 puncte. Licitezi 10–20 de puncte, câștigătorul alege atuul și celălalt conduce; răspunzi la culoare dacă poți. Contractul făcut = 1 punct de meci; primul la 5. Mâini ascunse. Mutări: licitez 12, pas, atu s, Jh.",
            make: || Box::new(twentynine::TwentyNine::new()),
        },
        Playable {
            id: "tute",
            title: "Tute",
            category: "Cărți",
            rules: "Joc spaniol de levate cu 40 de cărți pentru doi (o mână): 6 cărți fiecare, atu întors. Cât timp mai e pachet joci orice carte; apoi trebuie să răspunzi la culoare și să bați dacă poți. Câștigătorul poate anunța canto (rege+cal: 20, la atu 40), cambio (schimbi 7-ul de atu) sau tute (patru regi sau patru cai: câștigi pe loc). Ultima levată 10. Mutări: 3o, 12b, canto e, cambio, tute, trag.",
            make: || Box::new(tute::Tute::new()),
        },
        Playable {
            id: "sotron",
            title: "Șotron (Hopscotch)",
            category: "Familie",
            rules: "Joc de copii pentru doi: cursă pe pătratele 1–10. La tura ta arunci piatra: prudent (țintești pătratul următor: 80% reușită, +1) sau indraznet (peste un pătrat: 50%, +2; la ratare cazi un pătrat înapoi). Câștigă primul care duce piatra la 10 și se întoarce. Aruncarea se hotărăște la pasul forțat sar. Mutări: prudent, indraznet, sar.",
            make: || Box::new(sotron::Sotron::new()),
        },
        Playable {
            id: "duckduckgoose",
            title: "Duck Duck Goose",
            category: "Familie",
            rules: "Joc de cerc pentru copii, ca duel de risc pentru doi: în fiecare rundă un jucător atinge, celălalt e gâsca. Cel care atinge alege câte rațe spune (1–5): cu cât mai multe, cu atât are șanse mai mici să fie prins și mai multe puncte dacă scapă. Cursa se hotărăște la pasul forțat fuga; 10 runde, câștigă cine are mai multe puncte. Mutări: rate 1…rate 5, fuga.",
            make: || Box::new(duckduckgoose::DuckDuckGoose::new()),
        },
        Playable {
            id: "statuile",
            title: "Statuile (Statues)",
            category: "Familie",
            rules: "Joc de înghețat pentru copii, pentru doi: fiecare alergător trebuie să ajungă la linia 12; la rundele impare aleargă J1, la cele pare J2. Alergătorul alege în ascuns 1–3 pași, cel care strigă alege în ascuns când strigă stop (1–3); dacă face mai mulți pași decât a strigat, e prins în mișcare și se întoarce la start. Dezvăluirea e pasul forțat dezvaluie; primul la 12 câștigă. Mutări: 1, 2, 3, dezvaluie.",
            make: || Box::new(statuile::Statuile::new()),
        },
        Playable {
            id: "conkers",
            title: "Conkers",
            category: "Familie",
            rules: "Joc britanic de curte pentru doi: fiecare castană are 6 puncte de rezistență. La tura ta alegi usor (70% reușită, 1 pagubă) sau tare (45% reușită, 2 pagube; la ratare sfoara se poate încurca și pierzi 1). Prima castană ajunsă la 0 se sparge și pierde; după 60 de lovituri câștigă castana mai întreagă. Rezultatul se hotărăște la pasul forțat lovitura. Mutări: usor, tare, lovitura.",
            make: || Box::new(conkers::Conkers::new()),
        },
        Playable {
            id: "craps",
            title: "Craps",
            category: "Zaruri",
            rules: "Duel la masa de zaruri pentru doi: shooterul alege miza (5, 10, 20) și pariul, pass sau dont (don't pass); celălalt acoperă pariul opus. Come-out: 7 sau 11 câștigă pass; 2, 3, 12 = craps (don't pass câștigă la 2 și 3, la 12 miza se anulează); orice alt total devine punct, se aruncă până apare punctul (pass) sau 7 (don't pass). 16 runde, start 100 de jetoane, câștigă cine are mai multe. Mutări: pass 10, dont 5, arunc.",
            make: || Box::new(craps::Craps::new()),
        },
        Playable {
            id: "mexico",
            title: "Mexico (Mäxchen, Mia)",
            category: "Zaruri",
            rules: "Joc de bluf cu două zaruri și cupă: arunci ascuns și anunți o valoare strict mai mare decât ultimul anunț (31 cea mai mică … dublele … 21 «Mexico» cea mai mare). Următorul crede (și aruncă) sau contestă: mincinosul pierde o viață, contestatarul greșit tot una (la Mexico două). Câte 5 vieți. Mutări: arunc, anunt 65, cred, contest, dezvaluie.",
            make: || Box::new(mexico::Mexico::new()),
        },
        Playable {
            id: "schocken",
            title: "Schocken",
            category: "Zaruri",
            rules: "Joc german cu trei zaruri și 13 jetoane: până la trei aruncări pe rundă (păstrezi zarurile alese); clasament Schock Out (1-1-1) > Schock (1-1-x) > General > Straße > obișnuită (cel mai mare zar întâi). Cine pierde runda ia jetoane după valoarea mâinii câștigătoare; o repriză se termină când cineva ține toate cele 13 și o pierde; două reprize pierdute pierd meciul. Mutări: arunc, stop, re 13 (re-arunci zarurile 1 și 3), decontare.",
            make: || Box::new(schocken::Schocken::new()),
        },
        Playable {
            id: "passedix",
            title: "Passe-dix",
            category: "Zaruri",
            rules: "Joc francez cu trei zaruri pentru doi: banca (alternativ) alege miza (5, 10, 20) și partea, passe (total 11–18) sau manque (3–10); celălalt acoperă partea opusă; tripletele se re-aruncă. 20 de runde, start 100 de jetoane, câștigă cine are mai multe. Aruncarea e pasul forțat arunc. Mutări: passe 10, manque 5, arunc.",
            make: || Box::new(passedix::PasseDix::new()),
        },
        Playable {
            id: "dicebaseball",
            title: "Dice Baseball",
            category: "Zaruri",
            rules: "Baseball cu două zaruri pentru doi, fără decizii: 9 reprize, J1 bate apoi J2 până la trei outuri; reprize suplimentare (cel mult 12) la egalitate. Totalul fiecărei aruncări e un rezultat: 2 aruncare ratată, 3 triplu, 4 dublu, 5 walk, 6 fly out, 7 ground out, 8 simplu, 9 strike out, 10 minge afară (se reia), 11 dublă eliminare, 12 home run. Alergătorii avansează la lovituri; trecerea de baza a treia înscrie. Mutare: arunc.",
            make: || Box::new(dicebaseball::DiceBaseball::new()),
        },
        Playable {
            id: "dropdead",
            title: "Drop Dead",
            category: "Zaruri",
            rules: "Joc cu cinci zaruri și risc pentru doi, fără decizii: la tura ta arunci toate zarurile rămase; dacă nu apare niciun 2 sau 5, adaugi totalul la scor; altfel nu punctezi, iar zarurile cu 2 și 5 «cad». Tura se termină când au căzut toate cele cinci zaruri. 5 ture fiecare, câștigă scorul mai mare. Mutare: arunc.",
            make: || Box::new(dropdead::DropDead::new()),
        },
        // Q2-BEGIN
        Playable {
            id: "dame10",
            title: "Dame internaționale (10×10)",
            category: "Abstract",
            rules: "Tablă 10×10, 20 de piese pe jucător. Piesele simple mută un pas în diagonală înainte și capturează înainte și înapoi; damele zboară pe orice distanță. Captura e obligatorie, cu cele mai multe piese; piesele luate se scot la final. O piesă devine damă doar dacă își încheie mutarea pe ultimul rând. Remiză după 50 de plie doar cu dame sau 400 în total. Mutări: c4-d5, c4xe6xg8.",
            make: || Box::new(dame10::Dame10::new()),
        },
        Playable {
            id: "toguz",
            title: "Toguz Kumalak (Kazahstan)",
            category: "Abstract",
            rules: "2 × 9 gropi cu câte 9 bile și un depozit (kazan) pentru fiecare. Iei toate bilele dintr-o groapă proprie: prima rămâne, restul se seamănă câte una în sens trigonometric (o singură bilă trece mai departe). Dacă ultima bilă face o groapă a adversarului pară, o iei; dacă o face 3 (nu groapa 9, una per jucător, nu cu același număr ca a adversarului), groapa devine tuzdyk: orice bilă semănată acolo ajunge în depozitul tău. Primul la 82 câștigă. Mutări: 1–9.",
            make: || Box::new(toguz::Toguz::new()),
        },
        Playable {
            id: "sittuyin",
            title: "Sittuyin (șahul birman)",
            category: "Abstract",
            rules: "Tablă 8×8; cei 8 pioni sunt deja pe a3 b3 c3 d4 e4 f3 g3 h3 / a6 b6 c6 d5 e5 f6 g6 h6. Întâi fiecare își așază regele, generalul, 2 elefanți, 2 cai și 2 bărci în jumătatea proprie (K@e1). Apoi se mută: general = un pas în diagonală, elefant = un pas în diagonală sau înainte, cal ca în șah, barcă = turn; un pion aflat pe o diagonală mare în jumătatea adversă poate deveni general (d5=G) dacă nu mai ai. Mat câștigă; patul e interzis. Mutări: K@e1, e3-e4, e4xd5, d5=G.",
            make: || Box::new(sittuyin::Sittuyin::new()),
        },
        Playable {
            id: "gebeta",
            title: "Gebeta (Etiopia)",
            category: "Abstract",
            rules: "2 × 6 gropi cu câte 4 semințe, câte un depozit. Iei toate semințele dintr-o groapă proprie și le semeni în sens trigonometric. Dacă ultima cade într-o groapă ocupată, ridici tot ce e acolo și continui (lanț). Dacă cade într-o groapă goală a ta și cea opusă are semințe, iei semințele opuse plus a ta. Când o parte rămâne goală jocul se termină și fiecare își ia semințele de pe partea lui; câștigă cine are mai multe. Mutări: 1–6.",
            make: || Box::new(gebeta::Gebeta::new()),
        },
        Playable {
            id: "sprouts",
            title: "Sprouts (Conway–Paterson)",
            category: "Abstract",
            rules: "Joc topologic, 4 puncte la start. La rândul tău unești două puncte (sau un punct cu el însuși, printr-o buclă) cu o curbă care nu taie nimic, apoi pui un punct nou pe curbă. Din fiecare punct pleacă cel mult 3 curbe. Cine nu mai poate muta pierde. Poziția e dată combinatoriu (regiuni și frontiere); mutări: a-b (puncte), a-a pentru buclă, cu .1/.2 când există mai multe variante.",
            make: || Box::new(sprouts::Sprouts::new()),
        },
        Playable {
            id: "pegsol",
            title: "Peg solitaire (duel)",
            category: "Clasic",
            rules: "Tabla engleză cu 33 de găuri, 32 de bile, centrul liber. O mutare: o bilă sare ortogonal peste o vecină într-o gaură liberă și bila sărită se scoate. Duel: fiecare joacă propria tablă identică, pe rând; cine rămâne fără mutări iese din joc. La final câștigă cine are mai puține bile (egal: ultima bilă în centru). Mutări: d2-d4.",
            make: || Box::new(pegsol::PegSolitaire::new()),
        },
        Playable {
            id: "shuffleboard",
            title: "Shuffleboard de masă",
            category: "Clasic",
            rules: "Alunecă pucuri pe un traseu de 96 de unități: zonele 60 (1 p), 70 (2), 80 (3), 90–95 (4, agățat); de la 96 cad. Câte 4 pucuri pe rundă, pe rând. Alegi forța (tir 60), apoi «lansez»: pucul ajunge la ±5 de țintă; pucurile lovite transmit mișcarea. Scorează doar jucătorul cu pucul cel mai departe, cu pucurile lui aflate dincolo de cel mai bun al adversarului. Primul la 15. Mutări: tir 30…100, lansez.",
            make: || Box::new(shuffleboard::Shuffleboard::new()),
        },
        Playable {
            id: "yatzy",
            title: "Yatzy (Scandinavia)",
            category: "Zaruri",
            rules: "5 zaruri, 3 aruncări pe tură, 15 categorii: unu…sase (bonus 50 dacă suma ≥ 63), pereche, două perechi, trei/patru la fel, scara mică 1-5 (15), scara mare 2-6 (20), casă, șansă, yatzy (50). Aruncarea e mutare separată; ții zaruri (tine 3 3 5) sau scrii într-o categorie. Câștigă totalul cel mai mare. Mutări: arunc, tine 3 3 5, scor pereche.",
            make: || Box::new(yatzy::Yatzy::new()),
        },
        Playable {
            id: "ceelo",
            title: "Cee-lo (zaruri de stradă)",
            category: "Zaruri",
            rules: "Trei zaruri. 4-5-6 sau trei egale câștigă pe loc; 1-2-3 pierde pe loc; o pereche plus un zar diferit dă acel zar ca punct; orice altceva se rearuncă. Pariorul alege miza (5/10/20); bancherul aruncă până iese un rezultat, apoi pariorul; punctul mai mare câștigă, egal = remiză. Banca trece când bancherul pierde. 16 mâini, 50 de jetoane. Mutări: pariez 10, arunc.",
            make: || Box::new(ceelo::CeeLo::new()),
        },
        Playable {
            id: "oversevens",
            title: "Peste și sub șapte (Over and Under Seven)",
            category: "Zaruri",
            rules: "Pariorul alege «sub» (2–6), «peste» (8–12), plătite 1:1, sau «sapte» (exact 7), plătit 4:1, cu miza 5/10/20; banca (celălalt jucător) aruncă două zaruri («arunc»). Banca se schimbă la fiecare mână. 20 de mâini, 50 de jetoane; câștigă cine are mai multe. Mutări: sub 5, peste 10, sapte 5, arunc.",
            make: || Box::new(oversevens::OverUnder::new()),
        },
        Playable {
            id: "skunk",
            title: "Skunk (zaruri)",
            category: "Zaruri",
            rules: "Joc de zaruri «împinge-ți norocul», 5 runde. În tură arunci două zaruri cât vrei (arunc) sau te oprești (opresc). Fără 1: adaugi suma la tură. Un singur 1 (skunk): pierzi punctele turei. Doi de 1: pierzi tot totalul. Confirmi skunk-ul cu ok. Câștigă totalul cel mai mare după 5 runde.",
            make: || Box::new(skunk::Skunk::new()),
        },
        Playable {
            id: "hazard",
            title: "Hazard (strămoșul craps-ului)",
            category: "Zaruri",
            rules: "Aruncătorul strigă un main 5–9 și o miză. Prima aruncare cu două zaruri: main = nick (câștigă); 2 sau 3 = pierde; 11 și 12 depind de main (5/9: ambele pierd; 6/8: 11 pierde, 12 nick; 7: 11 nick, 12 pierde); orice altceva devine chance. Apoi se aruncă până iese chance (câștigă) sau main (pierde). Zarurile trec după trei pierderi la rând. 100 de jetoane, 20 de mâini. Mutări: main 7 10, arunc.",
            make: || Box::new(hazard::Hazard::new()),
        },
        Playable {
            id: "howzat",
            title: "Crichet cu zaruri (Howzat!)",
            category: "Zaruri",
            rules: "Cricket cu zaruri pentru doi: 2 reprize de 30 de mingi și 5 wicket-uri; J1 bate primul, J2 urmărește. La fiecare minge bătătorul alege «lovesc» (atac) sau «apar» (apărare), apoi aruncătorul dă «arunc»: atac 1–2 out, 3=1, 4=2, 5=4, 6=6 runs; apărare 1 out, 2–3=0, 4–5=1, 6=2 runs. Câștigă cine are mai multe runs. Mutări: lovesc, apar, arunc.",
            make: || Box::new(howzat::Howzat::new()),
        },
        Playable {
            id: "dreidel",
            title: "Dreidel",
            category: "Clasic",
            rules: "Joc de noroc cu titirezul. Fiecare începe cu 10 jetoane și pune 1 în pot (din nou când potul are 0–1 jetoane). Pe rând se învârte («invart»): nun = nimic, gimel = iei tot potul, hei = iei jumătate, shin = pui 1. Cine nu mai poate plăti pierde; după 300 de învârtiri câștigă cine are mai multe jetoane.",
            make: || Box::new(dreidel::Dreidel::new()),
        },
        Playable {
            id: "parimpar",
            title: "Pare sau impare",
            category: "Clasic",
            rules: "J1 este «impar», J2 este «par». La fiecare rundă fiecare alege în secret 1–5 degete (n1…n5), apoi se arată (arat). Suma impară dă runda lui J1, cea pară lui J2. Primul la 5 runde câștigă. Alegerea ta rămâne ascunsă până la arătare.",
            make: || Box::new(parimpar::ParImpar::new()),
        },
        Playable {
            id: "mehen",
            title: "Mehen (Egiptul antic)",
            category: "Clasic",
            rules: "Reconstrucție. Un traseu-șarpe de 18 pătrate (coada 0, capul 17), 3 piese fiecare. Se aruncă patru bețe (0–4 fețe luminoase, 0 = 6; aruncările 1, 4 și 6 dau încă o aruncare). O piesă intră pe coadă doar cu 1, merge spre cap și se întoarce pe același traseu (35 de pași), apoi iese acasă. Aterizezi pe adversar = îl trimiți afară; pe piesa ta nu poți ateriza. Câștigă primul cu toate 3 piesele acasă. Mutări: arunc, intra, m1/m2/m3, pas.",
            make: || Box::new(mehen::Mehen::new()),
        },
        Playable {
            id: "cinci_pietricele",
            title: "Cinci pietricele",
            category: "Copii",
            rules: "Jocul copiilor cu cinci pietricele, cu norocul în locul îndemânării. 5 etape (uni, doi, trei, patru, tot). În tură alegi «calm» (70 %, +1 etapă) sau «iute» (45 %, +2 etape), apoi «incerc». O reușită te lasă să continui; o greșeală dă rândul. Primul care termină etapa 5 câștigă.",
            make: || Box::new(cinci_pietricele::CinciPietricele::new()),
        },
        Playable {
            id: "jocuri_ata",
            title: "Jocuri cu ață (leagănul pisicii)",
            category: "Copii",
            rules: "Doi jucători își pasează figuri de ață în ordinea clasică: leagăn → patul soldatului → lumânări → ieslea → diamante → ochi de pisică → pește în farfurie → ceas → leagăn. Alegi «atent» (85 %, 1 punct) sau «rapid» (60 %, 2 puncte), apoi «preiau». Ața încurcată costă 1 punct și revine la leagăn; închiderea ciclului dă 3 puncte bonus. Primul la 12 câștigă.",
            make: || Box::new(jocuri_ata::JocuriAta::new()),
        },
        Playable {
            id: "skittles",
            title: "Table skittles (popice de masă)",
            category: "Clasic",
            rules: "Popice de masă cu norocul în locul îndemânării: 9 popice într-o grilă 3×3. La fiecare aruncare țintești o coloană (stanga/mijloc/dreapta), apoi «lansez»; popicele din coloana țintă cad cu 60 %, vecine 25 %, opuse 8 %. O tură are 3 aruncări; fiecare popică 1 punct (cea din mijloc 2); toate nouă = 5 bonus. 6 ture fiecare; câștigă cine are mai multe puncte.",
            make: || Box::new(skittles::Skittles::new()),
        },
        Playable {
            id: "doppelkopf",
            title: "Doppelkopf (2 jucători)",
            category: "Cărți",
            rules: "Adaptare pentru doi a jocului normal: 48 de cărți (9 10 J Q K A ×4 culori ×2), câte 24. Atuuri de la mare la mic: 10♥ (Dulle), Q♣ Q♠ Q♥ Q♦, J♣ J♠ J♥ J♦, A♦ 10♦ K♦ 9♦; culorile ♣ ♠ ♥ au ordinea A 10 K 9. Respecți culoarea sau atuul; din două cărți egale câștigă prima (a doua 10♥ o bate pe prima). Peste 120 din 240 de ochi câștigă mâna; primul la 2 mâini. Mutări: Qc, 10h, Ad, imparte (c=♣ s=♠ h=♥ d=♦).",
            make: || Box::new(doppelkopf::Doppelkopf::new()),
        },
        Playable {
            id: "tarock",
            title: "Tarock (2 jucători)",
            category: "Cărți",
            rules: "Adaptare pentru doi, fără licitație. 54 de cărți: 22 de tarocuri (t1 … t21 și Škis) + 4 culori × 8; câte 18, 18 rămân nefolosite. Respecți culoarea, altfel ești obligat să joci un tarok; tarokul bate orice carte de culoare, tarokul mai mare câștigă. Škis (skis) se joacă oricând, nu câștigă niciodată și rămâne al celui care l-a jucat. Puncte: K 5, Q 4, C 3, J 2, restul culorilor 1; tarocurile I, XXI, Škis 5, celelalte 1; ultima levată câștigată cu I dă +10. Primul la 2 mâini. Mutări: t7, skis, Kh, 10s, 1d, imparte.",
            make: || Box::new(tarock::Tarock::new()),
        },
        Playable {
            id: "sevens",
            title: "Șapte (Sevens)",
            category: "Cărți",
            rules: "Șapte în doi: câte 26 de cărți. Un 7 deschide culoarea la mijloc; apoi construiești în sus și în jos, cu o carte din aceeași culoare lipită de șir. Dacă nu poți juca, treci. Câștigă cine rămâne primul fără cărți. Mutări: cartea (ex. 7h) sau pas. Mâini ascunse.",
            make: || Box::new(sevens::Sevens::new()),
        },
        Playable {
            id: "skitgubbe",
            title: "Skitgubbe",
            category: "Cărți",
            rules: "Joc suedez de descărcare în două faze. Faza 1: cu pachet, joci o carte, adversarul trebuie să o bată cu una mai mare din aceeași culoare sau ia grămada; completezi la trei cărți. Faza 2: se joacă mâinile rămase, fără pachet; cine rămâne ultimul cu cărți e Skitgubbe (pierzătorul). Mutări: cartea, ridic (ia grămada), risc (riști o carte din pachet). Mâini și pachet ascunse.",
            make: || Box::new(skitgubbe::Skitgubbe::new()),
        },
        Playable {
            id: "ombre",
            title: "Ombre (L'Hombre)",
            category: "Cărți",
            rules: "Ombre în doi cu pachetul spaniol de 40 de cărți: câte 9 cărți. Licitezi (entrada) sau treci; Ombre alege atuul, poate arunca și trage cărți, apoi se joacă 9 levate. Matadorii (atuurile cele mai mari) bat restul. Ombre câștigă dacă ia mai multe levate decât adversarul. Primul la 8 puncte. Mutări: entrada, pas, atu <culoare>, scap <cărți>, gata, cartea, imparte. Mâini ascunse.",
            make: || Box::new(ombre::Ombre::new()),
        },
        Playable {
            id: "domino9",
            title: "Dominó cubano (doble nueve)",
            category: "Clasic",
            rules: "Set dublu-nouă de 55 de piese; câte 10, 35 în rezervă. Pui o piesă care se potrivește la un capăt (începe cel cu cel mai mare dublu); dacă nu ai piesă, tragi până poți, iar cu rezerva goală treci. Câștigă cine rămâne primul fără piese; la blocaj, cine are mai puține puncte. Mutări: piesa cu capătul (ex. 5-7 stanga), trage, pas. Mâini și rezervă ascunse.",
            make: || Box::new(domino9::Domino9::new()),
        },
        Playable {
            id: "narde",
            title: "Narde scurtă (Short Nardy)",
            category: "Clasic",
            rules: "Narde scurtă (rudă cu table): câte 15 piese, două zaruri (la dublă se joacă de patru ori), aduci toate piesele acasă și le scoți primul. Implementarea folosește motorul de table cu regulile standard. Mutări: arunc, apoi mutările pe puncte.",
            make: || Box::new(narde::Narde::new()),
        },
        Playable {
            id: "dog",
            title: "Dog (curse cu cărți)",
            category: "Clasic",
            rules: "Cursă pe inel cu cărți pentru doi: inel de 64 de câmpuri, 4 piese, 110 cărți. A sau K scot o piesă; cărțile mută piese după valoare (A = 1 sau 11, 4 = înainte sau înapoi, 7 = șapte pași de câte 1, J = schimb cu o piesă a adversarului, joker = orice carte). Aterizând exact pe adversar îl trimiți înapoi; piesa de pe start e protejată. Câștigă cine aduce primul toate cele 4 piese în casă. Mutări: K:intra, 5:p2, 4:p1-, 7:p1, J:p1<>o2, pas. Mâini ascunse.",
            make: || Box::new(dog::Dog::new()),
        },
        Playable {
            id: "toc",
            title: "Toc (jeu)",
            category: "Clasic",
            rules: "Cursă franceză cu cărți și popice pentru doi: inel de 80 de câmpuri, 4 piese, 54 de cărți. A sau K scot o piesă; cărțile mută după valoare; 4 merge numai înapoi; 7 = șapte pași de câte 1; valetul mută 11 sau schimbă; jokerul mută 15 sau scoate o piesă. Aterizând exact pe adversar îl trimiți înapoi; piesa de pe start e protejată. Câștigă cine aduce primul toate cele 4 piese în casă. Mâini ascunse.",
            make: || Box::new(toc::Toc::new()),
        },
        Playable {
            id: "trictrac",
            title: "Trictrac (simplificat)",
            category: "Clasic",
            rules: "Trictrac simplificat: 24 de puncte, câte 15 piese stivuite pe punctul de start, două zaruri, fără lovire. Scoți piese doar când toate sunt în ultimul sfert. Punctaj în găuri la sfârșitul turei: dublă +2, trei puncte din ultimul sfert cu 2+ piese +4, fiecare piesă scoasă +1. Primul la 15 găuri câștigă. Sistemul real de găuri (jan-uri, battre à vrai) nu e implementat. Mutări: arunc, zar:punct (ex. 4:1).",
            make: || Box::new(trictrac::Trictrac::new()),
        },
        Playable {
            id: "tangram",
            title: "Tangram (duel pe grilă)",
            category: "Clasic",
            rules: "Duel Tangram simplificat pe grilă 6×6: fiecare jucător are 7 piese (2 triunghiuri mici = 1 pătrățel, triunghi mediu = domino, pătrat = 2×2, paralelogram = S de 4, 2 triunghiuri mari = L de 3). Puneți alternativ câte o piesă (orice rotire sau oglindire), fără suprapunere; cine nu mai poate pune nicio piesă pierde. Mutări: piesă+orientare@colț (ex. q1@c3). Nu e puzzle-ul cu siluete.",
            make: || Box::new(tangram::Tangram::new()),
        },
        Playable {
            id: "zipzapzop",
            title: "Zip Zap Zop",
            category: "Party",
            rules: "Joc de numărat: numărați pe rând de la 1 la 30; spuneți zip la multiplii de 3, zap la multiplii de 5, zop la multiplii de 15. Un cuvânt greșit pierde. Fiecare are 2 pas-uri (numărul rămâne adversarului). Câștigă cine spune corect 30. Mutări: numărul, zip, zap, zop, pas.",
            make: || Box::new(zipzapzop::ZipZapZop::new()),
        },
        Playable {
            id: "wouldyourather",
            title: "Ai prefera…",
            category: "Party",
            rules: "Șase dileme. La fiecare, fiecare alege în secret răspunsul propriu (a sau b) și ghicește răspunsul celuilalt: mutări aa, ab, ba, bb (prima literă = răspunsul tău, a doua = ghicitul tău), apoi arat. Fiecare ghicit corect = 1 punct; mai multe puncte câștigă. Alegeri ascunse.",
            make: || Box::new(wouldyourather::WouldYouRather::new()),
        },
        Playable {
            id: "chinesewhispers",
            title: "Telefonul fără fir",
            category: "Party",
            rules: "Versiune în duel pentru doi: trimițătorul vede un mesaj de 4 cifre și îl lasă (las) sau strică o cifră cu ±1 (strica 2+). Receptorul vede mesajul ajuns și suma de control (suma cifrelor originale mod 10) și arată cifra stricată (restaur 1–4) sau zice restaur 0. Restaurarea corectă aduce punct receptorului, altfel trimițătorului. 6 runde, rolurile alternează. Originalul e ascuns.",
            make: || Box::new(chinesewhispers::ChineseWhispers::new()),
        },
        Playable {
            id: "namethattune",
            title: "Ghicește melodia",
            category: "Party",
            rules: "Șase melodii din domeniul public date ca nume de note. Licitezi de câte note ai nevoie (ofer N); adversarul coboară oferta sau cere dovada (ghicesc). Cel mai mic licitator aude doar N note și alege titlul dintre 4 variante (o1–o4), apoi arat. Corect: +1 pentru el, greșit: +1 pentru celălalt. Mai multe puncte câștigă. Titlul corect rămâne ascuns până la arat.",
            make: || Box::new(namethattune::NameThatTune::new()),
        },
        Playable {
            id: "babaoarba",
            title: "Baba-oarba",
            category: "Copii",
            rules: "Duel asimetric pe tablă 7×7: J1 e baba cu ochii legați, J2 fugarul. Baba mută (n, s, e, v, ne…, sta) sau încearcă să prindă o căsuță vecină (prind c4, apoi arat). După fiecare rundă aude un indiciu: cald (fugarul e la cel mult 2 căsuțe) sau rece. Fugarul face un pas. Baba câștigă prinzând fugarul; fugarul câștigă rezistând 30 de runde. Poziția fugarului e ascunsă.",
            make: || Box::new(babaoarba::BabaOarba::new()),
        },
        Playable {
            id: "capturetheflag",
            title: "Capture the Flag",
            category: "Copii",
            rules: "Tablă 9×7, jumătate de teritoriu fiecare (coloana e e neutră). Câte 3 alergători; un pas ortogonal pe mutare (b2-b3). Un alergător care intră în teritoriul inamic lângă un apărător, sau un intrus lângă piesa ta în teritoriul tău, e atins și stă 3 mutări în închisoare. Ajungi la steagul inamic și îl aduci în teritoriul tău ca să câștigi; purtătorul atins pierde steagul. 160 de mutări: remiză.",
            make: || Box::new(capturetheflag::CaptureTheFlag::new()),
        },
        // Q2-END
        // Q3-PL-BEGIN
        Playable {
            id: "shut_box",
            title: "Închide cutia (Shut the Box)",
            category: "Zaruri",
            rules: "Plăcuțe 1–9; arunci (arunc) două zaruri (unul, dacă 7–9 sunt închise) și închizi plăcuțe cu suma egală (inchid 1+2+6). Când nu mai poți, tura se încheie cu suma plăcuțelor deschise. Cutie închisă = victorie pe loc; 5 runde, cel mai mic total câștigă.",
            make: || Box::new(shut_box::ShutBox::new()),
        },
        Playable {
            id: "chuck",
            title: "Zarul norocos (Chuck-a-luck)",
            category: "Zaruri",
            rules: "Alegi un număr 1–6 și miza 5/10/20 (pariez 3 10), apoi arunci trei zaruri (arunc): fiecare zar egal cu numărul tău plătește miza 1:1, altfel pierzi miza. 12 ture fiecare, 100 de jetoane; câștigă cine are mai multe.",
            make: || Box::new(chuck::Chuck::new()),
        },
        Playable {
            id: "bar_dice",
            title: "Zaruri de bar (Bar dice)",
            category: "Zaruri",
            rules: "Cinci zaruri, cel mult 3 aruncări (arunc); după fiecare păstrezi cel puțin un zar (tin 6 5). Ai nevoie de Vas (6), Căpitan (5) și Echipaj (4); celelalte două zaruri sunt încărcătura (suma lor, 2–12), altfel 0. Încărcătura mai mare ia runda; primul la 4 runde (sau cele mai multe din 9).",
            make: || Box::new(bar_dice::BarDice::new()),
        },
        Playable {
            id: "cosmic",
            title: "Zaruri cosmice (după Cosmic Wimpout)",
            category: "Zaruri",
            rules: "Cinci zaruri cu fețele 2, 3, 4, 5, 10 și Soare (S, joker în triplete). Arunci (arunc), păstrezi zarurile care punctează integral (tin 10 5): 10 = 100, 5 = 50, trei la fel = 100 × fața (trei de 10 = 1000). Apoi continui sau banca; fără zaruri care punctează pierzi punctele turei. Primul la 2000.",
            make: || Box::new(cosmic::Cosmic::new()),
        },
        Playable {
            id: "tombola",
            title: "Tombola (Italia)",
            category: "Clasic",
            rules: "Fiecare alege 1–3 cartele de 3×9 (cartele 2, câte 1 punct); numerele din 1–90 se extrag (extrag) și se bifează automat. Primul rând cu 2/3/4/5 numere ia ambo (2), terno (3), quaterna (4), cinquina (6); tombola (cartelă completă) ia 15 și încheie jocul. Mai multe puncte câștigă.",
            make: || Box::new(tombola::Tombola::new()),
        },
        Playable {
            id: "pachinko",
            title: "Pachinko (Japonia)",
            category: "Clasic",
            rules: "20 de bile fiecare. Lansezi o bilă din coloana 1–9 (lansez 5), apoi cade: coboară prin 8 rânduri de cuie și ajunge într-un buzunar (plătește 3 2 1 2 6 2 1 2 3 bile). Coloanele impare ajung doar în buzunare impare, cele pare în buzunare pare. 15 lansări fiecare; mai multe bile câștigă.",
            make: || Box::new(pachinko::Pachinko::new()),
        },
        Playable {
            id: "fan_tan",
            title: "Fan-Tan (China)",
            category: "Clasic",
            rules: "Banca acoperă un morman de mărgele; pariezi pe «fan» 1–4 (restul la 4, 0 = 4) cu miza 5/10/20 (pun 3 10), apoi descopar. Pariul ghicit plătește 3:1 minus 5% comision, altfel pierzi miza. 10 runde, 100 de jetoane; câștigă cine are mai multe.",
            make: || Box::new(fan_tan::FanTan::new()),
        },
        Playable {
            id: "hanoi",
            title: "Turnurile din Hanoi (Tower of Hanoi)",
            category: "Clasic",
            rules: "Cursă: fiecare are propriul turn cu 5 discuri pe tija a și îl mută pe tija c (a-c = discul de sus de pe a trece pe c), niciodată un disc mare peste unul mic. Mutați pe rând; câștigă cine termină cu mai puține mutări (minim 31; egal = remiză).",
            make: || Box::new(hanoi::Hanoi::new()),
        },
        Playable {
            id: "pingpong",
            title: "Tenis de masă (Ping-pong)",
            category: "Clasic",
            rules: "Un set la 11 (diferență 2). Cine lovește alege zona (st, ct, dr) și forța (sigur, atac, smash): ct-atac; apoi schimb rezolvă lovitura. Forța mare = mai multe mingi ratate, dar mai greu de returnat; adversarul pierde 25% pentru fiecare zonă dintre poziția lui și minge. Serviciul trece la 2 puncte.",
            make: || Box::new(pingpong::PingPong::new()),
        },
        Playable {
            id: "hounds",
            title: "Hounds and Jackals (58 de găuri)",
            category: "Clasic",
            rules: "Reconstrucție modernă (regulile originale s-au pierdut). 5 piese pe jucător; 4 bețe dau 1–5 (arunc). Intri pe gaura 1 doar cu 1 (intra) și arunci iar. Piesa înaintează (p12); scurtături 6→20, 8→10, 20→6, 10→8; nu sari peste piesă proprie de pe 10/15/20/25. Gaura 30 exact; ieși (iese) cu 1 sau 5 după ce toate au intrat.",
            make: || Box::new(hounds::Hounds::new()),
        },
        Playable {
            id: "ataxx",
            title: "Ataxx (7×7)",
            category: "Abstract",
            rules: "Două piese în colțuri opuse. La mutare clonezi o piesă pe o groapă liberă la distanță 1 (a1-b2) sau o sari la distanță 2 (a1-c3); toate piesele adverse vecine cu destinația devin ale tale. Fără mutări: pas. Câștigă cine are mai multe piese când tabla se umple sau nimeni nu mai poate muta.",
            make: || Box::new(ataxx::Ataxx::new()),
        },
        Playable {
            id: "shatranj",
            title: "Shatranj (Persia)",
            category: "Abstract",
            rules: "Strămoșul șahului: ferz (un pas pe diagonală), alfil (sare exact 2 pe diagonală), cal, turn, rege; pionul avansează cu un pas și se promovează în ferz; fără rocadă. Câștigi prin mat, pat sau rege gol (cu excepția contra-capturii). Mutări: e2e3, b1c3.",
            make: || Box::new(shatranj::Shatranj::new()),
        },
        Playable {
            id: "shashki",
            title: "Dame rusești (Shashki)",
            category: "Abstract",
            rules: "8×8: piesele simple mută înainte, dar captură și înapoi; dama zboară pe diagonală și capturează de la distanță. Captura e obligatorie, salturile continuă; piesa care atinge ultimul rând în captură devine damă și continuă. Mutări: c3-d4, c3xe5xg7. Pierde cine nu mai poate muta.",
            make: || Box::new(shashki::Shashki::new()),
        },
        Playable {
            id: "o_an_quan",
            title: "Ô ăn quan (Vietnam)",
            category: "Abstract",
            rules: "Inel de 12 gropi: 5 mici pe latură (5 pietre) și două quan cu mandarin (10 puncte). Alegi o groapă proprie și sensul (3+ = orar, 3- = antiorar), semeni pietrele; următoarea plină se ia și se continuă; după o groapă goală, groapa plină următoare se capturează. Rândul gol: pune5. Câștigă cine adună mai multe puncte.",
            make: || Box::new(o_an_quan::OAnQuan::new()),
        },
        Playable {
            id: "hypergammon",
            title: "Hypergammon",
            category: "Zaruri",
            rules: "Table cu doar trei piese pe jucător, câte una pe punctele 24, 23 și 22; restul regulilor ca la table: lovire, bară, bear-off, dubla se joacă de patru ori. Mutări: 24/20, bar/22, 3/off. Primul care scoate cele 3 piese câștigă.",
            make: || Box::new(hypergammon::Hypergammon::new()),
        },
        Playable {
            id: "jacquet",
            title: "Jacquet (Franța)",
            category: "Clasic",
            rules: "15 piese pe jucător plecând din punctul 1 în colțuri opuse, ambii mută în același sens pe 24 de puncte; fără lovire, un punct ocupat de adversar e blocat. Prima piesă mutată (poștașul) trebuie să ajungă în ultimul sfert înainte să se miște altele. Zaruri automate, dubla de patru ori. Mutări: 1>5, 22>off. Primul care scoate toate piesele câștigă.",
            make: || Box::new(nardy::Nardy::jacquet()),
        },
        Playable {
            id: "moultezim",
            title: "Moultezim (Grecia / Turcia)",
            category: "Clasic",
            rules: "Variantă de table fără lovire: 15 piese pe jucător pornesc din colțuri opuse și merg în același sens pe 24 de puncte; un punct ocupat de adversar e blocat. O piesă trebuie dusă dincolo de punctul de plecare al adversarului (13) înainte de a muta a doua. Bear-off doar din ultimul sfert. Mutări: 1>5, 22>off. Primul care scoate toate piesele câștigă.",
            make: || Box::new(nardy::Nardy::moultezim()),
        },
        Playable {
            id: "chu_shogi",
            title: "Chu Shogi (12×12)",
            category: "Abstract",
            rules: "Șahul japonez mare pe tablă 12×12 cu 46 de piese pe parte (Leu, Regină, Tigru, Elefant ebriu, Căpcăun etc.). Mutări scrise ca traseu: g6-g7, leul poate face doi pași (g6-g7-g8), iar `+` promovează la intrarea în zona adversă. Captura prințului sau a regelui câștigă (versiune simplificată, fără regula de repetare).",
            make: || Box::new(chu_shogi::ChuShogi::new()),
        },
        Playable {
            id: "chowka_bara",
            title: "Chowka Bara",
            category: "Clasic",
            rules: "Joc de cursă indian pe tablă 5×5 cu patru piese pe jucător și patru scoici ca zaruri (4 și 8 dau o tură în plus). O piesă intră în joc doar cu 4 sau 8; piesele fac un tur pe marginea tablei, apoi intră în spirala interioară până la centru. Capturezi piese adverse pe pătratele neprotejate; câștigă cine aduce toate piesele în centru.",
            make: || Box::new(chowka_bara::ChowkaBara::new()),
        },
        Playable {
            id: "uckers",
            title: "Uckers",
            category: "Zaruri",
            rules: "Ludo marinăresc britanic: patru piese pe jucător, un singur zar, piesele ies doar cu 6 și fac un tur de 48 de pătrate, apoi urcă pe coloana de casă. Două piese proprii pe același pătrat formează un bloc (spate-în-spate) care nu poate fi capturat. Un 6 sau o captură dă o aruncare în plus; câștigă cine ajunge primul cu toate piesele acasă.",
            make: || Box::new(uckers::Uckers::new()),
        },
        Playable {
            id: "mau_mau",
            title: "Mau-Mau",
            category: "Cărți",
            rules: "Joc german de eliminare a cărților cu pachet de 32: 5 cărți fiecare; pui o carte de aceeași culoare sau valoare cu cea de deasupra. 7 = adversarul trage 2 (se pot însuma), 8 = adversarul sare, valetul = joker (alegi culoarea). Cine rămâne fără cărți ia punctele cărților adversarului; primul la 60 câștigă. Mâna adversarului este ascunsă.",
            make: || Box::new(mau_mau::MauMau::new()),
        },
        Playable {
            id: "muggins",
            title: "Muggins (All Fives)",
            category: "Cărți",
            rules: "Domino cu punctaj la capete: pui o piesă la un capăt al șirului (`3-4:S` sau `:D`); dacă suma capetelor deschise e multiplu de 5 marchezi acea sumă. Dacă nu ai piesă potrivită tragi (`pesc`) sau pasezi. Cine rămâne fără piese ia în plus punctele adversarului rotunjite la 5; primul la 61 câștigă. Piesele adversarului sunt ascunse.",
            make: || Box::new(muggins::Muggins::new()),
        },
        Playable {
            id: "schafkopf",
            title: "Schafkopf (2 jucători)",
            category: "Cărți",
            rules: "Joc bavarez de levate cu 32 de cărți, adaptat pentru doi: 8 cărți fiecare (restul nu se joacă). Licitezi un contract (`licit wenz` sau `licit solo-c`): atuul este format din Obere (Q), Untere (J) și culoarea aleasă; la wenz doar valeții. A 11, 10 = 10, K 4, Q 3, J 2; declarantul are nevoie de 61 din 120 de puncte. Meci de 6 mâini; mâna adversarului este ascunsă.",
            make: || Box::new(trick32::Trick32::schafkopf()),
        },
        Playable {
            id: "preferans",
            title: "Preferans (2 jucători)",
            category: "Cărți",
            rules: "Joc rusesc de levate cu 32 de cărți, adaptat pentru doi: 10 cărți fiecare și un talon de 2. Licitezi numărul de levate și atuul (`licit 6h`, `licit 7nt`; ordinea s < c < d < h < fără atu). Declarantul ia talonul (`talon`), aruncă 2 cărți (`pune 7h 8c`) și joacă; trebuie să răspunzi la culoare, iar fără culoare tai cu atu. Reușit = +2·(nivel−5), ratat = minus pe fiecare levată lipsă. 6 mâini; mâna adversarului este ascunsă.",
            make: || Box::new(trick32::Trick32::preferans()),
        },
        Playable {
            id: "ulti",
            title: "Ulti (2 jucători)",
            category: "Cărți",
            rules: "Joc maghiar de levate cu 32 de cărți, adaptat pentru doi: 10 cărți fiecare și un talon de 2. Contracte: `licit parti-h` (cel puțin 46 de puncte, atuul cupă) sau `licit ulti-h` (parti plus ultima levată câștigată cu 7 de atu). Aș și 10 valorează 10, ultima levată 10 (90 în total). Parti: ±1; ulti anunțat: ±4 în plus. 6 mâini; mâna adversarului este ascunsă.",
            make: || Box::new(trick32::Trick32::ulti()),
        },
        Playable {
            id: "brag",
            title: "Three Card Brag",
            category: "Cărți",
            rules: "Pocher englezesc cu 3 cărți pentru doi: ante 1 fisă, 3 cărți ascunse fiecare. Ierarhie: trio (A-A-A cel mai mare), culoare-secvență, secvență, culoare, pereche, carte mare. La rând: `ridic` (miza +1, până la 5), `vad` (egalezi și arăți cărțile; la egalitate pierde cine cere arătarea) sau `fold`. 10 mâini cu 30 de fise; mâna adversarului este ascunsă.",
            make: || Box::new(brag::Brag::new()),
        },
        Playable {
            id: "crossword",
            title: "Cuvinte încrucișate",
            category: "Cuvinte",
            rules: "Duel pe un careu generat la fiecare partidă (6–9 cuvinte românești care se încrucișează). Alegi un cuvânt și propui soluția: `3o:casa` (număr, o = orizontal / v = vertical). Corect = puncte egale cu lungimea și rămâi la rând; greșit = −1 și rândul trece. La final câștigă scorul mai mare. Definițiile și categoria sunt vizibile; litere se dezvăluie la încrucișări.",
            make: || Box::new(crossword::Crossword::new()),
        },
        Playable {
            id: "perquackey",
            title: "Perquackey",
            category: "Cuvinte",
            rules: "Joc de cuvinte cu 15 zaruri-litere, pe ture pentru doi. Zarurile se aruncă la începutul fiecărei runde (`arunc`); apoi pe rând propui un cuvânt de cel puțin 3 litere format din zaruri (`cuv casa`, fiecare zar o dată, un cuvânt doar o dată pe rundă) sau `pas`. Două `pas` la rând încheie runda. Punctaj: lungimea minus 2. 3 runde; scorul mai mare câștigă.",
            make: || Box::new(perquackey::Perquackey::new()),
        },
        Playable {
            id: "ligretto",
            title: "Ligretto",
            category: "Cărți",
            rules: "Jocul de cărți rapid, pe ture pentru doi. Fiecare are o stivă Ligretto de 10 cărți, 3 rânduri și o mână de 27; pe masă se construiesc grămezi ascendente pe culoare de la 1 la 10. Tura: câte mutări vrei (`l-c2`, `w-r1`, `r2-c1`, `l-cn` pentru un 1 nou), apoi `da` întoarce 3 cărți din mână. Pe rânduri pui cu 1 mai mic și altă culoare. Câștigă cine golește Ligretto; altfel punctele (cărți puse − 2× cărți rămase).",
            make: || Box::new(ligretto::Ligretto::new()),
        },
        Playable {
            id: "truth_dare",
            title: "Truth or Dare (Adevăr sau provocare)",
            category: "Party",
            rules: "Joc de petrecere pentru doi, familial. În 12 ture, jucătorul la rând alege `adevar` sau `provocare` (sau `refuz`, −1 punct); jocul arată întrebarea sau provocarea, el o execută, iar celălalt este judecător: `ok` (performer +1 la adevăr / +2 la provocare, judecător +1) sau `nu` (performer −1). Scorul mai mare câștigă.",
            make: || Box::new(truth_dare::TruthDare::new()),
        },
        Playable {
            id: "sticla",
            title: "Sticla (Spin the bottle)",
            category: "Party",
            rules: "Joc de petrecere familial cu sticla învârtită. Șase locuri în cerc (J1: 1, 3, 5; J2: 2, 4, 6). Învârtitorul rotește sticla (`invart`), apoi alege dificultatea probei `nivel 1`, `nivel 2` sau `nivel 3`; proprietarul locului arătat spune `fac` sau `refuz` (−1). Cine încearcă reușește cu 90% / 60% / 30% și primește atâtea puncte cât nivelul, altfel −1 (`incerc`). 8 ture; scorul mai mare câștigă.",
            make: || Box::new(sticla::Sticla::new()),
        },
        Playable {
            id: "sardines",
            title: "Sardines",
            category: "Party",
            rules: "Ascunsul inversat, două runde cu roluri schimbate, într-o casă cu 9 camere (grilă 3×3). Ascunsul alege în secret camera (`ascund N`); căutătorul pornește din camera 1 și face câte un pas (`ir N`), apoi `asculta` arată dacă l-a găsit sau un indiciu public (respirații aproape / liniște). Camerele încă posibile sunt marcate cu «?». Ascunsul poate sta (`sta`) sau se poate muta (`ma N`). Ascunsul primește atâtea puncte câți pași a făcut căutătorul (13 dacă nu e găsit în 10).",
            make: || Box::new(sardines::Sardines::new()),
        },
        Playable {
            id: "scaune_muzicale",
            title: "Scaunele muzicale",
            category: "Party",
            rules: "Doi dansatori pe un cerc de 8 locuri, cu un singur scaun marcat # în fiecare rundă. Pe rând faci un pas de 1, 2 sau 3 locuri (`pas1`, `pas2`, `pas3`; nu aterizezi pe adversar, nu poți sta pe loc). După fiecare mutare `asculta` poate opri muzica (1 șansă din 5): cine stă atunci pe scaun ia punctul rundei. Primul la 3 puncte câștigă.",
            make: || Box::new(scaune_muzicale::Scaune::new()),
        },
        Playable {
            id: "mikado",
            title: "Mikado (Pick-up sticks)",
            category: "Familie",
            rules: "Un morman de 21 de bețișoare (Mikado 20, Mandarini 10, Bonzi 5, Samurai 3, Kuli 2). Ridici bețișorul `ia N`: dacă nu are altele deasupra reușești sigur și continui; dacă este acoperit riști (`incerc`: șanse 50% pentru un bețișor acoperit, 25% pentru două …), iar la eșec pierzi rândul. `pas` încheie rândul după ce ai ridicat ceva. Când mormanul e gol câștigă cine are mai multe puncte.",
            make: || Box::new(mikado::Mikado::new()),
        },
        Playable {
            id: "kick_can",
            title: "Kick the Can",
            category: "Party",
            rules: "Joc de ascuns și prins pe un teren 7×7. J1 este Căutătorul (S), J2 conduce trei ascunși (1–3); cutia O stă în centru (d4). Fiecare face două pași pe tură în 8 direcții (`ne`, `s`…; ascunșii `h2 ne`). Căutătorul prinde un ascuns călcând pe el; un ascuns liber care pășește pe cutie îi eliberează pe toți prinșii. Căutătorul câștigă când toți 3 sunt prinși; ascunșii câștigă dacă rezistă 20 de ture.",
            make: || Box::new(kick_can::KickCan::new()),
        },
        Playable {
            id: "mingea",
            title: "Mingea (Dodgeball)",
            category: "Party",
            rules: "Două echipe de câte 4 jucători pe 7 coloane. Pe rând muți un jucător într-o coloană vecină liberă (`3>4`) sau arunci spre o coloană adversă (`arunc 4`). Mingea nu cade exact unde ai țintit: `zbor` hotărăște la întâmplare coloana (50% ținta, 20% fiecare vecină, 5% la două coloane). Cine e în coloana de aterizare iese. Câștigă echipa care rămâne cu jucători.",
            make: || Box::new(mingea::Mingea::new()),
        },
        // Q3-PL-END
        Playable {
            id: "kalah",
            title: "Kalah",
            category: "Abstract",
            rules: "Kalah 6×4: semănat antiorar (sari peste magazia adversarului); ultima sămânță în magazia ta = mutare în plus; într-o groapă proprie goală cu opusa nevidă capturezi opusa. Când o parte rămâne goală, restul merge în magazia celeilalte. Mutări: 1–6 (gropile tale).",
            make: || Box::new(kalah::Kalah::new()),
        },
        Playable {
            id: "foxgeese",
            title: "Vulpea și gâștele (Fox and Geese)",
            category: "Clasic",
            rules: "Tablă în cruce cu 33 de găuri. J1 = vulpea (d4), J2 = 13 gâște. Vulpea mută ortogonal sau sare peste o gâscă (d4xd6xf6); gâștele mută doar înainte/lateral. Gâștele câștigă blocând vulpea; vulpea câștigă dacă rămân sub 6 gâște. Mutări: d4-d5, d4xd6.",
            make: || Box::new(foxgeese::FoxGeese::new()),
        },
        Playable {
            id: "alquerque",
            title: "Alquerque",
            category: "Abstract",
            rules: "Tablă 5×5 cu diagonale prin punctele pare, 12 piese pe jucător. Pas pe o linie; captura (salt peste o piesă adversă pe loc liber) e obligatorie și se înlănțuie. Câștigă cine captează toate piesele sau blochează adversarul. Mutări: b3-c3, a3xc3xe3.",
            make: || Box::new(alquerque::Alquerque::new()),
        },
        Playable {
            id: "jungle",
            title: "Jungle (Dou Shou Qi)",
            category: "Abstract",
            rules: "Tablă 7×9, 8 animale pe jucător (șobolan 1 … elefant 8). Pas ortogonal; capturezi rang egal sau mai mic (șobolanul poate lua elefantul); doar șobolanul intră în apă; leul și tigrul sar lacul; în capcana adversă rangul e 0. Câștigi intrând în bârlogul adversarului sau capturând tot. Mutări: a1-a2, c3xc4.",
            make: || Box::new(jungle::Jungle::new()),
        },
        Playable {
            id: "chaturanga",
            title: "Chaturanga (India)",
            category: "Abstract",
            rules: "Strămoșul șahului: raja, mantri (pas diagonal), gaja (sare 2 pe diagonală), ashva (cal), ratha (car), padati (pion, promovează în mantri). Fără rocadă și pas dublu. Mat = pierdere, pat = remiză, raja rămas singur = pierdere. Mutări: e2e3, c1e3.",
            make: || Box::new(chaturanga::Chaturanga::new()),
        },
        Playable {
            id: "camelot",
            title: "Camelot",
            category: "Clasic",
            rules: "Tablă 160 de pătrate cu castele. 10 pioni + 4 cavaleri. Pas în 8 direcții, canter (sari peste o piesă proprie), salt (capturezi, obligatoriu); cavalerul poate încărca (canter + salturi). Câștigi cu două piese în castelul adversarului sau prin capturare/blocare. Mutări: d6-d7, e7xe9, f6-f8xf10.",
            make: || Box::new(camelot::Camelot::new()),
        },
        Playable {
            id: "fotbalmasa",
            title: "Fotbal de masă pe ture (Table football)",
            category: "Clasic",
            rules: "8 bare, fiecare acoperă 2 din 5 culoare. O tură = muți o bară (atac 3) sau, cu mingea, trimiți pe un culoar acoperit de bara ta (trimit 2): mingea se oprește la prima bară care acoperă culoarul (pasă sau interceptare); gol dacă trece pe culoarele 2–4. Primul la 5.",
            make: || Box::new(fotbalmasa::FotbalMasa::new()),
        },
        Playable {
            id: "tiddlywinks",
            title: "Tiddlywinks (pe ture)",
            category: "Clasic",
            rules: "Masă 11×7, 4 winks pe jucător, găleata în f4. Muți un wink liber cu până la 3 câmpuri (c4-d4); aterizarea pe un wink îl acoperă (îl blochează), în găleată îl scoate (3 puncte). Scor: 3 × potted + winks adverse acoperite. Se termină când cineva nu mai are winks libere sau după 80 de mutări.",
            make: || Box::new(tiddlywinks::Tiddlywinks::new()),
        },
        Playable {
            id: "joc15",
            title: "Jocul de 15 (cursă)",
            category: "Clasic",
            rules: "Fiecare jucător are propria tablă 4×4, amestecată identic. Pe rând, mutați o piesă lângă locul gol pe tabla voastră (ex. 7); câștigă primul care aduce piesele în ordinea 1–15. După 300 de mutări câștigă cel mai apropiat de ordine.",
            make: || Box::new(joc15::Joc15::new()),
        },
        Playable {
            id: "sudoku",
            title: "Sudoku duel",
            category: "Clasic",
            rules: "Grilă cu soluție unică. Pe rând alegeți celulă și cifră fără conflict (r3c4=5): corect = +1 (+2 pentru fiecare rând/coloană/pătrat completat), greșit = −1 și cifra se elimină din acea celulă. Câștigă cine are mai multe puncte când grila e plină.",
            make: || Box::new(sudoku::Sudoku::new()),
        },
        Playable {
            id: "biliard",
            title: "Biliard francez (carambol)",
            category: "Clasic",
            rules: "3 bile, masă fără buzunare. Lovești bila ta (albă J1, galbenă J2) cu unghi și forță (u45f2 = 45°, forța 2); dacă atinge ambele bile marchezi și mai joci. Primul la 10 puncte; fizică deterministă fără efect.",
            make: || Box::new(biliard::Biliard::new()),
        },
        Playable {
            id: "tavli",
            title: "Tavli (Portes, Plakoto, Fevga)",
            category: "Clasic",
            rules: "Tabla greacă: J1 alege varianta (portes, plakoto sau fevga). Portes = backgammon cu lovire, Plakoto = imobilizezi piesa adversă singură, Fevga = ambii în același sens, fără lovire și cu regula zidului. Zaruri automate; 15 piese; scoți piesele când toate sunt acasă. Mutări: 24/18, bar/20, 6/off.",
            make: || Box::new(tavli::Tavli::new()),
        },
        Playable {
            id: "gulbara",
            title: "Gul bara (Golful Persic)",
            category: "Clasic",
            rules: "Variantă de table fără lovituri: 15 piese pe jucător, colțuri opuse, același sens de mers; un punct ocupat de adversar e blocat. Dublurile dau 4 mutări (apoi 4 cu fiecare număr până la 6, după primele 3 aruncări). Scoți piesele când toate sunt acasă; primul care termină câștigă. Mutări: 24/18, 6/off.",
            make: || Box::new(gulbara::GulBara::new()),
        },
        Playable {
            id: "chinchirorin",
            title: "Chinchirorin (zaruri în bol)",
            category: "Zaruri",
            rules: "Trei zaruri; oya (banca) și ko alternează. Ko pariază 1–3 (pariez 2), apoi se aruncă (arunc, până la 3 încercări): Pinzoro 1-1-1 = 5×, Arashi (triplet) 3×, Shigoro 4-5-6 2×, punct = zarul singur dintr-o pereche; 1-2-3 pierde 2×. 12 runde sau până la faliment.",
            make: || Box::new(chinchirorin::Chinchirorin::new()),
        },
        Playable {
            id: "shipcrew",
            title: "Ship, Captain and Crew",
            category: "Zaruri",
            rules: "5 zaruri, 3 aruncări pe tură: ai nevoie, în ordine, de 6 (ship), 5 (captain), 4 (crew); celelalte două zaruri sunt încărcătura (scorul turei). După o aruncare reușită: pastrez sau rearunc. 5 ture pe jucător; câștigă suma cea mai mare. Mutări: arunc, pastrez, rearunc.",
            make: || Box::new(shipcrew::ShipCrew::new()),
        },
        Playable {
            id: "beetle",
            title: "Gândacul (Beetle)",
            category: "Zaruri",
            rules: "Arunci pe rând un zar: 1 corp, 2 cap, 3 picior (6), 4 antenă (2), 5 ochi (2), 6 coadă. Corpul primul; capul, picioarele și coada cer corpul; antenele și ochii cer capul. Primul gândac complet (13 piese) câștigă. Mutare: arunc.",
            make: || Box::new(beetle::Beetle::new()),
        },
        Playable {
            id: "kismet",
            title: "Zaruri colorate (Kismet)",
            category: "Zaruri",
            rules: "15 ture; până la 3 aruncări cu 5 zaruri (tine 3 3 5), apoi scor într-o categorie: unu…sase (bonus 35/55/75), 2perechi (aceeași culoare), 3la-fel, scara (30), culoare (35), casa (+15), casa-culoare (+20), 4la-fel (+25), sansa, kismet (+50). Zaruri impare = roșii, pare = albastre.",
            make: || Box::new(kismet::Kismet::new()),
        },
        Playable {
            id: "deadmansdice",
            title: "Dead Man's Dice (zarurile piratului)",
            category: "Zaruri",
            rules: "Fiecare are 2 zaruri și zarul Jolly Roger. Aruncate, zarurile se așază într-o grămadă dacă sunt egale cu cel de sus, cu 1 mai mari sau dau 7 împreună; jefuiești zarul de sus al adversarului dacă se potrivește. Runda: suma grămezii (JR deasupra dublează, JR neasezat −7). Primul la 100. Mutări: arunc, arunc-jr, jefuiesc.",
            make: || Box::new(deadmansdice::DeadMansDice::new()),
        },
        Playable {
            id: "snap",
            title: "Snap",
            category: "Copii",
            rules: "Pachet de 52 împărțit în două; pe rând întorci (intorc) câte o carte în mijloc. Când ultimele două cărți au aceeași valoare, cine e la mutare poate striga snap și ia grămada; un snap fals costă o carte. Cine rămâne fără cărți pierde.",
            make: || Box::new(snap::Snap::new()),
        },
        Playable {
            id: "simon",
            title: "Simon zice",
            category: "Copii",
            rules: "Simon dă o poruncă (simon-zice sari sau ordin sari); celălalt o execută (execut sari) doar dacă a zis „Simon zice”, altfel stă (stau). Răspuns corect = punct pentru cel care ascultă, greșit = punct pentru Simon. 12 runde, rolurile alternează.",
            make: || Box::new(simon::Simon::new()),
        },
        Playable {
            id: "settemezzo",
            title: "Sette e mezzo (șapte și jumătate)",
            category: "Cărți",
            rules: "Pachet italian de 40 de cărți: figurile (8, 9, 10) valorează ½, restul valoarea lor. Te apropii de 7½ fără să treci. Prima carte e ascunsă; carta = tragi, stau = rămâi. Ko pariază 1–3, banca câștigă la egalitate; 7½ natural (două cărți) plătește dublu. 12 runde.",
            make: || Box::new(settemezzo::SetteMezzo::new()),
        },
        Playable {
            id: "baccarat",
            title: "Baccarat (Punto Banco)",
            category: "Cărți",
            rules: "Punto Banco: parieți pe punto, banco (comision 5%) sau egal (8:1) cu 1–3 unități (punto 2); cărțile se trag după tabelul fix. Totalul = suma mod 10; 8–9 = natural. 20 de coupe; cine are mai multe jetoane câștigă. Mutări: punto N, banco N, egal N, impart.",
            make: || Box::new(baccarat::Baccarat::new()),
        },
        Playable {
            id: "hangman",
            title: "Spânzurătoarea (Hangman)",
            category: "Copii",
            rules: "Cine pune alege în secret un cuvânt din listă (cuvant casa); celălalt ghicește litere (litera a) sau cuvântul (ghicesc casa). 7 greșeli = pierdută. Reușită = 1 + (7 − greșeli) puncte. Două runde cu roluri inversate; cine are mai multe puncte câștigă.",
            make: || Box::new(hangman::Hangman::new()),
        },
        Playable {
            id: "douazeci",
            title: "Douăzeci de întrebări",
            category: "Party",
            rules: "Cine gândește alege un obiect din listă (obiect caine); celălalt pune întrebări da/nu numerotate (intreb 7) sau încearcă un răspuns (este caine); maxim 20 între întrebări și încercări. Cost = câte folosește (eșec = 21). Două runde cu roluri inversate; costul total mai mic câștigă.",
            make: || Box::new(douazeci::Douazeci::new()),
        },
        Playable {
            id: "ispy",
            title: "Ghicește ce văd (I Spy)",
            category: "Copii",
            rules: "Cine vede alege un obiect din cameră (vad masa) și spune litera lui; celălalt ghicește (este masa) sau cere indicii (indiciu culoare / loc / marime). Maxim 6 încercări (eșec = 7). Două runde cu roluri inversate; costul total mai mic câștigă.",
            make: || Box::new(ispy::ISpy::new()),
        },
        Playable {
            id: "comori",
            title: "Vânătoarea de comori",
            category: "Party",
            rules: "Pe o grilă 8×8, cine ascunde alege celula (ascund d5); celălalt sapă (sap c3) și primește indiciul cald/rece după distanța Manhattan. 12 săpături (eșec = 13). Două runde cu roluri inversate; costul total mai mic câștigă.",
            make: || Box::new(comori::Comori::new()),
        },
        Playable {
            id: "mima",
            title: "Mima (Charades)",
            category: "Party",
            rules: "Adaptare fără mimă fizică: cine mimează alege un cuvânt (mimez delfin) și dă indicii pe rând (gest categorie / silabe / prima-litera / ultima-litera / lungime); celălalt ghicește (ghicesc delfin) sau cere mai-multe. Cost = indicii + greșeli (eșec = 8). Două runde; costul mai mic câștigă.",
            make: || Box::new(mima::Mima::new()),
        },
        Playable {
            id: "crima",
            title: "Crima cu ochiade (Wink Murder)",
            category: "Party",
            rules: "Zece suspecți în cerc. J1 = ucigașul (sunt 4), face cu ochiul și un suspect moare (clipesc 7); J2 = detectivul primește un martor (4 suspecți, unul e ucigașul) și acuză (acuz 3) sau așteaptă (astept). Acuzație corectă = detectivul câștigă; 3 greșeli sau 2 suspecți rămași = ucigașul.",
            make: || Box::new(crima::Crima::new()),
        },
        Playable {
            id: "sheepshead",
            title: "Sheepshead",
            category: "Cărți",
            rules: "Levate cu 32 de cărți și atuuri fixe (Q, J, caro); adaptare pentru doi, cu blind și cărți îngropate. Mutări: iau, pas, ingrop 7c, apoi cartea jucată (ex. Qc).",
            make: || Box::new(sheepshead::Sheepshead::new()),
        },
        Playable {
            id: "tysiac",
            title: "Tysiąc",
            category: "Cărți",
            rules: "Tysiąc (o mie) pentru doi: licitație 100–300, hlášky/mariaje și două musiks; primul la 1000 sau 6 mâini. Mutări: licitez 110, pas, musik 1, las 9h, apoi cartea (ex. Ah) sau Kh marjas.",
            make: || Box::new(tysiac::Tysiac::new()),
        },
        Playable {
            id: "mariash",
            title: "Mariáš",
            category: "Cărți",
            rules: "Mariáš ceh pentru doi: forhont, talon de două cărți, atu, hlášky 20/40, ultima levată +10; 6 mâini. Mutări: las 7g (două cărți), atu inimi, apoi cartea (ex. Ag) sau Kg hlaska.",
            make: || Box::new(mariash::Mariash::new()),
        },
        Playable {
            id: "texas42",
            title: "Texas 42",
            category: "Clasic",
            rules: "Levate cu domino pentru doi: licitație 30–42, atu, 42 de puncte; primul la 4 semne. Mutări: licitez 32, pas, atu 5, apoi piesa (ex. 6-4).",
            make: || Box::new(texas42::Texas42::new()),
        },
        Playable {
            id: "pit",
            title: "Pit",
            category: "Party",
            rules: "Schimburi ascunse de cărți între doi jucători; corner = toate cele 6 cărți ale unei mărfi. Mutări: ofer grau 2, dau ovaz, schimb.",
            make: || Box::new(pit::Pit::new()),
        },
        Playable {
            id: "mahjong_am",
            title: "American Mah Jongg",
            category: "Clasic",
            rules: "Mah jongg simplificat pentru doi: 152 de piese cu jokeri, pung-uri expuse, 4 seturi + pereche sau șapte perechi. Mutări: arunc 5k, pung, pas, mahjong.",
            make: || Box::new(mahjong_am::MahjongAm::new()),
        },
        Playable {
            id: "damas",
            title: "Dame braziliene (Damas)",
            category: "Abstract",
            rules: "Dame internaționale pe 8×8: piesele simple mută înainte și capturează în toate direcțiile, damele zboară, captura este obligatorie și trebuie să fie cea mai mare. Remiză după 20 de mutări pe parte fără captură sau mutare de piesă simplă. Mutări: c3-d4, captură c3xe5xg7.",
            make: || Box::new(damas::Damas::new()),
        },
        Playable {
            id: "renju",
            title: "Renju",
            category: "Abstract",
            rules: "Gomoku pe 15×15 cu restricții pentru Negru (J1): linia lungă (6+), dublu-patru și dublu-trei deschis sunt interzise; Negru câștigă cu exact cinci, Alb cu cinci sau mai multe. Mutări: h8 (coloane a–o, rânduri 1–15).",
            make: || Box::new(renju::Renju::new()),
        },
        Playable {
            id: "tablut",
            title: "Tablut",
            category: "Abstract",
            rules: "Hnefatafl lapon pe 9×9: 16 atacatori (J1, primul) contra 8 apărători și un rege. Toate piesele mută ca turnul; captură în clește; regele scapă pe orice pătrat de margine, atacatorii îl capturează pe patru laturi. Mutări: e1-e3.",
            make: || Box::new(tablut::Tablut::new()),
        },
        Playable {
            id: "lasca",
            title: "Lasca",
            category: "Abstract",
            rules: "Dama lui Emanuel Lasker pe 7×7: piesa capturată trece la baza coloanei tale, se ia doar piesa de deasupra; ofițerii mută și capturează în toate direcțiile; captura e obligatorie. Mutări: a1-b2, a1xc3xe5.",
            make: || Box::new(lasca::Lasca::new()),
        },
        Playable {
            id: "ruleta",
            title: "Ruletă (Roulette)",
            category: "Clasic",
            rules: "Ruletă europeană (0–36) pentru doi: 100 de jetoane fiecare, 12 învârtiri, un singur pariu pe învârtire (5, 10 sau 20). Număr plin 35:1, roșu/negru, par/impar, mic/mare 1:1, duzine și coloane 2:1. Mutări: rosu 10, plin 17 5, duz2 20, invart.",
            make: || Box::new(ruleta::Ruleta::new()),
        },
        Playable {
            id: "marbles",
            title: "Bile (par sau impar)",
            category: "Clasic",
            rules: "Joc antic de ghicit: cel care ține ascunde 1–5 bile, celălalt ghicește par sau impar; ghicești bine = iei atâtea bile, rău = le dai. 10 bile fiecare, 12 runde, rolurile alternează. Mutări: ascund 3, par, impar, arat.",
            make: || Box::new(marbles::Marbles::new()),
        },
        Playable {
            id: "mahjong_tw",
            title: "Mahjong taiwanez",
            category: "Clasic",
            rules: "Mahjong taiwanez pentru doi: 144 de piese cu 8 flori, mâini de 16 piese, câștigi cu 5 seturi și o pereche; florile se înlocuiesc din zidul mort și valorează tai. Mutări: trage, d5m, pung, kong, chi2m3m, mahjong, pas.",
            make: || Box::new(mahjong_tw::MahjongTw::new()),
        },
        Playable {
            id: "matador",
            title: "Matador (domino)",
            category: "Clasic",
            rules: "Domino în care jumătățile se pun cu suma 7 față de capăt (1+6, 2+5, 3+4); matadorii 0-0, 1-6, 2-5, 3-4 se pun oriunde. 7 piese fiecare, tragi dacă nu poți, câștigă cine termină primul. Mutări: 6-1@s=1, trag, pas.",
            make: || Box::new(matador::Matador::new()),
        },
        Playable {
            id: "bagatelle",
            title: "Bagatelle",
            category: "Clasic",
            rules: "Masă înclinată cu cuie și 9 căsuțe: alegi culoarul de lansare (1–5) și puterea (1–3), apoi lași bila să curgă; loviturile tari merg mai drept, dar pot sări peste masă. Nouă bile fiecare; câștigă cine are mai multe puncte. Mutări: tir 3 2, lansez.",
            make: || Box::new(bagatelle::Bagatelle::new()),
        },
        Playable {
            id: "latrunculi",
            title: "Latrunculi",
            category: "Clasic",
            rules: "Joc roman de masă, reconstituire modernă pe 8×8: 16 piese fiecare, toate mută ca turnul, captură în clește de către piesa mutată; câștigi lăsând adversarul cu o piesă sau fără mutări. Mutări: a2-a5.",
            make: || Box::new(latrunculi::Latrunculi::new()),
        },
        Playable {
            id: "salta",
            title: "Salta",
            category: "Clasic",
            rules: "Jocul lui Büttgenbach (1899) pe 10×10: fără capturi; duci toate cele 15 piese pe pătratele de start ale adversarului. Pași diagonali (în orice direcție) sau salturi peste o piesă adversă înainte; săritura e obligatorie. Mutări: a1-b2, c3xe5.",
            make: || Box::new(salta::Salta::new()),
        },
        Playable {
            id: "chaturaji",
            title: "Chaturaji",
            category: "Clasic",
            rules: "Șahul indian cu patru armate, adaptat pentru doi (fiecare conduce două armate aliate). Arunci două zaruri, fiecare joacă un tip de piesă: 1/5 pion sau rege, 2 barcă, 3 cal, 4/6 elefant. Capturezi ambii regi adverși. Mutări: arunc, a2a3, pas.",
            make: || Box::new(chaturaji::Chaturaji::new()),
        },
        Playable {
            id: "sjoelen",
            title: "Sjoelen",
            category: "Clasic",
            rules: "Șuflecatul olandez: 30 de discuri, 3 runde, le împingi în patru porți numerotate (2, 3, 4, 1); doar discurile ratate se aruncă din nou. Fiecare serie completă de patru porți valorează 20 de puncte. Mutări: stanga, centru, dreapta, atent, arunc.",
            make: || Box::new(sjoelen::Sjoelen::new()),
        },
        Playable {
            id: "nonogram",
            title: "Nonogram (Picross), duel",
            category: "Clasic",
            rules: "Doi jucători rezolvă aceeași imagine ascunsă 8×8 după indiciile rândurilor și coloanelor: revendici o căsuță necunoscută ca plină (a1) sau goală (a1x); apoi verific o dezvăluie. Corect +1, greșit −2. Mutări: a1, a1x, verific.",
            make: || Box::new(nonogram::Nonogram::new()),
        },
        Playable {
            id: "snooker",
            title: "Snooker",
            category: "Clasic",
            rules: "Snooker simplificat, o singură frame, cu fizică 2D a bilelor: bagi o roșie, apoi o culoare, alternativ; faulturile dau adversarului minimum 4. Lovitură: bila:buzunar:forța, de ex. r4:ne:2 (buzunare sv, nv, se, ne, ms, mn; forța 1–3).",
            make: || Box::new(snooker::Snooker::new()),
        },
        Playable {
            id: "scattergories",
            title: "Scattergories",
            category: "Party",
            rules: "Joc de cuvinte pentru doi: 3 runde, fiecare cu o literă și 4 categorii (animal, mâncare, țară, obiect). Amândoi aleg în secret un cuvânt din listă; cuvintele identice dau 0, cele unice 1–3 (cu cât mai rar, cu atât mai mult). Mutări: caine, pas, arat.",
            make: || Box::new(scattergories::Scattergories::new()),
        },
        Playable {
            id: "nhie",
            title: "Eu niciodată",
            category: "Party",
            rules: "Joc de petrecere pentru doi, cu 5 degete fiecare și un profil ascuns de lucruri făcute. Spui o afirmație pe care n-ai făcut-o; celălalt răspunde sincer și coboară un deget dacă a făcut-o. Mutări: eu-niciodata-5, raspund.",
            make: || Box::new(nhie::Nhie::new()),
        },
        Playable {
            id: "spoons",
            title: "Spoons (lingurile)",
            category: "Party",
            rules: "Joc de petrecere cu cărți pentru doi, cu o singură lingură: tragi o carte, arunci una, strângi patru la fel și iei lingura; cine pierde primește o literă din SPOONS. 12 runde, câștigă cine are mai puține litere. Mutări: iau, dau K, lingura, mai-departe.",
            make: || Box::new(spoons::Spoons::new()),
        },
        Playable {
            id: "parcel",
            title: "Treci pachetul (Pass the Parcel)",
            category: "Party",
            rules: "Joc de petrecere cu 8 straturi: pasezi pachetul (gratis) sau îl păstrezi (costă 1); apoi muzica se poate opri (15% plus 10% la fiecare acțiune) și cel care ține pachetul desface stratul și ia premiul (−3 până la 5). Câștigă cine are mai multe puncte după 8 straturi. Mutări: pasez, pastrez, muzica.",
            make: || Box::new(parcel::Parcel::new()),
        },
        Playable {
            id: "klondike",
            title: "Solitaire (Klondike), duel",
            category: "Cărți",
            rules: "Klondike clasic (tragere de câte o carte), ca duel: fiecare are propria amestecare și face câte o mutare; câștigă primul care duce toate cele 52 de cărți pe fundații. Mutări: d, w>f, w>3, 3>f, 3>5:2.",
            make: || Box::new(klondike::Klondike::new()),
        },
        Playable {
            id: "jass",
            title: "Jass (Schieber)",
            category: "Cărți",
            rules: "Jass elvețian pentru doi, pachet de 36 de cărți, 9 cărți fiecare, 4 mâini: cel care conduce alege atuul (atu h) sau îl dă mai departe (schiebe). Valetul de atu 20, noua 14, asul 11; ultima levată +5, match +100. Mutări: atu c, schiebe, 10h, Js.",
            make: || Box::new(jass::Jass::new()),
        },
        Playable {
            id: "manille",
            title: "Manille",
            category: "Cărți",
            rules: "Manille franceză pentru doi, 32 de cărți, 8 fiecare, o carte întoarsă dă atuul, 4 mâini. Ordine 10 > A > K > Q > J > 9 > 8 > 7, puncte 5-4-3-2-1; răspunzi la culoare și, dacă poți, mai sus, altfel tai. Mutări: 10h, As, 7c.",
            make: || Box::new(manille::Manille::new()),
        },
        Playable {
            id: "klaverjas",
            title: "Klaverjas",
            category: "Cărți",
            rules: "Klaverjas olandez (Amsterdam) pentru doi, 32 de cărți, 8 fiecare, o carte întoarsă dă atuul, 4 mâini. Valetul de atu 20, noua 14; ordine simplă A > 10 > K > Q > J; ultima levată +10, pit +100. Răspunzi la culoare, bați atuul dacă poți. Mutări: Jh, 10s, Ac.",
            make: || Box::new(klaverjas::Klaverjas::new()),
        },
        Playable {
            id: "sueca",
            title: "Sueca",
            category: "Cărți",
            rules: "Sueca portugheză pentru doi, pachet de 40 de cărți (fără 8, 9, 10), 10 fiecare, o carte întoarsă dă atuul, 4 mâini. Ordine A > 7 > K > J > Q > 6 > 5 > 4 > 3 > 2, puncte A 11, 7 10, K 4, J 3, Q 2; răspunzi la culoare dacă poți. Mutări: Ah, 7s, Qc.",
            make: || Box::new(sueca::Sueca::new()),
        },
        Playable {
            id: "rps",
            title: "Piatră, hârtie, foarfece",
            category: "Copii",
            rules: "Piatra sparge foarfeca, foarfeca taie hârtia, hârtia acoperă piatra. Amândoi aleg în secret, apoi arată; primul la 5 runde câștigate (maximum 15 runde, egalitățile se reiau). Mutări: piatra, hartie, foarfeca, arat.",
            make: || Box::new(rps::Rps::new()),
        },
        Playable {
            id: "coarda",
            title: "Sărit coarda (Jump rope)",
            category: "Copii",
            rules: "8 runde, jucătorii alternează sărind și învârtind. Cel care învârte alege viteza (lent, mediu, rapid); săritorul alege trucuri (simplu 1, incrucisat 3, dublu 5, triplu 9 puncte) apoi sar, sau opresc ca să bage șirul; o greșeală îl pierde. Mutări: mediu, dublu, sar, opresc.",
            make: || Box::new(coarda::Coarda::new()),
        },
        Playable {
            id: "capra",
            title: "Capra (Leapfrog)",
            category: "Copii",
            rules: "Cursă pe un șir de 10 căsuțe: 3 săritori fiecare, care merg spre partea opusă. Faci un pas pe căsuța vecină liberă sau sari peste un săritor alăturat pe căsuța liberă din spatele lui; câștigă primul cu toți cei trei pe ultimele căsuțe. Mutări: 3-4, 2-4, pas.",
            make: || Box::new(capra::Capra::new()),
        },
        Playable {
            id: "kim",
            title: "Jocul lui Kim (Kim's Game)",
            category: "Copii",
            rules: "Joc de memorie, 5 runde cu tăvi de 6 până la 14 obiecte: privești tava (acopera), un obiect e scos în secret (scot), apoi ghicești după nume care lipsește; ghicit = 1 punct, greșit = rândul trece. Mutări: acopera, scot, cheie, verific.",
            make: || Box::new(kim::Kim::new()),
        },
        Playable {
            id: "slapjack",
            title: "Slapjack",
            category: "Copii",
            rules: "Joc de reflexe cu cărți adaptat pe ture: întorci cartea de sus pe teanc; când apare un valet celălalt poate da cu palma (65% reușită, altfel teancul îl ia cel care a întors valetul). Palma pe altceva costă o carte. Mutări: joc, palma, rezolv.",
            make: || Box::new(slapjack::Slapjack::new()),
        },
        Playable {
            id: "chohan",
            title: "Chō-han (Chō-han bakuchi)",
            category: "Zaruri",
            rules: "Pariu japonez cu zaruri: două zaruri sub un pahar, pariezi cho (pară) sau han (impară) cu 5, 10 sau 20 de jetoane; celălalt primește partea opusă. 10 runde, 100 de jetoane fiecare, chemătorul alternează. Mutări: cho 10, han 5, agit.",
            make: || Box::new(chohan::Chohan::new()),
        },
        Playable {
            id: "balut",
            title: "Balut",
            category: "Zaruri",
            rules: "Joc filipinez cu zaruri: 5 zaruri, până la 3 aruncări pe tură, 7 categorii completate o dată (patru, cinci, sase, scara 50, casa, sansa, balut 30). 7 ture fiecare; câștigă cine are mai multe puncte. Mutări: arunc, tine 6 6, tine -, scor sase.",
            make: || Box::new(balut::Balut::new()),
        },
        Playable {
            id: "tali",
            title: "Knucklebones (Tali)",
            category: "Zaruri",
            rules: "Oscioarele antice: patru astragale cu fețele 1, 3, 4, 6; Venus (toate diferite) bate orice, Canis (patru de 1) pierde în fața oricărei alte aruncări, altfel contează suma. 10 runde, 20 de jetoane fiecare. Mutări: miza 2, arunc.",
            make: || Box::new(tali::Tali::new()),
        },
        Playable {
            id: "crag",
            title: "Crag",
            category: "Zaruri",
            rules: "Joc american cu 3 zaruri și 13 categorii, 13 ture fiecare: arunci, poți rearunca orice zaruri o dată, apoi scrii o categorie liberă (unu..sase, trei-la-fel 25, treisprezece 26, scări 20, crag 50). Mutări: arunc, tine 6 6, tine -, scor crag.",
            make: || Box::new(crag::Crag::new()),
        },
        Playable {
            id: "bobing",
            title: "Bo Bing (Pua Tiong Chiu)",
            category: "Zaruri",
            rules: "Joc cu zaruri de Festivalul Lunii din Xiamen: șase zaruri într-un castron, premii (63 de prăjituri de lună) după combinație: Zhuangyuan cea mai bună, apoi Duitang, Sanhong, Sijin, Erju, Yixiu (după numărul de 4). Fiecare categorie are rezervă limitată; câștigă cine are mai multe puncte după 12 aruncări fiecare. Mutare: arunc.",
            make: || Box::new(bobing::Bobing::new()),
        },
        // B2-reg-begin
        Playable {
            id: "ambuteiaj",
            title: "Ambuteiaj (duel de mașini pe tablă 6×6)",
            category: "Familie",
            rules: "Variantă originală: mașini de 2–3 pătrate pe o tablă 6×6; A (J1) iese pe dreapta rândului 3, B (J2) pe stânga rândului 4. Mutezi o singură mașină pe tură (și pe cele ale adversarului): A+2, C-1. Câștigă cine își eliberează primul ținta; 120 de mutări: remiză. Mutări: A+2, C-1.",
            make: || Box::new(brand_util::B(ambuteiaj::Ambuteiaj::new())),
        },
        Playable {
            id: "banchiza",
            title: "Banchiza (pinguini pe sloiuri hexagonale)",
            category: "Familie",
            rules: "Variantă originală: 60 de plăci hexagonale cu 1–3 pești. Fiecare pune 4 pinguini pe plăci cu un singur pește (pune x,y), apoi muți un pinguin în linie dreaptă peste plăci libere (mut x,y>x2,y2); placa părăsită dispare, peștii sunt ai tăi. Cine nu poate muta pasează. Câștigă cine are mai mulți pești. Mutări: pune x,y | mut x,y>x2,y2 | pas.",
            make: || Box::new(brand_util::B(banchiza::Banchiza::new())),
        },
        Playable {
            id: "balans",
            title: "Balansul Pietrelor (greutăți pe o grindă)",
            category: "Familie",
            rules: "Variantă originală: o grindă cu 13 poziții (−6..6) și greutățile 1–6 pentru fiecare. Pui pe rând o greutate pe o poziție liberă (pune g@p, ex. pune 4@-3). Momentul total (greutate × poziție) nu are voie să depășească 18; cine răstoarnă grinda pierde, 12 greutăți puse fără răsturnare = remiză. Mutări: pune g@p.",
            make: || Box::new(brand_util::B(balans::Balans::new())),
        },
        Playable {
            id: "pietre_puzzle",
            title: "Piese de Mozaic (puzzle cu forme în doi)",
            category: "Familie",
            rules: "Variantă originală: tablă 5×6 cu 6 găuri și 6 piese comune de câte 4 celule (I, O, T, S, L, J). Pe rând pui o piesă nefolosită, în orice orientare (pune T@r,c,o). Cine nu mai poate pune nicio piesă pierde. Mutări: pune T@r,c,o.",
            make: || Box::new(brand_util::B(pietre_puzzle::PietrePuzzle::new())),
        },
        Playable {
            id: "firul",
            title: "Firul Luminii (plăci cu drumuri pe tablă 6×6)",
            category: "Familie",
            rules: "Variantă originală: plăci cu 4 drumuri între 8 capete; pui placa în celula în care intră jetonul tău, rotită 0–3 sferturi (pune t,r; mâinile sunt deschise), apoi jetoanele aleargă pe drumuri. Cine iese de pe tablă e eliminat, ciocnirea îi elimină pe amândoi; câștigă ultimul rămas. Mutări: pune t,r.",
            make: || Box::new(brand_util::B(firul::Firul::new())),
        },
        Playable {
            id: "genial",
            title: "Mozaicul Priceput (domino hexagonal în șase culori)",
            category: "Familie",
            rules: "Variantă originală: tablă hexagonală cu 91 de celule; plăcuțe domino de două culori, mâinile sunt deschise. Pui plăcuța i cu prima culoare în (q,r) și a doua în direcția d (pune i@q,r,d). Scorul pe culoare = șirurile de aceeași culoare pornind din fiecare jumătate (max 18). Câștigă cine are cea mai bună culoare cea mai slabă. Mutări: pune i@q,r,d.",
            make: || Box::new(brand_util::B(genial::Genial::new())),
        },
        Playable {
            id: "pelerin",
            title: "Drumul Pelerinului (călătorie cu popasuri pe o potecă)",
            category: "Familie",
            rules: "Variantă originală: potecă cu 30 de popasuri, hanuri obligatorii la 6, 13, 20, 29. Muți cel din urmă: mut k pe un popas liber înainte, apoi efectul (priveliște, atelier: cumpar/treci, izvor, fermă, templu: dau 0-3, han: mananc/sar). Câștigă cine are mai multe puncte (3 monede = 1 punct). Mutări: mut k, cumpar, treci, dau n, mananc, sar.",
            make: || Box::new(brand_util::B(pelerin::Pelerin::new())),
        },
        Playable {
            id: "morcovi",
            title: "Cursa Morcovilor (cursă cu resursă de energie)",
            category: "Familie",
            rules: "Variantă originală: potecă cu 41 de câmpuri; avans k costă k(k+1)/2 morcovi. Câmpuri cu morcovi (avans k:iau / avans k:dau), salată la 12, 24, 36 (oprire obligatorie), inapoi (primești 10 morcovi pe câmp), odihna. Câștigă primul care ajunge la 40 având cel mult 20 de morcovi. Mutări: avans k[:iau|:dau], inapoi, odihna, treci.",
            make: || Box::new(brand_util::B(morcovi::Morcovi::new())),
        },
        Playable {
            id: "rapide",
            title: "Rapidele (canoe pe râu spre cascadă)",
            category: "Familie",
            rules: "Variantă originală: râu cu 22 de câmpuri și cascadă la 21; 3 canoe fiecare, mâini deschise de cărți-vâsle 1–4. vasla c,k mută canoea c cu k câmpuri și încarcă pietre; acostez c la malurile 12, 16, 20 pune încărcătura la loc sigur. Curentul împinge canoele cu 1 la finalul rundei; la cascadă pierzi tot. Câștigă cine are mai multe puncte. Mutări: vasla c,k, acostez c.",
            make: || Box::new(brand_util::B(rapide::Rapide::new())),
        },
        Playable {
            id: "vapor",
            title: "Vaporul de pe Râu (cursă cu viteză și cărbune)",
            category: "Familie",
            rules: "Variantă originală: râu cu 60 de câmpuri și 3 benzi; mutarea v+1,b2 schimbă viteza (±1 gratis, ±2 costă 1 cărbune; 1–4) și banda. Bancurile de nisip te opresc și costă cărbune, pasagerii dau puncte, primul la capăt +4, al doilea +2. Câștigă cine are mai multe puncte. Mutări: v+1,b2.",
            make: || Box::new(brand_util::B(vapor::Vapor::new())),
        },
        Playable {
            id: "lumina",
            title: "Lumina Pădurii (copaci cu umbre pe tablă hexagonală)",
            category: "Familie",
            rules: "Variantă originală: 12 zile, soarele își schimbă direcția; copacii mai înalți fac umbră. Pui 2 copaci (pune q,r), apoi planta (1 lumină), creste, culeg (copac 3, 4 lumină, puncte după inel), pas. Lumina o primești la finalul zilei pentru copacii neumbriți. Câștigă cine are mai multe puncte. Mutări: pune/planta/creste/culeg q,r, pas.",
            make: || Box::new(brand_util::B(lumina::Lumina::new())),
        },
        Playable {
            id: "zgarie_nori",
            title: "Cartierul Înalt (majorități de zgârie-nori)",
            category: "Familie",
            rules: "Variantă originală: 16 cartiere, cărți deschise de 4 pe rundă; etaj n adaugă un etaj în cartierul cărții, muta a>n îți mută etajul de sus dintr-un cartier vecin, arunc n. La finalul celor 3 runde cartierul dă punctele-etaje celui cu majoritatea (egal: cine e deasupra). Câștigă cine are mai multe puncte. Mutări: etaj n, muta a>n, arunc n.",
            make: || Box::new(brand_util::B(zgarie_nori::ZgarieNori::new())),
        },
        Playable {
            id: "turnuri_piatra",
            title: "Turnuri de Piatră (cavaleri pe castele în creștere)",
            category: "Familie",
            rules: "Variantă originală pe tablă 6×6: fiecare are 4 cavaleri; pune c3, mut a1>a2 (cel mult o treaptă mai sus), etaj c3 (treaptă pe celulă vecină). Câștigă cine urcă primul un cavaler pe înălțimea 4; după 80 de acțiuni, cine are cavalerii mai sus. Mutări: pune c3, mut a1>a2, etaj c3.",
            make: || Box::new(brand_util::B(turnuri_piatra::TurnuriPiatra::new())),
        },
        Playable {
            id: "templele",
            title: "Templele din Junglă (explorare și temple pe grilă 5×5)",
            category: "Familie",
            rules: "Variantă originală: 3 exploratori, 3 acțiuni pe tură: mut a1>a2, templu c3 (+1 etaj), comoara c3, pas. De 3 ori, după 8 ture, fiecare templu dă etajele celui cu mai mulți exploratori pe celulă sau în vecinătate. Câștigă cine are mai multe puncte. Mutări: mut a1>a2, templu c3, comoara c3, pas.",
            make: || Box::new(brand_util::B(templele::Templele::new())),
        },
        Playable {
            id: "caravana",
            title: "Caravana de Mirodenii (negoț între doi neguțători)",
            category: "Familie",
            rules: "Variantă originală: 6 mărfuri și cămile, piață de 5 cărți, mâini deschise (max 7). Mutări: ia M, camile, schimb A:B, vand M n (diamantele, aurul și argintul se vând doar câte cel puțin 2; bonus la 3+ cărți). Jocul se termină când 3 feluri de jetoane s-au golit sau piața nu se mai reface; cine are mai multe cămile +5. Câștigă cine are mai multe puncte. Mutări: ia M, camile, schimb A:B, vand M n.",
            make: || Box::new(brand_util::B(caravana::Caravana::new())),
        },
        Playable {
            id: "poteca",
            title: "Poteca Pietrelor (cinci poteci de culori)",
            category: "Familie",
            rules: "Variantă originală: cărți de 5 culori (1–10), mâini deschise de 8. Joci (joc C n pe poteca culorii dacă e mai mare decât ultima, sau arunc C n), apoi tragi (trage | ia C). Fiecare potecă valorează −4, −3, −1, 2, 6, 11, 17, 24, 32, 41 după număr de cărți. Câștigă scorul mai mare când pachetul e gol. Mutări: joc C n, arunc C n, trage, ia C.",
            make: || Box::new(brand_util::B(poteca::Poteca::new())),
        },
        Playable {
            id: "evadare",
            title: "Evadarea din Temniță (pirați care fug printr-un tunel)",
            category: "Familie",
            rules: "Variantă originală: tunel cu 36 de pătrate (6 simboluri) și barcă; fiecare are 6 pirați și mâna deschisă de 6 cărți-simbol. 3 acțiuni pe tură: avans p,S (cel mai apropiat pătrat liber cu simbolul S), retras p (trage o carte), gata. Câștigă primul care îmbarcă toți pirații. Mutări: avans p,S, retras p, gata.",
            make: || Box::new(brand_util::B(evadare::Evadare::new())),
        },
        Playable {
            id: "cetate_intrigi",
            title: "Intrigile Cetății (alegere de roluri și construcție de cartiere)",
            category: "Familie",
            rules: "Variantă originală pentru 2: 6 roluri (Asasin, Hoț, Rege, Negustor, Arhitect, Căpitan), fiecare ia câte 2 pe rundă (alege r). Cu rolul tău: tinta r, aur sau carte, construiesc i (3 pentru Arhitect) sau gata, distrug i / nu (Căpitan). Mâini deschise. La 7 cartiere se termină; scor = costuri + 3 culori complete + 2 primul. Mutări: alege r, tinta r, aur, carte, construiesc i, gata, distrug i, nu.",
            make: || Box::new(brand_util::B(cetate_intrigi::Cetate::new())),
        },
        Playable {
            id: "menajerie",
            title: "Menajeria (cărucioare cu animale și țarcuri)",
            category: "Familie",
            rules: "Variantă originală: 5 specii, 3 cărucioare de 3 locuri. trage (apoi pune k) sau ia k (luat căruciorul în țarcurile tale: capacități 3, 4, 5, o specie în fiecare) și ieși din rundă. Scor: pătratul numărului de animale per țarc (+2 dacă e plin). Câștigă scorul mai mare. Mutări: trage, pune k, ia k.",
            make: || Box::new(brand_util::B(menajerie::Menajerie::new())),
        },
        Playable {
            id: "tinut_zane",
            title: "Ținutul Zânelor (călătorie cu cărți de transport)",
            category: "Familie",
            rules: "Variantă originală: 10 orașe în cerc cu scurtături, drumuri cu teren și cost 1–2. ia t (carte din piața de 4) sau mut X plătind cărțile potrivite; primul care ajunge într-un oraș ia valoarea lui (1–3). Se termină la 8 orașe revendicate sau după 80 de mutări. Câștigă scorul mai mare. Mutări: ia t, mut X, descarto t.",
            make: || Box::new(brand_util::B(tinut_zane::TinutZane::new())),
        },
        Playable {
            id: "postalioni",
            title: "Poștalionii (rute de livrare între orașe)",
            category: "Familie",
            rules: "Variantă originală: 12 orașe în cerc cu scurtături, mâini deschise. joc X (extinde ruta la un capăt vecin), nou X, livrez (rută ≥3: 3, 6, 10, 15, 21 puncte), apoi ia k (din 4 cărți deschise) sau trage. Se termină când pachetul e gol. Câștigă scorul mai mare. Mutări: joc X, nou X, livrez, ia k, trage.",
            make: || Box::new(brand_util::B(postalioni::Postalioni::new())),
        },
        Playable {
            id: "cazanul",
            title: "Cazanul Vrăjit (pariu cu pungi de ingrediente)",
            category: "Familie",
            rules: "Variantă originală: tragi ingrediente din punga ta (scot, forțat), apoi mai sau opresc. Albele (a1, a2, a3) explodează cazanul peste 7. Fără explozie: monede = valoarea și 1 punct la 4; cu explozie alegi bani sau puncte. Cumperi ingrediente (cumpar a3/r2/z3, gata). 6 runde; câștigă cine are mai multe puncte. Mutări: scot, mai, opresc, bani, puncte, cumpar X, gata.",
            make: || Box::new(brand_util::B(cazanul::Cazanul::new())),
        },
        Playable {
            id: "santier",
            title: "Șantierul Piramidei (pietre pe corăbii spre șantiere)",
            category: "Familie",
            rules: "Variantă originală: 5 runde, 4 corăbii și 4 șantiere (piramidă, obelisc, templu, piață). iau (+2 pietre), incarc s, navighez s,k, pas. Pietrele se descarcă în ordinea locurilor: piramida dă 1, 2, 3, ... puncte, obeliscul n×n/2 la final, templul pe perechi, piața 1 punct și 1 piatră. Câștigă scorul mai mare. Mutări: iau, incarc s, navighez s,k, pas.",
            make: || Box::new(brand_util::B(santier::Santier::new())),
        },
        Playable {
            id: "marele_bazar",
            title: "Marele Bazar (neguțători pe o grilă de locații)",
            category: "Familie",
            rules: "Variantă originală: grilă 4×4 de locații (piețe, bănci, ateliere, palate, bijutieri, fântâni). mut a1>b2 (1–2 pași ortogonali; pe adversar plătești 2 lire), apoi se aplică automat acțiunea locației. Câștigă primul cu 5 rubine; după 100 de mutări, mai multe rubine și lire. Mutări: mut a1>b2.",
            make: || Box::new(brand_util::B(marele_bazar::MareleBazar::new())),
        },
        Playable {
            id: "curierii_vraji",
            title: "Curierii Vrăjitoarelor (roluri curajoase sau fricoase)",
            category: "Familie",
            rules: "Variantă originală pe 8 locuri: 4 roluri (muta, colecteaza, livreaza, odihna), fiecare în mod f (fricos, sigur) sau c (curajos, mare). Un rol curajos rămâne în așteptare: dacă adversarul joacă același rol, curajosul se anulează. muta:f>n, rol:f|c. 4 runde; câștigă scorul mai mare. Mutări: muta:f>n, colecteaza:c, livreaza:f, odihna:c.",
            make: || Box::new(brand_util::B(curierii_vraji::Curierii::new())),
        },
        Playable {
            id: "vamesul",
            title: "Vameșul din Poartă (bluf cu geanta de negustor)",
            category: "Familie",
            rules: "Variantă originală: 6 runde, rolurile alternează. Negustorul pune 1–4 cărți în geantă și declară un fel legal (sac a,b,c,d>t); vameșul alege las sau inspectez, apoi verific. Cinstit și inspectat: vameșul plătește; altfel penalități, contrabanda valorează 6. Informație ascunsă: mâna și geanta. Câștigă monede + marfă mai multe. Mutări: sac a,b,c,d>t, las, inspectez, verific.",
            make: || Box::new(brand_util::B(vamesul::Vamesul::new())),
        },
        Playable {
            id: "agenti",
            title: "Agenții Tăcuți (agenți pe o stradă cu valori secrete)",
            category: "Familie",
            rules: "Variantă originală: 6 agenți (A–F) pe o stradă de 10 pași; fiecare jucător are în secret valori distincte 0–5. Pe rând muți orice agent cu 1–3 (mut X,k). După 24 de mutări scorul = Σ valoarea ta × poziția. Informație ascunsă: vezi doar valorile tale. Câștigă scorul mai mare. Mutări: mut X,k.",
            make: || Box::new(brand_util::B(agenti::Agenti::new())),
        },
        Playable {
            id: "tablou",
            title: "Schița Ascunsă (indicii dintr-un tablou)",
            category: "Familie",
            rules: "Variantă originală: 6 cărți-tablou cu câte 4 simboluri (A–J); una e ținta, cunoscută doar de schițator. Schițatorul spune 1–3 simboluri adevărate (pun S, gata); celălalt ghicește (ghic k, apoi verific). Corect: ghicitor +2, schițator +1; greșit: schițator +2. 6 runde. Informație ascunsă: ținta. Mutări: pun S, gata, ghic k, verific.",
            make: || Box::new(brand_util::B(tablou::Tablou::new())),
        },
        Playable {
            id: "detectiv",
            title: "Orașul Detectivilor (urmărirea suspecților pe hartă)",
            category: "Familie",
            rules: "Variantă originală: grilă 6×6, 6 suspecți, 8 pași de timp; crima la pasul 5 în celula X. arata s,t dezvăluie public poziția suspectului s la pasul t (apoi afisez), acuz s acuză (apoi verific): corect câștigi, greșit pierzi. Suspecții care nu puteau ajunge în X sunt eliminați. Informație ascunsă: traseele. Mutări: arata s,t, afisez, acuz s, verific.",
            make: || Box::new(brand_util::B(detectiv::Detectiv::new())),
        },
        Playable {
            id: "tinuturi_negociate",
            title: "Ținuturi Negociate (prețul tău, alegerea mea)",
            category: "Familie",
            rules: "Variantă originală: 8 runde. Se dezvăluie 3 plăcuțe de teren (scot); vânzătorul pune prețuri (pret a,b), cumpărătorul alege una (ia k) și o așază (pune r,c), apoi vânzătorul ia una din rest. Peisaj 5×5; grupurile conexe valorează n(n+1)/2 × valoarea terenului, plus monedele. Câștigă scorul mai mare. Mutări: scot, pret a,b, ia k, pune r,c.",
            make: || Box::new(brand_util::B(tinuturi_negociate::TinuturiNegociate::new())),
        },
        Playable {
            id: "peisaj",
            title: "Peisaj Liniștit (plăcuțe hexagonale cu margini de teren)",
            category: "Familie",
            rules: "Variantă originală: plăcuțe hexagonale cu 6 margini de teren (p pădure, c câmp, a apă, s sat). Pe rând așezi plăcuța curentă (se vede și următoarea) lipită de cele puse: pune q,r,rot. Fiecare margine potrivită +1, nepotrivită −1. 36 de plăcuțe; câștigă scorul mai mare. Mutări: pune q,r,rot.",
            make: || Box::new(brand_util::B(peisaj::Peisaj::new())),
        },
        Playable {
            id: "cariera",
            title: "Cariera Visată (formula succesului și zarul)",
            category: "Familie",
            rules: "Variantă originală: fiecare alege public o formulă (tinta 1–4) de faimă / avere / fericire; apoi zar (forțat) și munca f|a|h (+d, dublu cu cariera potrivită) sau schimb A|N|F (alegi cariera). Câștigă primul care atinge toate țintele; după 160 de ture, cine e mai aproape. Mutări: tinta k, zar, munca f|a|h, schimb A|N|F.",
            make: || Box::new(brand_util::B(cariera::Cariera::new())),
        },
        Playable {
            id: "salariu",
            title: "Ziua de Salariu (cursă de buget pe trei luni)",
            category: "Familie",
            rules: "Variantă originală: 3 luni × 30 de zile. zar (forțat) te mută; poștă (−6..+4), ofertă (cumpar 5 monede, vinzi cu 9 la o zi de vânzare, sau las), salariu +8 la fiecare 30 de zile. Câștigă cine are mai mulți bani la final (loturile nevândute valorează 3). Mutări: zar, cumpar, las.",
            make: || Box::new(brand_util::B(salariu::Salariu::new())),
        },
        Playable {
            id: "plapuma",
            title: "Plapuma (petice, timp și nasturi)",
            category: "Familie",
            rules: "Variantă originală: plapumă 7×7, 16 petice în cerc cu cost în nasturi, timp și venit. ia i@r,c,o (unul din 3 petice după marcaj) sau avans (cu un loc înaintea adversarului, primești nasturi). Mută cine e în urmă. Pista are 40 de locuri; venit la 8, 16, 24, 32. Scor: nasturi + 2/celulă acoperită − 2/celulă goală. Mutări: ia i@r,c,o, avans.",
            make: || Box::new(brand_util::B(plapuma::Plapuma::new())),
        },
        Playable {
            id: "pisici",
            title: "Pisicile din Barcă (pisici-poliomino pe barcă)",
            category: "Familie",
            rules: "Variantă originală: 5 runde; se dezvăluie 4 pisici (scot), se aleg pe rând (ia k) și se așază pe barca 5×6 (pune r,c,o) sau se aruncă (arunc, −1). Scor: celule acoperite + valoarea pisicilor + 3 pe rând plin. Câștigă scorul mai mare. Mutări: scot, ia k, pune r,c,o, arunc.",
            make: || Box::new(brand_util::B(pisici::Pisici::new())),
        },
        Playable {
            id: "triunghiuri",
            title: "Triunghiurile Numerotate (domino triunghiular)",
            category: "Familie",
            rules: "Variantă originală, mâini deschise: 35 de plăcuțe triunghiulare cu numere 0–4 în colțuri. pune i@x,y,s,k pe o celulă lipită de cele puse (s = u sau d, k rotația), cu colțurile comune egale; scor = suma plăcuței. trage / pas dacă nu poți. Cine rămâne fără plăcuțe ia 10 + restul adversarului. Mutări: pune i@x,y,s,k, trage, pas.",
            make: || Box::new(brand_util::B(triunghiuri::Triunghiuri::new())),
        },
        Playable {
            id: "banane",
            title: "Banane-Cuvinte (cursă de cuvinte cu literele din mână)",
            category: "Familie",
            rules: "Variantă originală: fiecare are 10 litere deschise; cuvant xyz (≥3 litere, din dicționar, folosește literele din mână, apoi tragi una), arunc l (înapoi în sac, tragi 3), pas. Câștigă primul fără litere; după 80 de mutări, mai puține litere. Mutări: cuvant xyz, arunc l, pas.",
            make: || Box::new(brand_util::B(banane::Banane::new())),
        },
        Playable {
            id: "turnuri_litere",
            title: "Turnuri de Litere (cuvinte încrucișate cu litere suprapuse)",
            category: "Familie",
            rules: "Variantă originală: tablă 6×6 cu stive de până la 3 litere, mâna de 7 litere ascunsă. pune L@r,c (o literă, pe celulă goală sau peste stivă, cu altă literă); toate șirurile de ≥2 celule prin ea trebuie să fie cuvinte din dicționar. Scor: suma înălțimilor cuvintelor. pas. Informație ascunsă: mâna. Mutări: pune L@r,c, pas.",
            make: || Box::new(brand_util::B(turnuri_litere::TurnuriLitere::new())),
        },
        Playable {
            id: "lanturi",
            title: "Lanțuri Hoteliere (fuziuni și acțiuni pe o tablă mică)",
            category: "Familie",
            rules: "Variantă originală: tablă 5×6, 3 lanțuri A, B, C, piață de 4 plăcuțe. pune b3 (fără vecini: izolată; lângă izolate: fond A; lângă lanț: îl crește; între două: fuziune, lanțul mic dispare cu bonus), apoi cumperi până la 3 acțiuni (compro A, gata). La final se vând acțiunile și se dau bonusuri. Câștigă banii mai mulți. Mutări: pune b3, fond A, compro A, gata.",
            make: || Box::new(brand_util::B(lanturi::Lanturi::new())),
        },
        Playable {
            id: "bambuzeria",
            title: "Bambuzeria (grădinar, panda și obiective de bambus)",
            category: "Familie",
            rules: "Variantă originală: grădină 5×5 în trei culori. O acțiune pe tură: gradinar c3 (crește parcela și vecinii de aceeași culoare), panda c3 (mănâncă un segment), ploaie c3 (+1; maxim 2). Obiectivele (3 deschise) se completează automat. Primul la 20 de puncte câștigă; după 60 de acțiuni, scorul mai mare. Mutări: gradinar c3, panda c3, ploaie c3.",
            make: || Box::new(brand_util::B(bambuzeria::Bambuzeria::new())),
        },
        // B2-reg-end
        // B6-reg-begin
        Playable {
            id: "vorbe_scurte",
            title: "Descrierea în trei vorbe",
            category: "Party",
            rules: "Variantă originală: o țintă ascunsă dintre 12 elemente (A tip 1–3, B mărime 1–2, C mișcare 1–2). Cine descrie dă cel mult 3 indicii adevărate, apoi celălalt ghicește; rolurile alternează, 6 runde. Informație ascunsă: ținta. Mutări: ind X=v, gata, ghic k, verific.",
            make: || Box::new(brand_util::B(vorbe_scurte::new())),
        },
        Playable {
            id: "semne_icoane",
            title: "Limbajul Icoanelor",
            category: "Party",
            rules: "Variantă originală: o țintă ascunsă dintre 32 de elemente cu patru atribute (icoană, culoare, număr, formă). Cine „vorbește” doar prin icoane poate pune până la 4 indicii adevărate; celălalt ghicește. 6 runde, rolurile alternează. Mutări: ind X=v, gata, ghic k, verific.",
            make: || Box::new(brand_util::B(semne_icoane::new())),
        },
        Playable {
            id: "grunturi",
            title: "Grunțuri Alese",
            category: "Party",
            rules: "Variantă originală: doar „cuvinte scurte” permise: din cele trei atribute ale țintei (sunet, mărime, culoare) poți indica doar primele două, cel mult 2 indicii. Ținta e ascunsă; ghicitorul alege dintre 18 elemente. 6 runde. Mutări: ind X=v, gata, ghic k, verific.",
            make: || Box::new(brand_util::B(grunturi::new())),
        },
        Playable {
            id: "gesturi_mute",
            title: "Mimica Mută",
            category: "Party",
            rules: "Variantă originală: mimul nu spune cuvinte, doar arată doi dintre cei trei „gesturi” (atributele B și C ale țintei dintre 18 elemente), cel mult 2 indicii; ghicitorul alege ținta. 6 runde, rolurile alternează. Mutări: ind X=v, gata, ghic k, verific.",
            make: || Box::new(brand_util::B(gesturi_mute::new())),
        },
        Playable {
            id: "trei_runde",
            title: "Aceleași Cărți, Trei Runde",
            category: "Party",
            rules: "Variantă originală: o țintă ascunsă dintre 18 elemente; limita de indicii scade pe parcurs: 3, 3, 2, 2, 1, 1 în cele 6 runde (ca la „aceleași cărți, reguli tot mai stricte”). Cine descrie și cine ghicește alternează. Mutări: ind X=v, gata, ghic k, verific.",
            make: || Box::new(brand_util::B(trei_runde::new())),
        },
        Playable {
            id: "schite_rapide",
            title: "Schițe Grăbite",
            category: "Party",
            rules: "Variantă originală: schițatorul „desenează” țintei (dintre 20 de elemente) trăsături adevărate, cel mult 3 (A formă 1–5, B contur 1–2, C umbră 1–2); celălalt ghicește. 6 runde, rolurile alternează. Mutări: ind X=v, gata, ghic k, verific.",
            make: || Box::new(brand_util::B(schite_rapide::new())),
        },
        Playable {
            id: "crima_din_port",
            title: "Crima din Port: Ancheta",
            category: "Party",
            rules: "Variantă originală: criminalul (dintre 36 de suspecți, cu patru atribute: armă, indiciu, loc, motiv) e cunoscut doar de expert, care poate lăsa cel mult 3 indicii; anchetatorul ghicește. 6 runde, rolurile alternează. Mutări: ind X=v, gata, ghic k, verific.",
            make: || Box::new(brand_util::B(crima_din_port::new())),
        },
        Playable {
            id: "primarul_si_lupul",
            title: "Primarul și Lupul",
            category: "Party",
            rules: "Variantă originală: primarul știe ținta (12 elemente cu atributele tip, mărime, culoare) și răspunde adevărat; celălalt jucător are cel mult 5 întrebări (intreb X=v) și apoi ghicește. Rolurile alternează, 6 runde. Mutări: intreb X=v, ghic k, raspuns, verific.",
            make: || Box::new(brand_util::B(primarul_si_lupul::new())),
        },
        Playable {
            id: "intrusul_tacut",
            title: "Intrusul Tăcut",
            category: "Party",
            rules: "Variantă originală: un obiect secret dintre 16 (rând A, coloană B, valori 1–4), necunoscut ambilor; pe rând întrebați da/nu (cel mult 6 întrebări în total, răspunsurile sunt publice), iar oricând cineva poate ghici: corect +2, greșit +1 pentru celălalt. 6 runde. Mutări: intreb X=v, ghic k, raspuns, verific.",
            make: || Box::new(brand_util::B(intrusul_tacut::new())),
        },
        Playable {
            id: "bentita",
            title: "Banderola pe Frunte",
            category: "Party",
            rules: "Variantă originală: fiecare are un obiect (dintre 18) pe care îl vede doar adversarul. Pe rând puneți întrebări da/nu despre al vostru (intreb X=v, cel mult 5 pe jucător) sau ghiciți: corect +2, greșit +1 pentru celălalt. 6 runde. Mutări: intreb X=v, ghic k, raspuns, verific.",
            make: || Box::new(brand_util::B(bentita::new())),
        },
        Playable {
            id: "povesti_negre",
            title: "Cazuri Întunecate",
            category: "Party",
            rules: "Variantă originală: un caz ascuns dintre 32, definit de cinci fapte da/nu (noaptea, afară, singur, armă, martor). Pe rând întrebați despre un fapt (cel mult 6 întrebări în total, răspunsuri publice) sau ghiciți cazul: corect +2, greșit +1 pentru celălalt. 6 runde. Mutări: intreb X=v, ghic k, raspuns, verific.",
            make: || Box::new(brand_util::B(povesti_negre::new())),
        },
        Playable {
            id: "indicii_cascada",
            title: "Indicii în Cascadă",
            category: "Party",
            rules: "Variantă originală: un cuvânt-țintă ascuns dintre 18; la fiecare mutare ori ceri un indiciu nou (mai mult, cel mult 3), ori ghicești. Primul ghicit corect ia 1 + indiciile nefolosite; greșeala dă 1 adversarului. 6 runde. Mutări: mai mult, ghic k, arata, verific.",
            make: || Box::new(brand_util::B(indicii_cascada::new())),
        },
        Playable {
            id: "zece_usi",
            title: "Zece Uși, Câteva Adevărate",
            category: "Party",
            rules: "Variantă originală: zece uși, dintre care 4 (număr cunoscut) sunt bune, pe o mască ascunsă. Pe rând alegi o ușă (alege i, apoi arata): bună +1 și continui; rea: pierzi punctele nesalvate și tura trece. Cu stop salvezi. 6 întrebări. Mutări: alege i, stop, arata.",
            make: || Box::new(brand_util::B(zece_usi::new())),
        },
        Playable {
            id: "valul_ideilor",
            title: "Valul Ideilor",
            category: "Party",
            rules: "Variantă originală: o listă de 12 idei, 6 dintre ele corecte, pe o mască ascunsă; cele de sus valorează 3, 2 sau 1 punct. Pe rând alegi o idee (alege i, apoi arata): corectă = puncte și continui; greșită = tura trece. 6 liste. Mutări: alege i, arata.",
            make: || Box::new(brand_util::B(valul_ideilor::new())),
        },
        Playable {
            id: "categoria_indrazneata",
            title: "Categoria Îndrăzneață",
            category: "Party",
            rules: "Variantă originală: opt căsuțe pe o mască ascunsă; categoria (cat j: 2, 3, 4 sau 5 căsuțe bune) o alege ADVERSARUL celui la rând. Apoi alegi căsuțe (alege i, arata) ca la „zece uși”: bună +1 și continui; rea: pierzi banca, tura trece; stop salvează. Mutări: cat j, alege i, stop, arata.",
            make: || Box::new(brand_util::B(categoria_indrazneata::new())),
        },
        Playable {
            id: "totem_grabit",
            title: "Totemul Grăbit",
            category: "Party",
            rules: "Variantă originală: 8 cărți-simbol fiecare (4 simboluri). Joci pe masă un simbol (joc s); dacă e la fel ca simbolul de deasupra stivei adversarului, are loc un duel de reflexe (prind), câștigat cu șansa 50% ± 8% pe serie; pierzătorul ia toate cărțile de pe masă. Câștigă cine rămâne fără cărți în mână. Mutări: joc s, prind.",
            make: || Box::new(brand_util::B(totem_grabit::new())),
        },
        Playable {
            id: "palmuta_pe_cuvant",
            title: "Pălmuța pe Cuvânt",
            category: "Party",
            rules: "Variantă originală: se numără în ritm cinci cuvinte (pâine, sare, vin, miere, lapte), ciclic. Joci un simbol (joc s) pe masă; dacă e cel numărat acum, toți dau palma (prind), cel mai lent ia masa. Câștigă cine rămâne primul fără cărți în mână (8 cărți, 5 simboluri). Mutări: joc s, prind.",
            make: || Box::new(brand_util::B(palmuta_pe_cuvant::new())),
        },
        Playable {
            id: "duel_categorii",
            title: "Duel de Categorii",
            category: "Party",
            rules: "Variantă originală: 10 cărți-simbol fiecare (6 simboluri). Joci un simbol (joc s); dacă se potrivește cu cel de deasupra stivei adversarului, începe un duel de categorii (prind), câștigat cu șansa 50% ± 8% pe serie; pierzătorul ia masa. Câștigă primul fără cărți în mână. Mutări: joc s, prind.",
            make: || Box::new(brand_util::B(duel_categorii::new())),
        },
        Playable {
            id: "duelul_argumentelor",
            title: "Duelul Argumentelor",
            category: "Party",
            rules: "Variantă originală: mână de 5 cărți cu trei valori 0–4 (ridicol, convingător, periculos). Alegeți în secret două cărți (joc i,j); publicul are gusturi ascunse 0–3 (unul arătat) și scorul fiecărei alegeri e suma valorilor × gusturi; cel mai mare ia runda. 6 runde. Mutări: joc i,j, voteaza.",
            make: || Box::new(brand_util::B(duelul_argumentelor::new())),
        },
        Playable {
            id: "legenda_potrivita",
            title: "Legenda Potrivită",
            category: "Party",
            rules: "Variantă originală: pentru fiecare imagine-scenă (public, trei valori 0–3) alegeți în secret o legendă din mână (joc i); publicul are gusturi ascunse, iar scorul = valorile legendei × (gusturi + scenă). Cel mai mare ia runda. 6 runde. Mutări: joc i, voteaza.",
            make: || Box::new(brand_util::B(legenda_potrivita::new())),
        },
        Playable {
            id: "banda_nebuna",
            title: "Banda Desenată Nebună",
            category: "Party",
            rules: "Variantă originală: banda continuă: scena unei runde e cartea câștigătoare din runda precedentă. Alegeți în secret o carte (joc i); publicul are gusturi ascunse; scorul = valorile cărții × (gusturi + scenă). Cel mai mare ia runda. 6 runde. Mutări: joc i, voteaza.",
            make: || Box::new(brand_util::B(banda_nebuna::new())),
        },
        Playable {
            id: "favoritul_judecatorului",
            title: "Favoritul Judecătorului",
            category: "Party",
            rules: "Variantă originală: judecătorul alege în secret un favorit dintre 5 răspunsuri (alege f); celălalt ghicește cu miza 1–3 (ghic g s). Corect: ghicitorul +miză; greșit: judecătorul +1. Istoricul alegerilor e public. 8 runde, rolurile alternează. Mutări: alege f, ghic g s, dezvalui.",
            make: || Box::new(brand_util::B(favoritul_judecatorului::new())),
        },
        Playable {
            id: "ganduri_impartasite",
            title: "Gânduri Împărtășite",
            category: "Party",
            rules: "Variantă originală: unul alege în secret un răspuns din 6 (alege f), celălalt ghicește (ghic g). Ghicitorul primește max(0, 3 − |f − g|), cel care a ales primește min(3, |f − g|). Istoricul e public. 8 runde, rolurile alternează. Mutări: alege f, ghic g, dezvalui.",
            make: || Box::new(brand_util::B(ganduri_impartasite::new())),
        },
        Playable {
            id: "obiect_fulger",
            title: "Obiectul Fulger",
            category: "Party",
            rules: "Variantă originală: cinci obiecte cu culori naturale; pe fiecare carte apar două, în culori oarecare. Fiecare alege în secret obiectul de prins după regula noastră (prind k); apoi verific: corect +1, greșit −1; dacă ambii au nimerit, cel mai rapid (șansă pe serie) ia cartea. 16 cărți. Mutări: prind k, verific.",
            make: || Box::new(brand_util::B(obiect_fulger::ObiectFulger::new())),
        },
        Playable {
            id: "fitilul_cuvintelor",
            title: "Fitilul Cuvintelor",
            category: "Party",
            rules: "Variantă originală: un fitil ascuns durează 6–14 cuvinte. Pe rând spui 1–3 cuvinte (spun k), apoi suna: dacă fitilul s-a ars, cel care a spus ultimul pierde, iar celălalt primește cuvintele lui notate în rundă. 5 runde. Mutări: spun k, suna.",
            make: || Box::new(brand_util::B(fitilul_cuvintelor::Fitil::new())),
        },
        Playable {
            id: "pariu_pe_cifre",
            title: "Pariu pe Cifre",
            category: "Party",
            rules: "Variantă originală: o cifră ascunsă T (0–99) cu interval public de 40. Fiecare estimează în secret (est g), apoi pariază pe o estimare cu 1–3 jetoane (pariez j s). Cea mai apropiată estimare aduce +2 autorului și +jetoanele pariorilor. 5 runde. Mutări: est g, pariez j s, verific.",
            make: || Box::new(brand_util::B(pariu_pe_cifre::Pariu::new())),
        },
        Playable {
            id: "provocari_trasnite",
            title: "Provocări Trăsnite",
            category: "Party",
            rules: "Variantă originală: în fiecare rundă se oferă 3 sarcini (recompensă, șansă de reușită); cel la rând alege una (iau k), adversarul pariază pe da/nu/fara, apoi incerc decide cu zar ascuns. Reușită +r, ratată −1; pariu corect +1, greșit −1. 8 runde. Mutări: iau k, pe da, pe nu, fara, incerc.",
            make: || Box::new(brand_util::B(provocari_trasnite::Trasnite::new())),
        },
        Playable {
            id: "provocarea_mintii",
            title: "Provocarea Minții",
            category: "Party",
            rules: "Variantă originală: cel la rând alege dificultatea (dif 1–3) a unui șir de patru termeni (aritmetic, geometric sau alternat); răspunde cu al cincilea (rasp n). Corect +dificultatea, greșit +1 adversarului. 8 runde. AI-ul rezolvă șirul (nu greșește). Mutări: dif d, rasp n.",
            make: || Box::new(brand_util::B(provocarea_mintii::Mintii::new())),
        },
        Playable {
            id: "zaruri_povestit",
            title: "Zaruri de Povestit",
            category: "Party",
            rules: "Variantă originală: 12 zaruri cu șase simboluri la vedere. Pe rând luați un zar și îl puneți la fața sau la spatele lanțului vostru (ia k f|s); o legătură a→b valorează după (b−a) mod 6: 1, 3, 0, 1, 0, 2. Câștigă lanțul cu scorul mai mare. Mutări: ia k f, ia k s.",
            make: || Box::new(brand_util::B(zaruri_povestit::Zaruri::new())),
        },
        Playable {
            id: "definitii_false",
            title: "Definiții Mincinoase",
            category: "Party",
            rules: "Variantă originală: opt definiții cu plauzibilitate publică 1–9; adevărul (șansă proporțională cu plauzibilitatea) îl știe doar scriitorul, care alege o momeală (momeala b). Ghicitorul alege între două variante (cred 1|2). Ghicit: ghicitorul +2; păcălit: scriitorul +2. 6 runde. Mutări: momeala b, cred k, dezvalui.",
            make: || Box::new(brand_util::B(definitii_false::Definitii::new())),
        },
        Playable {
            id: "ciocnirea_literelor",
            title: "Ciocnirea Literelor",
            category: "Party",
            rules: "Variantă originală: în fiecare rundă se stabilește lungimea cuvintelor (4–6 litere). Pe rând spuneți un cuvânt din dicționar (cuv CUVANT) a cărui literă de început nu a fost încă folosită; cine nu mai are variante (nu am) pierde runda. 4 runde. Mutări: cuv CUVANT, nu am.",
            make: || Box::new(brand_util::B(ciocnirea_literelor::Ciocnire::new())),
        },
        Playable {
            id: "nuanta_ghicita",
            title: "Nuanța Ghicită",
            category: "Party",
            rules: "Variantă originală: o grilă 8×8; ținta e cunoscută doar de cel care dă indicii despre bandă (rand b, col b, cel mult 2), celălalt pune un marcaj (ghic r c). Exact 3, vecin 2, la distanță 2 → 1 punct; indicatorul primește 1 (+1 pentru un singur indiciu) dacă ghicitul ia ≥2. 6 runde. Mutări: rand b, col b, gata, ghic r c, verific.",
            make: || Box::new(brand_util::B(nuanta_ghicita::Nuanta::new())),
        },
        Playable {
            id: "camere_fitil",
            title: "Două Camere și un Fitil",
            category: "Party",
            rules: "Variantă originală: șase personaje în două camere. J1 știe în secret Președintele, J2 Artificierul. Fiecare mută de trei ori (muta c); la final verific: dacă sunt în aceeași cameră câștigă J2, altfel J1. Mutări: muta c, verific.",
            make: || Box::new(brand_util::B(camere_fitil::Camere::new())),
        },
        Playable {
            id: "aruncari_cupe",
            title: "Aruncări în Cupe",
            category: "Party",
            rules: "Variantă originală: șase cupe în triunghi la fiecare; la rând arunci de două ori (tintesc c, apoi arunc). Șanse 65/50/35% după rând, +10% la cel mult 3 cupe rămase; două reușite la rând dau o aruncare în plus. Pierde cine rămâne fără cupe. Mutări: tintesc c, arunc.",
            make: || Box::new(brand_util::B(aruncari_cupe::Cupe::new())),
        },
        Playable {
            id: "duel_gloante",
            title: "Duel cu Gloanțe Ascunse",
            category: "Party",
            rules: "Variantă originală: fiecare are un încărcător de 8 cărți (5 încărcate) în ordine ascunsă. În fiecare rundă alegeți în secret foc, scut (max. 2) sau pas; apoi rezolv. Un foc cu carte încărcată rănește dacă adversarul nu s-a ferit. 3 vieți, 8 runde. Mutări: foc, scut, pas, rezolv.",
            make: || Box::new(brand_util::B(duel_gloante::Duel::new())),
        },
        Playable {
            id: "vama_bacsis",
            title: "Vama cu Bacșiș",
            category: "Party",
            rules: "Variantă originală: negustorul pune 1–3 cărți în sac (sac a,b,c,d), declară un tip legal și un bacșiș (decl t m); vameșul lasă sau inspectează, apoi verific. Cinstit și inspectat: vameșul plătește 2 pe carte; altfel amendă 3 pe carte nepotrivită. 6 runde. Mutări: sac a,b,c,d, decl t m, las, inspectez, verific.",
            make: || Box::new(brand_util::B(vama_bacsis::Vama::new())),
        },
        Playable {
            id: "gandaci_blufati",
            title: "Gândaci Bluffați",
            category: "Party",
            rules: "Variantă originală: 36 de cărți-gâză (6 feluri), 12 în mână. Joci o carte cu fața în jos și anunți un fel (pas t c); adversarul crede sau nu (adevar/minciuna), apoi arata. Cine greșește sau e prins ia cartea; 4 de același fel pierd. Mutări: pas t c, adevar, minciuna, arata.",
            make: || Box::new(brand_util::B(gandaci_blufati::Gandaci::new())),
        },
        Playable {
            id: "bal_masti",
            title: "Balul Măștilor Schimbate",
            category: "Party",
            rules: "Variantă originală: fiecare are o mască (rol) pe care o vede doar dacă se uită (privesc); schimb/simulez nu se pot deosebi; rol R anunță un rol (accept sau provoc, apoi arata). Anunț corect: puterea + acuzatorul −2; fals: anunțatorul −2. Primul la 10 monede. Mutări: privesc, schimb, simulez, rol R, accept, provoc, arata.",
            make: || Box::new(brand_util::B(bal_masti::Bal::new())),
        },
        Playable {
            id: "reguli_schimbatoare",
            title: "Reguli Mereu Schimbate",
            category: "Party",
            rules: "Variantă originală: cărți Obiect, Scop și Regulă; reguli de tras și de jucat (1–3) se schimbă din mers. Joacă obiecte, scopuri sau reguli (joc X); cine are ambele obiecte ale scopului curent câștigă. Informație ascunsă: mâinile. Plafon 80 de mutări. Mutări: joc X, pas.",
            make: || Box::new(brand_util::B(reguli_schimbatoare::Reguli::new())),
        },
        Playable {
            id: "palat_intrigi",
            title: "Intrigi de Palat",
            category: "Party",
            rules: "Variantă originală: patru roluri (Vistier, Călău, Hoț, Paznic), două cărți secrete fiecare. Acțiuni anunțate (taxa, calau, hot) pot fi acuzate (provoc) sau blocate (bloc R); cine minte pierde o influență. venit, lovitura. Pierde cine rămâne fără influență. Mutări: venit, taxa, calau, hot, lovitura, accept, provoc, bloc R, arata, pierd i.",
            make: || Box::new(brand_util::B(palat_intrigi::Palat::new())),
        },
        // B6-reg-end
        Playable {
            id: "plapuma_cal",
            title: "Plapuma cu Petice (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 4×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: grupuri de cel puțin 3 piese la fel: 2 p/piesă; vecini diferiți: 1 p/pereche.",
            make: || crate::brand_grid::make("plapuma_cal"),
        },
        Playable {
            id: "poteci_munte",
            title: "Poteci de Munte (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 4×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Se zidește din margine spre interior: lângă marginea tablei sau lângă o piesă a ta. Scor: rând sau coloană cu toate felurile: 4 p; vecini la fel: 1 p/pereche.",
            make: || crate::brand_grid::make("poteci_munte"),
        },
        Playable {
            id: "poiana_ursului",
            title: "Poiana Ursului (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 5×4 cu piese de 3 feluri (A, B, C), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: grupuri de cel puțin 2 piese la fel: 1 p/piesă; rând sau coloană dintr-un singur fel: 5 p.",
            make: || crate::brand_grid::make("poiana_ursului"),
        },
        Playable {
            id: "strada_case",
            title: "Strada Noilor Case (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 5×4 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: seturi complete: 6 p/set; diagonale la fel: 1 p/pereche.",
            make: || crate::brand_grid::make("strada_case"),
        },
        Playable {
            id: "exploratori",
            title: "Exploratorii Templului (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 4×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Se zidește din margine spre interior: lângă marginea tablei sau lângă o piesă a ta. Scor: patru colțuri la fel: 8 p; vecini la fel: 1 p/pereche; mai multe piese decât adversarul la un fel: 3 p/fel.",
            make: || crate::brand_grid::make("exploratori"),
        },
        Playable {
            id: "orasul_comun",
            title: "Orașul Comun (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 4×4 cu piese de 3 feluri (A, B, C), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: mai multe piese decât adversarul la un fel: 4 p/fel; grupuri de cel puțin 2 piese la fel: 1 p/piesă.",
            make: || crate::brand_grid::make("orasul_comun"),
        },
        Playable {
            id: "atlas_pierdut",
            title: "Atlasul Pierdut (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 5×5 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: rând sau coloană dintr-un singur fel: 6 p; seturi complete: 3 p/set; vecini diferiți: 1 p/pereche.",
            make: || crate::brand_grid::make("atlas_pierdut"),
        },
        Playable {
            id: "parc_dino",
            title: "Parcul Dinozaurilor (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 4×3 cu piese de 6 feluri (A, B, C, D, E), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: rând sau coloană cu toate felurile: 5 p; mai multe piese decât adversarul la un fel: 2 p/fel; vecini la fel: 1 p/pereche.",
            make: || crate::brand_grid::make("parc_dino"),
        },
        Playable {
            id: "asezari_noi",
            title: "Așezările Noi (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 5×5 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: grupuri de cel puțin 4 piese la fel: 2 p/piesă; piese A pe margine: 1 p/piesă.",
            make: || crate::brand_grid::make("asezari_noi"),
        },
        Playable {
            id: "curtea_reginei",
            title: "Curtea Reginei (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 5×5 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: grupuri de cel puțin 2 piese la fel: 1 p/piesă; piesele A: +2 p fiecare; patru colțuri la fel: 5 p.",
            make: || crate::brand_grid::make("curtea_reginei"),
        },
        Playable {
            id: "sat_mic",
            title: "Satul Mic (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 4×4 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 5 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: grupuri de cel puțin 3 piese la fel: 3 p/piesă; seturi complete: 4 p/set.",
            make: || crate::brand_grid::make("sat_mic"),
        },
        Playable {
            id: "turnul_cifrelor",
            title: "Turnul Cifrelor (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 4×5 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Se zidește din margine spre interior: lângă marginea tablei sau lângă o piesă a ta. Scor: diagonale la fel: 2 p/pereche; rând sau coloană dintr-un singur fel: 6 p; vecini diferiți: 1 p/pereche.",
            make: || crate::brand_grid::make("turnul_cifrelor"),
        },
        Playable {
            id: "pavilion_vesel",
            title: "Pavilionul Vesel (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 5×4 cu piese de 5 feluri (A, B, C, D, E), luate pe rând din oferta comună de 5 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: rând sau coloană cu toate felurile: 3 p; vecini la fel: 1 p/pereche; mai multe piese decât adversarul la un fel: 2 p/fel.",
            make: || crate::brand_grid::make("pavilion_vesel"),
        },
        Playable {
            id: "oras_straturi",
            title: "Orașul în Straturi (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 4×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: grupuri de cel puțin 2 piese la fel: 1 p/piesă; rând sau coloană cu toate felurile: 3 p; mai multe piese decât adversarul la un fel: 2 p/fel.",
            make: || crate::brand_grid::make("oras_straturi"),
        },
        Playable {
            id: "padurea_spiridusilor",
            title: "Pădurea Spiridușilor (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 4×5 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: vecini diferiți: 2 p/pereche; diagonale la fel: 1 p/pereche.",
            make: || crate::brand_grid::make("padurea_spiridusilor"),
        },
        Playable {
            id: "pictura_manastire",
            title: "Pictura Mănăstirii (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 5×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Se zidește din margine spre interior: lângă marginea tablei sau lângă o piesă a ta. Scor: seturi complete: 5 p/set; rând sau coloană dintr-un singur fel: 4 p; piesele A: +1 p fiecare.",
            make: || crate::brand_grid::make("pictura_manastire"),
        },
        Playable {
            id: "vanatorii_campie",
            title: "Vânătorii de Câmpie (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 5×4 cu piese de 4 feluri (A, B, C, D), luate pe rând din oferta comună de 4 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Dacă ai o celulă liberă lângă o piesă a ta, trebuie să așezi acolo. Scor: grupuri de cel puțin 3 piese la fel: 2 p/piesă; mai multe piese decât adversarul la un fel: 3 p/fel; piesele A: +1 p fiecare.",
            make: || crate::brand_grid::make("vanatorii_campie"),
        },
        Playable {
            id: "gradina_corali",
            title: "Grădina de Corali (puzzle de plăci)",
            category: "Familie",
            rules: "Fiecare își umple propria tablă 4×4 cu piese de 3 feluri (A, B, C), luate pe rând din oferta comună de 3 piese. Mutare: «număr_ofertă celulă», ex. «1 b3». Piesa merge în orice celulă liberă. Scor: grupuri de cel puțin 3 piese la fel: 3 p/piesă; vecini la fel: 1 p/pereche; piese A pe margine: 1 p/piesă.",
            make: || crate::brand_grid::make("gradina_corali"),
        },
        Playable {
            id: "duel_giuvaeruri",
            title: "Duelul Giuvaerelor (motor de resurse)",
            category: "Familie",
            rules: "Strângi resurse (R, V, A, G) și cumperi cărți din oferta de 4; cărțile au cost și puncte. Câștigă cine are primul 14 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 10 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia rva»: trei resurse de feluri diferite. Unele cărți dau o reducere permanentă la un fel de resursă.",
            make: || crate::brand_market::make("duel_giuvaeruri"),
        },
        Playable {
            id: "atelier_mecanisme",
            title: "Atelierul de Mecanisme (motor de resurse)",
            category: "Familie",
            rules: "Strângi resurse (R, V, A, G) și cumperi cărți din oferta de 4; cărțile au cost și puncte. Câștigă cine are primul 12 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 8 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia1 x»: o resursă aleasă plus una la sorți. Unele cărți dau o reducere permanentă la un fel de resursă.",
            make: || crate::brand_market::make("atelier_mecanisme"),
        },
        Playable {
            id: "caravana_aromelor",
            title: "Caravana Aromelor (motor de resurse)",
            category: "Familie",
            rules: "Strângi resurse (R, V, A, G) și cumperi cărți din oferta de 5; cărțile au cost și puncte. Câștigă cine are primul 16 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 10 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia2 x»: două resurse la fel (sau «ia1 x» dacă ai doar un loc). Cărțile dau doar puncte. «schimb x»: 2 resurse x devin 1 din tipul următor.",
            make: || crate::brand_market::make("caravana_aromelor"),
        },
        Playable {
            id: "golemii_atelier",
            title: "Golemii din Atelier (motor de resurse)",
            category: "Familie",
            rules: "Strângi resurse (R, V, A, G, N) și cumperi cărți din oferta de 5; cărțile au cost și puncte. Câștigă cine are primul 18 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 12 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia2 x»: două resurse la fel (sau «ia1 x» dacă ai doar un loc). Cărțile dau doar puncte. «schimb x»: 2 resurse x devin 1 din tipul următor.",
            make: || crate::brand_market::make("golemii_atelier"),
        },
        Playable {
            id: "lampioane_lac",
            title: "Lampioane pe Lac (motor de resurse)",
            category: "Familie",
            rules: "Strângi resurse (R, V, A, G) și cumperi cărți din oferta de 4; cărțile au cost și puncte. Câștigă cine are primul 13 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 9 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia1 x»: o resursă aleasă plus una la sorți. Cărțile dau doar puncte. «schimb x»: 2 resurse x devin 1 din tipul următor.",
            make: || crate::brand_market::make("lampioane_lac"),
        },
        Playable {
            id: "rivalii_colonizarii",
            title: "Rivalii Colonizării (motor de resurse)",
            category: "Familie",
            rules: "Strângi resurse (R, V, A, G, N) și cumperi cărți din oferta de 4; cărțile au cost și puncte. Câștigă cine are primul 12 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 10 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia rva»: trei resurse de feluri diferite. Unele cărți dau venit: la fiecare mutare primești resursa respectivă.",
            make: || crate::brand_market::make("rivalii_colonizarii"),
        },
        Playable {
            id: "ferma_animalelor",
            title: "Ferma Animalelor (motor de resurse)",
            category: "Familie",
            rules: "Strângi resurse (R, V, A, G) și cumperi cărți din oferta de 4; cărțile au cost și puncte. Câștigă cine are primul 14 puncte (runda se încheie egal; la egalitate, mai puține cărți). Cel mult 10 resurse în mână. Mutări: «cumpar i», «las x» (arunci o resursă dacă nu poți altceva). «ia2 x»: două resurse la fel (sau «ia1 x» dacă ai doar un loc). Unele cărți dau venit: la fiecare mutare primești resursa respectivă.",
            make: || crate::brand_market::make("ferma_animalelor"),
        },
        Playable {
            id: "linii_urbane",
            title: "Linii Urbane (rute feroviare)",
            category: "Familie",
            rules: "Hartă mică de 9 orașe și 15 rute. Strângi cărți colorate (r, v, a, g, n; J = joker) și ocupi rute plătind atâtea cărți cât e lungimea, de culoarea rutei (rutele «*» acceptă orice culoare). Fiecare are 3 bilete publice (oraș-oraș): conectate = bonus, neconectate = pierdere. Mutări: «ocup n» (ruta n), «trag i» (cartea vizibilă i plus una oarbă), «trag oarba» (două oarbe). Sfârșit când cineva rămâne cu 2 trenuri sau mai puține; punctele rutelor cresc cu lungimea.",
            make: || crate::brand_rail::make("linii_urbane"),
        },
        Playable {
            id: "cai_continentale",
            title: "Căi Ferate Continentale (rute feroviare)",
            category: "Familie",
            rules: "Hartă mare de 14 orașe și 24 de rute. Strângi cărți colorate (r, v, a, g, n; J = joker) și ocupi rute plătind atâtea cărți cât e lungimea, de culoarea rutei (rutele «*» acceptă orice culoare). Fiecare are 3 bilete publice: conectate = bonus, neconectate = pierdere. Mutări: «ocup n», «trag i» (cartea vizibilă i plus una oarbă), «trag oarba». Ai 24 de trenuri; sfârșit când cineva rămâne cu 2 sau mai puține. Scor pe rute după lungime.",
            make: || crate::brand_rail::make("cai_continentale"),
        },
        Playable {
            id: "peloton",
            title: "Pelotonul (cursă cu cărți)",
            category: "Familie",
            rules: "Cursă pe 44 de trepte; fiecare are doi alergători: sprinterul (s: viteze 2–9, explozive) și rulorul (r: 3–7, regulat), fiecare cu 3 cărți deschise. Joci o carte și avansezi atâtea trepte; dacă te oprești imediat în spatele cuiva primești +1 (aspirație); dacă ajungi singur în față cu o carte ≥ 5, adaugi oboseală (mai multe cărți de 2 în pachet). Primul care trece linia câștigă. Mutare: «s1» (sprinter, cartea 1), «r2» (rulor, cartea 2).",
            make: || crate::brand_race::make("peloton"),
        },
        Playable {
            id: "cursa_motoare",
            title: "Cursa Motoarelor (cursă cu cărți)",
            category: "Familie",
            rules: "Cursă cu un bolid pe 40 de trepte și 4 curbe (la 8, 17, 26, 34). Ai 3 cărți deschise de viteză 1–5. Joci o carte («c1») sau o joci cu supraturare («c1+»: +2 viteză, +1 căldură). Dacă treci o curbă cu viteza peste 4, primești căldură egală cu depășirea; o carte de 1–2 fără supraturare răcește cu 1. La 6 căldură motorul cedează și pierzi. Aspirație: +1 dacă te oprești imediat în spatele adversarului. Primul la linie câștigă.",
            make: || crate::brand_race::make("cursa_motoare"),
        },
        Playable {
            id: "fronturi_mari",
            title: "Fronturile Mari (război pe hartă)",
            category: "Strategie",
            rules: "Hartă de 12 teritorii (grilă 3×4) cu valori 1–3; fiecare are 3 teritorii-casă și 6 neutre. La începutul turei primești unități egal cu jumătate din venit (suma valorilor teritoriilor tale, minim 1) și le plasezi într-un teritoriu al tău («pun n»); apoi până la 3 mutări «mut a b toti|unu» către un vecin (la cel inamic sau neutru e atac: pierderi egale, atacatorul câștigă dacă are mai multe unități) și «gata». Câștigi luând capitala adversarului (teritoriul 2 sau 11); după 60 de ture decide scorul.",
            make: || crate::brand_war::make("fronturi_mari"),
        },
        Playable {
            id: "tratat_fragil",
            title: "Tratatul Fragil (război pe hartă)",
            category: "Strategie",
            rules: "Aceeași hartă de 12 teritorii, dar fiecare e un centru și unitățile sunt armate de forță 1. La început primești armate până egalezi numărul centrelor tale (max 3) și le pui într-un teritoriu-casă («pun n»). Până la 3 mutări «mut a b unu»: o armată trece într-un vecin; forța ei = 1 + sprijinul (câte teritorii vecine țintei, ale tale, au ≥2 armate); apărătorul are forța lui + sprijin. Atacatorul ocupă dacă are strict mai multă forță; la egalitate se întoarce. Câștigi cu 7 teritorii din 12; «gata» încheie tura.",
            make: || crate::brand_war::make("tratat_fragil"),
        },
        Playable {
            id: "darurile_serii",
            title: "Darurile Serii (duel cu cărți ascunse)",
            category: "Familie",
            rules: "Șase daruri (valori 2,2,3,3,5,6), câte o carte pentru fiecare punct de valoare. În fiecare rundă faci o dată fiecare acțiune: «ascund c» (1 carte ascunsă, numărată la final), «arunc a b» (2 cărți ies din joc), «ofer a b c» (arăți 3: adversarul ia una cu «ia c», tu păstrezi două), «impart a b + c d» (două perechi: adversarul alege «pereche 1|2»). La final fiecare dar merge la cine are mai multe cărți la el. Câștigi cu 11 puncte sau 4 daruri; după 6 runde decide scorul. Informație ascunsă (HOTSEAT_SAFE).",
            make: || Box::new(darurile_serii::Darurile::new()),
        },
        Playable {
            id: "insule_punti",
            title: "Insulele Punților (poduri între insule)",
            category: "Familie",
            rules: "Douăsprezece insule legate prin 16 rute. Ai 3 cărți deschise (fiecare e o insulă). Pe rând joci o carte: «pun a b» (pod pe ruta a-b care atinge insula cărții), «tai a b» (tai un pod al adversarului pe o rută care atinge insula cărții) sau «descarc a» (schimbi cartea). La mutările 12, 24 și 36 se numără: insula e a celui cu mai multe poduri la ea (marcajul rămâne la egalitate), fiecare insulă marcată = 1 punct; apoi podurile se retrag. Câștigă cine are mai multe puncte.",
            make: || Box::new(insule_punti::Punti::new()),
        },
        Playable {
            id: "pasari_sarma",
            title: "Păsări pe Sârmă (colectare de culori)",
            category: "Familie",
            rules: "Pe o sârmă comună stau păsări de 5 culori (r, v, a, g, n). Ai o mână deschisă de 5 cărți. La rând așezi o pasăre la capătul stâng sau drept («st r», «dr v»). Dacă la acel capăt se formează un șir de cel puțin 2 păsări la fel, le iei pe toate în colecție. Apoi tragi o carte la întâmplare (pachet de 40). La final, fiecare culoare colectată valorează 1+2+…+n puncte pentru n păsări. Câștigă cine are mai multe puncte.",
            make: || Box::new(pasari_sarma::Pasari::new()),
        },
        Playable {
            id: "carti_cascada",
            title: "Cărți în Cascadă (cursă de cărți)",
            category: "Familie",
            rules: "Fiecare are un pachet de 40 de cărți (4 culori × 1..10), trase la întâmplare, și 3 «posturi» deschise. Pe masă sunt cel mult 4 stive comune (una pe culoare) care cresc 1, 2, 3…10. Mutări: «joc i» (postul i pe o stivă: 1 deschide o stivă, altă carte trebuie să urmeze exact vârful stivei de aceeași culoare; în loc apare o carte nouă) sau «schimb i» (schimbi postul cu o carte nouă din pachet). Cine își golește pachetul și posturile câștigă; după 400 de mutări decid cărțile jucate minus cele rămase.",
            make: || Box::new(carti_cascada::Cascada::new()),
        },
        Playable {
            id: "debarcader",
            title: "Debarcaderul Norocos (risc la port)",
            category: "Familie",
            rules: "Joc de risc la un port. La rând tragi bărci una câte una («trag»): fiecare are un pavilion (4 feluri) și o încărcătură de 1–3 monede. Dacă apare al doilea pavilion de același fel în tura asta, barca se scufundă: pierzi tot ce ai tras și tura se termină. După cel puțin o barcă poți să te oprești («opresc») și să încasezi încărcăturile plus 1 monedă pe dulgher angajat. La începutul turei, cu 6 monede, poți angaja un dulgher («angajez»: +1 influență). Câștigă primul cu 4 influențe.",
            make: || Box::new(debarcader::Debarcader::new()),
        },
        Playable {
            id: "reteaua_luminii",
            title: "Rețeaua Luminii (economie energetică)",
            category: "Strategie",
            rules: "Duel economic: 8 orașe pe un inel, 4 centrale la vânzare, combustibil la preț variabil. Faci o acțiune pe rând: «centrala i» (cumperi o centrală, ai cel mult 2), «combustibil c|p» (cărbune/petrol; energia verde nu are nevoie), «oras k» (conectezi orașul: 5 + costul legăturii cu rețeaua ta, +4 dacă îl are adversarul), «produc» (arzi combustibil, alimentezi orașe și încasezi 4 + 3 pe oraș) sau «pas». Primul cu 7 orașe declanșează sfârșitul; câștigă cine alimentează mai multe orașe, apoi cine are mai mulți bani.",
            make: || Box::new(reteaua_luminii::Retea::new()),
        },
        Playable {
            id: "poteca_turmelor",
            title: "Poteca Turmelor (drum cu vite)",
            category: "Strategie",
            rules: "Doi crescători duc vite spre depozitul de la capătul potecii (12 trepte). O acțiune pe rând: «merg n» (1–3 trepte; dacă aterizezi pe treapta adversarului îi plătești 1 monedă), «iau r» (cumperi o vită din rasa r = 1..4 cu 2 monede, doar pe treptele-piață 3, 6, 9) sau «lucrez» (+2 monede). La depozit livrezi automat: 3 puncte pentru fiecare rasă diferită + 1 pentru fiecare vită, +3 monede, și te întorci la start. După 3 livrări câștigă cine are mai multe puncte.",
            make: || Box::new(poteca_turmelor::Poteca::new()),
        },
        Playable {
            id: "planete_verzi",
            title: "Planetele Verzi (terraformare)",
            category: "Strategie",
            rules: "Doi coloniști terraformează 8 planete de 7 tipuri (inel de tipuri 0..6); fiecare are un tip-casă (0 și 4). «colonizez k» costă 1 + distanța pe inel dintre tipul planetei și tipul-casă, minus nivelul tău de terraformare (minim 1), minereu și 2 credite; prima planetă e liberă oriunde, următoarele trebuie să fie vecine cu una a ta. «cercetez» (3 cunoștințe) urcă nivelul (max 3); «transform»: 2 minereu → 1 credit; «colectez»: +2 minereu, +1 credit. După 14 mutări fiecare câștigă cine are mai multe puncte.",
            make: || Box::new(planete_verzi::Planete::new()),
        },
        Playable {
            id: "targul_marii",
            title: "Târgul Mării (comerț pe mare)",
            category: "Strategie",
            rules: "Negustori în 8 porturi pe un inel; portul i produce marfa i mod 4 (preț 1,2,3,4). Fiecare are 4 cărți de acțiune: «fondez k» (casă într-un port liber, vecin cu una a ta; prima oriunde; costă 2 monede, prima e gratis), «produc» (fiecare casă dă o marfă, max 6), «vand» (vinzi toate mărfurile; prețul × casele tale de acel fel) și «recuperez» (cărțile jucate revin în mână, +1 monedă). Când un jucător are 6 case, runda finală se încheie: 3 puncte/casă, +1 pe fel de marfă, +1 la 3 monede.",
            make: || Box::new(targul_marii::Targ::new()),
        },
    ]
}

/// All games playable today (base list + wave 17 / Q6 registry).
pub fn playable() -> Vec<Playable> {
    let mut v = playable_base();
    v.extend(q6_reg::list());
    v.extend(b1_reg::playables());
    v
}

/// The full catalog of target games (all categories).
pub fn catalog() -> &'static [E] {
    catalog::CATALOG
}

pub fn find(id: &str) -> Option<Playable> {
    playable().into_iter().find(|p| p.id == id)
}
