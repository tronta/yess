use rayon::prelude::*;

mod playfield;
use playfield::*;

/// Main function performs a brute force solving
fn main() {
    println!("Started searching for solutions. Please wait. This will take long ...");

    // This closure translates the number of a field to its x and y coordinates
    let f2xy = |p| {
        if p > 54 {
            return (0, 9);
        };

        if p > 51 {
            return (p - 52, 8);
        };

        if p > 48 {
            return (p - 49, 7);
        };

        if p > 44 {
            return (p - 45, 6);
        };

        if p > 39 {
            return (p - 40, 5);
        };

        if p > 33 {
            return (p - 34, 4);
        };

        if p > 26 {
            return (p - 27, 3);
        };

        if p > 18 {
            return (p - 19, 2);
        };

        if p > 9 {
            return (p - 10, 1);
        };

        (p, 0)
    };

    // Brute force trying for a type all positions and rotations, incl mirroring
    // r in 0..8 for none symmetrical parts
    // r in 0..4 for pieces with one symmetrical axis
    // r in 0..2 for pieces with two symmetrical axis

    // As the field is symmetrical we look only at the upper triangle for the first piece
    (0..40usize).into_par_iter().for_each( |s0| { // use rayon for the different fields of the first piece
    let mut pf = Playfield::new();
    for r0 in 0..8 {
        if !pf.set_piece( f2xy(s0), 0, r0 ) { continue; } // if piece does not fit there successfull try the next possition
        println!("Piece 0: {}, {}", s0, r0);
        for s1 in 0..55 {
        for r1 in 0..8 {
            if !pf.set_piece( f2xy(s1), 1, r1 ) { continue; }
            println!("Piece 1: {}, {}", s1, r1);
            for s2 in 0..55 {
            for r2 in 0..8 {
                if !pf.set_piece( f2xy(s2), 2, r2 ) { continue; }
                println!("Piece 2: {}, {}", s2, r2);
                for s3 in 0..55 {
                for r3 in 0..8 {
                    if !pf.set_piece( f2xy(s3), 3, r3 ) { continue; }
                    println!("Piece 3: {}, {}", s3, r3);
                    for s4 in 0..55 {
                    for r4 in 0..8 {
                        if !pf.set_piece( f2xy(s4), 4, r4 ) { continue; }

                        for s5 in 0..55 {
                        for r5 in 0..4 {
                            if !pf.set_piece( f2xy(s5), 5, r5 ) { continue; }

                            for s6 in 0..55 {
                            for r6 in 0..4 {
                                if !pf.set_piece( f2xy(s6), 6, r6 ) { continue; }

                                for s7 in 0..55 {
                                for r7 in 0..4 {
                                    if !pf.set_piece( f2xy(s7), 7, r7 ) { continue; }

                                    for s8 in 0..55 {
                                    for r8 in 0..4 {
                                        if !pf.set_piece( f2xy(s8), 8, r8 ) { continue; }

                                        for s9 in 0..55 {
                                        for r9 in 0..2 {
                                            if !pf.set_piece( f2xy(s9), 9, r9 ) { continue; }

                                            for s10 in 0..55 {
                                                if !pf.set_piece( f2xy(s10), 10, 0 ) { continue; }

                                                for s11 in 0..55 {
                                                    if !pf.set_piece( f2xy(s11), 11, 0 ) { continue; }
                                                    println!("Field:\n{:?}", pf);
                                                    break;
                                                }
                                                pf.remove_piece(11);
                                            }
                                            pf.remove_piece(10);
                                        }
                                        }
                                        pf.remove_piece(9);
                                    }
                                    }
                                    pf.remove_piece(8);
                                }
                                }
                                pf.remove_piece(7);
                            }
                            }
                            pf.remove_piece(6);
                        }
                        }
                        pf.remove_piece(5);
                    }
                    }
                    pf.remove_piece(4);
                }
                }
                pf.remove_piece(3);
            }
            }
            pf.remove_piece(2);
        }
        }
        pf.remove_piece(1);
    }
    }
    );
}
