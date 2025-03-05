use crate::set1::challenge3; 

pub fn find_the_xored_string() {
    let vec:Vec<char> = INPUT_STR.replace("\n", "").chars().collect();

    for thing in vec.chunks(60) {
        println!("{}", challenge3::single_char_xor_decryption_char_array(thing));
    }


}

#[cfg(test)]
mod tests {

use super::*;

    #[test]
    pub fn do_stuff() {
        find_the_xored_string();
    }
}

const INPUT_STR:&str = "0e3647e8592d35514a081243582536ed3de6734059001e3f535ce6271032
334b041de124f73c18011a50e608097ac308ecee501337ec3e100854201d
40e127f51c10031d0133590b1e490f3514e05a54143d08222c2a4071e351
45440b171d5c1b21342e021c3a0eee7373215c4024f0eb733cf006e2040c
22015e420b07ef21164d5935e82338452f42282c1836e42536284c450de3
043b452e0268e7eb005a080b360f0642e6e342005217ef04a42f3e43113d
581e0829214202063d70030845e5301f5a5212ed0818e22f120b211b171b
ea0b342957394717132307133f143a1357e9ed1f5023034147465c052616
0c300b355c2051373a051851ee154a023723414c023a08171e1b4f17595e
550c3e13e80246320b0bec09362542243be42d1d5d060e203e1a0c66ef48
e159464a582a6a0c50471310084f6b1703221d2e7a54502b2b205c433afa
ec58ea200e3005090e1725005739eda7342aed311001383fff7c58ef1f11
01305424231c0d2c41f105057f74510d335440332f1038ec17275f5814e1
05f12f380720ea2b19e24a07e53c142128354e2827f25a08fb401c3126a6
0d17272f53063954163d050a541b1f1144305ae37d4932431b1f33140b1b
0b4f070f071fe92c200e1fa05e4b272e50201b5d493110e429482c100730
100a3148080f227fe60a132f0c10174fe3f63d1a5d38eb414ca8e82f2b05
0a19e83c58400a023b13234572e6e4272bf67434331631e63b5e0f00175c
54520c2ceb45530e0f78111d0b0707e01e4bf43b0606073854324421e6f9
09e7585353ee4a34190de1354e481c373a1b2b0a136127383e271212191f
0f060d09fb4f2d5024022c5ff6463c390c2b5f1a5532071a31f33503fcea
371d39121605584f48217235ee1e0602445c162e4942254c071954321d29
4a0900e63e5f161e15554045f3594c2a6a77e4e52711602beaf53ae53bed
29011616565d2a372a605bee39eced31183fe068185c3b445b391fe53232
e4102337000303452a1e2f2b29493f54ed5a037b3e08311b625cfd005009
2d560d4b0618203249312a310d5f541f295c3f0f25235c2b20037d1600f3
2c245155e8253708391a7ceb0d05005c3e080f3f0f0e5a16583b111f4448
493804044d262eec3759594f212d562420105d6a39e70a0f3957f347070c
e72d1d1f103807590f4339575e00381074485d2d580249f744052605e11d
e131570ae95307143a71131729552d001057a4540a1f425b190b572dee34
2c1655342f02581c202b0a5c17a358291e1506f325550f05365e165c1c5f
e318164df80b043e5406296e5359271d152f552e155a43eda81f23231d1c
001de0413e174e18192c061e4b3d1b5626f90e3e1429544a20ee150d0c20
32e902193219033c58191302441a5c1b584825ea140c290927aaea53e23c
3a36363a732e32ea3f0e430508204b332c382a19292d5b291122e123446a
1804115614031f5f571f2b143c5d3c1b257a4b37350f18445a3e08341c3d
21f2fb250b2e55151e77253a3f0e5f4b2030370a4155e720e73914e35a4a
510a55583a3c491221397c123a2b14a8305b3b09e71b241d0e51202e1a32
1b51202f4917232b512a141d6812f03c455df05e5a1c2cee14390b3b593a
5f5731e5203116ee131a4a4b24112cef5d0822f035e6547d3a0014462f26
0028fb522104f771501a555d3f581e30e9ec3e49e3e63123432f07794145
1459f6312f000e5a1373e346e40f211e1b0b0e17000f391f170552150500
7e301e18325717e3412e022f087be30e5641080151357714e0e0eee15e11
533258e9360f513b083aa51d2824222f40200a470537ecec392d31070b38
07e32c180dfa56496a461627542115132a4c284050495b23e2245b093159
2d3c230a1e5a300f6c3e26ed0d1709434950fd6f1e121335054129e4e4ec
ef22fa2112311b11584ce43434f46f521a215433f9514fe33d313a3e0838
34e7f336270c08010f2f544f0f1c1e235c0222644c2632efec061de2115f
121a42395d4c560d213b0c0a26a7e4f4382718153d5e511158a10b2c021e
e05d414dfa40222f0c382a03235f4d0d04372d4b7855105e26e44f2e0555
7f3a4f1351f85b0344223e1177e14707190c0e311f4ca633f5f3e9352372
01424d5d1a322a0d381717130e181d07240c2c19ecee750b1a37085d014c
16012c5de55a0314a8260e2759e439123ca0c81c321d454e4e0ee14f4c1d
0b1415512f38580e4e2a227def242643183c224f0ea146443403022fe9fd
43eb2b1078322a02192d5b5e0c360d584d0b5e2c13072912ee32f03f4155
002a52553e08361b0be0074b573e201c164c093a5c0f0159333b59770d5b
38e63c1c5244301a5a01f26930321256143e1ae05e1120a9eaf20a192d58
7d54140a152ef4035f09083ded531ee04df55848020656a1342e502649eb
0c211dfe101702015516341136252f3f06f73247133113f5642d083a3417
015e3d51433f3c003e5e28030b1d413eee186824504b241e0f0d32373e2b
2d465040ec130c5c0e2704aa17010c40095207223669110f22f45ea155f7
14552e2b341e5ce0195351066a23e3283e0ee935444b255a1c5c3cef7614
372b453d5a357c05142be65b3c17f92d2b134853390a312bf92a531b513d
5658265f4c0ce4440a20322f591a413034292b312206a01be6453a512d21
1c585c19f31f785324f8583d1ee02620342b10a236263f105011ee5b0e14
0f522b550818591a752e5fea0e033322ee5e280a4a1b244f5a2b35341255
39093c1ced331b264127173f1312e2455fa33b31012c1f4d073c553f5d5e
18f82d5d07e2430b3b3c1b5b49effb0313173f5d4a2e5c134555ff6b1d1a
550a20234202726341190311295254f4064205aa515ae0145a23071c4e18
3f2047024e3ce4555a1b39fa145455012c3afb0f2d11134846182e3c575b
e3e456571937762828065443153b51152e262f09c937024405284f236432
012f580c3536ec5c021574541d5c41123a4e661d5f0f5f344a083e3a5e4c
4216252d01eb0a2a4623621b48360d312c29f33e380650447617124b3e71
54141e59323606390204e95f1206520e5c084510034d30171c5e744f335d
1e30061401600b342e171059526d1949431a3f412f56594c183711ea4837
3131254f11e76f550e1e4d26f1391f44363b151c31281ff45259351da0e6
5def250d0f3505385f22e9f4112633005d272d092e0138275851f943e90e
0939165718303b445210095c16390cf04f19450e06f4545c0a0c320e3e23
1e0b0b1f573f3d0fe05d43090fa8482242300819313142325b1f4b19365b
0d3b2a5d271e463d2203765245065d5d684a051e5815265b52f3171d3004
6af423303817a43324394af15a5c482e3b16f5a46f1e0b5c1201214b5fe4
4030544f3f51151e436e04203a5e3b287ee303490a43fb3b28042f36504e
1a2d5a03fc0e2c04384046242e2b5e1548101825eb2f285f1a210f022141
122355e90122281deeed3ba05636003826525d5551572d07030d4935201f
2a3c484a15410d3b16375d4665271b5c4ce7ee37083d3e512b45204f17f6
03222801255c2c211a7aeb1e042b4e38e8f1293143203139fb202c325f2b
06542a28041956350e292bf3fe5c32133a2a171b3a3e4e4e3101381529e3
4a5209ef24e5f3225e503b143d0e5747323fe7ee3d5b1b5110395619e65a
1fee0a3945563d2b5703701817584b5f5b54702522f5031b561929ea2d1e
e7271935100e3c31211b23113a3a5524e02241181a251d521ff52f3c5a76
144a0efee02f0f5f1d353a1c112e1909234f032953ec591e0a58e55d2cf4
efee0cf00d0955500210015311467543544708eb590d113d30443d080c1e
1a562c1f7e2b0030094f051c03e30f4d501a0fe22a2817edfc5e470c3843
1c3df1135321a8e9241a5607f8305d571aa546001e3254555a11511924
eb1d3f54ec0fea341a097c502ff1111524e24f5b553e49e8576b5b0e1e33
72413e2f5329e332ec563b5e65185efefd2c3b4e5f0b5133246d214a401d
352a0ae632183d200a162e5346110552131514e0553e51003e220d47424b
1d005c58135f3c1b53300c3b49263928f55625454f3be259361ded1f0834
2d2457524a1e1204255934174d442a1a7d130f350a123c4a075f5be73e30
0c0518582d131f39575925e0231833370c482b270e183810415d5aec1900
453b181df1572735380b0446097f00111f1425070b2e1958102ceb592928
010a4a2d0b0926082d2f1525562d1d070a7a08152f5b4438a4150b132e20
2b395d0d5d015d41335d21250de33e3d42152d3f557d1e44e4ee22255d2d
4a1b5c272d0d1c45072639362e402dee2853e51311262b17aa72eb390410
e7015f0215352030574b4108e44d0e1a204418e62325ff7f34052f234b2d
1d563c13202346071d39e34055402b0b392c27f552222d3deb3843ee2c16
29332a521f3c1b0811e33e1a25520e323e75e01c17473f55071226120d3d
210b35ee1a0a5335222e35033905170c4f3104eb032d425058367d5a2bf2
1e553809415efb1c460f2f0ffafaec491e4d4e49510452e8245a366a4106
e1f92cee0e10142514e7ec13155c412fe901092f1f0fa738280c5eee5e04
3526291e0b2a5f486a3051041f4c16372f5402e6f70b31a03525190b161a
260e5e1f0c2e4d7528ef11552fefe247201e4752085c1da903563c162a4b
2a14ff2e3265e604075e523b24455c364a7f284f3a43051d52152f1119e8
5f02e55a4b1300063640ef10151002565f0b0c010033a1cbef5d3634484a
1b121c585b495a5e033a09037f2d1754072c2d49084055172a3c220bed4f
1613400e1632435c0018482aa55b363d26290ae4405ded280f2b0c271536
4011250ce02119464a1de43113170356342c272d1d3355555e5706245e0a
16272d5e545953002e10020875e223010719555410f91ce518420e382456
0d4037320345f945241a1d090a545a310142442131464f4d10562ae4f05a
07ee4d4ae12e571e313c1636313134233e495459e548317708563c2c1b2f
e75803294b36565225552c3406304f0201e43323291b5e0e2159025c2f25
5e63194411490c44494232237e1b323108573d3f391d1f3537e4165a2b35
51000a3a264c503b5852072a5636f04f5cea58a42838f5fca876415c3521
3c14130be511275932055a30aa2d03470c51060009f210543002585f5713
10f0370c5823115200e5015d083e2f1a5df91d68065c1b03f0080855e529
02ec00f1462d034123151ba6fc07eb3d5e54e85a3f3ee532fb41791a060b
0c29274232f93efb3d465544e45e491b042ced245100e3f05c14134c254b
5741235f051e080401a8013c065627e8ee5432205114243d54320e133f2d
4a4d181635411f5d084e31ed230c16506d5125415e060e4dcd0e5f3708e3
2d531c3e22065a5eee07310c145305131800063e4a20094b2006ea131240
e7335c1c4308160be6aa551a0f5a58243e0b10ee470047683c345e1c5b0c
5434505ee22a18110d20342e4b53062c4d79042a0a02422e225b2523e95a
3252212407115c07e15eee06391d0519e9271b641330011f383410281f0e
2cee2b355233292b595d1c69592f483b54584f7154fd4928560752e333a1
17272b272f110df5e91c560a39104510240b5c4b0c1c570871e422351927
c32550ec3f132c0c2458503ae5241d3c0d7911480a073826315620403615
16e11c270d2b010650145de2290b0beb1e120a3a354b2104064f3b533c4e
505746313d4d2e3455290a281ee81d50007e1148252528025237715a342a
1c0a13163e404e40242142061d34185421160220fa031f7a423a08f2e01a
101d303802f51b0c08ef461259315b553823e622a12d565509e23c624139
0a3d1309e4384c0eed383846545a035a41ee1771513b090a031e15f45159
2d4944092a1965542507003b23195758403e175a0a450c5c38114de21141
eb100fe63a031c4b35eb591845e428441c0d5b0037131f5c160a31243619
c155ef0d19143e24392507a202581a25491b135c27571d5c5b35250f0bef
0e1d510556485e39557e044e2cf10457523016473f500b1e36370c17591c
7e5a19250a5e152b46f5130a094cef08e84704ef10197324464b0114017a
3b56f126390008343d3c400232ed201667211f0b1a1413080202530b08e2
4912321b61c90a0cf6ef0a0a0c0f17fa62eb385e2616194526701aff5fe6
2c57114b0400152d4f2aeb18ed41386c2e3a023a281d1a311eefe750ebab
3a4353282114593b3e36446d2c5e1e582e335337022930331f211604576a
295f3bfae9271ae8065a3b4417545c3e5b0df11a53351c78530915392d2e
074a122ee01b17131e4e124e2322a9560ce4120e37582b24e1036fe93f30
3c08290121090ef72f25e4f220323444532d3fe71f34553c7b2726131009
12e84a3308590357a719e74c4f2133690a20031a0b045af63551325b1219
0e3d4fe03f56523cf40f29e4353455120e3a4f2f26f6a30a2b3e0c5b085a
57f3315c33e41c0f523426232d0651395c1525274e314d0219163b5f181f
53471622182739e9e25b473d74e1e7023d095a3134e62d1366563004120e
230a06431935391d5e0b5543223a3bed2b4358f555401e1b3b5c36470d11
22100330e03b4812e6120f163b1ef6abebe6f602545ef9a459e33d334c2a
463405faa655563a43532cfe154bec32fe3345eb2c2700340811213e5006
14241340112b2916017c270a0652732ee8121132385a6c020c040e2be15b
251119225c573b105d5c0a371c3d421ef23e22377fee334e0228561b2d15
2e4c2e373b434b0d0b1b340c300e4b195614130ea03c234c292e14530c46
0d2c3f08560ee32e5a5b6413355215384442563e69ec294a0eef561e3053
193c100c0b24231c012273e10d2e12552723586120020b02e45632265e5f
2c175a11553d4b0b16025e2534180964245b125e5d6e595d1d2a0710580b
213a175ff30855e4001b305000263f5a5c3c5100163cee00114e3518f33a
10ed33e65b003012e7131e161d5e2e270b4645f358394118330f5a5b241b
33e80130f45708395457573406422a3b0d03e6e5053d0d2d151c083337a2
551be2082b1563c4ec2247140400124d4b6508041b5a472256093aea1847
7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
0864eb4935144c501103a71851370719301bec57093a0929ea3f18060e55
2d395e57143359e80efffb13330633ea19e323077b4814571e5a3de73a1f
52e73c1d53330846243c422d3e1b374b5209543903e3195c041c251b7c04
2f3c2c28273a12520b482f18340d565d1fe84735474f4a012e1a13502523
23340f39064e306a08194d544647522e1443041d5ee81f5a18415e34a45f
475a392637565757730a0c4a517b2821040e1709e028071558021f164c54
100b2135190505264254005618f51152136125370eef27383e45350118ed
3947452914e0223f1d040943313c193f295b221e573e1b5723391d090d1f
2c33141859392b04155e3d4e393b322526ee3e581d1b3d6817374d0c085b
c2ea5821200f1b755b2d13130f04e26625ea3a5b1e37144d3e473c24030d
ee15025d2019f757305e3f010e2a453a205f1919391e1a04e86d1a350119
1a5beb4946180fe0002a031a050b41e5164c58795021e1e45c59e2495c20
1121394f1e381c3647005b7326250514272b55250a49183be5454ba518eb
1ee55936102a465d5004371f2e382f1d03144f170d2b0eed042ee341eb19
ec1014ef3ff1272c3408220a41163708140b2e340e505c560c1e4cf82704
274b341a454a27a0263408292e362c201c0401462049523b2d55e5132d54
e259032c444b091e2e4920023f1a7ce40908255228e36f0f2424394b3c48
34130cf8223f23084813e745e006531a1e464b005e0e1ee405413fe22b4e
4af201080c0928420c2d491f6e5121e451223b070dee54244b3efc470a0e
771c161f795df81c22101408465ae7ef0c0604733ee03a20560c1512f217
2f3a142c4155073a200f04166c565634020a59ea04244ff7413c4bc10858
240d4752e5fa5a4e1ce255505602e55d4c575e2b59f52b4e0c0a0b464019
21341927f3380232396707232ae424ea123f5b371d4f65e2471dfbede611
e10e1c3b1d4d28085c091f135b585709332c56134e4844552f45eb41172a
3f1b5a343f034832193b153c482f1705392f021f5f0953290c4c43312b36
3810161aea7001fb5d502b285945255d4ef80131572d2c2e59730e2c3035
4d59052e1f2242403d440a13263e1d2dea0612125e16033b180834030829
022917180d07474c295f793e42274b0e1e16581036225c1211e41e04042f
ec2b41054f2a5f56065e5e0e1f56e13e0a702e1b2f2137020e363a2ae2a4
53085a3b34e75a1caa2e5d031f261f5f044350312f37455d493f131f3746
0c295f1724e90b001a4e015d27091a0b3256302c303d51a05956e6331531
e42b315ce21f0def38144d20242845fa3f3b3b0ce8f4fb2d31ed1d54134b
2957023141335d35372813263b46581af6535a16404d0b4ff12a207648ec
e4421e301de25c43010c504e0f562f2018421ce137443b41134b5f542047
0c5600294e085c1d3622292c480d261213e05c1334385108c145f3090612
062d2e02267404241f4966e6e010052d3224e72856100b1d22f65a30e863
324950394700e11a01201a0564525706f1013f353319076b4c0d015a2e24
2a1be80e2013571522483b1e20321a4e03285d211a444d113924e8f41a1f
27193ae2302208e73010eaa1292001045737013e10e4745aed2c105b25fb
1b135d46eaef103e1d330a14337a2a4302441c1631ed07e7100c743a0e35
1a0957115c293b1c0de853245b5b18e2e12d28421b3230245d7b4a55f355
e7360e2b3846202a2926fa495e3302ed064d127a17343a1f11032b40e8f5
06e8f90a3118381c5414157d1434050210363e30500511a00a3d56e10438
30021931f7193e25a0540ef52658350929380974fb035b1a5d2c042959c7
151b0c24052d0e56025404390e5a3909edec0d03070f040cff710825363e
2a2328120b2203320810134a0c0a0ef30b25460bec011c1e26e913575a51
e12d0948ed3c511416151d1c54082b3e385d14f838510bec4e4b5f585321
1559305c3a49192a010f04ec11001a3d5a5621e5535358353206521f013f
172c2c155a3a322009505c290516a2c4e4405a1e0a1e353b6e1a5a4e2f09
552c34e2432b0df1132b130841000d4007232339a2092a593f142b0a0117
0931432e452d3aea1d02587d3a3e56ed2a3050e2f9363df366331e421947
0250094823545b20163f1d0a36a92228ed25564d1a304deae8035c32370d
4314380e264e2359e6a412504a424328e84434ff30236649353315344a00
25e33540550d3c15135b0eed451cfd1812eaf2063f085d6e214d121c342f
37513b2d0a4e3e5211372a3a01334c5d51030c46463e3756290c0d0e1222
132f175e4c4af1120138e1f2085a3804471f5824555d083de6123f533123
0de11936062d3d2f12193e135f38ff5e1a531d1426523746004e2c063a27
49241aee1802311611a50de9592009e936270108214a0c4213a01f09545f
02e14d2babee204a5c4337135821360d021b7831305963ee0737072f0deb
1512371119050c0c1142245a004f033650481830230a1925085c1a172726
3be62f230a4b50526ec9345100252aa729eafa59221b3fa517304e500a15
5e57f231333c3d0c470a47551733511031362a3bed0f334a3f3136104230
eb24015d051a151f245905061a37ea273d2239fe02463a5e314d565f0457
23025f415d290a594e3b5940313347a11c5e41531ff15a385a183829780a
51e0035f2deb3b163eabe8550e2e0414491f573b5419234a28183044e112
1d54e8390b26585f3aef5f14206672240c4a5e5d31e01b4d406e351401fa
e555173e242c753b275d4ee50b2f26501402a71b1b5733ec19ee34284aed
2ee8f023401c09383b084d623ef324ee5a33065a6d5e365b092c5d0d4501
3f4e024d4b161e144d5e3b140d1e2944465b491d265603a705373c231240
544f0d4ea6091e00e62d3e130d4f005139f339001a3b480c221b730be75e
5f1f4f3e0a0dec3b5128e32960e42d0fee02275528154b10e65c36555a2e
ea3e311b5b0f5f220b1f1b2914f12111f41213e06232224df5ec0114470d
51203f1e01e5563851284013514a565e53125223052f47100e5011100201
3f5bee2305217838582be55958a00245265b0308ec56525b5c114c2d5407
e6e74818e53602160e45372029eb4de72754ec3f49290d2f5901014c0e7f
08e715e612380a5c1908285a1222073a023c562907384e4f470444483f34
1110382b5225343ba6092133483e2d683e1e280227084a1e405e3a341513
415f240f0c53e3f7196e2252fb0105347f345e531f535a344bf439220916
5722e7f7fa2f4c2e057e2a025e2dec31413439aa12265f5a3458f81a4b15
135839401856f337a72fec475a060de239a650163a55392a5b303f051415
56090f18023a2b16e2364407050d48e1541408281d3aa3e84c5b264c1f33
1725f9540aec5e10ed293e4e5a5a2d2125f053251a55395d1c2044022231
292d523ff86a180620075f325e02566659f30423525a053a01f0087f4b3b
17fe493808f25309251e1325596ce32b42311e5d0c2f58652640582a4b17
67381a5afb7128150a0043e45b173d2111155c49092d2635370a3a201826
e62d021d36e03b205d5f1f295c094608342a412122583f3bfc34190be62c
393a055f59060d454a235326e844243a30285c14e316272524f4f0444f51
352c3c5b2b5845244f55494940194721f80b120f07392b7c2c5a0508111e
2f1219430151e60f11150b101e295736361b1e053e4d08f83f230e2c383a
ef5b1d492610e834330f5cf3a2485d324f2822084f41111f582957191b19
1e3e223704fe1d2e1f592753e5550f15170b231b4234e945301f5605a670
300d322759ea0337015c662a0e073809543f2741104835512d0624551751
373727ef1f41084d0b5c0c0137283b1337026aea1c5ae115064ffa183402
09152b11e1233e5a0e302a521c5a33181e180026463744a82c024b4bf04e
1df61df1263fee59135c13400950153d3c5c59183b020b1d2d2c492f4968
e2000c405a01ede30c4c082e2537443c120f38fc57c43651423e5c3beb1d
1922182420191b293e163d58020b005f454a0621051a38e80b090a463ee9
39513f2d47042c0fe5134419ec48490f150f323a5ee7a7e0201e193a5e1b
2037200a2b1013567b35fb4a0f322c2f49435d091920521c302b413f5f35
775d1a345b483b35a02a4c3e17ee3a3d5a5b57153613264f23041922432f
35125b3e0a1d2257eb002a26455e1a2f042e1545e92f0b3408032c4f3551
2d4c392321300a18ed4f3e2c314d20500052aa3917e55d0d29500754282e
381b2e263758f63c474a1c23110c2d5f1c220412e91043580656080c0427
081ce1e5350b6a3535f0e6592e5b543432340e38f008e0324102e45a3f25
30040c181615362e4d1016160a4a5c006eeb1d2422355a3f1028ff192a07
53f6354d4b5d121974245c14f0225713331f2e381810101428571725e432
1a2c06372d5b1419742150042d25003c2650512834ef16e51d183f0f0508
3d191107251100ee2e4125405a44174f061e0e1e5959e606530e06ed245e
3f592d47512dec5922500e460e1de7183b4c3c2e583942255a0c5d4d2305
3438001e482a002d56113a1fe13bed542d3508e22f4e22221431121c1539
ed445a5d28415073eb18022ef836274d573a48090f2a663058194901405d
215b143954fc313c1e28584b51e729ef31013b232bfb4c52e2322a2d4557
5244102e1c3d304450ee01761924e62ff2173305e15809102b2125284dfc
171a3f010f3639056f2be71c2047581de32e05a20833e1221b0e25362459
2958280de238084f5a1c292e005be71f3b311e1f415809383d3862260238
361f56ecee120156375862eb3627185c2519545149e2e50b1f3b0c4e3352
e6115f440634e4005d273611e41c5d383c3814537b3d23362b084024345b
10370656372e0236eb4f3303e216505f0e465228383729394faa2f205f34
2e125b2f2c1d0f1f170e0c51331f0c06291610345c0603791f33253f0e0c
1c2b080526133aeb3e23571d4cfa1e48057a2a010a490a50391b09514f2e
59383ae11237e5450029162d2e1d3e09221a160e42ea06ea0ca7c7ecf4ea
3d3024f34d5c07464bea3b185e110d3a10395d3b2632343cf30ca2e6065a
262f111c0e15441a4825111b185f1e5756243206125f4603e97e79582d27
2d5801ee2654113e2da00b58e9260d643c10423e1d1f42093b0d0f7d5102
3649211f210456051e290f1b4c584d0749220c280b2a50531f262901503e
52053e3e152b5b2b4415580fec57ef5c08e5ed43cc2d2e5b40355d0d2017
6d3917263f030c4b55f0025d501e57504a122729293c4c5819680d3001ed
1e313323324e5e177b171cf70c371541395c0e2b7726e42505483014362e
1910e4f7253f0a012057e03b1e3b4201362b224ff60e0b3a1d115b043957
200c1e0b242e5e3b4755f61e3be05c040908f1234358e55562711d2efa0f
0737e0160b1d13132044080d2325f1f0ee2f00354f2106471131020a5d0b
3f21060de62c052a17576e2ce729242b3e3621300627f01e52580a480050
1b381a11351f4f5d22040c3c4b3e7d263714e8e61a571d107a34260a4a51
edf52314e111207c0b23eb482f441d211f306137152407040e08530a783e
3c054e2d4e2905275e640220f74f1a193f54e1ed5b4e2a290eab27a55147
33522817335316ea2f3df957e25e02030601514f09f74c2fedee102d3114
5d05231d03313826164156110c44e4111f4658005e115e300f413b430300
380bf53a4331f74627492c133fe8eb3141ee39040def040c1a0ae914e3ed
5b00f0211f0a091e05582e22f05a5d262e0ce352251d25100b102b11e339
36053935f051f959093252411e2d5af81f360c0fa15d0b373b1d26323b77
501424184202206215e05944505c4817514540445b0207025de05b050932
0a5a114515536f553a352c513f0b12f700345fa51d5efb28222676e559ea
561b0557403f5f534a574638411e2d3b3c133f79555c333215e6f5f9e7ec
6658f7210218110f00062752e305f21601442c5310162445ed4d175630f3
0e2154253c4a22f02e1b0933351314071b521513235031250c18120024a1
e03555453d1e31775f37331823164c341c09e310463438481019fb0b12fa
37eee654410e4007501f2c0e42faf50125075b2b46164f165a1003097f08
2a5332145851553926523965582e5b2f530d5d1e292046344feaed461517
583d2b06251f551d2f5451110911e6034147481a05166e1f241a5817015b
1f2d3f5c310c315402200010e24135592435f71b4640540a041012ee1b3f
5b2010060e2f5a4d045e0b36192f79181b0732183b4a261038340032f434
3a5557340be6f5315c35112912393503320f54065f0e275a3b5853352008
1c595d183539220eec123478535337110424f90a355af44c267be848173f
41053f5cef5f6f56e4f5410a5407281600200b2649460a2e3a3c38492a0c
4c071a57e9356ee415103c5c53e254063f2019340969e30a2e381d5b2555
32042f46431d2c44607934ed180c1028136a5f2b26092e3b2c4e2930585a";
