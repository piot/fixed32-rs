/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/fixed32-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use crate::Fp;
pub const SIN_TABLE: [Fp; 256] = [
    Fp(0),
    Fp(1608),
    Fp(3216),
    Fp(4821),
    Fp(6424),
    Fp(8022),
    Fp(9616),
    Fp(11204),
    Fp(12785),
    Fp(14359),
    Fp(15924),
    Fp(17479),
    Fp(19024),
    Fp(20557),
    Fp(22078),
    Fp(23586),
    Fp(25080),
    Fp(26558),
    Fp(28020),
    Fp(29466),
    Fp(30893),
    Fp(32303),
    Fp(33692),
    Fp(35062),
    Fp(36410),
    Fp(37736),
    Fp(39040),
    Fp(40320),
    Fp(41576),
    Fp(42806),
    Fp(44011),
    Fp(45190),
    Fp(46341),
    Fp(47464),
    Fp(48559),
    Fp(49624),
    Fp(50660),
    Fp(51665),
    Fp(52639),
    Fp(53581),
    Fp(54491),
    Fp(55368),
    Fp(56212),
    Fp(57022),
    Fp(57798),
    Fp(58538),
    Fp(59244),
    Fp(59914),
    Fp(60547),
    Fp(61145),
    Fp(61705),
    Fp(62228),
    Fp(62714),
    Fp(63162),
    Fp(63572),
    Fp(63944),
    Fp(64277),
    Fp(64571),
    Fp(64827),
    Fp(65043),
    Fp(65220),
    Fp(65358),
    Fp(65457),
    Fp(65516),
    Fp(65536),
    Fp(65516),
    Fp(65457),
    Fp(65358),
    Fp(65220),
    Fp(65043),
    Fp(64827),
    Fp(64571),
    Fp(64277),
    Fp(63944),
    Fp(63572),
    Fp(63162),
    Fp(62714),
    Fp(62228),
    Fp(61705),
    Fp(61145),
    Fp(60547),
    Fp(59914),
    Fp(59244),
    Fp(58538),
    Fp(57798),
    Fp(57022),
    Fp(56212),
    Fp(55368),
    Fp(54491),
    Fp(53581),
    Fp(52639),
    Fp(51665),
    Fp(50660),
    Fp(49624),
    Fp(48559),
    Fp(47464),
    Fp(46341),
    Fp(45190),
    Fp(44011),
    Fp(42806),
    Fp(41576),
    Fp(40320),
    Fp(39040),
    Fp(37736),
    Fp(36410),
    Fp(35062),
    Fp(33692),
    Fp(32303),
    Fp(30893),
    Fp(29466),
    Fp(28020),
    Fp(26558),
    Fp(25080),
    Fp(23586),
    Fp(22078),
    Fp(20557),
    Fp(19024),
    Fp(17479),
    Fp(15924),
    Fp(14359),
    Fp(12785),
    Fp(11204),
    Fp(9616),
    Fp(8022),
    Fp(6424),
    Fp(4821),
    Fp(3216),
    Fp(1608),
    Fp(0),
    Fp(-1608),
    Fp(-3216),
    Fp(-4821),
    Fp(-6424),
    Fp(-8022),
    Fp(-9616),
    Fp(-11204),
    Fp(-12785),
    Fp(-14359),
    Fp(-15924),
    Fp(-17479),
    Fp(-19024),
    Fp(-20557),
    Fp(-22078),
    Fp(-23586),
    Fp(-25080),
    Fp(-26558),
    Fp(-28020),
    Fp(-29466),
    Fp(-30893),
    Fp(-32303),
    Fp(-33692),
    Fp(-35062),
    Fp(-36410),
    Fp(-37736),
    Fp(-39040),
    Fp(-40320),
    Fp(-41576),
    Fp(-42806),
    Fp(-44011),
    Fp(-45190),
    Fp(-46341),
    Fp(-47464),
    Fp(-48559),
    Fp(-49624),
    Fp(-50660),
    Fp(-51665),
    Fp(-52639),
    Fp(-53581),
    Fp(-54491),
    Fp(-55368),
    Fp(-56212),
    Fp(-57022),
    Fp(-57798),
    Fp(-58538),
    Fp(-59244),
    Fp(-59914),
    Fp(-60547),
    Fp(-61145),
    Fp(-61705),
    Fp(-62228),
    Fp(-62714),
    Fp(-63162),
    Fp(-63572),
    Fp(-63944),
    Fp(-64277),
    Fp(-64571),
    Fp(-64827),
    Fp(-65043),
    Fp(-65220),
    Fp(-65358),
    Fp(-65457),
    Fp(-65516),
    Fp(-65536),
    Fp(-65516),
    Fp(-65457),
    Fp(-65358),
    Fp(-65220),
    Fp(-65043),
    Fp(-64827),
    Fp(-64571),
    Fp(-64277),
    Fp(-63944),
    Fp(-63572),
    Fp(-63162),
    Fp(-62714),
    Fp(-62228),
    Fp(-61705),
    Fp(-61145),
    Fp(-60547),
    Fp(-59914),
    Fp(-59244),
    Fp(-58538),
    Fp(-57798),
    Fp(-57022),
    Fp(-56212),
    Fp(-55368),
    Fp(-54491),
    Fp(-53581),
    Fp(-52639),
    Fp(-51665),
    Fp(-50660),
    Fp(-49624),
    Fp(-48559),
    Fp(-47464),
    Fp(-46341),
    Fp(-45190),
    Fp(-44011),
    Fp(-42806),
    Fp(-41576),
    Fp(-40320),
    Fp(-39040),
    Fp(-37736),
    Fp(-36410),
    Fp(-35062),
    Fp(-33692),
    Fp(-32303),
    Fp(-30893),
    Fp(-29466),
    Fp(-28020),
    Fp(-26558),
    Fp(-25080),
    Fp(-23586),
    Fp(-22078),
    Fp(-20557),
    Fp(-19024),
    Fp(-17479),
    Fp(-15924),
    Fp(-14359),
    Fp(-12785),
    Fp(-11204),
    Fp(-9616),
    Fp(-8022),
    Fp(-6424),
    Fp(-4821),
    Fp(-3216),
    Fp(-1608),
];
pub const COS_TABLE: [Fp; 256] = [
    Fp(65536),
    Fp(65516),
    Fp(65457),
    Fp(65358),
    Fp(65220),
    Fp(65043),
    Fp(64827),
    Fp(64571),
    Fp(64277),
    Fp(63944),
    Fp(63572),
    Fp(63162),
    Fp(62714),
    Fp(62228),
    Fp(61705),
    Fp(61145),
    Fp(60547),
    Fp(59914),
    Fp(59244),
    Fp(58538),
    Fp(57798),
    Fp(57022),
    Fp(56212),
    Fp(55368),
    Fp(54491),
    Fp(53581),
    Fp(52639),
    Fp(51665),
    Fp(50660),
    Fp(49624),
    Fp(48559),
    Fp(47464),
    Fp(46341),
    Fp(45190),
    Fp(44011),
    Fp(42806),
    Fp(41576),
    Fp(40320),
    Fp(39040),
    Fp(37736),
    Fp(36410),
    Fp(35062),
    Fp(33692),
    Fp(32303),
    Fp(30893),
    Fp(29466),
    Fp(28020),
    Fp(26558),
    Fp(25080),
    Fp(23586),
    Fp(22078),
    Fp(20557),
    Fp(19024),
    Fp(17479),
    Fp(15924),
    Fp(14359),
    Fp(12785),
    Fp(11204),
    Fp(9616),
    Fp(8022),
    Fp(6424),
    Fp(4821),
    Fp(3216),
    Fp(1608),
    Fp(0),
    Fp(-1608),
    Fp(-3216),
    Fp(-4821),
    Fp(-6424),
    Fp(-8022),
    Fp(-9616),
    Fp(-11204),
    Fp(-12785),
    Fp(-14359),
    Fp(-15924),
    Fp(-17479),
    Fp(-19024),
    Fp(-20557),
    Fp(-22078),
    Fp(-23586),
    Fp(-25080),
    Fp(-26558),
    Fp(-28020),
    Fp(-29466),
    Fp(-30893),
    Fp(-32303),
    Fp(-33692),
    Fp(-35062),
    Fp(-36410),
    Fp(-37736),
    Fp(-39040),
    Fp(-40320),
    Fp(-41576),
    Fp(-42806),
    Fp(-44011),
    Fp(-45190),
    Fp(-46341),
    Fp(-47464),
    Fp(-48559),
    Fp(-49624),
    Fp(-50660),
    Fp(-51665),
    Fp(-52639),
    Fp(-53581),
    Fp(-54491),
    Fp(-55368),
    Fp(-56212),
    Fp(-57022),
    Fp(-57798),
    Fp(-58538),
    Fp(-59244),
    Fp(-59914),
    Fp(-60547),
    Fp(-61145),
    Fp(-61705),
    Fp(-62228),
    Fp(-62714),
    Fp(-63162),
    Fp(-63572),
    Fp(-63944),
    Fp(-64277),
    Fp(-64571),
    Fp(-64827),
    Fp(-65043),
    Fp(-65220),
    Fp(-65358),
    Fp(-65457),
    Fp(-65516),
    Fp(-65536),
    Fp(-65516),
    Fp(-65457),
    Fp(-65358),
    Fp(-65220),
    Fp(-65043),
    Fp(-64827),
    Fp(-64571),
    Fp(-64277),
    Fp(-63944),
    Fp(-63572),
    Fp(-63162),
    Fp(-62714),
    Fp(-62228),
    Fp(-61705),
    Fp(-61145),
    Fp(-60547),
    Fp(-59914),
    Fp(-59244),
    Fp(-58538),
    Fp(-57798),
    Fp(-57022),
    Fp(-56212),
    Fp(-55368),
    Fp(-54491),
    Fp(-53581),
    Fp(-52639),
    Fp(-51665),
    Fp(-50660),
    Fp(-49624),
    Fp(-48559),
    Fp(-47464),
    Fp(-46341),
    Fp(-45190),
    Fp(-44011),
    Fp(-42806),
    Fp(-41576),
    Fp(-40320),
    Fp(-39040),
    Fp(-37736),
    Fp(-36410),
    Fp(-35062),
    Fp(-33692),
    Fp(-32303),
    Fp(-30893),
    Fp(-29466),
    Fp(-28020),
    Fp(-26558),
    Fp(-25080),
    Fp(-23586),
    Fp(-22078),
    Fp(-20557),
    Fp(-19024),
    Fp(-17479),
    Fp(-15924),
    Fp(-14359),
    Fp(-12785),
    Fp(-11204),
    Fp(-9616),
    Fp(-8022),
    Fp(-6424),
    Fp(-4821),
    Fp(-3216),
    Fp(-1608),
    Fp(0),
    Fp(1608),
    Fp(3216),
    Fp(4821),
    Fp(6424),
    Fp(8022),
    Fp(9616),
    Fp(11204),
    Fp(12785),
    Fp(14359),
    Fp(15924),
    Fp(17479),
    Fp(19024),
    Fp(20557),
    Fp(22078),
    Fp(23586),
    Fp(25080),
    Fp(26558),
    Fp(28020),
    Fp(29466),
    Fp(30893),
    Fp(32303),
    Fp(33692),
    Fp(35062),
    Fp(36410),
    Fp(37736),
    Fp(39040),
    Fp(40320),
    Fp(41576),
    Fp(42806),
    Fp(44011),
    Fp(45190),
    Fp(46341),
    Fp(47464),
    Fp(48559),
    Fp(49624),
    Fp(50660),
    Fp(51665),
    Fp(52639),
    Fp(53581),
    Fp(54491),
    Fp(55368),
    Fp(56212),
    Fp(57022),
    Fp(57798),
    Fp(58538),
    Fp(59244),
    Fp(59914),
    Fp(60547),
    Fp(61145),
    Fp(61705),
    Fp(62228),
    Fp(62714),
    Fp(63162),
    Fp(63572),
    Fp(63944),
    Fp(64277),
    Fp(64571),
    Fp(64827),
    Fp(65043),
    Fp(65220),
    Fp(65358),
    Fp(65457),
    Fp(65516),
];
pub const ASIN_TABLE: [Fp; 256] = [
    Fp(-102944),
    Fp(-94730),
    Fp(-91321),
    Fp(-88699),
    Fp(-86484),
    Fp(-84529),
    Fp(-82758),
    Fp(-81127),
    Fp(-79605),
    Fp(-78172),
    Fp(-76815),
    Fp(-75521),
    Fp(-74282),
    Fp(-73092),
    Fp(-71944),
    Fp(-70834),
    Fp(-69758),
    Fp(-68713),
    Fp(-67697),
    Fp(-66706),
    Fp(-65739),
    Fp(-64793),
    Fp(-63868),
    Fp(-62962),
    Fp(-62074),
    Fp(-61201),
    Fp(-60345),
    Fp(-59502),
    Fp(-58674),
    Fp(-57858),
    Fp(-57055),
    Fp(-56263),
    Fp(-55482),
    Fp(-54711),
    Fp(-53950),
    Fp(-53199),
    Fp(-52456),
    Fp(-51722),
    Fp(-50997),
    Fp(-50279),
    Fp(-49568),
    Fp(-48865),
    Fp(-48169),
    Fp(-47479),
    Fp(-46796),
    Fp(-46119),
    Fp(-45448),
    Fp(-44782),
    Fp(-44122),
    Fp(-43467),
    Fp(-42817),
    Fp(-42172),
    Fp(-41532),
    Fp(-40897),
    Fp(-40265),
    Fp(-39639),
    Fp(-39016),
    Fp(-38397),
    Fp(-37782),
    Fp(-37171),
    Fp(-36563),
    Fp(-35959),
    Fp(-35358),
    Fp(-34761),
    Fp(-34166),
    Fp(-33575),
    Fp(-32987),
    Fp(-32401),
    Fp(-31819),
    Fp(-31239),
    Fp(-30662),
    Fp(-30087),
    Fp(-29515),
    Fp(-28945),
    Fp(-28378),
    Fp(-27813),
    Fp(-27250),
    Fp(-26689),
    Fp(-26130),
    Fp(-25573),
    Fp(-25019),
    Fp(-24466),
    Fp(-23915),
    Fp(-23365),
    Fp(-22818),
    Fp(-22272),
    Fp(-21727),
    Fp(-21184),
    Fp(-20643),
    Fp(-20103),
    Fp(-19565),
    Fp(-19027),
    Fp(-18492),
    Fp(-17957),
    Fp(-17424),
    Fp(-16892),
    Fp(-16361),
    Fp(-15831),
    Fp(-15302),
    Fp(-14774),
    Fp(-14247),
    Fp(-13721),
    Fp(-13196),
    Fp(-12672),
    Fp(-12149),
    Fp(-11626),
    Fp(-11104),
    Fp(-10583),
    Fp(-10063),
    Fp(-9543),
    Fp(-9024),
    Fp(-8505),
    Fp(-7987),
    Fp(-7469),
    Fp(-6952),
    Fp(-6435),
    Fp(-5919),
    Fp(-5403),
    Fp(-4888),
    Fp(-4372),
    Fp(-3857),
    Fp(-3342),
    Fp(-2828),
    Fp(-2314),
    Fp(-1799),
    Fp(-1285),
    Fp(-771),
    Fp(-257),
    Fp(257),
    Fp(771),
    Fp(1285),
    Fp(1799),
    Fp(2314),
    Fp(2828),
    Fp(3342),
    Fp(3857),
    Fp(4372),
    Fp(4888),
    Fp(5403),
    Fp(5919),
    Fp(6435),
    Fp(6952),
    Fp(7469),
    Fp(7987),
    Fp(8505),
    Fp(9024),
    Fp(9543),
    Fp(10063),
    Fp(10583),
    Fp(11104),
    Fp(11626),
    Fp(12149),
    Fp(12672),
    Fp(13196),
    Fp(13721),
    Fp(14247),
    Fp(14774),
    Fp(15302),
    Fp(15831),
    Fp(16361),
    Fp(16892),
    Fp(17424),
    Fp(17957),
    Fp(18492),
    Fp(19027),
    Fp(19565),
    Fp(20103),
    Fp(20643),
    Fp(21184),
    Fp(21727),
    Fp(22272),
    Fp(22818),
    Fp(23365),
    Fp(23915),
    Fp(24466),
    Fp(25019),
    Fp(25573),
    Fp(26130),
    Fp(26689),
    Fp(27250),
    Fp(27813),
    Fp(28378),
    Fp(28945),
    Fp(29515),
    Fp(30087),
    Fp(30662),
    Fp(31239),
    Fp(31819),
    Fp(32401),
    Fp(32987),
    Fp(33575),
    Fp(34166),
    Fp(34761),
    Fp(35358),
    Fp(35959),
    Fp(36563),
    Fp(37171),
    Fp(37782),
    Fp(38397),
    Fp(39016),
    Fp(39639),
    Fp(40265),
    Fp(40897),
    Fp(41532),
    Fp(42172),
    Fp(42817),
    Fp(43467),
    Fp(44122),
    Fp(44782),
    Fp(45448),
    Fp(46119),
    Fp(46796),
    Fp(47479),
    Fp(48169),
    Fp(48865),
    Fp(49568),
    Fp(50279),
    Fp(50997),
    Fp(51722),
    Fp(52456),
    Fp(53199),
    Fp(53950),
    Fp(54711),
    Fp(55482),
    Fp(56263),
    Fp(57055),
    Fp(57858),
    Fp(58674),
    Fp(59502),
    Fp(60345),
    Fp(61201),
    Fp(62074),
    Fp(62962),
    Fp(63868),
    Fp(64793),
    Fp(65739),
    Fp(66706),
    Fp(67697),
    Fp(68713),
    Fp(69758),
    Fp(70834),
    Fp(71944),
    Fp(73092),
    Fp(74282),
    Fp(75521),
    Fp(76815),
    Fp(78172),
    Fp(79605),
    Fp(81127),
    Fp(82758),
    Fp(84529),
    Fp(86484),
    Fp(88699),
    Fp(91321),
    Fp(94730),
    Fp(102944),
];
pub const ACOS_TABLE: [Fp; 256] = [
    Fp(205887),
    Fp(197674),
    Fp(194264),
    Fp(191643),
    Fp(189428),
    Fp(187473),
    Fp(185702),
    Fp(184070),
    Fp(182548),
    Fp(181116),
    Fp(179759),
    Fp(178465),
    Fp(177226),
    Fp(176035),
    Fp(174888),
    Fp(173778),
    Fp(172702),
    Fp(171657),
    Fp(170640),
    Fp(169649),
    Fp(168682),
    Fp(167737),
    Fp(166812),
    Fp(165906),
    Fp(165017),
    Fp(164145),
    Fp(163288),
    Fp(162446),
    Fp(161618),
    Fp(160802),
    Fp(159998),
    Fp(159206),
    Fp(158425),
    Fp(157655),
    Fp(156894),
    Fp(156142),
    Fp(155400),
    Fp(154666),
    Fp(153940),
    Fp(153222),
    Fp(152512),
    Fp(151809),
    Fp(151113),
    Fp(150423),
    Fp(149740),
    Fp(149063),
    Fp(148391),
    Fp(147726),
    Fp(147066),
    Fp(146411),
    Fp(145761),
    Fp(145116),
    Fp(144476),
    Fp(143840),
    Fp(143209),
    Fp(142582),
    Fp(141959),
    Fp(141341),
    Fp(140726),
    Fp(140114),
    Fp(139507),
    Fp(138903),
    Fp(138302),
    Fp(137704),
    Fp(137110),
    Fp(136519),
    Fp(135931),
    Fp(135345),
    Fp(134763),
    Fp(134183),
    Fp(133606),
    Fp(133031),
    Fp(132459),
    Fp(131889),
    Fp(131322),
    Fp(130757),
    Fp(130194),
    Fp(129633),
    Fp(129074),
    Fp(128517),
    Fp(127962),
    Fp(127409),
    Fp(126858),
    Fp(126309),
    Fp(125761),
    Fp(125215),
    Fp(124671),
    Fp(124128),
    Fp(123587),
    Fp(123047),
    Fp(122508),
    Fp(121971),
    Fp(121435),
    Fp(120901),
    Fp(120368),
    Fp(119835),
    Fp(119304),
    Fp(118774),
    Fp(118246),
    Fp(117718),
    Fp(117191),
    Fp(116665),
    Fp(116140),
    Fp(115616),
    Fp(115092),
    Fp(114570),
    Fp(114048),
    Fp(113527),
    Fp(113006),
    Fp(112487),
    Fp(111967),
    Fp(111449),
    Fp(110931),
    Fp(110413),
    Fp(109896),
    Fp(109379),
    Fp(108863),
    Fp(108347),
    Fp(107831),
    Fp(107316),
    Fp(106801),
    Fp(106286),
    Fp(105772),
    Fp(105257),
    Fp(104743),
    Fp(104229),
    Fp(103715),
    Fp(103201),
    Fp(102687),
    Fp(102173),
    Fp(101659),
    Fp(101144),
    Fp(100630),
    Fp(100116),
    Fp(99601),
    Fp(99086),
    Fp(98571),
    Fp(98056),
    Fp(97541),
    Fp(97025),
    Fp(96508),
    Fp(95992),
    Fp(95474),
    Fp(94957),
    Fp(94439),
    Fp(93920),
    Fp(93401),
    Fp(92881),
    Fp(92361),
    Fp(91839),
    Fp(91318),
    Fp(90795),
    Fp(90272),
    Fp(89748),
    Fp(89222),
    Fp(88697),
    Fp(88170),
    Fp(87642),
    Fp(87113),
    Fp(86583),
    Fp(86052),
    Fp(85520),
    Fp(84987),
    Fp(84452),
    Fp(83916),
    Fp(83379),
    Fp(82841),
    Fp(82301),
    Fp(81759),
    Fp(81217),
    Fp(80672),
    Fp(80126),
    Fp(79579),
    Fp(79029),
    Fp(78478),
    Fp(77925),
    Fp(77370),
    Fp(76813),
    Fp(76255),
    Fp(75694),
    Fp(75131),
    Fp(74566),
    Fp(73998),
    Fp(73429),
    Fp(72856),
    Fp(72282),
    Fp(71705),
    Fp(71125),
    Fp(70542),
    Fp(69957),
    Fp(69369),
    Fp(68777),
    Fp(68183),
    Fp(67586),
    Fp(66985),
    Fp(66381),
    Fp(65773),
    Fp(65162),
    Fp(64547),
    Fp(63928),
    Fp(63305),
    Fp(62678),
    Fp(62047),
    Fp(61411),
    Fp(60771),
    Fp(60126),
    Fp(59477),
    Fp(58822),
    Fp(58162),
    Fp(57496),
    Fp(56825),
    Fp(56148),
    Fp(55464),
    Fp(54775),
    Fp(54078),
    Fp(53375),
    Fp(52665),
    Fp(51947),
    Fp(51221),
    Fp(50488),
    Fp(49745),
    Fp(48994),
    Fp(48233),
    Fp(47462),
    Fp(46681),
    Fp(45889),
    Fp(45086),
    Fp(44270),
    Fp(43441),
    Fp(42599),
    Fp(41742),
    Fp(40870),
    Fp(39982),
    Fp(39075),
    Fp(38150),
    Fp(37205),
    Fp(36238),
    Fp(35247),
    Fp(34230),
    Fp(33186),
    Fp(32110),
    Fp(31000),
    Fp(29852),
    Fp(28661),
    Fp(27423),
    Fp(26129),
    Fp(24771),
    Fp(23339),
    Fp(21817),
    Fp(20185),
    Fp(18414),
    Fp(16459),
    Fp(14245),
    Fp(11623),
    Fp(8213),
    Fp(0),
];
