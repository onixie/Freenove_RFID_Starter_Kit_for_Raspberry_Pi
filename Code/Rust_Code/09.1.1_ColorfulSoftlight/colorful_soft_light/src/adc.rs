pub mod ads7830 {
    use bitflags::bitflags;

    bitflags! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct Address : u16 {
            const Preset = 0b_10010_00;
            const A0 = 0b_01;
            const A1 = 0b_10;
        }

        #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct Command : u8 {
            const Channel0  = 0b_1000_0000;
            const Channel2  = 0b_1001_0000;
            const Channel4  = 0b_1010_0000;
            const Channel6  = 0b_1011_0000;
            const Channel1  = 0b_1100_0000;
            const Channel3  = 0b_1101_0000;
            const Channel5  = 0b_1110_0000;
            const Channel7  = 0b_1111_0000;
            const InternalReferenceOn = 0b_0000_1000;
            const AdcOn = 0b_0000_0100;
        }
    }

    impl Default for Address {
        fn default() -> Self {
            Self::Preset
        }
    }
}
