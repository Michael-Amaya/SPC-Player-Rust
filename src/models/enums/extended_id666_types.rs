use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive, Eq, Hash, PartialEq)]
pub enum ExtendedID666Types {
    SongName = 0x1,
    GameName = 0x2,
    ArtistName = 0x3,
    DumperName = 0x4,
    DateDumped = 0x5,
    EmulatorUsed = 0x6,
    Comments = 0x7,
    OfficialSoundtrackTitle = 0x10,
    OSTDisc = 0x11,
    OSTTrack = 0x12,
    PublisherName = 0x13,
    CopyrightYear = 0x14,
    IntroductionLength = 0x30,
    LoopLength = 0x31,
    EndLength = 0x32,
    FadeLength = 0x33,
    MutedChannels = 0x34,
    NumTimesLoop = 0x35,
    Amplification = 0x36,
}
