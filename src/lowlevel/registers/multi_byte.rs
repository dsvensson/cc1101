// multi-byte burst regions
burst!(multi, read = Burst, write = Burst, {
    #[doc = "Power Amplifier Table"]
    PATABLE @ 0x3E,
    #[doc = "FIFO Access"]
    FIFO    @ 0x3F,
});
