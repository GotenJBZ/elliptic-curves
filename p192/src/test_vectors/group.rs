//! Test vectors for the secp192r1 group.

use hex_literal::hex;

/// Repeated addition of the generator.
///
/// These are the first 20 test vectors for P-192 from: <http://point-at-infinity.org/ecc/nisttv>
pub const ADD_TEST_VECTORS: &[([u8; 24], [u8; 24])] = &[
    (
        hex!("188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012"),
        hex!("07192B95FFC8DA78631011ED6B24CDD573F977A11E794811"),
    ),
    (
        hex!("DAFEBF5828783F2AD35534631588A3F629A70FB16982A888"),
        hex!("DD6BDA0D993DA0FA46B27BBC141B868F59331AFA5C7E93AB"),
    ),
    (
        hex!("76E32A2557599E6EDCD283201FB2B9AADFD0D359CBB263DA"),
        hex!("782C37E372BA4520AA62E0FED121D49EF3B543660CFD05FD"),
    ),
    (
        hex!("35433907297CC378B0015703374729D7A4FE46647084E4BA"),
        hex!("A2649984F2135C301EA3ACB0776CD4F125389B311DB3BE32"),
    ),
    (
        hex!("10BB8E9840049B183E078D9C300E1605590118EBDD7FF590"),
        hex!("31361008476F917BADC9F836E62762BE312B72543CCEAEA1"),
    ),
    (
        hex!("A37ABC6C431F9AC398BF5BD1AA6678320ACE8ECB93D23F2A"),
        hex!("851B3CAEC99908DBFED7040A1BBDA90E081F7C5710BC68F0"),
    ),
    (
        hex!("8DA75A1F75DDCD7660F923243060EDCE5DE37F007011FCFD"),
        hex!("57CB5FCF6860B35418240DB8FDB3C01DD4B702F96409FFB5"),
    ),
    (
        hex!("2FA1F92D1ECCE92014771993CC14899D4B5977883397EDDE"),
        hex!("A338AFDEF78B7214273B8B5978EF733FF2DD8A8E9738F6C0"),
    ),
    (
        hex!("818A4D308B1CABB74E9E8F2BA8D27C9E1D9D375AB980388F"),
        hex!("01D1AA5E208D87CD7C292F7CBB457CDF30EA542176C8E739"),
    ),
    (
        hex!("AA7C4F9EF99E3E96D1AEDE2BD9238842859BB150D1FE9D85"),
        hex!("3212A36547EDC62901EE3658B2F4859460EB5EB2491397B0"),
    ),
    (
        hex!("1C995995EB76324F1844F7164D22B652280940370628A2AA"),
        hex!("EF1765CE37E9EB73029F556400FA77BDB34CB8611AAA9C04"),
    ),
    (
        hex!("1061343F3D456D0ECA013877F8C9E7B28FCCDCDA67EEB8AB"),
        hex!("5A064CAA2EA6B03798FEF8E3E7A48648681EAC020B27293F"),
    ),
    (
        hex!("112AF141D33EFB9F2F68821E051E4EA004144A363C4A090A"),
        hex!("6E0CBE3BFC5293F72A2C1726E081E09E7F10A094432B1C1E"),
    ),
    (
        hex!("13B9310646EBC93B591746B3F7C64E05DEE08843DE1081C1"),
        hex!("1EDCEA63B44142DD15F3B427EC41A1EC4FBACA95E186E6B4"),
    ),
    (
        hex!("8C9595E63B56B633BA3546B2B5414DE736DE4A9E7578B1E7"),
        hex!("266B762A934F00C17CF387993AA566B6AD7537CDD98FC7B1"),
    ),
    (
        hex!("B7310B4548FBFDBD29005092A5355BFCD99473733048AFDF"),
        hex!("FF9EAE9EDCD27C1E42D8585C4546D9491845C56629CF2290"),
    ),
    (
        hex!("44275CD2E1F46DC3F9F57636C2B4213B8BB445930510FF8A"),
        hex!("EFAD8348FDE30C87DE438612A818E98D9B76A67AD25DDFD0"),
    ),
    (
        hex!("C1B4DB0227210613A6CA15C428024E40B6513365D72591A3"),
        hex!("1E26B286BCA1D08F4FE8F801267DF9FD7782EC3EC3F47F53"),
    ),
    (
        hex!("C0626BCF247DE5D307FD839238D72688774FC97A1CF8AD1B"),
        hex!("9CDC99D753973DC197E12778E829C804EC1A6B4E71FAA20A"),
    ),
    (
        hex!("BB6F082321D34DBD786A1566915C6DD5EDF879AB0F5ADD67"),
        hex!("91E4DD8A77C4531C8B76DEF2E5339B5EB95D5D9479DF4C8D"),
    ),
];

/// Scalar multiplication with the generator.
///
/// These are the test vectors for P-192 from <http://point-at-infinity.org/ecc/nisttv>
/// that are not part of [`ADD_TEST_VECTORS`].
pub const MUL_TEST_VECTORS: &[([u8; 24], [u8; 24], [u8; 24])] = &[
    (
        hex!("00000000000000000000000000000000018ebbb95eed0e13"),
        hex!("81E6E0F14C9302C8A8DCA8A038B73165E9687D0490CD9F85"),
        hex!("F58067119EED8579388C4281DC645A27DB7764750E812477"),
    ),
    (
        hex!("000000000000000000159d893d4cdd747246cdca43590e13"),
        hex!("B357B10AC985C891B29FB37DA56661CCCF50CEC21128D4F6"),
        hex!("BA20DC2FA1CC228D3C2D8B538C2177C2921884C6B7F0D96F"),
    ),
    (
        hex!("41ffc1fffffe01fffc0003fffe0007c001fff00003fff07f"),
        hex!("74FEC215F253C6BD845831E059B318C87F727B136A700B91"),
        hex!("4B702B15B126A703E7A7CEC3E0EC81F8DFCA73A59F5D88B9"),
    ),
    (
        hex!("000f8000000007ffffff00ffff000ffffff001fffc000000"),
        hex!("0C40230F9C4B8C0FD91F2C604FCBA9B87C2DFA153F010B4F"),
        hex!("5FC4F5771F467971B2C82752413833A68CE00F4A9A692B02"),
    ),
    (
        hex!("400000003803ffffffcfffffe0800000001ffffe03ffff1f"),
        hex!("28783BBF6208E1FF0F965FD8DC0C26FF1D8E02B433EDF2F7"),
        hex!("A5852BBC44FD8164C1ABA9A3EC7A88E461D5D77ABD743E87"),
    ),
    (
        hex!("7ffffffffe0000007ffffe003fffffe0007fff1ffffe0800"),
        hex!("45DAF0A306121BDB3B82E734CB44FDF65C9930F0E4FD2068"),
        hex!("F039FACE58EB7DE34E3374ADB28DF81F019C4548BAA75B64"),
    ),
    (
        hex!("7ffffe000001f800007fffffffc00001c007c00070000700"),
        hex!("1D5EC85004EA2ABA905CEF98A818A8C3516D7CB69A6FD575"),
        hex!("4008F35F5820F66C902195644162E5AA231DD69C9E1ECC97"),
    ),
    (
        hex!("00ffffffe00000ffffffc0007e0000000fe0000fffff0000"),
        hex!("F063727C2EA4D358AB02F6B0BEEB14DBEAF2E8A1DB3208EE"),
        hex!("427418C015553361769B6A0C42923C4CA103740B6DCD9703"),
    ),
    (
        hex!("0000001ffe060003e0001ffffe0000070000000000004007"),
        hex!("DC81D33CA6604B1EFE49386CD492979EF807B8BAEB8566E3"),
        hex!("D454247FF478514556333B3901C9F1CCC18DBC9AB938CFA0"),
    ),
    (
        hex!("7fff80000000000007ff0000000000000000fffe0800001f"),
        hex!("D932741DF6AA0E1EED24279150436C752AA5ADCFD0698D72"),
        hex!("9759B6D2EF21D885E94CDFF219F17004D8763401DAB021B5"),
    ),
    (
        hex!("00007fffffffffffffc00007ffffe0fffffffffffff800ff"),
        hex!("571477E9D9F2A628780742257F7250C4224C483B30F3A97E"),
        hex!("1AD35EE3177D22DD5F01B5A46FFDEC547B6A41786EBB8C8F"),
    ),
    (
        hex!("7ffffc03ff807fffe0001fffff800fff800001ffff0001ff"),
        hex!("4C69939642792776C826DB8B4EBF4BD8C03FC9DFA2AEC822"),
        hex!("29BF35BE52A6036E07EBA5741CFEB4C143310216EF1B9A2E"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d2281d"),
        hex!("BB6F082321D34DBD786A1566915C6DD5EDF879AB0F5ADD67"),
        hex!("6E1B2275883BACE37489210D1ACC64A046A2A26B8620B372"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d2281e"),
        hex!("C0626BCF247DE5D307FD839238D72688774FC97A1CF8AD1B"),
        hex!("63236628AC68C23E681ED88717D637FA13E594B18E055DF5"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d2281f"),
        hex!("C1B4DB0227210613A6CA15C428024E40B6513365D72591A3"),
        hex!("E1D94D79435E2F70B01707FED9820601887D13C13C0B80AC"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22820"),
        hex!("44275CD2E1F46DC3F9F57636C2B4213B8BB445930510FF8A"),
        hex!("10527CB7021CF37821BC79ED57E71671648959852DA2202F"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22821"),
        hex!("B7310B4548FBFDBD29005092A5355BFCD99473733048AFDF"),
        hex!("00615161232D83E1BD27A7A3BAB926B5E7BA3A99D630DD6F"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22822"),
        hex!("8C9595E63B56B633BA3546B2B5414DE736DE4A9E7578B1E7"),
        hex!("D99489D56CB0FF3E830C7866C55A9948528AC8322670384E"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22823"),
        hex!("13B9310646EBC93B591746B3F7C64E05DEE08843DE1081C1"),
        hex!("E123159C4BBEBD22EA0C4BD813BE5E12B045356A1E79194B"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22824"),
        hex!("112AF141D33EFB9F2F68821E051E4EA004144A363C4A090A"),
        hex!("91F341C403AD6C08D5D3E8D91F7E1F6080EF5F6BBCD4E3E1"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22825"),
        hex!("1061343F3D456D0ECA013877F8C9E7B28FCCDCDA67EEB8AB"),
        hex!("A5F9B355D1594FC86701071C185B79B697E153FDF4D8D6C0"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22826"),
        hex!("1C995995EB76324F1844F7164D22B652280940370628A2AA"),
        hex!("10E89A31C816148CFD60AA9BFF0588414CB3479EE55563FB"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22827"),
        hex!("AA7C4F9EF99E3E96D1AEDE2BD9238842859BB150D1FE9D85"),
        hex!("CDED5C9AB81239D6FE11C9A74D0B7A6A9F14A14DB6EC684F"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22828"),
        hex!("818A4D308B1CABB74E9E8F2BA8D27C9E1D9D375AB980388F"),
        hex!("FE2E55A1DF72783283D6D08344BA831FCF15ABDE893718C6"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22829"),
        hex!("2FA1F92D1ECCE92014771993CC14899D4B5977883397EDDE"),
        hex!("5CC7502108748DEBD8C474A687108CBF0D22757168C7093F"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d2282a"),
        hex!("8DA75A1F75DDCD7660F923243060EDCE5DE37F007011FCFD"),
        hex!("A834A030979F4CABE7DBF247024C3FE12B48FD069BF6004A"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d2282b"),
        hex!("A37ABC6C431F9AC398BF5BD1AA6678320ACE8ECB93D23F2A"),
        hex!("7AE4C3513666F7240128FBF5E44256F0F7E083A8EF43970F"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d2282c"),
        hex!("10BB8E9840049B183E078D9C300E1605590118EBDD7FF590"),
        hex!("CEC9EFF7B8906E84523607C919D89D40CED48DABC331515E"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d2282d"),
        hex!("35433907297CC378B0015703374729D7A4FE46647084E4BA"),
        hex!("5D9B667B0DECA3CFE15C534F88932B0DDAC764CEE24C41CD"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d2282e"),
        hex!("76E32A2557599E6EDCD283201FB2B9AADFD0D359CBB263DA"),
        hex!("87D3C81C8D45BADF559D1F012EDE2B600C4ABC99F302FA02"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d2282f"),
        hex!("DAFEBF5828783F2AD35534631588A3F629A70FB16982A888"),
        hex!("229425F266C25F05B94D8443EBE4796FA6CCE505A3816C54"),
    ),
    (
        hex!("ffffffffffffffffffffffff99def836146bc9b1b4d22830"),
        hex!("188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012"),
        hex!("F8E6D46A003725879CEFEE1294DB32298C06885EE186B7EE"),
    ),
];