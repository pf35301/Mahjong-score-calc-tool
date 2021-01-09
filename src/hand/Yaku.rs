pub enum Yaku {
    //1翻
    Reach,               //リーチ
    AllSimples,         //断么九
    AllRuns,            //平和
    ConcealedSelfDraw, //門前ツモ
    FirstTurnWin,      //一発
    DoubleRun,          //一盃口
    FinalTileWin,      //河底撈魚 海底摸月
    KingsTileDraw,     //嶺上開花
    DoubleReach,        //ダブルリーチ
    AddAQuad,          //搶槓
    //2翻
    AllTriples,              //対々和
    ThreeColorRuns,         //三色同順
    SevenPairs,              //七対子
    FullStraight,            //一気通貫
    MixedOutsideHand,       //混全帯么九
    ThreeConcealedTriples,  //三暗刻
    LittleDragons,           //小三元
    AllTerminalsAndHonors, //混老頭
    ThreeColorTiples,       //三色同刻
    ThreeQuads,              //三槓子
    //3翻
    HalfFlush,        //混一色
    PureOutsideHand, //清全帯
    TwoDoubleRuns,   //二盃口
    //6翻
    FullFlush, //清一色
    //13翻
    FourConcealedTriples, //四暗刻
    ThirteenOrphans,       //国士無双
    BigDoragons,           //大三元
    FourWinds,             //四喜和
    AllHonors,             //字一色
    AllTerminals,          //清老頭
    BlessingOfEarth,      //地和
    AllGreen,              //緑一色
    NineGates,             //九蓮宝燈
    FourQuads,             //四槓子
    BlessingOfHeaven,     //天和
}

impl Yaku {
    pub fn Han(self) -> u32 {
        match self {
            Self::Reach
            | Self::AllSimples
            | Self::AllRuns
            | Self::ConcealedSelfDraw
            | Self::FirstTurnWin
            | Self::DoubleRun
            | Self::FinalTileWin
            | Self::KingsTileDraw
            | Self::DoubleReach
            | Self::AddAQuad => 1,
            Self::AllTriples
            | Self::ThreeColorRuns
            | Self::SevenPairs
            | Self::FullStraight
            | Self::MixedOutsideHand
            | Self::ThreeConcealedTriples
            | Self::LittleDragons
            | Self::AllTerminalsAndHonors
            | Self::ThreeColorTiples
            | Self::ThreeQuads => 2,
            Self::HalfFlush | Self::PureOutsideHand | Self::TwoDoubleRuns => 3,
            Self::FullFlush => 6,
            Self::FourConcealedTriples
            | Self::ThirteenOrphans
            | Self::BigDoragons
            | Self::FourWinds
            | Self::AllHonors
            | Self::AllTerminals
            | Self::BlessingOfEarth
            | Self::AllGreen
            | Yaku::NineGates
            | Yaku::FourQuads
            | Yaku::BlessingOfHeaven => 13,
        }
    }
}
