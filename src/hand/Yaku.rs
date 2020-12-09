pub enum Yaku {
    //1翻
    Reach, //リーチ
    All_Simples, //断么九
    All_Runs, //平和
    Concealed_Self_Draw, //門前ツモ
    First_Turn_Win, //一発
    Double_Run, //一盃口
    Final_Tile_Win, //河底撈魚 海底摸月
    Kings_Tile_Draw, //嶺上開花
    Double_Reach, //ダブルリーチ
    Add_A_Quad, //搶槓
    //2翻
    All_Triples, //対々和
    Three_Color_Runs, //三色同順
    Seven_Pairs, //七対子
    Full_straight, //一気通貫
    Mixed_Outside_Hand, //混全帯么九
    Three_Concealed_Triples, //三暗刻
    Little_Dragons, //小三元
    All_Terminals_And_Honors, //混老頭
    Three_Color_Tiples, //三色同刻
    Three_Quads, //三槓子
    //3翻
    Half_Flush, //混一色
    Pure_Outside_Hand, //清全帯
    Two_Double_Runs, //二盃口
    //6翻
    Full_Flush, //清一色
    //13翻
    Four_Concealed_Triples, //四暗刻
    Thirteen_Orphans, //国士無双
    Big_Doragons, //大三元
    Four_Winds, //四喜和
    All_Honors, //字一色
    All_Terminals, //清老頭
    Blessing_Of_Earth, //地和
    All_Green, //緑一色
    Nine_Gates, //九蓮宝燈
    Four_Quads, //四槓子
    Blessing_Of_Heaven, //天和
}