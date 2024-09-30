fn func() {
    let mut state = 0;
    let mut symbol = 0;
    let mut msg = 0;
    let q0 = 0;
    let q1 = 1;
    let q2 = 2;
    let qn = 3;
    let qy = 4;
    let A = 0;
    let B = 1;
    let move_left = 0;
    let move_right = 1;

    if state != 3 && state != 1 {
        if state == q1 {
            if symbol == A {
                state = qn;
            } else {
                if symbol == B {
                    symbol = B;
                    move_right;
                    state = q1;
                } else {
                    if symbol == 1 {
                        symbol = 0;
                        move_left;
                        state = q0;
                    } else {
                        if symbol == 0 {
                            symbol = 0;
                            move_right;
                            state = q1;
                        } else {
                            if symbol == 2 {
                                symbol = 2;
                                move_right;
                                state = q1;
                            }
                        }
                    }
                }
            }
        } else {
            if state == q0 {
                if symbol == 1 {
                    symbol = 2;
                    move_right;
                    state = q1;
                } else {
                    if symbol == 0 {
                        symbol = 0;
                        move_left;
                        state = q0;
                    } else {
                        if symbol == B {
                            symbol = B;
                            move_left;
                            state = q0;
                        } else {
                            if symbol == A {
                                symbol = A;
                                move_right;
                                state = q2;
                            } else {
                                if symbol == 2 {
                                    symbol = 2;
                                    move_left;
                                    state = q0;
                                }
                            }
                        }
                    }
                }
            } else {
                if state == q2 {
                    if symbol == A {
                        state = qy;
                    } else {
                        if symbol == 2 {
                            symbol = 1;
                            move_right;
                            state = q2;
                        } else {
                            if symbol == B {
                                symbol = B;
                                move_right;
                                state = q2;
                            } else {
                                if symbol == 0 {
                                    symbol = 0;
                                    move_right;
                                    state = q2;
                                } else {
                                    if symbol == 1 {
                                        symbol = 1;
                                        move_left;
                                        state = q0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
