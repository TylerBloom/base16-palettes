use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtelierPalette {
    Cave(Cave),
    CaveLight(CaveLight),
    Dune(Dune),
    DuneLight(DuneLight),
    Estuary(Estuary),
    EstuaryLight(EstuaryLight),
    Forest(Forest),
    ForestLight(ForestLight),
    Heath(Heath),
    HeathLight(HeathLight),
    Lakeside(Lakeside),
    LakesideLight(LakesideLight),
    Plateau(Plateau),
    PlateauLight(PlateauLight),
    Savanna(Savanna),
    SavannaLight(SavannaLight),
    Seaside(Seaside),
    SeasideLight(SeasideLight),
    SulphurPool(SulphurPool),
    SulphurPoolLight(SulphurPoolLight),
}

create_palette! {
    Cave,
    "19171c",
    "26232a",
    "585260",
    "655f6d",
    "7e7887",
    "8b8792",
    "e2dfe7",
    "efecf4",
    "be4678",
    "aa573c",
    "a06e3b",
    "2a9292",
    "398bc6",
    "576ddb",
    "955ae7",
    "bf40bf",
}

create_palette! {
    CaveLight,
    "efecf4",
    "e2dfe7",
    "8b8792",
    "7e7887",
    "655f6d",
    "585260",
    "26232a",
    "19171c",
    "be4678",
    "aa573c",
    "a06e3b",
    "2a9292",
    "398bc6",
    "576ddb",
    "955ae7",
    "bf40bf",
}

create_palette! {
    Dune,
    "20201d",
    "292824",
    "6e6b5e",
    "7d7a68",
    "999580",
    "a6a28c",
    "e8e4cf",
    "fefbec",
    "d73737",
    "b65611",
    "ae9513",
    "60ac39",
    "1fad83",
    "6684e1",
    "b854d4",
    "d43552",
}

create_palette! {
    DuneLight,
    "fefbec",
    "e8e4cf",
    "a6a28c",
    "999580",
    "7d7a68",
    "6e6b5e",
    "292824",
    "20201d",
    "d73737",
    "b65611",
    "ae9513",
    "60ac39",
    "1fad83",
    "6684e1",
    "b854d4",
    "d43552",
}

create_palette! {
    Estuary,
    "22221b",
    "302f27",
    "5f5e4e",
    "6c6b5a",
    "878573",
    "929181",
    "e7e6df",
    "f4f3ec",
    "ba6236",
    "ae7313",
    "a5980d",
    "7d9726",
    "5b9d48",
    "36a166",
    "5f9182",
    "9d6c7c",
}

create_palette! {
    EstuaryLight,
    "f4f3ec",
    "e7e6df",
    "929181",
    "878573",
    "6c6b5a",
    "5f5e4e",
    "302f27",
    "22221b",
    "ba6236",
    "ae7313",
    "a5980d",
    "7d9726",
    "5b9d48",
    "36a166",
    "5f9182",
    "9d6c7c",
}

create_palette! {
    Forest,
    "1b1918",
    "2c2421",
    "68615e",
    "766e6b",
    "9c9491",
    "a8a19f",
    "e6e2e0",
    "f1efee",
    "f22c40",
    "df5320",
    "c38418",
    "7b9726",
    "3d97b8",
    "407ee7",
    "6666ea",
    "c33ff3",
}

create_palette! {
    ForestLight,
    "f1efee",
    "e6e2e0",
    "a8a19f",
    "9c9491",
    "766e6b",
    "68615e",
    "2c2421",
    "1b1918",
    "f22c40",
    "df5320",
    "c38418",
    "7b9726",
    "3d97b8",
    "407ee7",
    "6666ea",
    "c33ff3",
}

create_palette! {
    Heath,
    "1b181b",
    "292329",
    "695d69",
    "776977",
    "9e8f9e",
    "ab9bab",
    "d8cad8",
    "f7f3f7",
    "ca402b",
    "a65926",
    "bb8a35",
    "918b3b",
    "159393",
    "516aec",
    "7b59c0",
    "cc33cc",
}

create_palette! {
    HeathLight,
    "f7f3f7",
    "d8cad8",
    "ab9bab",
    "9e8f9e",
    "776977",
    "695d69",
    "292329",
    "1b181b",
    "ca402b",
    "a65926",
    "bb8a35",
    "918b3b",
    "159393",
    "516aec",
    "7b59c0",
    "cc33cc",
}

create_palette! {
    Lakeside,
    "161b1d",
    "1f292e",
    "516d7b",
    "5a7b8c",
    "7195a8",
    "7ea2b4",
    "c1e4f6",
    "ebf8ff",
    "d22d72",
    "935c25",
    "8a8a0f",
    "568c3b",
    "2d8f6f",
    "257fad",
    "6b6bb8",
    "b72dd2",
}

create_palette! {
    LakesideLight,
    "ebf8ff",
    "c1e4f6",
    "7ea2b4",
    "7195a8",
    "5a7b8c",
    "516d7b",
    "1f292e",
    "161b1d",
    "d22d72",
    "935c25",
    "8a8a0f",
    "568c3b",
    "2d8f6f",
    "257fad",
    "6b6bb8",
    "b72dd2",
}

create_palette! {
    Plateau,
    "1b1818",
    "292424",
    "585050",
    "655d5d",
    "7e7777",
    "8a8585",
    "e7dfdf",
    "f4ecec",
    "ca4949",
    "b45a3c",
    "a06e3b",
    "4b8b8b",
    "5485b6",
    "7272ca",
    "8464c4",
    "bd5187",
}

create_palette! {
    PlateauLight,
    "f4ecec",
    "e7dfdf",
    "8a8585",
    "7e7777",
    "655d5d",
    "585050",
    "292424",
    "1b1818",
    "ca4949",
    "b45a3c",
    "a06e3b",
    "4b8b8b",
    "5485b6",
    "7272ca",
    "8464c4",
    "bd5187",
}

create_palette! {
    Savanna,
    "171c19",
    "232a25",
    "526057",
    "5f6d64",
    "78877d",
    "87928a",
    "dfe7e2",
    "ecf4ee",
    "b16139",
    "9f713c",
    "a07e3b",
    "489963",
    "1c9aa0",
    "478c90",
    "55859b",
    "867469",
}

create_palette! {
    SavannaLight,
    "ecf4ee",
    "dfe7e2",
    "87928a",
    "78877d",
    "5f6d64",
    "526057",
    "232a25",
    "171c19",
    "b16139",
    "9f713c",
    "a07e3b",
    "489963",
    "1c9aa0",
    "478c90",
    "55859b",
    "867469",
}

create_palette! {
    Seaside,
    "131513",
    "242924",
    "5e6e5e",
    "687d68",
    "809980",
    "8ca68c",
    "cfe8cf",
    "f4fbf4",
    "e6193c",
    "87711d",
    "98981b",
    "29a329",
    "1999b3",
    "3d62f5",
    "ad2bee",
    "e619c3",
}

create_palette! {
    SeasideLight,
    "f4fbf4",
    "cfe8cf",
    "8ca68c",
    "809980",
    "687d68",
    "5e6e5e",
    "242924",
    "131513",
    "e6193c",
    "87711d",
    "98981b",
    "29a329",
    "1999b3",
    "3d62f5",
    "ad2bee",
    "e619c3",
}

create_palette! {
    SulphurPool,
    "202746",
    "293256",
    "5e6687",
    "6b7394",
    "898ea4",
    "979db4",
    "dfe2f1",
    "f5f7ff",
    "c94922",
    "c76b29",
    "c08b30",
    "ac9739",
    "22a2c9",
    "3d8fd1",
    "6679cc",
    "9c637a",
}

create_palette! {
    SulphurPoolLight,
    "f5f7ff",
    "dfe2f1",
    "979db4",
    "898ea4",
    "6b7394",
    "5e6687",
    "293256",
    "202746",
    "c94922",
    "c76b29",
    "c08b30",
    "ac9739",
    "22a2c9",
    "3d8fd1",
    "6679cc",
    "9c637a",
}
