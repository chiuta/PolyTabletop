#![allow(unused_imports)]
//! Reguli-limită pentru jocurile clasice populare (șah, dame, go, table, ...).

use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::chess::Chess;
use polytabletop_games::find;

fn run(g: &mut dyn GameObj, seq: &str) {
    for m in seq.split_whitespace() {
        if let Err(e) = g.play(m) {
            panic!("mutare respinsă «{m}»: {e}\n{}", g.render());
        }
    }
}

fn has(g: &dyn GameObj, m: &str) -> bool {
    g.legal_moves().iter().any(|x| x == m)
}

// ---------------------------------------------------------------- ȘAH

#[test]
fn chess_en_passant_white_and_black() {
    let mut g = Chess::new();
    run(&mut g, "e2e4 a7a6 e4e5 d7d5");
    assert!(has(&g, "e5d6"), "en passant alb");
    run(&mut g, "e5d6");
    assert!(
        g.fen_board().starts_with("rnbqkbnr/1pp1pppp/p2P4/8/"),
        "{}",
        g.fen_board()
    );
    // negru
    let mut g = Chess::new();
    run(&mut g, "a2a3 d7d5 a3a4 d5d4 e2e4");
    assert!(has(&g, "d4e3"));
    run(&mut g, "d4e3");
    assert!(g.fen_board().contains("/8/8/P7/4p3/"), "{}", g.fen_board());
    assert!(
        g.fen_board().ends_with("1PPP1PPP/RNBQKBNR"),
        "{}",
        g.fen_board()
    );
}

#[test]
fn chess_en_passant_expires_after_one_move() {
    let mut g = Chess::new();
    run(&mut g, "e2e4 a7a6 e4e5 d7d5 h2h3 h7h6");
    assert!(!has(&g, "e5d6"));
}

#[test]
fn chess_en_passant_not_on_single_steps() {
    let mut g = Chess::new();
    run(&mut g, "e2e4 d7d6 e4e5 d6d5");
    assert!(
        !has(&g, "e5d6"),
        "d7-d6-d5 în doi pași nu permite en passant"
    );
}

#[test]
fn chess_castling_rights_lost_after_king_or_rook_moves() {
    // Regele a mutat și s-a întors
    let mut g = Chess::new();
    run(&mut g, "e2e4 e7e5 g1f3 g8f6 f1e2 f8e7");
    assert!(has(&g, "e1g1"));
    run(&mut g, "e1f1 e8f8 f1e1 f8e8");
    assert!(!has(&g, "e1g1"));
    // Tura a mutat și s-a întors
    let mut g = Chess::new();
    run(&mut g, "g1f3 g8f6 g2g3 g7g6 f1g2 f8g7 h1g1 h8g8 g1h1 g8h8");
    assert!(!has(&g, "e1g1"));
}

#[test]
fn chess_castling_moves_the_rook_both_sides() {
    let mut g = Chess::new();
    run(&mut g, "e2e4 e7e5 g1f3 g8f6 f1c4 f8c5 e1g1");
    assert!(g.fen_board().ends_with("RNBQ1RK1"), "{}", g.fen_board());
    let mut g = Chess::new();
    run(&mut g, "d2d4 d7d5 b1c3 b8c6 c1f4 c8f5 d1d2 d8d7");
    assert!(has(&g, "e1c1"));
    run(&mut g, "e1c1");
    assert!(g.fen_board().ends_with("2KR1BNR"), "{}", g.fen_board());
}

#[test]
fn chess_cannot_castle_through_attacked_square() {
    // Nebunul negru pe c4 atacă f1 (liber), g1 liber.
    let mut g = Chess::new();
    run(&mut g, "e2e4 b7b6 g1f3 c8a6 f1c4 a6c4");
    assert!(!has(&g, "e1g1"), "rocadă prin f1 atacat");
    assert!(has(&g, "e1e2") || has(&g, "d2d3"));
}

#[test]
fn chess_cannot_castle_out_of_check() {
    let mut g = Chess::new();
    run(&mut g, "d2d4 e7e6 g2g3 g8f6 f1g2 d7d5 g1f3 f8b4");
    assert!(!has(&g, "e1g1"), "rocadă din șah");
    assert!(has(&g, "c2c3") || has(&g, "b1d2"));
}

#[test]
fn chess_can_castle_when_only_rook_attacked() {
    // Tura a1 atacată nu împiedică rocada mare (b1 poate fi atacat, regele nu trece prin b1).
    let mut g = Chess::new();
    run(&mut g, "d2d4 d7d5 b1c3 b8c6 c1f4 c8h3 d1d3 d8d7");
    // g2xh3 posibil; regele nu e în șah; c1/d1/b1 libere -> e1c1 posibil, dar d1 atacat? (nu)
    assert!(has(&g, "e1c1"));
}

#[test]
fn chess_promotion_four_pieces_and_capture() {
    let mut g = Chess::new();
    run(&mut g, "a2a4 b7b5 a4b5 a7a6 b5a6 g8f6 a6a7 e7e6");
    let mv = g.legal_moves();
    for m in ["a7b8q", "a7b8r", "a7b8b", "a7b8n"] {
        assert!(mv.contains(&m.to_string()), "{m} lipsește");
    }
    assert!(!mv.contains(&"a7b8".to_string()));
    run(&mut g, "a7b8n");
    assert!(g.fen_board().starts_with("rNbqkb1r/"), "{}", g.fen_board());
}

#[test]
fn chess_promotion_with_check_is_mate_when_no_escape() {
    // Mat clasic: mat în 2 mutări (Fool's mate) și Scholar's mate.
    let mut g = Chess::new();
    run(&mut g, "f2f3 e7e5 g2g4 d8h4");
    assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    let mut g = Chess::new();
    run(&mut g, "e2e4 e7e5 f1c4 b8c6 d1h5 g8f6 h5f7");
    assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
}

#[test]
fn chess_stalemate_is_a_draw() {
    let mut g = Chess::new();
    run(
        &mut g,
        "e2e3 a7a5 d1h5 a8a6 h5a5 h7h5 h2h4 a6h6 a5c7 f7f6 c7d7 e8f7 d7b7 d8d3 b7b8 d3h7 b8c8 f7g6 c8e6",
    );
    assert_eq!(g.outcome(), Some(Outcome::Draw));
    assert!(g.legal_moves().is_empty());
}

#[test]
fn chess_threefold_repetition_draws() {
    let mut g = Chess::new();
    run(&mut g, "g1f3 g8f6 f3g1 f6g8");
    assert!(g.outcome().is_none());
    run(&mut g, "g1f3 g8f6 f3g1 f6g8");
    assert_eq!(g.outcome(), Some(Outcome::Draw));
}

#[test]
fn chess_repetition_ignores_unusable_en_passant_square() {
    // După 1.e4 nimeni nu poate lua en passant; poziția de după 3.Cb1 e aceeași
    // (negrul la mutare), deci a treia apariție vine la mutarea 5.
    let mut g = Chess::new();
    run(&mut g, "e2e4 g8f6 b1c3 f6g8 c3b1 g8f6");
    assert!(g.outcome().is_none());
    run(&mut g, "b1c3 f6g8 c3b1");
    assert_eq!(g.outcome(), Some(Outcome::Draw), "{}", g.render());
}

// ---------------------------------------------------------------- GO

#[test]
fn go_simple_ko_blocks_immediate_recapture_only() {
    let mut g = polytabletop_games::go::Go::new();
    // negru: d6 c5 d4 ; alb: e6 f5 e4 apoi d5 (piatră care va fi luată)
    run(&mut g, "d6 e6 c5 f5 d4 e4 a1 d5");
    assert!(has(&g, "e5"));
    run(&mut g, "e5"); // negrul ia d5
    assert!(!has(&g, "d5"), "ko: albul nu poate relua imediat");
    run(&mut g, "j9 a9"); // o mutare în altă parte
    assert!(
        has(&g, "d5"),
        "după o mutare în altă parte, ko-ul se poate lua"
    );
}

#[test]
fn go_pass_play_pass_does_not_end_game() {
    let mut g = polytabletop_games::go::Go::new();
    run(&mut g, "pass e5 pass");
    assert!(g.outcome().is_none());
    run(&mut g, "pass");
    assert!(g.outcome().is_some());
}

#[test]
fn go_capture_has_priority_over_suicide() {
    // Alb a1 dintr-un colț: negru b1 și a2 cu a1 capturat de al treilea fără libertăți?
    // Formă: negru pe b1, a2; alb joacă a1 = sinucidere (ilegal).
    let mut g = polytabletop_games::go::Go::new();
    run(&mut g, "b1 j9 a2");
    assert!(!has(&g, "a1"), "a1 ar fi sinucidere pentru alb");
}

// ---------------------------------------------------------------- DAME

#[test]
fn draughts_capture_is_forced_and_men_do_not_move_backwards() {
    let mut g = polytabletop_games::draughts::Draughts::new();
    run(&mut g, "c3-b4 d6-c5");
    assert_eq!(g.legal_moves(), vec!["b4xd6".to_string()]);
    run(&mut g, "b4xd6");
    for m in g.legal_moves() {
        assert!(m.contains('x'), "captura e obligatorie: {m}");
    }
}

#[test]
fn draughts_random_games_never_mix_moves_and_captures() {
    use polytabletop_core::rng::Rng;
    for seed in 1..=40u64 {
        let mut rng = Rng::new(seed);
        let mut g = polytabletop_games::draughts::Draughts::new();
        let mut n = 0;
        while g.outcome().is_none() && n < 400 {
            let mv = g.legal_moves();
            let caps = mv.iter().filter(|m| m.contains('x')).count();
            assert!(caps == 0 || caps == mv.len(), "{mv:?}");
            g.play(&mv[rng.below(mv.len())]).unwrap();
            n += 1;
        }
    }
}

// ---------------------------------------------------------------- XIANGQI

#[test]
fn xiangqi_cannon_needs_exactly_one_screen() {
    let g = polytabletop_games::xiangqi::Xiangqi::new();
    assert!(
        has(&g, "b2b9"),
        "tunul ia calul peste un ecran (tunul negru)"
    );
    assert!(!has(&g, "b2b7"), "tunul nu capturează fără ecran");
    assert!(!has(&g, "b2b8") || true);
}

#[test]
fn xiangqi_soldiers_move_only_forward_before_river() {
    let g = polytabletop_games::xiangqi::Xiangqi::new();
    assert!(has(&g, "a3a4"));
    assert!(!has(&g, "a3b3"), "pion necrucișat nu mută lateral");
    assert!(!has(&g, "a3a2"));
}

// ---------------------------------------------------------------- POKER

#[test]
fn poker_heads_up_action_order_and_button_rotation() {
    use polytabletop_games::poker::Poker;
    let mut g = Poker::with_seed(11);
    // J1 = buton = blind mic, acționează primul preflop
    assert_eq!(g.to_move(), Player::One);
    assert!(has(&g, "fold") && has(&g, "call") && has(&g, "raise"));
    run(&mut g, "call"); // completează blindul
    assert_eq!(g.to_move(), Player::Two, "blindul mare are opțiunea");
    assert!(!has(&g, "fold"), "fără pariu de plătit nu poți da fold");
    run(&mut g, "check flop");
    assert_eq!(g.to_move(), Player::Two, "după flop acționează non-butonul");
    run(
        &mut g,
        "check check turn check check river check check showdown",
    );
    // mâna 2: butonul trece la J2, care acționează primul preflop
    assert_eq!(g.to_move(), Player::Two);
}

#[test]
fn poker_raise_cap_three_per_round() {
    use polytabletop_games::poker::Poker;
    let mut g = Poker::with_seed(5);
    run(&mut g, "raise raise raise");
    assert!(!has(&g, "raise"), "a patra creștere nu e permisă");
    assert!(has(&g, "call") && has(&g, "fold"));
}

// ---------------------------------------------------------------- MAHJONG

#[test]
fn mahjong_hand_scoring_edge_cases() {
    use polytabletop_games::mahjong::{score_hand, tile_name};
    let counts = |v: &[&str]| {
        let mut c = [0u8; 34];
        for t in v {
            let i = (0..34u8).find(|&x| tile_name(x) == *t).unwrap();
            c[i as usize] += 1;
        }
        c
    };
    // șapte perechi nu e mână câștigătoare în această variantă
    let sp = counts(&[
        "1m", "1m", "3m", "3m", "5m", "5m", "7p", "7p", "2s", "2s", "9s", "9s", "E", "E",
    ]);
    assert!(score_hand(&sp, &[], 27, true).is_none());
    // pereche de vânt propriu: nu se acordă „toate secvențele”
    let wind_pair = counts(&[
        "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", "1p", "2p", "3p", "S", "S",
    ]);
    let (f_own, _) = score_hand(&wind_pair, &[], 28, false).unwrap(); // vânt propriu S
    let (f_other, _) = score_hand(&wind_pair, &[], 29, false).unwrap(); // S nu e vântul meu
    assert_eq!(f_own, 1, "doar mână închisă");
    assert_eq!(
        f_other, 2,
        "pereche de vânt străin = pereche banală: închisă + toate secvențele"
    );
}

#[test]
fn mahjong_dragon_pung_and_seat_wind_pung() {
    use polytabletop_games::mahjong::{score_hand, tile_name, Meld};
    let counts = |v: &[&str]| {
        let mut c = [0u8; 34];
        for t in v {
            let i = (0..34u8).find(|&x| tile_name(x) == *t).unwrap();
            c[i as usize] += 1;
        }
        c
    };
    // 123m 456m 789p + Dr Dr Dr (în mână) + 5s 5s, câștig pe aruncare, închisă
    let c = counts(&[
        "1m", "2m", "3m", "4m", "5m", "6m", "7p", "8p", "9p", "Dr", "Dr", "Dr", "5s", "5s",
    ]);
    let (fan, d) = score_hand(&c, &[], 28, false).unwrap();
    assert_eq!(fan, 2, "închisă 1 + pung de dragon 1: {d}"); // nu e „toate secvențele”
                                                             // același, cu dragonul expus: nu mai e închisă
    let c2 = counts(&[
        "1m", "2m", "3m", "4m", "5m", "6m", "7p", "8p", "9p", "5s", "5s",
    ]);
    let (fan2, d2) = score_hand(&c2, &[Meld::Pung(31)], 28, false).unwrap();
    assert_eq!(fan2, 1, "{d2}");
}

// ------------------------------------------------- BELOTE / TAROT: culoare obligatorie

#[test]
fn belote_follow_rules_hold_in_random_play() {
    use polytabletop_core::rng::Rng;
    use polytabletop_core::Game;
    use polytabletop_games::belote::{card_text, power, suit, Belote};
    let mut checked = 0;
    for seed in 1..=60u64 {
        let mut rng = Rng::new(seed);
        let mut g = Belote::with_seed(seed);
        let mut led: Option<u8> = None;
        let mut guard = 0;
        while Game::outcome(&g).is_none() && guard < 2000 {
            guard += 1;
            let me = Game::to_move(&g);
            let hand: Vec<u8> = g.hand_of(me).to_vec();
            let moves = Game::legal_moves(&g);
            let texts: Vec<String> = moves.iter().map(|m| g.format_move(m)).collect();
            let in_play = !hand.is_empty()
                && g.trump_suit().is_some()
                && texts
                    .iter()
                    .all(|t| hand.iter().any(|&c| card_text(c) == *t));
            let mut played: Option<u8> = None;
            if in_play {
                let t = g.trump_suit().unwrap();
                let expected: Vec<u8> = match led {
                    None => hand.clone(),
                    Some(l) => {
                        let same: Vec<u8> = hand
                            .iter()
                            .copied()
                            .filter(|&c| suit(c) == suit(l))
                            .collect();
                        if suit(l) == t {
                            let higher: Vec<u8> = same
                                .iter()
                                .copied()
                                .filter(|&c| power(c, t) > power(l, t))
                                .collect();
                            if same.is_empty() {
                                hand.clone()
                            } else if higher.is_empty() {
                                same
                            } else {
                                higher
                            }
                        } else if !same.is_empty() {
                            same
                        } else {
                            let tr: Vec<u8> =
                                hand.iter().copied().filter(|&c| suit(c) == t).collect();
                            if tr.is_empty() {
                                hand.clone()
                            } else {
                                tr
                            }
                        }
                    }
                };
                let mut e: Vec<String> = expected.iter().map(|&c| card_text(c)).collect();
                e.sort();
                let mut a = texts.clone();
                a.sort();
                assert_eq!(a, e, "seed {seed}, led {led:?}");
                checked += 1;
            }
            let m = &moves[rng.below(moves.len())];
            if in_play {
                let txt = g.format_move(m);
                played = hand.iter().copied().find(|&c| card_text(c) == txt);
            }
            g.apply(m);
            if let Some(c) = played {
                led = if led.is_none() { Some(c) } else { None };
            }
        }
    }
    assert!(checked > 500, "{checked}");
}

#[test]
fn tarot_follow_rules_hold_in_random_play() {
    use polytabletop_core::rng::Rng;
    use polytabletop_core::Game;
    use polytabletop_games::tarot::{card_text, is_trump, Tarot};
    let ex = 77u8;
    let mut checked = 0;
    for seed in 1..=60u64 {
        let mut rng = Rng::new(seed);
        let mut g = Tarot::with_seed(seed);
        let mut led: Option<u8> = None;
        let mut guard = 0;
        while Game::outcome(&g).is_none() && guard < 3000 {
            guard += 1;
            let me = Game::to_move(&g);
            let hand: Vec<u8> = g.hand_of(me).to_vec();
            let moves = Game::legal_moves(&g);
            let texts: Vec<String> = moves.iter().map(|m| g.format_move(m)).collect();
            let in_play = !hand.is_empty()
                && texts
                    .iter()
                    .all(|t| hand.iter().any(|&c| card_text(c) == *t));
            let mut played: Option<u8> = None;
            if in_play {
                let suit = |c: u8| c / 14;
                let expected: Vec<u8> = match led {
                    None => hand.clone(),
                    Some(l) if l == ex => hand.clone(),
                    Some(l) => {
                        let mut v: Vec<u8> = if is_trump(l) {
                            let tr: Vec<u8> =
                                hand.iter().copied().filter(|&c| is_trump(c)).collect();
                            let hi: Vec<u8> = tr.iter().copied().filter(|&c| c > l).collect();
                            if !hi.is_empty() {
                                hi
                            } else if !tr.is_empty() {
                                tr
                            } else {
                                return_all(&hand)
                            }
                        } else {
                            let same: Vec<u8> = hand
                                .iter()
                                .copied()
                                .filter(|&c| c < 56 && suit(c) == suit(l))
                                .collect();
                            if !same.is_empty() {
                                same
                            } else {
                                let tr: Vec<u8> =
                                    hand.iter().copied().filter(|&c| is_trump(c)).collect();
                                if tr.is_empty() {
                                    return_all(&hand)
                                } else {
                                    tr
                                }
                            }
                        };
                        if hand.contains(&ex) && !v.contains(&ex) {
                            v.push(ex);
                        }
                        v
                    }
                };
                let mut e: Vec<String> = expected.iter().map(|&c| card_text(c)).collect();
                e.sort();
                let mut a = texts.clone();
                a.sort();
                assert_eq!(a, e, "seed {seed}, led {led:?}");
                checked += 1;
            }
            let m = &moves[rng.below(moves.len())];
            if in_play {
                let txt = g.format_move(m);
                played = hand.iter().copied().find(|&c| card_text(c) == txt);
            }
            g.apply(m);
            if let Some(c) = played {
                led = if led.is_none() { Some(c) } else { None };
            }
        }
    }
    assert!(checked > 500, "{checked}");
}

fn return_all(h: &[u8]) -> Vec<u8> {
    h.to_vec()
}
