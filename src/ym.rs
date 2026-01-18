use soundlog::Instance;
use soundlog::VgmBuilder;
use soundlog::chip::{Ym2203Spec, Ymf262Spec};

#[rustfmt::skip]
pub const OPL3_OPS_BY_CH: [(u8, u8); 18] = [
    (0, 3), (1, 4), (2, 5), (6, 9), (7, 10), (8, 11), (12, 15), (13, 16), (14, 17),
    (18, 21), (19, 22), (20, 23), (24, 27), (25, 28), (26, 29), (30, 33), (31, 34), (32, 35),
];

#[rustfmt::skip]
pub const OPL3_OP_MAP: [(u8, u8); 36] = [
    (0, 0x00), (0, 0x01), (0, 0x02), (0, 0x03), (0, 0x04), (0, 0x05), (0, 0x08), (0, 0x09), (0, 0x0A),
    (0, 0x0B), (0, 0x0C), (0, 0x0D), (0, 0x10), (0, 0x11), (0, 0x12), (0, 0x13), (0, 0x14), (0, 0x15),
    (1, 0x00), (1, 0x01), (1, 0x02), (1, 0x03), (1, 0x04), (1, 0x05), (1, 0x08), (1, 0x09), (1, 0x0A),
    (1, 0x0B), (1, 0x0C), (1, 0x0D), (1, 0x10), (1, 0x11), (1, 0x12), (1, 0x13), (1, 0x14), (1, 0x15),
];

pub fn init_ym2203(b: &mut VgmBuilder, port: u8) {
    let _ = port;
    let _ = b;
}

pub fn init_ymf262(b: &mut VgmBuilder) {
    // enable OPL3 mode
    b.add_chip_write(
        Instance::Primary,
        Ymf262Spec {
            port: 1,
            register: 0x05,
            value: 0x01,
        },
    );
    // op2
    b.add_chip_write(
        Instance::Primary,
        Ymf262Spec {
            port: 1,
            register: 0x04,
            value: 0x00,
        },
    );
}

pub fn init_ym2203_channel_and_op(
    b: &mut VgmBuilder,
    instance: u8,
    ch: u8,
    fnum_val: u16,
    block_val: u8,
    tl: u8,
) {
    // Fixed parameters (sin wave)
    let dt_ml: u8 = 0x01;
    let ksl_ar: u8 = 31;
    let dr: u8 = 0;
    let sr: u8 = 0;
    let sl_rr: u8 = 0;
    let alg_fb: u8 = 0x07;
    let instance: Instance = (instance as usize).into();

    // write f-number low/high before operator setup
    let low = (fnum_val & 0xFF) as u8;
    let high = (((fnum_val >> 8) & 0x07) as u8) | ((block_val & 0x07) << 3);
    let use_op = 0u8; // OP1
    b.add_chip_write(
        instance,
        Ym2203Spec {
            register: 0xA0 + ch,
            value: low,
        },
    );
    b.add_chip_write(
        instance,
        Ym2203Spec {
            register: 0xA4 + ch,
            value: high,
        },
    );
    b.add_chip_write(
        instance,
        Ym2203Spec {
            register: 0xB0 + ch,
            value: alg_fb,
        },
    );

    for op in 0u8..4u8 {
        let dt_ml_reg = 0x30 + op * 4 + ch;
        let tl_reg = 0x40 + op * 4 + ch;
        let ksl_ar_reg = 0x50 + op * 4 + ch;
        let dr_reg = 0x60 + op * 4 + ch;
        let sr_reg = 0x70 + op * 4 + ch;
        let sl_rr_reg = 0x80 + op * 4 + ch;

        b.add_chip_write(
            instance,
            Ym2203Spec {
                register: dt_ml_reg,
                value: dt_ml,
            },
        );
        let tl_val = if op == use_op { tl } else { 0x3F };
        b.add_chip_write(
            instance,
            Ym2203Spec {
                register: tl_reg,
                value: tl_val,
            },
        );
        b.add_chip_write(
            instance,
            Ym2203Spec {
                register: ksl_ar_reg,
                value: ksl_ar,
            },
        );
        b.add_chip_write(
            instance,
            Ym2203Spec {
                register: dr_reg,
                value: dr,
            },
        );
        b.add_chip_write(
            instance,
            Ym2203Spec {
                register: sr_reg,
                value: sr,
            },
        );
        b.add_chip_write(
            instance,
            Ym2203Spec {
                register: sl_rr_reg,
                value: sl_rr,
            },
        );
    }

    // rewrite frequency after operator setup
    b.add_chip_write(
        instance,
        Ym2203Spec {
            register: 0xA0 + ch,
            value: low,
        },
    );
    b.add_chip_write(
        instance,
        Ym2203Spec {
            register: 0xA4 + ch,
            value: high,
        },
    );
}

pub fn init_ymf262_channel_and_op(
    b: &mut VgmBuilder,
    ch: u8,
    fnum_val: u16,
    block_val: u8,
    tl: u8,
) {
    // Fixed defaults (sin wave)
    let dt_ml: u8 = 0x01;
    let ar_dr: u8 = 0xC0;
    let sr_rr: u8 = 0x00;
    let waveform: u8 = 0x00; // sine

    let low = (fnum_val & 0xFF) as u8;
    let high = (((fnum_val >> 8) & 0x03) as u8) | ((block_val & 0x07) << 2);

    let freq_port = if ch >= 9 { 1 } else { 0 };
    let freq_idx = ch % 9;

    // write f-number low/high before operator setup
    b.add_chip_write(
        Instance::Primary,
        Ymf262Spec {
            port: freq_port,
            register: 0xA0 + freq_idx,
            value: low,
        },
    );
    b.add_chip_write(
        Instance::Primary,
        Ymf262Spec {
            port: freq_port,
            register: 0xB0 + freq_idx,
            value: high,
        },
    );

    let (op_mod, op_car) = if (ch as usize) < OPL3_OPS_BY_CH.len() {
        OPL3_OPS_BY_CH[ch as usize]
    } else {
        (0u8, 3u8)
    };

    for &op in &[op_mod, op_car] {
        let (port, off) = OPL3_OP_MAP[op as usize];
        // use provided modulator TL for modulator, carrier TL remains 0
        let tl_val = if op == op_mod { 0x3F } else { tl };
        b.add_chip_write(
            Instance::Primary,
            Ymf262Spec {
                port,
                register: 0x20 + off,
                value: dt_ml,
            },
        );
        b.add_chip_write(
            Instance::Primary,
            Ymf262Spec {
                port,
                register: 0x40 + off,
                value: tl_val,
            },
        );
        b.add_chip_write(
            Instance::Primary,
            Ymf262Spec {
                port,
                register: 0x60 + off,
                value: ar_dr,
            },
        );
        b.add_chip_write(
            Instance::Primary,
            Ymf262Spec {
                port,
                register: 0x80 + off,
                value: sr_rr,
            },
        );
        b.add_chip_write(
            Instance::Primary,
            Ymf262Spec {
                port,
                register: 0xE0 + off,
                value: waveform,
            },
        );
    }

    // rewrite frequency after operator setup
    b.add_chip_write(
        Instance::Primary,
        Ymf262Spec {
            port: freq_port,
            register: 0xA0 + freq_idx,
            value: low,
        },
    );
    b.add_chip_write(
        Instance::Primary,
        Ymf262Spec {
            port: freq_port,
            register: 0xB0 + freq_idx,
            value: high,
        },
    );
}

pub fn ym2203_keyon(
    b: &mut VgmBuilder,
    instance: u8,
    ch: u8,
    fnum_val: u16,
    block_val: u8,
    tl: u8,
) {
    let instance: Instance = (instance as usize).into();
    let low = (fnum_val & 0xFF) as u8;
    let high = (((fnum_val >> 8) & 0x07) as u8) | ((block_val & 0x07) << 3);

    let use_op = 0u8;
    for op in 0u8..4u8 {
        let tl_reg = 0x40 + op * 4 + ch;
        let tl_val = if op == use_op { tl } else { 0x3F };
        b.add_chip_write(
            instance,
            Ym2203Spec {
                register: tl_reg,
                value: tl_val,
            },
        );
    }
    // set frequency
    b.add_chip_write(
        instance,
        Ym2203Spec {
            register: 0xA0 + ch,
            value: low,
        },
    );
    b.add_chip_write(
        instance,
        Ym2203Spec {
            register: 0xA4 + ch,
            value: high,
        },
    );
    // key-on
    b.add_chip_write(
        instance,
        Ym2203Spec {
            register: 0x28,
            value: 0xF0 | (ch & 0x0F),
        },
    );
}

pub fn ymf262_keyon(b: &mut VgmBuilder, ch: u8, fnum_val: u16, block_val: u8, tl: u8) {
    let low = (fnum_val & 0xFF) as u8;
    let high = (((fnum_val >> 8) & 0x03) as u8) | ((block_val & 0x07) << 2);
    let port: u8 = if ch >= 9 { 1 } else { 0 };
    let reg_ch = ch % 9;

    let (op_mod, op_car) = if (ch as usize) < OPL3_OPS_BY_CH.len() {
        OPL3_OPS_BY_CH[ch as usize]
    } else {
        (0u8, 3u8)
    };
    for &op in &[op_mod, op_car] {
        let (port, off) = OPL3_OP_MAP[op as usize];
        let tl_val = if op == op_mod { 0x3F } else { tl };
        b.add_chip_write(
            Instance::Primary,
            Ymf262Spec {
                port,
                register: 0x40 + off,
                value: tl_val,
            },
        );
    }
    // set frequency
    b.add_chip_write(
        Instance::Primary,
        Ymf262Spec {
            port,
            register: 0xA0 + reg_ch,
            value: low,
        },
    );
    // key-on
    b.add_chip_write(
        Instance::Primary,
        Ymf262Spec {
            port,
            register: 0xB0 + reg_ch,
            value: high | 0x20,
        },
    );
}
