#[allow(unused_imports)]
use crate::chip::*;
#[allow(unused_imports)]
use crate::vgm::command::*;
#[test]
fn spec_decode_vgm_bytes_all() {
    {
        let s = PsgSpec { value: 0xAA };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x50, 0xAA]);
    }
    {
        let s = Ym2413Spec {
            register: 0x10,
            value: 0x22,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x51, 0x10, 0x22]);
    }
    {
        let s0 = Ym2612Spec {
            port: 0,
            register: 0x2A,
            value: 0x55,
        };
        let s1 = Ym2612Spec {
            port: 1,
            register: 0x2A,
            value: 0x66,
        };
        let mut b0 = Vec::new();
        let mut b1 = Vec::new();
        s0.to_vgm_bytes(&mut b0);
        s1.to_vgm_bytes(&mut b1);
        assert_eq!(b0, vec![0x52, 0x2A, 0x55]);
        assert_eq!(b1, vec![0x53, 0x2A, 0x66]);
    }
    {
        let s = Ym2151Spec {
            register: 0x01,
            value: 0x02,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x54, 0x01, 0x02]);
    }
    {
        let s = SegaPcmSpec {
            offset: 0x1234,
            value: 0x77,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC0, 0x12, 0x34, 0x77]);
    }
    {
        let s = Rf5c68Spec {
            offset: 0x0201,
            value: 0x88,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC1, 0x02, 0x01, 0x88]);
    }
    {
        let s = Ym2203Spec {
            register: 0x05,
            value: 0x06,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x55, 0x05, 0x06]);
    }
    {
        let s0 = Ym2608Spec {
            port: 0,
            register: 0x07,
            value: 0x08,
        };
        let s1 = Ym2608Spec {
            port: 1,
            register: 0x07,
            value: 0x09,
        };
        let mut b0 = Vec::new();
        let mut b1 = Vec::new();
        s0.to_vgm_bytes(&mut b0);
        s1.to_vgm_bytes(&mut b1);
        assert_eq!(b0, vec![0x56, 0x07, 0x08]);
        assert_eq!(b1, vec![0x57, 0x07, 0x09]);
    }
    {
        let s0 = Ym2610Spec {
            port: 0,
            register: 0x0A,
            value: 0x0B,
        };
        let s1 = Ym2610Spec {
            port: 1,
            register: 0x0A,
            value: 0x0C,
        };
        let mut b0 = Vec::new();
        let mut b1 = Vec::new();
        s0.to_vgm_bytes(&mut b0);
        s1.to_vgm_bytes(&mut b1);
        assert_eq!(b0, vec![0x58, 0x0A, 0x0B]);
        assert_eq!(b1, vec![0x59, 0x0A, 0x0C]);
    }
    {
        let s = Ym3812Spec {
            register: 0x11,
            value: 0x22,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x5A, 0x11, 0x22]);
    }
    {
        let s = Ym3526Spec {
            register: 0x12,
            value: 0x23,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x5B, 0x12, 0x23]);
    }
    {
        let s = Y8950Spec {
            register: 0x13,
            value: 0x24,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x5C, 0x13, 0x24]);
    }
    {
        let s0 = Ymf262Spec {
            port: 0,
            register: 0x20,
            value: 0x21,
        };
        let s1 = Ymf262Spec {
            port: 1,
            register: 0x20,
            value: 0x22,
        };
        let mut b0 = Vec::new();
        let mut b1 = Vec::new();
        s0.to_vgm_bytes(&mut b0);
        s1.to_vgm_bytes(&mut b1);
        assert_eq!(b0, vec![0x5E, 0x20, 0x21]);
        assert_eq!(b1, vec![0x5F, 0x20, 0x22]);
    }
    {
        let s = Ymf278bSpec {
            port: 0x01,
            register: 0x02,
            value: 0x03,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD0, 0x01, 0x02, 0x03]);
    }
    {
        let s = Ymf271Spec {
            port: 0x02,
            register: 0x03,
            value: 0x04,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD1, 0x02, 0x03, 0x04]);
    }
    {
        let s = Ymz280bSpec {
            register: 0x30,
            value: 0x31,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x5D, 0x30, 0x31]);
    }
    {
        let s = Rf5c164Spec {
            register: 0x40,
            value: 0x41,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB1, 0x40, 0x41]);
    }
    {
        let s = PwmSpec {
            register: 0x01,
            value: 0x00112233,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB2, 0x01, 0x11, 0x22, 0x33]);
    }
    {
        let s = Ay8910Spec {
            register: 0x02,
            value: 0x03,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xA0, 0x02, 0x03]);
    }
    {
        let s = GbDmgSpec {
            register: 0x04,
            value: 0x05,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB3, 0x04, 0x05]);
    }
    {
        let s = NesApuSpec {
            register: 0x06,
            value: 0x07,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB4, 0x06, 0x07]);
    }
    {
        let s = MultiPcmSpec {
            register: 0x08,
            value: 0x09,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB5, 0x08, 0x09]);
    }
    {
        let s = Upd7759Spec {
            register: 0x0A,
            value: 0x0B,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB6, 0x0A, 0x0B]);
    }
    {
        let s = Okim6258Spec {
            register: 0x0C,
            value: 0x0D,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB7, 0x0C, 0x0D]);
    }
    {
        let s = Okim6295Spec {
            register: 0x0E,
            value: 0x0F,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB8, 0x0E, 0x0F]);
    }
    {
        // TODO: Implement K051649Spec VGM byte serialization
        // let result = std::panic::catch_unwind(|| {
        //     let s = K051649Spec {
        //         register: 0x1234,
        //         value: 0x11,
        //     };
        //     let mut buf = Vec::new();
        //     s.to_vgm_bytes(&mut buf);
        // });
        // assert!(result.is_err(), "K051649Spec should panic/unimplemented");
    }
    {
        let s = K054539Spec {
            register: 0x1122,
            value: 0x33,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD3, 0x11, 0x22, 0x33]);
    }
    {
        let s = Huc6280Spec {
            register: 0x12,
            value: 0x13,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB9, 0x12, 0x13]);
    }
    {
        let s = C140Spec {
            register: 0x3344,
            value: 0x55,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD4, 0x33, 0x44, 0x55]);
    }
    {
        let s = K053260Spec {
            register: 0x14,
            value: 0x15,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xBA, 0x14, 0x15]);
    }
    {
        let s = PokeySpec {
            register: 0x16,
            value: 0x17,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xBB, 0x16, 0x17]);
    }
    {
        let s = QsoundSpec {
            register: 0x21,
            value: 0x1234,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC4, 0x21, 0x12, 0x34]);
    }
    {
        let s = ScspSpec {
            offset: 0x5566,
            value: 0x77,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC5, 0x55, 0x66, 0x77]);
    }
    {
        let s = WonderSwanSpec {
            offset: 0x2233,
            value: 0x44,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC6, 0x22, 0x33, 0x44]);
    }
    {
        let s = VsuSpec {
            offset: 0x4455,
            value: 0x66,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC7, 0x44, 0x55, 0x66]);
    }
    {
        let s = Saa1099Spec {
            register: 0x18,
            value: 0x19,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xBD, 0x18, 0x19]);
    }
    {
        let s = Es5503Spec {
            register: 0x8899,
            value: 0xAA,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD5, 0x88, 0x99, 0xAA]);
    }
    {
        let s = Es5506v16Spec {
            register: 0x1A,
            value: 0xBEEF,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD6, 0x1A, 0xBE, 0xEF]);
    }
    {
        let s = X1010Spec {
            offset: 0x7788,
            value: 0x99,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC8, 0x77, 0x88, 0x99]);
    }
    {
        let s = C352Spec {
            register: 0x3344,
            value: 0xCAFE,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xE1, 0x33, 0x44, 0xCA, 0xFE]);
    }
    {
        let s = Ga20Spec {
            register: 0x1B,
            value: 0x1C,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xBF, 0x1B, 0x1C]);
    }
    {
        let s = MikeySpec {
            register: 0x1D,
            value: 0x1E,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x40, 0x1D, 0x1E]);
    }
    {
        let s = GameGearPsgSpec { value: 0x7F };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x4F, 0x7F]);
    }
    {
        let s = Scc1Spec {
            port: 0x05,
            register: 0x06,
            value: 0x07,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD2, 0x05, 0x06, 0x07]);
    }
}
