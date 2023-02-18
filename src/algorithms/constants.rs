// contants used to test run()
cfg_if! {
    if #[cfg(test)] {
        pub const REPETITIONS: i32 = 10;
        pub const FLOOR: usize = 0;
        pub const CEIL: usize = 100;
        pub const SIZE: usize = 30;
    }
}
