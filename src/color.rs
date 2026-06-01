use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;
use skia_safe::Color;

static COLORS: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("black", 0x000000);
    m.insert("silver", 0xC0C0C0);
    m.insert("gray", 0x808080);
    m.insert("white", 0xFFFFFF);
    m.insert("maroon", 0x800000);
    m.insert("red", 0xFF0000);
    m.insert("purple", 0x800080);
    m.insert("fuchsia", 0xFF00FF);
    m.insert("green", 0x008000);
    m.insert("lime", 0x00FF00);
    m.insert("olive", 0x808000);
    m.insert("yellow", 0xFFFF00);
    m.insert("navy", 0x000080);
    m.insert("blue", 0x0000FF);
    m.insert("teal", 0x008080);
    m.insert("aqua", 0x00FFFF);
    m.insert("orange", 0xFFA500);
    m.insert("aliceblue", 0xF0F8FF);
    m.insert("antiquewhite", 0xFAEBD7);
    m.insert("aquamarine", 0x7FFFD4);
    m.insert("azure", 0xF0FFFF);
    m.insert("beige", 0xF5F5DC);
    m.insert("bisque", 0xFFE4C4);
    m.insert("blanchedalmond", 0xFFEBCD);
    m.insert("blueviolet", 0x8A2BE2);
    m.insert("brown", 0xA52A2A);
    m.insert("burlywood", 0xDEB887);
    m.insert("cadetblue", 0x5F9EA0);
    m.insert("chartreuse", 0x7FFF00);
    m.insert("chocolate", 0xD2691E);
    m.insert("coral", 0xFF7F50);
    m.insert("cornflowerblue", 0x6495ED);
    m.insert("cornsilk", 0xFFF8DC);
    m.insert("crimson", 0xDC143C);
    m.insert("cyan", 0x00FFFF);
    m.insert("darkblue", 0x00008B);
    m.insert("darkcyan", 0x008B8B);
    m.insert("darkgoldenrod", 0xB8860B);
    m.insert("darkgray", 0xA9A9A9);
    m.insert("darkgreen", 0x006400);
    m.insert("darkgrey", 0xA9A9A9);
    m.insert("darkkhaki", 0xBDB76B);
    m.insert("darkmagenta", 0x8B008B);
    m.insert("darkolivegreen", 0x556B2F);
    m.insert("darkorange", 0xFF8C00);
    m.insert("darkorchid", 0x9932CC);
    m.insert("darkred", 0x8B0000);
    m.insert("darksalmon", 0xE9967A);
    m.insert("darkseagreen", 0x8FBC8F);
    m.insert("darkslateblue", 0x483D8B);
    m.insert("darkslategray", 0x2F4F4F);
    m.insert("darkslategrey", 0x2F4F4F);
    m.insert("darkturquoise", 0x00CED1);
    m.insert("darkviolet", 0x9400D3);
    m.insert("deeppink", 0xFF1493);
    m.insert("deepskyblue", 0x00BFFF);
    m.insert("dimgray", 0x696969);
    m.insert("dimgrey", 0x696969);
    m.insert("dodgerblue", 0x1E90FF);
    m.insert("firebrick", 0xB22222);
    m.insert("floralwhite", 0xFFFAF0);
    m.insert("forestgreen", 0x228B22);
    m.insert("gainsboro", 0xDCDCDC);
    m.insert("ghostwhite", 0xF8F8FF);
    m.insert("gold", 0xFFD700);
    m.insert("goldenrod", 0xDAA520);
    m.insert("greenyellow", 0xADFF2F);
    m.insert("grey", 0x808080);
    m.insert("honeydew", 0xF0FFF0);
    m.insert("hotpink", 0xFF69B4);
    m.insert("indianred", 0xCD5C5C);
    m.insert("indigo", 0x4B0082);
    m.insert("ivory", 0xFFFFF0);
    m.insert("khaki", 0xF0E68C);
    m.insert("lavender", 0xE6E6FA);
    m.insert("lavenderblush", 0xFFF0F5);
    m.insert("lawngreen", 0x7CFC00);
    m.insert("lemonchiffon", 0xFFFACD);
    m.insert("lightblue", 0xADD8E6);
    m.insert("lightcoral", 0xF08080);
    m.insert("lightcyan", 0xE0FFFF);
    m.insert("lightgoldenrodyellow", 0xFAFAD2);
    m.insert("lightgray", 0xD3D3D3);
    m.insert("lightgreen", 0x90EE90);
    m.insert("lightgrey", 0xD3D3D3);
    m.insert("lightpink", 0xFFB6C1);
    m.insert("lightsalmon", 0xFFA07A);
    m.insert("lightseagreen", 0x20B2AA);
    m.insert("lightskyblue", 0x87CEFA);
    m.insert("lightslategray", 0x778899);
    m.insert("lightslategrey", 0x778899);
    m.insert("lightsteelblue", 0xB0C4DE);
    m.insert("lightyellow", 0xFFFFE0);
    m.insert("limegreen", 0x32CD32);
    m.insert("linen", 0xFAF0E6);
    m.insert("magenta", 0xFF00FF);
    m.insert("mediumaquamarine", 0x66CDAA);
    m.insert("mediumblue", 0x0000CD);
    m.insert("mediumorchid", 0xBA55D3);
    m.insert("mediumpurple", 0x9370DB);
    m.insert("mediumseagreen", 0x3CB371);
    m.insert("mediumslateblue", 0x7B68EE);
    m.insert("mediumspringgreen", 0x00FA9A);
    m.insert("mediumturquoise", 0x48D1CC);
    m.insert("mediumvioletred", 0xC71585);
    m.insert("midnightblue", 0x191970);
    m.insert("mintcream", 0xF5FFFA);
    m.insert("mistyrose", 0xFFE4E1);
    m.insert("moccasin", 0xFFE4B5);
    m.insert("navajowhite", 0xFFDEAD);
    m.insert("oldlace", 0xFDF5E6);
    m.insert("olivedrab", 0x6B8E23);
    m.insert("orangered", 0xFF4500);
    m.insert("orchid", 0xDA70D6);
    m.insert("palegoldenrod", 0xEEE8AA);
    m.insert("palegreen", 0x98FB98);
    m.insert("paleturquoise", 0xAFEEEE);
    m.insert("palevioletred", 0xDB7093);
    m.insert("papayawhip", 0xFFEFD5);
    m.insert("peachpuff", 0xFFDAB9);
    m.insert("peru", 0xCD853F);
    m.insert("pink", 0xFFC0CB);
    m.insert("plum", 0xDDA0DD);
    m.insert("powderblue", 0xB0E0E6);
    m.insert("rosybrown", 0xBC8F8F);
    m.insert("royalblue", 0x4169E1);
    m.insert("saddlebrown", 0x8B4513);
    m.insert("salmon", 0xFA8072);
    m.insert("sandybrown", 0xF4A460);
    m.insert("seagreen", 0x2E8B57);
    m.insert("seashell", 0xFFF5EE);
    m.insert("sienna", 0xA0522D);
    m.insert("skyblue", 0x87CEEB);
    m.insert("slateblue", 0x6A5ACD);
    m.insert("slategray", 0x708090);
    m.insert("slategrey", 0x708090);
    m.insert("snow", 0xFFFAFA);
    m.insert("springgreen", 0x00FF7F);
    m.insert("steelblue", 0x4682B4);
    m.insert("tan", 0xD2B48C);
    m.insert("thistle", 0xD8BFD8);
    m.insert("tomato", 0xFF6347);
    m.insert("turquoise", 0x40E0D0);
    m.insert("violet", 0xEE82EE);
    m.insert("wheat", 0xF5DEB3);
    m.insert("whitesmoke", 0xF5F5F5);
    m.insert("yellowgreen", 0x9ACD32);
    m.insert("rebeccapurple", 0x663399);
    m.insert("red50", 0xFFEBEE);
    m.insert("red100", 0xFFCDD2);
    m.insert("red200", 0xEF9A9A);
    m.insert("red300", 0xE57373);
    m.insert("red400", 0xEF5350);
    m.insert("red500", 0xF44336);
    m.insert("red600", 0xE53935);
    m.insert("red700", 0xD32F2F);
    m.insert("red800", 0xC62828);
    m.insert("red900", 0xB71C1C);
    m.insert("redA100", 0xFF8A80);
    m.insert("redA200", 0xFF5252);
    m.insert("redA400", 0xFF1744);
    m.insert("redA700", 0xD50000);
    m.insert("pink50", 0xFCE4EC);
    m.insert("pink100", 0xF8BBD0);
    m.insert("pink200", 0xF48FB1);
    m.insert("pink300", 0xF06292);
    m.insert("pink400", 0xEC407A);
    m.insert("pink500", 0xE91E63);
    m.insert("pink600", 0xD81B60);
    m.insert("pink700", 0xC2185B);
    m.insert("pink800", 0xAD1457);
    m.insert("pink900", 0x880E4F);
    m.insert("pinkA100", 0xFF80AB);
    m.insert("pinkA200", 0xFF4081);
    m.insert("pinkA400", 0xF50057);
    m.insert("pinkA700", 0xC51162);
    m.insert("purple50", 0xF3E5F5);
    m.insert("purple100", 0xE1BEE7);
    m.insert("purple200", 0xCE93D8);
    m.insert("purple300", 0xBA68C8);
    m.insert("purple400", 0xAB47BC);
    m.insert("purple500", 0x9C27B0);
    m.insert("purple600", 0x8E24AA);
    m.insert("purple700", 0x7B1FA2);
    m.insert("purple800", 0x6A1B9A);
    m.insert("purple900", 0x4A148C);
    m.insert("purpleA100", 0xEA80FC);
    m.insert("purpleA200", 0xE040FB);
    m.insert("purpleA400", 0xD500F9);
    m.insert("purpleA700", 0xAA00FF);
    m.insert("deeppurple50", 0xEDE7F6);
    m.insert("deeppurple100", 0xD1C4E9);
    m.insert("deeppurple200", 0xB39DDB);
    m.insert("deeppurple300", 0x9575CD);
    m.insert("deeppurple400", 0x7E57C2);
    m.insert("deeppurple500", 0x673AB7);
    m.insert("deeppurple600", 0x5E35B1);
    m.insert("deeppurple700", 0x512DA8);
    m.insert("deeppurple800", 0x4527A0);
    m.insert("deeppurple900", 0x311B92);
    m.insert("deeppurpleA100", 0xB388FF);
    m.insert("deeppurpleA200", 0x7C4DFF);
    m.insert("deeppurpleA400", 0x651FFF);
    m.insert("deeppurpleA700", 0x6200EA);
    m.insert("indigo50", 0xE8EAF6);
    m.insert("indigo100", 0xC5CAE9);
    m.insert("indigo200", 0x9FA8DA);
    m.insert("indigo300", 0x7986CB);
    m.insert("indigo400", 0x5C6BC0);
    m.insert("indigo500", 0x3F51B5);
    m.insert("indigo600", 0x3949AB);
    m.insert("indigo700", 0x303F9F);
    m.insert("indigo800", 0x283593);
    m.insert("indigo900", 0x1A237E);
    m.insert("indigoA100", 0x8C9EFF);
    m.insert("indigoA200", 0x536DFE);
    m.insert("indigoA400", 0x3D5AFE);
    m.insert("indigoA700", 0x304FFE);
    m.insert("blue50", 0xE3F2FD);
    m.insert("blue100", 0xBBDEFB);
    m.insert("blue200", 0x90CAF9);
    m.insert("blue300", 0x64B5F6);
    m.insert("blue400", 0x42A5F5);
    m.insert("blue500", 0x2196F3);
    m.insert("blue600", 0x1E88E5);
    m.insert("blue700", 0x1976D2);
    m.insert("blue800", 0x1565C0);
    m.insert("blue900", 0x0D47A1);
    m.insert("blueA100", 0x82B1FF);
    m.insert("blueA200", 0x448AFF);
    m.insert("blueA400", 0x2979FF);
    m.insert("blueA700", 0x2962FF);
    m.insert("lightblue50", 0xE1F5FE);
    m.insert("lightblue100", 0xB3E5FC);
    m.insert("lightblue200", 0x81D4FA);
    m.insert("lightblue300", 0x4FC3F7);
    m.insert("lightblue400", 0x29B6F6);
    m.insert("lightblue500", 0x03A9F4);
    m.insert("lightblue600", 0x039BE5);
    m.insert("lightblue700", 0x0288D1);
    m.insert("lightblue800", 0x0277BD);
    m.insert("lightblue900", 0x01579B);
    m.insert("lightblueA100", 0x80D8FF);
    m.insert("lightblueA200", 0x40C4FF);
    m.insert("lightblueA400", 0x00B0FF);
    m.insert("lightblueA700", 0x0091EA);
    m.insert("cyan50", 0xE0F7FA);
    m.insert("cyan100", 0xB2EBF2);
    m.insert("cyan200", 0x80DEEA);
    m.insert("cyan300", 0x4DD0E1);
    m.insert("cyan400", 0x26C6DA);
    m.insert("cyan500", 0x00BCD4);
    m.insert("cyan600", 0x00ACC1);
    m.insert("cyan700", 0x0097A7);
    m.insert("cyan800", 0x00838F);
    m.insert("cyan900", 0x006064);
    m.insert("cyanA100", 0x84FFFF);
    m.insert("cyanA200", 0x18FFFF);
    m.insert("cyanA400", 0x00E5FF);
    m.insert("cyanA700", 0x00B8D4);
    m.insert("teal50", 0xE0F2F1);
    m.insert("teal100", 0xB2DFDB);
    m.insert("teal200", 0x80CBC4);
    m.insert("teal300", 0x4DB6AC);
    m.insert("teal400", 0x26A69A);
    m.insert("teal500", 0x009688);
    m.insert("teal600", 0x00897B);
    m.insert("teal700", 0x00796B);
    m.insert("teal800", 0x00695C);
    m.insert("teal900", 0x004D40);
    m.insert("tealA100", 0xA7FFEB);
    m.insert("tealA200", 0x64FFDA);
    m.insert("tealA400", 0x1DE9B6);
    m.insert("tealA700", 0x00BFA5);
    m.insert("green50", 0xE8F5E9);
    m.insert("green100", 0xC8E6C9);
    m.insert("green200", 0xA5D6A7);
    m.insert("green300", 0x81C784);
    m.insert("green400", 0x66BB6A);
    m.insert("green500", 0x4CAF50);
    m.insert("green600", 0x43A047);
    m.insert("green700", 0x388E3C);
    m.insert("green800", 0x2E7D32);
    m.insert("green900", 0x1B5E20);
    m.insert("greenA100", 0xB9F6CA);
    m.insert("greenA200", 0x69F0AE);
    m.insert("greenA400", 0x00E676);
    m.insert("greenA700", 0x00C853);
    m.insert("lightgreen50", 0xF1F8E9);
    m.insert("lightgreen100", 0xDCEDC8);
    m.insert("lightgreen200", 0xC5E1A5);
    m.insert("lightgreen300", 0xAED581);
    m.insert("lightgreen400", 0x9CCC65);
    m.insert("lightgreen500", 0x8BC34A);
    m.insert("lightgreen600", 0x7CB342);
    m.insert("lightgreen700", 0x689F38);
    m.insert("lightgreen800", 0x558B2F);
    m.insert("lightgreen900", 0x33691E);
    m.insert("lightgreenA100", 0xCCFF90);
    m.insert("lightgreenA200", 0xB2FF59);
    m.insert("lightgreenA400", 0x76FF03);
    m.insert("lightgreenA700", 0x64DD17);
    m.insert("lime50", 0xF9FBE7);
    m.insert("lime100", 0xF0F4C3);
    m.insert("lime200", 0xE6EE9C);
    m.insert("lime300", 0xDCE775);
    m.insert("lime400", 0xD4E157);
    m.insert("lime500", 0xCDDC39);
    m.insert("lime600", 0xC0CA33);
    m.insert("lime700", 0xAFB42B);
    m.insert("lime800", 0x9E9D24);
    m.insert("lime900", 0x827717);
    m.insert("limeA100", 0xF4FF81);
    m.insert("limeA200", 0xEEFF41);
    m.insert("limeA400", 0xC6FF00);
    m.insert("limeA700", 0xAEEA00);
    m.insert("yellow50", 0xFFFDE7);
    m.insert("yellow100", 0xFFF9C4);
    m.insert("yellow200", 0xFFF59D);
    m.insert("yellow300", 0xFFF176);
    m.insert("yellow400", 0xFFEE58);
    m.insert("yellow500", 0xFFEB3B);
    m.insert("yellow600", 0xFDD835);
    m.insert("yellow700", 0xFBC02D);
    m.insert("yellow800", 0xF9A825);
    m.insert("yellow900", 0xF57F17);
    m.insert("yellowA100", 0xFFFF8D);
    m.insert("yellowA200", 0xFFFF00);
    m.insert("yellowA400", 0xFFEA00);
    m.insert("yellowA700", 0xFFD600);
    m.insert("amber50", 0xFFF8E1);
    m.insert("amber100", 0xFFECB3);
    m.insert("amber200", 0xFFE082);
    m.insert("amber300", 0xFFD54F);
    m.insert("amber400", 0xFFCA28);
    m.insert("amber500", 0xFFC107);
    m.insert("amber600", 0xFFB300);
    m.insert("amber700", 0xFFA000);
    m.insert("amber800", 0xFF8F00);
    m.insert("amber900", 0xFF6F00);
    m.insert("amberA100", 0xFFE57F);
    m.insert("amberA200", 0xFFD740);
    m.insert("amberA400", 0xFFC400);
    m.insert("amberA700", 0xFFAB00);
    m.insert("orange50", 0xFFF3E0);
    m.insert("orange100", 0xFFE0B2);
    m.insert("orange200", 0xFFCC80);
    m.insert("orange300", 0xFFB74D);
    m.insert("orange400", 0xFFA726);
    m.insert("orange500", 0xFF9800);
    m.insert("orange600", 0xFB8C00);
    m.insert("orange700", 0xF57C00);
    m.insert("orange800", 0xEF6C00);
    m.insert("orange900", 0xE65100);
    m.insert("orangeA100", 0xFFD180);
    m.insert("orangeA200", 0xFFAB40);
    m.insert("orangeA400", 0xFF9100);
    m.insert("orangeA700", 0xFF6D00);
    m.insert("deeporange50", 0xFBE9E7);
    m.insert("deeporange100", 0xFFCCBC);
    m.insert("deeporange200", 0xFFAB91);
    m.insert("deeporange300", 0xFF8A65);
    m.insert("deeporange400", 0xFF7043);
    m.insert("deeporange500", 0xFF5722);
    m.insert("deeporange600", 0xF4511E);
    m.insert("deeporange700", 0xE64A19);
    m.insert("deeporange800", 0xD84315);
    m.insert("deeporange900", 0xBF360C);
    m.insert("deeporangeA100", 0xFF9E80);
    m.insert("deeporangeA200", 0xFF6E40);
    m.insert("deeporangeA400", 0xFF3D00);
    m.insert("deeporangeA700", 0xDD2C00);
    m.insert("brown50", 0xEFEBE9);
    m.insert("brown100", 0xD7CCC8);
    m.insert("brown200", 0xBCAAA4);
    m.insert("brown300", 0xA1887F);
    m.insert("brown400", 0x8D6E63);
    m.insert("brown500", 0x795548);
    m.insert("brown600", 0x6D4C41);
    m.insert("brown700", 0x5D4037);
    m.insert("brown800", 0x4E342E);
    m.insert("brown900", 0x3E2723);
    m.insert("gray50", 0xFAFAFA);
    m.insert("gray100", 0xF5F5F5);
    m.insert("gray200", 0xEEEEEE);
    m.insert("gray300", 0xE0E0E0);
    m.insert("gray400", 0xBDBDBD);
    m.insert("gray500", 0x9E9E9E);
    m.insert("gray600", 0x757575);
    m.insert("gray700", 0x616161);
    m.insert("gray800", 0x424242);
    m.insert("gray900", 0x212121);
    m.insert("grey50", 0xFAFAFA);
    m.insert("grey100", 0xF5F5F5);
    m.insert("grey200", 0xEEEEEE);
    m.insert("grey300", 0xE0E0E0);
    m.insert("grey400", 0xBDBDBD);
    m.insert("grey500", 0x9E9E9E);
    m.insert("grey600", 0x757575);
    m.insert("grey700", 0x616161);
    m.insert("grey800", 0x424242);
    m.insert("grey900", 0x212121);
    m.insert("bluegray50", 0xECEFF1);
    m.insert("bluegray100", 0xCFD8DC);
    m.insert("bluegray200", 0xB0BEC5);
    m.insert("bluegray300", 0x90A4AE);
    m.insert("bluegray400", 0x78909C);
    m.insert("bluegray500", 0x607D8B);
    m.insert("bluegray600", 0x546E7A);
    m.insert("bluegray700", 0x455A64);
    m.insert("bluegray800", 0x37474F);
    m.insert("bluegray900", 0x263238);
    m.insert("bluegrey50", 0xECEFF1);
    m.insert("bluegrey100", 0xCFD8DC);
    m.insert("bluegrey200", 0xB0BEC5);
    m.insert("bluegrey300", 0x90A4AE);
    m.insert("bluegrey400", 0x78909C);
    m.insert("bluegrey500", 0x607D8B);
    m.insert("bluegrey600", 0x546E7A);
    m.insert("bluegrey700", 0x455A64);
    m.insert("bluegrey800", 0x37474F);
    m.insert("bluegrey900", 0x263238);
    m
});
static LONG_HEX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#?([0-9a-fA-F]{6})$").unwrap());
static SHORT_HEX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#?([0-9a-fA-F]{3})$").unwrap());
static RGB_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^rgb\s*\(\s*(2(?:5[0-5]|[0-4]\d)|1\d\d|[1-9]?\d)\s*(?:,|\s)\s*(2(?:5[0-5]|[0-4]\d)|1\d\d|[1-9]?\d)\s*(?:,|\s)\s*(2(?:5[0-5]|[0-4]\d)|1\d\d|[1-9]?\d)\s*\)$"
    ).unwrap()
});
static HSL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^hsl\s*\(\s*(\d+(?:\.\d+)?)(?:deg)?\s*(?:,|\s)\s*(100(?:\.0+)?|[1-9]\d(?:\.\d+)?)%\s*(?:,|\s)\s*(100(?:\.0+)?|[1-9]\d(?:\.\d+)?)%\s*\)$"
    ).unwrap()
});

pub fn from_hsl(h: f32, s: f32, l: f32) -> Option<Color> {
    if !(0.0..=360.0).contains(&h) || !(0.0..=100.0).contains(&s) || !(0.0..=100.0).contains(&l) {
        return None;
    }

    let h = if h == 360.0 { 0.0 } else { h };
    let s = s / 100.0;
    let l = l / 100.0;

    if s == 0.0 {
        let gray = (l * 255.0).round() as u8;
        return Some(Color::from_rgb(gray, gray, gray));
    }

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (r1, g1, b1) = match h {
        h if (0.0..60.0).contains(&h) => (c, x, 0.0),
        h if (60.0..120.0).contains(&h) => (x, c, 0.0),
        h if (120.0..180.0).contains(&h) => (0.0, c, x),
        h if (180.0..240.0).contains(&h) => (0.0, x, c),
        h if (240.0..300.0).contains(&h) => (x, 0.0, c),
        _ => (c, 0.0, x), // 300-360
    };

    let to_byte = |v: f32| ((v + m) * 255.0).round().clamp(0.0, 255.0) as u8;

    Some(Color::from_rgb(to_byte(r1), to_byte(g1), to_byte(b1)))
}

pub fn parse(s: &str) -> Option<Color> {
    if let Some(color) = COLORS.get(s) {
        return Some(Color::new(0xff000000 | color));
    }
    if let Some(m) = LONG_HEX_RE.captures(s)
        && let Some(capture) = m.get(1)
        && let Ok(color) = u32::from_str_radix(capture.as_str(), 16)
    {
        return Some(Color::new(0xff000000 | color));
    }
    if let Some(m) = SHORT_HEX_RE.captures(s)
        && let Some(capture) = m.get(1)
        && let Ok(color) = u32::from_str_radix(capture.as_str(), 16)
    {
        let r = (color >> 8) as u8;
        let g = ((color >> 4) & 0xF) as u8;
        let b = (color & 0xF) as u8;
        return Some(Color::from_rgb(r << 4 | r, g << 4 | g, b << 4 | b));
    }
    if let Some(m) = RGB_RE.captures(s)
        && let Some(r) = m.get(1)
        && let Some(g) = m.get(2)
        && let Some(b) = m.get(3)
        && let Ok(r) = r.as_str().parse::<u8>()
        && let Ok(g) = g.as_str().parse::<u8>()
        && let Ok(b) = b.as_str().parse::<u8>()
    {
        return Some(Color::from_rgb(r, g, b));
    }
    if let Some(m) = HSL_RE.captures(s)
        && let Some(h) = m.get(1)
        && let Some(s) = m.get(2)
        && let Some(l) = m.get(3)
        && let Ok(h) = h.as_str().parse::<f32>()
        && let Ok(s) = s.as_str().parse::<f32>()
        && let Ok(l) = l.as_str().parse::<f32>()
        && let Some(color) = from_hsl(h, s, l)
    {
        return Some(color);
    }
    None
}
