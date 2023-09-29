#[test]
fn test_a9_c3_7a_0() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xc3, 0x7a]);
    cpu.register_a = 22;
    cpu.status = 162;
    cpu.pc = 0x8000;
    cpu.register_s = 215;
    cpu.register_x = 214;
    cpu.register_y = 9;

    cpu.run();

    assert_eq!(cpu.register_a, 195);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 215);
    assert_eq!(cpu.register_x, 214);
    assert_eq!(cpu.register_y, 9);
}

#[test]
fn test_a9_5d_4f_1() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x5d, 0x4f]);
    cpu.register_a = 24;
    cpu.status = 103;
    cpu.pc = 0x8000;
    cpu.register_s = 119;
    cpu.register_x = 252;
    cpu.register_y = 141;

    cpu.run();

    assert_eq!(cpu.register_a, 93);
    assert_eq!(cpu.status, 101);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 119);
    assert_eq!(cpu.register_x, 252);
    assert_eq!(cpu.register_y, 141);
}

#[test]
fn test_a9_ed_53_2() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xed, 0x53]);
    cpu.register_a = 73;
    cpu.status = 239;
    cpu.pc = 0x8000;
    cpu.register_s = 227;
    cpu.register_x = 94;
    cpu.register_y = 112;

    cpu.run();

    assert_eq!(cpu.register_a, 237);
    assert_eq!(cpu.status, 237);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 227);
    assert_eq!(cpu.register_x, 94);
    assert_eq!(cpu.register_y, 112);
}

#[test]
fn test_a9_01_af_3() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x01, 0xaf]);
    cpu.register_a = 70;
    cpu.status = 39;
    cpu.pc = 0x8000;
    cpu.register_s = 4;
    cpu.register_x = 116;
    cpu.register_y = 7;

    cpu.run();

    assert_eq!(cpu.register_a, 1);
    assert_eq!(cpu.status, 37);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 4);
    assert_eq!(cpu.register_x, 116);
    assert_eq!(cpu.register_y, 7);
}

#[test]
fn test_a9_06_8e_4() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x06, 0x8e]);
    cpu.register_a = 240;
    cpu.status = 224;
    cpu.pc = 0x8000;
    cpu.register_s = 121;
    cpu.register_x = 232;
    cpu.register_y = 182;

    cpu.run();

    assert_eq!(cpu.register_a, 6);
    assert_eq!(cpu.status, 96);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 121);
    assert_eq!(cpu.register_x, 232);
    assert_eq!(cpu.register_y, 182);
}

#[test]
fn test_a9_0e_76_5() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x0e, 0x76]);
    cpu.register_a = 30;
    cpu.status = 169;
    cpu.pc = 0x8000;
    cpu.register_s = 185;
    cpu.register_x = 103;
    cpu.register_y = 195;

    cpu.run();

    assert_eq!(cpu.register_a, 14);
    assert_eq!(cpu.status, 41);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 185);
    assert_eq!(cpu.register_x, 103);
    assert_eq!(cpu.register_y, 195);
}

#[test]
fn test_a9_e3_37_6() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe3, 0x37]);
    cpu.register_a = 235;
    cpu.status = 170;
    cpu.pc = 0x8000;
    cpu.register_s = 93;
    cpu.register_x = 220;
    cpu.register_y = 135;

    cpu.run();

    assert_eq!(cpu.register_a, 227);
    assert_eq!(cpu.status, 168);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 93);
    assert_eq!(cpu.register_x, 220);
    assert_eq!(cpu.register_y, 135);
}

#[test]
fn test_a9_61_2b_7() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x61, 0x2b]);
    cpu.register_a = 220;
    cpu.status = 98;
    cpu.pc = 0x8000;
    cpu.register_s = 118;
    cpu.register_x = 205;
    cpu.register_y = 35;

    cpu.run();

    assert_eq!(cpu.register_a, 97);
    assert_eq!(cpu.status, 96);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 118);
    assert_eq!(cpu.register_x, 205);
    assert_eq!(cpu.register_y, 35);
}

#[test]
fn test_a9_99_ec_8() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x99, 0xec]);
    cpu.register_a = 231;
    cpu.status = 172;
    cpu.pc = 0x8000;
    cpu.register_s = 198;
    cpu.register_x = 145;
    cpu.register_y = 136;

    cpu.run();

    assert_eq!(cpu.register_a, 153);
    assert_eq!(cpu.status, 172);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 198);
    assert_eq!(cpu.register_x, 145);
    assert_eq!(cpu.register_y, 136);
}

#[test]
fn test_a9_b4_c4_9() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xb4, 0xc4]);
    cpu.register_a = 190;
    cpu.status = 45;
    cpu.pc = 0x8000;
    cpu.register_s = 225;
    cpu.register_x = 44;
    cpu.register_y = 65;

    cpu.run();

    assert_eq!(cpu.register_a, 180);
    assert_eq!(cpu.status, 173);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 225);
    assert_eq!(cpu.register_x, 44);
    assert_eq!(cpu.register_y, 65);
}

#[test]
fn test_a9_47_a4_10() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x47, 0xa4]);
    cpu.register_a = 180;
    cpu.status = 106;
    cpu.pc = 0x8000;
    cpu.register_s = 21;
    cpu.register_x = 51;
    cpu.register_y = 176;

    cpu.run();

    assert_eq!(cpu.register_a, 71);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 21);
    assert_eq!(cpu.register_x, 51);
    assert_eq!(cpu.register_y, 176);
}

#[test]
fn test_a9_5e_93_11() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x5e, 0x93]);
    cpu.register_a = 251;
    cpu.status = 231;
    cpu.pc = 0x8000;
    cpu.register_s = 223;
    cpu.register_x = 37;
    cpu.register_y = 136;

    cpu.run();

    assert_eq!(cpu.register_a, 94);
    assert_eq!(cpu.status, 101);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 223);
    assert_eq!(cpu.register_x, 37);
    assert_eq!(cpu.register_y, 136);
}

#[test]
fn test_a9_bb_ed_12() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xbb, 0xed]);
    cpu.register_a = 172;
    cpu.status = 101;
    cpu.pc = 0x8000;
    cpu.register_s = 41;
    cpu.register_x = 223;
    cpu.register_y = 248;

    cpu.run();

    assert_eq!(cpu.register_a, 187);
    assert_eq!(cpu.status, 229);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 41);
    assert_eq!(cpu.register_x, 223);
    assert_eq!(cpu.register_y, 248);
}

#[test]
fn test_a9_48_83_13() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x48, 0x83]);
    cpu.register_a = 30;
    cpu.status = 167;
    cpu.pc = 0x8000;
    cpu.register_s = 176;
    cpu.register_x = 2;
    cpu.register_y = 106;

    cpu.run();

    assert_eq!(cpu.register_a, 72);
    assert_eq!(cpu.status, 37);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 176);
    assert_eq!(cpu.register_x, 2);
    assert_eq!(cpu.register_y, 106);
}

#[test]
fn test_a9_0f_45_14() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x0f, 0x45]);
    cpu.register_a = 43;
    cpu.status = 234;
    cpu.pc = 0x8000;
    cpu.register_s = 22;
    cpu.register_x = 161;
    cpu.register_y = 119;

    cpu.run();

    assert_eq!(cpu.register_a, 15);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 22);
    assert_eq!(cpu.register_x, 161);
    assert_eq!(cpu.register_y, 119);
}

#[test]
fn test_a9_93_d3_15() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x93, 0xd3]);
    cpu.register_a = 203;
    cpu.status = 110;
    cpu.pc = 0x8000;
    cpu.register_s = 14;
    cpu.register_x = 221;
    cpu.register_y = 36;

    cpu.run();

    assert_eq!(cpu.register_a, 147);
    assert_eq!(cpu.status, 236);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 14);
    assert_eq!(cpu.register_x, 221);
    assert_eq!(cpu.register_y, 36);
}

#[test]
fn test_a9_bb_6e_16() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xbb, 0x6e]);
    cpu.register_a = 29;
    cpu.status = 34;
    cpu.pc = 0x8000;
    cpu.register_s = 239;
    cpu.register_x = 4;
    cpu.register_y = 224;

    cpu.run();

    assert_eq!(cpu.register_a, 187);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 239);
    assert_eq!(cpu.register_x, 4);
    assert_eq!(cpu.register_y, 224);
}

#[test]
fn test_a9_75_0a_17() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x75, 0x0a]);
    cpu.register_a = 187;
    cpu.status = 238;
    cpu.pc = 0x8000;
    cpu.register_s = 51;
    cpu.register_x = 56;
    cpu.register_y = 178;

    cpu.run();

    assert_eq!(cpu.register_a, 117);
    assert_eq!(cpu.status, 108);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 51);
    assert_eq!(cpu.register_x, 56);
    assert_eq!(cpu.register_y, 178);
}

#[test]
fn test_a9_6b_ac_18() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x6b, 0xac]);
    cpu.register_a = 32;
    cpu.status = 174;
    cpu.pc = 0x8000;
    cpu.register_s = 52;
    cpu.register_x = 13;
    cpu.register_y = 81;

    cpu.run();

    assert_eq!(cpu.register_a, 107);
    assert_eq!(cpu.status, 44);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 52);
    assert_eq!(cpu.register_x, 13);
    assert_eq!(cpu.register_y, 81);
}

#[test]
fn test_a9_03_e2_19() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x03, 0xe2]);
    cpu.register_a = 121;
    cpu.status = 32;
    cpu.pc = 0x8000;
    cpu.register_s = 132;
    cpu.register_x = 250;
    cpu.register_y = 44;

    cpu.run();

    assert_eq!(cpu.register_a, 3);
    assert_eq!(cpu.status, 32);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 132);
    assert_eq!(cpu.register_x, 250);
    assert_eq!(cpu.register_y, 44);
}

#[test]
fn test_a9_1b_0c_20() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x1b, 0x0c]);
    cpu.register_a = 200;
    cpu.status = 174;
    cpu.pc = 0x8000;
    cpu.register_s = 13;
    cpu.register_x = 10;
    cpu.register_y = 108;

    cpu.run();

    assert_eq!(cpu.register_a, 27);
    assert_eq!(cpu.status, 44);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 13);
    assert_eq!(cpu.register_x, 10);
    assert_eq!(cpu.register_y, 108);
}

#[test]
fn test_a9_cf_13_21() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xcf, 0x13]);
    cpu.register_a = 150;
    cpu.status = 36;
    cpu.pc = 0x8000;
    cpu.register_s = 171;
    cpu.register_x = 176;
    cpu.register_y = 102;

    cpu.run();

    assert_eq!(cpu.register_a, 207);
    assert_eq!(cpu.status, 164);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 171);
    assert_eq!(cpu.register_x, 176);
    assert_eq!(cpu.register_y, 102);
}

#[test]
fn test_a9_89_fd_22() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x89, 0xfd]);
    cpu.register_a = 169;
    cpu.status = 103;
    cpu.pc = 0x8000;
    cpu.register_s = 28;
    cpu.register_x = 151;
    cpu.register_y = 217;

    cpu.run();

    assert_eq!(cpu.register_a, 137);
    assert_eq!(cpu.status, 229);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 28);
    assert_eq!(cpu.register_x, 151);
    assert_eq!(cpu.register_y, 217);
}

#[test]
fn test_a9_36_ed_23() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x36, 0xed]);
    cpu.register_a = 151;
    cpu.status = 163;
    cpu.pc = 0x8000;
    cpu.register_s = 70;
    cpu.register_x = 130;
    cpu.register_y = 253;

    cpu.run();

    assert_eq!(cpu.register_a, 54);
    assert_eq!(cpu.status, 33);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 70);
    assert_eq!(cpu.register_x, 130);
    assert_eq!(cpu.register_y, 253);
}

#[test]
fn test_a9_fb_7f_24() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xfb, 0x7f]);
    cpu.register_a = 13;
    cpu.status = 111;
    cpu.pc = 0x8000;
    cpu.register_s = 205;
    cpu.register_x = 140;
    cpu.register_y = 235;

    cpu.run();

    assert_eq!(cpu.register_a, 251);
    assert_eq!(cpu.status, 237);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 205);
    assert_eq!(cpu.register_x, 140);
    assert_eq!(cpu.register_y, 235);
}

#[test]
fn test_a9_ec_be_25() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xec, 0xbe]);
    cpu.register_a = 142;
    cpu.status = 173;
    cpu.pc = 0x8000;
    cpu.register_s = 15;
    cpu.register_x = 233;
    cpu.register_y = 177;

    cpu.run();

    assert_eq!(cpu.register_a, 236);
    assert_eq!(cpu.status, 173);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 15);
    assert_eq!(cpu.register_x, 233);
    assert_eq!(cpu.register_y, 177);
}

#[test]
fn test_a9_47_fb_26() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x47, 0xfb]);
    cpu.register_a = 105;
    cpu.status = 169;
    cpu.pc = 0x8000;
    cpu.register_s = 7;
    cpu.register_x = 23;
    cpu.register_y = 155;

    cpu.run();

    assert_eq!(cpu.register_a, 71);
    assert_eq!(cpu.status, 41);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 7);
    assert_eq!(cpu.register_x, 23);
    assert_eq!(cpu.register_y, 155);
}

#[test]
fn test_a9_8f_a2_27() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x8f, 0xa2]);
    cpu.register_a = 236;
    cpu.status = 166;
    cpu.pc = 0x8000;
    cpu.register_s = 250;
    cpu.register_x = 110;
    cpu.register_y = 121;

    cpu.run();

    assert_eq!(cpu.register_a, 143);
    assert_eq!(cpu.status, 164);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 250);
    assert_eq!(cpu.register_x, 110);
    assert_eq!(cpu.register_y, 121);
}

#[test]
fn test_a9_fc_7c_28() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xfc, 0x7c]);
    cpu.register_a = 200;
    cpu.status = 46;
    cpu.pc = 0x8000;
    cpu.register_s = 205;
    cpu.register_x = 113;
    cpu.register_y = 182;

    cpu.run();

    assert_eq!(cpu.register_a, 252);
    assert_eq!(cpu.status, 172);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 205);
    assert_eq!(cpu.register_x, 113);
    assert_eq!(cpu.register_y, 182);
}

#[test]
fn test_a9_77_b1_29() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x77, 0xb1]);
    cpu.register_a = 227;
    cpu.status = 238;
    cpu.pc = 0x8000;
    cpu.register_s = 174;
    cpu.register_x = 220;
    cpu.register_y = 24;

    cpu.run();

    assert_eq!(cpu.register_a, 119);
    assert_eq!(cpu.status, 108);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 174);
    assert_eq!(cpu.register_x, 220);
    assert_eq!(cpu.register_y, 24);
}

#[test]
fn test_a9_e1_f7_30() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe1, 0xf7]);
    cpu.register_a = 6;
    cpu.status = 34;
    cpu.pc = 0x8000;
    cpu.register_s = 55;
    cpu.register_x = 230;
    cpu.register_y = 122;

    cpu.run();

    assert_eq!(cpu.register_a, 225);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 55);
    assert_eq!(cpu.register_x, 230);
    assert_eq!(cpu.register_y, 122);
}

#[test]
fn test_a9_f3_30_31() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xf3, 0x30]);
    cpu.register_a = 58;
    cpu.status = 42;
    cpu.pc = 0x8000;
    cpu.register_s = 168;
    cpu.register_x = 30;
    cpu.register_y = 145;

    cpu.run();

    assert_eq!(cpu.register_a, 243);
    assert_eq!(cpu.status, 168);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 168);
    assert_eq!(cpu.register_x, 30);
    assert_eq!(cpu.register_y, 145);
}

#[test]
fn test_a9_f8_22_32() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xf8, 0x22]);
    cpu.register_a = 220;
    cpu.status = 100;
    cpu.pc = 0x8000;
    cpu.register_s = 64;
    cpu.register_x = 248;
    cpu.register_y = 72;

    cpu.run();

    assert_eq!(cpu.register_a, 248);
    assert_eq!(cpu.status, 228);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 64);
    assert_eq!(cpu.register_x, 248);
    assert_eq!(cpu.register_y, 72);
}

#[test]
fn test_a9_37_46_33() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x37, 0x46]);
    cpu.register_a = 253;
    cpu.status = 36;
    cpu.pc = 0x8000;
    cpu.register_s = 82;
    cpu.register_x = 211;
    cpu.register_y = 243;

    cpu.run();

    assert_eq!(cpu.register_a, 55);
    assert_eq!(cpu.status, 36);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 82);
    assert_eq!(cpu.register_x, 211);
    assert_eq!(cpu.register_y, 243);
}

#[test]
fn test_a9_f2_4d_34() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xf2, 0x4d]);
    cpu.register_a = 226;
    cpu.status = 168;
    cpu.pc = 0x8000;
    cpu.register_s = 165;
    cpu.register_x = 157;
    cpu.register_y = 75;

    cpu.run();

    assert_eq!(cpu.register_a, 242);
    assert_eq!(cpu.status, 168);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 165);
    assert_eq!(cpu.register_x, 157);
    assert_eq!(cpu.register_y, 75);
}

#[test]
fn test_a9_41_83_35() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x41, 0x83]);
    cpu.register_a = 6;
    cpu.status = 170;
    cpu.pc = 0x8000;
    cpu.register_s = 33;
    cpu.register_x = 211;
    cpu.register_y = 116;

    cpu.run();

    assert_eq!(cpu.register_a, 65);
    assert_eq!(cpu.status, 40);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 33);
    assert_eq!(cpu.register_x, 211);
    assert_eq!(cpu.register_y, 116);
}

#[test]
fn test_a9_25_c1_36() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x25, 0xc1]);
    cpu.register_a = 1;
    cpu.status = 234;
    cpu.pc = 0x8000;
    cpu.register_s = 13;
    cpu.register_x = 158;
    cpu.register_y = 111;

    cpu.run();

    assert_eq!(cpu.register_a, 37);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 13);
    assert_eq!(cpu.register_x, 158);
    assert_eq!(cpu.register_y, 111);
}

#[test]
fn test_a9_c2_ed_37() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xc2, 0xed]);
    cpu.register_a = 137;
    cpu.status = 233;
    cpu.pc = 0x8000;
    cpu.register_s = 239;
    cpu.register_x = 35;
    cpu.register_y = 234;

    cpu.run();

    assert_eq!(cpu.register_a, 194);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 239);
    assert_eq!(cpu.register_x, 35);
    assert_eq!(cpu.register_y, 234);
}

#[test]
fn test_a9_c6_a1_38() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xc6, 0xa1]);
    cpu.register_a = 247;
    cpu.status = 230;
    cpu.pc = 0x8000;
    cpu.register_s = 139;
    cpu.register_x = 47;
    cpu.register_y = 169;

    cpu.run();

    assert_eq!(cpu.register_a, 198);
    assert_eq!(cpu.status, 228);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 139);
    assert_eq!(cpu.register_x, 47);
    assert_eq!(cpu.register_y, 169);
}

#[test]
fn test_a9_83_a1_39() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x83, 0xa1]);
    cpu.register_a = 251;
    cpu.status = 107;
    cpu.pc = 0x8000;
    cpu.register_s = 31;
    cpu.register_x = 124;
    cpu.register_y = 190;

    cpu.run();

    assert_eq!(cpu.register_a, 131);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 31);
    assert_eq!(cpu.register_x, 124);
    assert_eq!(cpu.register_y, 190);
}

#[test]
fn test_a9_a0_3f_40() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xa0, 0x3f]);
    cpu.register_a = 228;
    cpu.status = 99;
    cpu.pc = 0x8000;
    cpu.register_s = 6;
    cpu.register_x = 169;
    cpu.register_y = 10;

    cpu.run();

    assert_eq!(cpu.register_a, 160);
    assert_eq!(cpu.status, 225);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 6);
    assert_eq!(cpu.register_x, 169);
    assert_eq!(cpu.register_y, 10);
}

#[test]
fn test_a9_86_37_41() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x86, 0x37]);
    cpu.register_a = 92;
    cpu.status = 34;
    cpu.pc = 0x8000;
    cpu.register_s = 128;
    cpu.register_x = 145;
    cpu.register_y = 55;

    cpu.run();

    assert_eq!(cpu.register_a, 134);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 128);
    assert_eq!(cpu.register_x, 145);
    assert_eq!(cpu.register_y, 55);
}

#[test]
fn test_a9_7e_7d_42() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x7e, 0x7d]);
    cpu.register_a = 13;
    cpu.status = 96;
    cpu.pc = 0x8000;
    cpu.register_s = 155;
    cpu.register_x = 231;
    cpu.register_y = 17;

    cpu.run();

    assert_eq!(cpu.register_a, 126);
    assert_eq!(cpu.status, 96);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 155);
    assert_eq!(cpu.register_x, 231);
    assert_eq!(cpu.register_y, 17);
}

#[test]
fn test_a9_39_b4_43() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x39, 0xb4]);
    cpu.register_a = 148;
    cpu.status = 102;
    cpu.pc = 0x8000;
    cpu.register_s = 83;
    cpu.register_x = 169;
    cpu.register_y = 241;

    cpu.run();

    assert_eq!(cpu.register_a, 57);
    assert_eq!(cpu.status, 100);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 83);
    assert_eq!(cpu.register_x, 169);
    assert_eq!(cpu.register_y, 241);
}

#[test]
fn test_a9_cf_7c_44() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xcf, 0x7c]);
    cpu.register_a = 77;
    cpu.status = 40;
    cpu.pc = 0x8000;
    cpu.register_s = 165;
    cpu.register_x = 4;
    cpu.register_y = 180;

    cpu.run();

    assert_eq!(cpu.register_a, 207);
    assert_eq!(cpu.status, 168);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 165);
    assert_eq!(cpu.register_x, 4);
    assert_eq!(cpu.register_y, 180);
}

#[test]
fn test_a9_25_48_45() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x25, 0x48]);
    cpu.register_a = 42;
    cpu.status = 235;
    cpu.pc = 0x8000;
    cpu.register_s = 32;
    cpu.register_x = 68;
    cpu.register_y = 190;

    cpu.run();

    assert_eq!(cpu.register_a, 37);
    assert_eq!(cpu.status, 105);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 32);
    assert_eq!(cpu.register_x, 68);
    assert_eq!(cpu.register_y, 190);
}

#[test]
fn test_a9_40_d2_46() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x40, 0xd2]);
    cpu.register_a = 37;
    cpu.status = 35;
    cpu.pc = 0x8000;
    cpu.register_s = 67;
    cpu.register_x = 54;
    cpu.register_y = 135;

    cpu.run();

    assert_eq!(cpu.register_a, 64);
    assert_eq!(cpu.status, 33);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 67);
    assert_eq!(cpu.register_x, 54);
    assert_eq!(cpu.register_y, 135);
}

#[test]
fn test_a9_32_73_47() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x32, 0x73]);
    cpu.register_a = 125;
    cpu.status = 39;
    cpu.pc = 0x8000;
    cpu.register_s = 75;
    cpu.register_x = 63;
    cpu.register_y = 46;

    cpu.run();

    assert_eq!(cpu.register_a, 50);
    assert_eq!(cpu.status, 37);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 75);
    assert_eq!(cpu.register_x, 63);
    assert_eq!(cpu.register_y, 46);
}

#[test]
fn test_a9_8a_4c_48() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x8a, 0x4c]);
    cpu.register_a = 47;
    cpu.status = 226;
    cpu.pc = 0x8000;
    cpu.register_s = 110;
    cpu.register_x = 231;
    cpu.register_y = 151;

    cpu.run();

    assert_eq!(cpu.register_a, 138);
    assert_eq!(cpu.status, 224);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 110);
    assert_eq!(cpu.register_x, 231);
    assert_eq!(cpu.register_y, 151);
}

#[test]
fn test_a9_27_b2_49() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x27, 0xb2]);
    cpu.register_a = 16;
    cpu.status = 104;
    cpu.pc = 0x8000;
    cpu.register_s = 22;
    cpu.register_x = 234;
    cpu.register_y = 252;

    cpu.run();

    assert_eq!(cpu.register_a, 39);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 22);
    assert_eq!(cpu.register_x, 234);
    assert_eq!(cpu.register_y, 252);
}

#[test]
fn test_a9_cb_10_50() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xcb, 0x10]);
    cpu.register_a = 55;
    cpu.status = 38;
    cpu.pc = 0x8000;
    cpu.register_s = 216;
    cpu.register_x = 73;
    cpu.register_y = 90;

    cpu.run();

    assert_eq!(cpu.register_a, 203);
    assert_eq!(cpu.status, 164);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 216);
    assert_eq!(cpu.register_x, 73);
    assert_eq!(cpu.register_y, 90);
}

#[test]
fn test_a9_12_fd_51() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x12, 0xfd]);
    cpu.register_a = 21;
    cpu.status = 47;
    cpu.pc = 0x8000;
    cpu.register_s = 52;
    cpu.register_x = 222;
    cpu.register_y = 76;

    cpu.run();

    assert_eq!(cpu.register_a, 18);
    assert_eq!(cpu.status, 45);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 52);
    assert_eq!(cpu.register_x, 222);
    assert_eq!(cpu.register_y, 76);
}

#[test]
fn test_a9_26_d2_52() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x26, 0xd2]);
    cpu.register_a = 236;
    cpu.status = 108;
    cpu.pc = 0x8000;
    cpu.register_s = 188;
    cpu.register_x = 86;
    cpu.register_y = 117;

    cpu.run();

    assert_eq!(cpu.register_a, 38);
    assert_eq!(cpu.status, 108);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 188);
    assert_eq!(cpu.register_x, 86);
    assert_eq!(cpu.register_y, 117);
}

#[test]
fn test_a9_41_8d_53() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x41, 0x8d]);
    cpu.register_a = 159;
    cpu.status = 35;
    cpu.pc = 0x8000;
    cpu.register_s = 187;
    cpu.register_x = 128;
    cpu.register_y = 76;

    cpu.run();

    assert_eq!(cpu.register_a, 65);
    assert_eq!(cpu.status, 33);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 187);
    assert_eq!(cpu.register_x, 128);
    assert_eq!(cpu.register_y, 76);
}

#[test]
fn test_a9_2f_70_54() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x2f, 0x70]);
    cpu.register_a = 169;
    cpu.status = 33;
    cpu.pc = 0x8000;
    cpu.register_s = 31;
    cpu.register_x = 9;
    cpu.register_y = 138;

    cpu.run();

    assert_eq!(cpu.register_a, 47);
    assert_eq!(cpu.status, 33);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 31);
    assert_eq!(cpu.register_x, 9);
    assert_eq!(cpu.register_y, 138);
}

#[test]
fn test_a9_1c_45_55() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x1c, 0x45]);
    cpu.register_a = 7;
    cpu.status = 109;
    cpu.pc = 0x8000;
    cpu.register_s = 68;
    cpu.register_x = 245;
    cpu.register_y = 83;

    cpu.run();

    assert_eq!(cpu.register_a, 28);
    assert_eq!(cpu.status, 109);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 68);
    assert_eq!(cpu.register_x, 245);
    assert_eq!(cpu.register_y, 83);
}

#[test]
fn test_a9_d7_f7_56() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xd7, 0xf7]);
    cpu.register_a = 73;
    cpu.status = 32;
    cpu.pc = 0x8000;
    cpu.register_s = 101;
    cpu.register_x = 92;
    cpu.register_y = 172;

    cpu.run();

    assert_eq!(cpu.register_a, 215);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 101);
    assert_eq!(cpu.register_x, 92);
    assert_eq!(cpu.register_y, 172);
}

#[test]
fn test_a9_f0_68_57() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xf0, 0x68]);
    cpu.register_a = 72;
    cpu.status = 103;
    cpu.pc = 0x8000;
    cpu.register_s = 213;
    cpu.register_x = 170;
    cpu.register_y = 191;

    cpu.run();

    assert_eq!(cpu.register_a, 240);
    assert_eq!(cpu.status, 229);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 213);
    assert_eq!(cpu.register_x, 170);
    assert_eq!(cpu.register_y, 191);
}

#[test]
fn test_a9_67_d2_58() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x67, 0xd2]);
    cpu.register_a = 150;
    cpu.status = 97;
    cpu.pc = 0x8000;
    cpu.register_s = 170;
    cpu.register_x = 47;
    cpu.register_y = 23;

    cpu.run();

    assert_eq!(cpu.register_a, 103);
    assert_eq!(cpu.status, 97);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 170);
    assert_eq!(cpu.register_x, 47);
    assert_eq!(cpu.register_y, 23);
}

#[test]
fn test_a9_80_27_59() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x80, 0x27]);
    cpu.register_a = 216;
    cpu.status = 225;
    cpu.pc = 0x8000;
    cpu.register_s = 194;
    cpu.register_x = 36;
    cpu.register_y = 63;

    cpu.run();

    assert_eq!(cpu.register_a, 128);
    assert_eq!(cpu.status, 225);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 194);
    assert_eq!(cpu.register_x, 36);
    assert_eq!(cpu.register_y, 63);
}

#[test]
fn test_a9_d8_97_60() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xd8, 0x97]);
    cpu.register_a = 206;
    cpu.status = 167;
    cpu.pc = 0x8000;
    cpu.register_s = 138;
    cpu.register_x = 224;
    cpu.register_y = 100;

    cpu.run();

    assert_eq!(cpu.register_a, 216);
    assert_eq!(cpu.status, 165);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 138);
    assert_eq!(cpu.register_x, 224);
    assert_eq!(cpu.register_y, 100);
}

#[test]
fn test_a9_9a_a4_61() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x9a, 0xa4]);
    cpu.register_a = 104;
    cpu.status = 107;
    cpu.pc = 0x8000;
    cpu.register_s = 162;
    cpu.register_x = 198;
    cpu.register_y = 12;

    cpu.run();

    assert_eq!(cpu.register_a, 154);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 162);
    assert_eq!(cpu.register_x, 198);
    assert_eq!(cpu.register_y, 12);
}

#[test]
fn test_a9_40_dc_62() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x40, 0xdc]);
    cpu.register_a = 80;
    cpu.status = 96;
    cpu.pc = 0x8000;
    cpu.register_s = 117;
    cpu.register_x = 11;
    cpu.register_y = 46;

    cpu.run();

    assert_eq!(cpu.register_a, 64);
    assert_eq!(cpu.status, 96);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 117);
    assert_eq!(cpu.register_x, 11);
    assert_eq!(cpu.register_y, 46);
}

#[test]
fn test_a9_37_de_63() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x37, 0xde]);
    cpu.register_a = 228;
    cpu.status = 101;
    cpu.pc = 0x8000;
    cpu.register_s = 244;
    cpu.register_x = 221;
    cpu.register_y = 255;

    cpu.run();

    assert_eq!(cpu.register_a, 55);
    assert_eq!(cpu.status, 101);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 244);
    assert_eq!(cpu.register_x, 221);
    assert_eq!(cpu.register_y, 255);
}

#[test]
fn test_a9_fa_0a_64() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xfa, 0x0a]);
    cpu.register_a = 196;
    cpu.status = 232;
    cpu.pc = 0x8000;
    cpu.register_s = 213;
    cpu.register_x = 74;
    cpu.register_y = 203;

    cpu.run();

    assert_eq!(cpu.register_a, 250);
    assert_eq!(cpu.status, 232);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 213);
    assert_eq!(cpu.register_x, 74);
    assert_eq!(cpu.register_y, 203);
}

#[test]
fn test_a9_e5_79_65() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe5, 0x79]);
    cpu.register_a = 43;
    cpu.status = 233;
    cpu.pc = 0x8000;
    cpu.register_s = 16;
    cpu.register_x = 184;
    cpu.register_y = 144;

    cpu.run();

    assert_eq!(cpu.register_a, 229);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 16);
    assert_eq!(cpu.register_x, 184);
    assert_eq!(cpu.register_y, 144);
}

#[test]
fn test_a9_f5_13_66() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xf5, 0x13]);
    cpu.register_a = 155;
    cpu.status = 42;
    cpu.pc = 0x8000;
    cpu.register_s = 38;
    cpu.register_x = 49;
    cpu.register_y = 151;

    cpu.run();

    assert_eq!(cpu.register_a, 245);
    assert_eq!(cpu.status, 168);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 38);
    assert_eq!(cpu.register_x, 49);
    assert_eq!(cpu.register_y, 151);
}

#[test]
fn test_a9_42_3f_67() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x42, 0x3f]);
    cpu.register_a = 21;
    cpu.status = 171;
    cpu.pc = 0x8000;
    cpu.register_s = 205;
    cpu.register_x = 208;
    cpu.register_y = 48;

    cpu.run();

    assert_eq!(cpu.register_a, 66);
    assert_eq!(cpu.status, 41);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 205);
    assert_eq!(cpu.register_x, 208);
    assert_eq!(cpu.register_y, 48);
}

#[test]
fn test_a9_ca_52_68() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xca, 0x52]);
    cpu.register_a = 28;
    cpu.status = 238;
    cpu.pc = 0x8000;
    cpu.register_s = 106;
    cpu.register_x = 146;
    cpu.register_y = 244;

    cpu.run();

    assert_eq!(cpu.register_a, 202);
    assert_eq!(cpu.status, 236);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 106);
    assert_eq!(cpu.register_x, 146);
    assert_eq!(cpu.register_y, 244);
}

#[test]
fn test_a9_36_78_69() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x36, 0x78]);
    cpu.register_a = 157;
    cpu.status = 232;
    cpu.pc = 0x8000;
    cpu.register_s = 50;
    cpu.register_x = 175;
    cpu.register_y = 153;

    cpu.run();

    assert_eq!(cpu.register_a, 54);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 50);
    assert_eq!(cpu.register_x, 175);
    assert_eq!(cpu.register_y, 153);
}

#[test]
fn test_a9_e0_ce_70() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe0, 0xce]);
    cpu.register_a = 17;
    cpu.status = 32;
    cpu.pc = 0x8000;
    cpu.register_s = 251;
    cpu.register_x = 156;
    cpu.register_y = 15;

    cpu.run();

    assert_eq!(cpu.register_a, 224);
    assert_eq!(cpu.status, 160);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 251);
    assert_eq!(cpu.register_x, 156);
    assert_eq!(cpu.register_y, 15);
}

#[test]
fn test_a9_e3_d7_71() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe3, 0xd7]);
    cpu.register_a = 218;
    cpu.status = 107;
    cpu.pc = 0x8000;
    cpu.register_s = 233;
    cpu.register_x = 255;
    cpu.register_y = 160;

    cpu.run();

    assert_eq!(cpu.register_a, 227);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 233);
    assert_eq!(cpu.register_x, 255);
    assert_eq!(cpu.register_y, 160);
}

#[test]
fn test_a9_7a_1b_72() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x7a, 0x1b]);
    cpu.register_a = 76;
    cpu.status = 225;
    cpu.pc = 0x8000;
    cpu.register_s = 245;
    cpu.register_x = 48;
    cpu.register_y = 144;

    cpu.run();

    assert_eq!(cpu.register_a, 122);
    assert_eq!(cpu.status, 97);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 245);
    assert_eq!(cpu.register_x, 48);
    assert_eq!(cpu.register_y, 144);
}

#[test]
fn test_a9_dc_70_73() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xdc, 0x70]);
    cpu.register_a = 94;
    cpu.status = 231;
    cpu.pc = 0x8000;
    cpu.register_s = 118;
    cpu.register_x = 83;
    cpu.register_y = 188;

    cpu.run();

    assert_eq!(cpu.register_a, 220);
    assert_eq!(cpu.status, 229);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 118);
    assert_eq!(cpu.register_x, 83);
    assert_eq!(cpu.register_y, 188);
}

#[test]
fn test_a9_04_71_74() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x04, 0x71]);
    cpu.register_a = 70;
    cpu.status = 46;
    cpu.pc = 0x8000;
    cpu.register_s = 217;
    cpu.register_x = 154;
    cpu.register_y = 186;

    cpu.run();

    assert_eq!(cpu.register_a, 4);
    assert_eq!(cpu.status, 44);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 217);
    assert_eq!(cpu.register_x, 154);
    assert_eq!(cpu.register_y, 186);
}

#[test]
fn test_a9_6f_47_75() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x6f, 0x47]);
    cpu.register_a = 157;
    cpu.status = 101;
    cpu.pc = 0x8000;
    cpu.register_s = 5;
    cpu.register_x = 80;
    cpu.register_y = 122;

    cpu.run();

    assert_eq!(cpu.register_a, 111);
    assert_eq!(cpu.status, 101);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 5);
    assert_eq!(cpu.register_x, 80);
    assert_eq!(cpu.register_y, 122);
}

#[test]
fn test_a9_d2_02_76() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xd2, 0x02]);
    cpu.register_a = 248;
    cpu.status = 238;
    cpu.pc = 0x8000;
    cpu.register_s = 131;
    cpu.register_x = 180;
    cpu.register_y = 167;

    cpu.run();

    assert_eq!(cpu.register_a, 210);
    assert_eq!(cpu.status, 236);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 131);
    assert_eq!(cpu.register_x, 180);
    assert_eq!(cpu.register_y, 167);
}

#[test]
fn test_a9_72_28_77() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x72, 0x28]);
    cpu.register_a = 96;
    cpu.status = 109;
    cpu.pc = 0x8000;
    cpu.register_s = 164;
    cpu.register_x = 236;
    cpu.register_y = 61;

    cpu.run();

    assert_eq!(cpu.register_a, 114);
    assert_eq!(cpu.status, 109);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 164);
    assert_eq!(cpu.register_x, 236);
    assert_eq!(cpu.register_y, 61);
}

#[test]
fn test_a9_a5_00_78() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xa5, 0x00]);
    cpu.register_a = 103;
    cpu.status = 46;
    cpu.pc = 0x8000;
    cpu.register_s = 189;
    cpu.register_x = 245;
    cpu.register_y = 12;

    cpu.run();

    assert_eq!(cpu.register_a, 165);
    assert_eq!(cpu.status, 172);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 189);
    assert_eq!(cpu.register_x, 245);
    assert_eq!(cpu.register_y, 12);
}

#[test]
fn test_a9_1d_4b_79() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x1d, 0x4b]);
    cpu.register_a = 48;
    cpu.status = 167;
    cpu.pc = 0x8000;
    cpu.register_s = 136;
    cpu.register_x = 166;
    cpu.register_y = 186;

    cpu.run();

    assert_eq!(cpu.register_a, 29);
    assert_eq!(cpu.status, 37);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 136);
    assert_eq!(cpu.register_x, 166);
    assert_eq!(cpu.register_y, 186);
}

#[test]
fn test_a9_e6_d7_80() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xe6, 0xd7]);
    cpu.register_a = 182;
    cpu.status = 33;
    cpu.pc = 0x8000;
    cpu.register_s = 58;
    cpu.register_x = 242;
    cpu.register_y = 76;

    cpu.run();

    assert_eq!(cpu.register_a, 230);
    assert_eq!(cpu.status, 161);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 58);
    assert_eq!(cpu.register_x, 242);
    assert_eq!(cpu.register_y, 76);
}

#[test]
fn test_a9_2a_03_81() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x2a, 0x03]);
    cpu.register_a = 10;
    cpu.status = 166;
    cpu.pc = 0x8000;
    cpu.register_s = 182;
    cpu.register_x = 241;
    cpu.register_y = 199;

    cpu.run();

    assert_eq!(cpu.register_a, 42);
    assert_eq!(cpu.status, 36);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 182);
    assert_eq!(cpu.register_x, 241);
    assert_eq!(cpu.register_y, 199);
}

#[test]
fn test_a9_1b_1f_82() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x1b, 0x1f]);
    cpu.register_a = 81;
    cpu.status = 166;
    cpu.pc = 0x8000;
    cpu.register_s = 182;
    cpu.register_x = 191;
    cpu.register_y = 237;

    cpu.run();

    assert_eq!(cpu.register_a, 27);
    assert_eq!(cpu.status, 36);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 182);
    assert_eq!(cpu.register_x, 191);
    assert_eq!(cpu.register_y, 237);
}

#[test]
fn test_a9_8e_89_83() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x8e, 0x89]);
    cpu.register_a = 184;
    cpu.status = 231;
    cpu.pc = 0x8000;
    cpu.register_s = 184;
    cpu.register_x = 70;
    cpu.register_y = 130;

    cpu.run();

    assert_eq!(cpu.register_a, 142);
    assert_eq!(cpu.status, 229);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 184);
    assert_eq!(cpu.register_x, 70);
    assert_eq!(cpu.register_y, 130);
}

#[test]
fn test_a9_ee_72_84() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xee, 0x72]);
    cpu.register_a = 79;
    cpu.status = 161;
    cpu.pc = 0x8000;
    cpu.register_s = 199;
    cpu.register_x = 155;
    cpu.register_y = 10;

    cpu.run();

    assert_eq!(cpu.register_a, 238);
    assert_eq!(cpu.status, 161);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 199);
    assert_eq!(cpu.register_x, 155);
    assert_eq!(cpu.register_y, 10);
}

#[test]
fn test_a9_13_72_85() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x13, 0x72]);
    cpu.register_a = 200;
    cpu.status = 99;
    cpu.pc = 0x8000;
    cpu.register_s = 223;
    cpu.register_x = 196;
    cpu.register_y = 97;

    cpu.run();

    assert_eq!(cpu.register_a, 19);
    assert_eq!(cpu.status, 97);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 223);
    assert_eq!(cpu.register_x, 196);
    assert_eq!(cpu.register_y, 97);
}

#[test]
fn test_a9_5a_97_86() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x5a, 0x97]);
    cpu.register_a = 11;
    cpu.status = 97;
    cpu.pc = 0x8000;
    cpu.register_s = 100;
    cpu.register_x = 206;
    cpu.register_y = 126;

    cpu.run();

    assert_eq!(cpu.register_a, 90);
    assert_eq!(cpu.status, 97);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 100);
    assert_eq!(cpu.register_x, 206);
    assert_eq!(cpu.register_y, 126);
}

#[test]
fn test_a9_b7_f6_87() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xb7, 0xf6]);
    cpu.register_a = 41;
    cpu.status = 111;
    cpu.pc = 0x8000;
    cpu.register_s = 212;
    cpu.register_x = 37;
    cpu.register_y = 102;

    cpu.run();

    assert_eq!(cpu.register_a, 183);
    assert_eq!(cpu.status, 237);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 212);
    assert_eq!(cpu.register_x, 37);
    assert_eq!(cpu.register_y, 102);
}

#[test]
fn test_a9_2e_c5_88() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x2e, 0xc5]);
    cpu.register_a = 92;
    cpu.status = 230;
    cpu.pc = 0x8000;
    cpu.register_s = 230;
    cpu.register_x = 178;
    cpu.register_y = 232;

    cpu.run();

    assert_eq!(cpu.register_a, 46);
    assert_eq!(cpu.status, 100);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 230);
    assert_eq!(cpu.register_x, 178);
    assert_eq!(cpu.register_y, 232);
}

#[test]
fn test_a9_db_92_89() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xdb, 0x92]);
    cpu.register_a = 65;
    cpu.status = 45;
    cpu.pc = 0x8000;
    cpu.register_s = 63;
    cpu.register_x = 48;
    cpu.register_y = 76;

    cpu.run();

    assert_eq!(cpu.register_a, 219);
    assert_eq!(cpu.status, 173);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 63);
    assert_eq!(cpu.register_x, 48);
    assert_eq!(cpu.register_y, 76);
}

#[test]
fn test_a9_90_f0_90() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x90, 0xf0]);
    cpu.register_a = 61;
    cpu.status = 227;
    cpu.pc = 0x8000;
    cpu.register_s = 143;
    cpu.register_x = 184;
    cpu.register_y = 154;

    cpu.run();

    assert_eq!(cpu.register_a, 144);
    assert_eq!(cpu.status, 225);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 143);
    assert_eq!(cpu.register_x, 184);
    assert_eq!(cpu.register_y, 154);
}

#[test]
fn test_a9_9d_6c_91() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x9d, 0x6c]);
    cpu.register_a = 145;
    cpu.status = 226;
    cpu.pc = 0x8000;
    cpu.register_s = 113;
    cpu.register_x = 189;
    cpu.register_y = 234;

    cpu.run();

    assert_eq!(cpu.register_a, 157);
    assert_eq!(cpu.status, 224);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 113);
    assert_eq!(cpu.register_x, 189);
    assert_eq!(cpu.register_y, 234);
}

#[test]
fn test_a9_b4_5a_92() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xb4, 0x5a]);
    cpu.register_a = 84;
    cpu.status = 35;
    cpu.pc = 0x8000;
    cpu.register_s = 244;
    cpu.register_x = 183;
    cpu.register_y = 239;

    cpu.run();

    assert_eq!(cpu.register_a, 180);
    assert_eq!(cpu.status, 161);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 244);
    assert_eq!(cpu.register_x, 183);
    assert_eq!(cpu.register_y, 239);
}

#[test]
fn test_a9_73_95_93() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x73, 0x95]);
    cpu.register_a = 132;
    cpu.status = 229;
    cpu.pc = 0x8000;
    cpu.register_s = 205;
    cpu.register_x = 147;
    cpu.register_y = 198;

    cpu.run();

    assert_eq!(cpu.register_a, 115);
    assert_eq!(cpu.status, 101);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 205);
    assert_eq!(cpu.register_x, 147);
    assert_eq!(cpu.register_y, 198);
}

#[test]
fn test_a9_27_85_94() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x27, 0x85]);
    cpu.register_a = 56;
    cpu.status = 234;
    cpu.pc = 0x8000;
    cpu.register_s = 17;
    cpu.register_x = 22;
    cpu.register_y = 118;

    cpu.run();

    assert_eq!(cpu.register_a, 39);
    assert_eq!(cpu.status, 104);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 17);
    assert_eq!(cpu.register_x, 22);
    assert_eq!(cpu.register_y, 118);
}

#[test]
fn test_a9_17_97_95() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x17, 0x97]);
    cpu.register_a = 255;
    cpu.status = 45;
    cpu.pc = 0x8000;
    cpu.register_s = 190;
    cpu.register_x = 152;
    cpu.register_y = 185;

    cpu.run();

    assert_eq!(cpu.register_a, 23);
    assert_eq!(cpu.status, 45);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 190);
    assert_eq!(cpu.register_x, 152);
    assert_eq!(cpu.register_y, 185);
}

#[test]
fn test_a9_14_4c_96() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x14, 0x4c]);
    cpu.register_a = 249;
    cpu.status = 163;
    cpu.pc = 0x8000;
    cpu.register_s = 166;
    cpu.register_x = 210;
    cpu.register_y = 200;

    cpu.run();

    assert_eq!(cpu.register_a, 20);
    assert_eq!(cpu.status, 33);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 166);
    assert_eq!(cpu.register_x, 210);
    assert_eq!(cpu.register_y, 200);
}

#[test]
fn test_a9_fc_66_97() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0xfc, 0x66]);
    cpu.register_a = 38;
    cpu.status = 105;
    cpu.pc = 0x8000;
    cpu.register_s = 237;
    cpu.register_x = 117;
    cpu.register_y = 2;

    cpu.run();

    assert_eq!(cpu.register_a, 252);
    assert_eq!(cpu.status, 233);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 237);
    assert_eq!(cpu.register_x, 117);
    assert_eq!(cpu.register_y, 2);
}

#[test]
fn test_a9_48_5c_98() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x48, 0x5c]);
    cpu.register_a = 242;
    cpu.status = 97;
    cpu.pc = 0x8000;
    cpu.register_s = 86;
    cpu.register_x = 98;
    cpu.register_y = 75;

    cpu.run();

    assert_eq!(cpu.register_a, 72);
    assert_eq!(cpu.status, 97);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 86);
    assert_eq!(cpu.register_x, 98);
    assert_eq!(cpu.register_y, 75);
}

#[test]
fn test_a9_02_ea_99() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![0xa9, 0x02, 0xea]);
    cpu.register_a = 119;
    cpu.status = 45;
    cpu.pc = 0x8000;
    cpu.register_s = 224;
    cpu.register_x = 167;
    cpu.register_y = 2;

    cpu.run();

    assert_eq!(cpu.register_a, 2);
    assert_eq!(cpu.status, 45);
    assert_eq!(cpu.pc,   0x8003);
    assert_eq!(cpu.register_s, 224);
    assert_eq!(cpu.register_x, 167);
    assert_eq!(cpu.register_y, 2);
}

