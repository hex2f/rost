use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Fel" => "Err",
        "Utmärkt" => "Ok",
        "Tråd" => "String",
        "Uppslagsverk" => "HashMap",
        "Vanlig" => "Default",
        "Bekymmer" => "Error",
        "Möjligen" => "Option",
        "Något" | "Nalta" => "Some",
        "Inget" | "Nada" => "None",
        "Resultat" | "SummaKardemumma" | => "Result",
        "Själv" => "Self",
        "samlingar" | "inkasso" => "collections",
        "utskrift" => "println",
        "stanna" => "break",
        "jämlöpande" => "async",
        "invänta" => "await",
        "krets" => "loop",
        "flytta" => "move",
        "kista" => "crate",
        "Låda" => "Box",
        "oåtkomlig_kod" => "unreachable_code",
        "som" => "as",
        "oföränderlig" => "const",
        "egenskap" => "trait",
        "typ" => "type",
        "osäker" => "unsafe",
        "innuti" => "in",
        "från" => "from",
        "anpassbar" => "dyn",
        "öppna" => "unwrap",
        "vanlig" => "default",
        "som_hänvisning" => "as_ref",
        "utvändig" => "extern",
        "falsk" => "false",
        "sann" => "true",
        "funktion" => "fn",
        "förtär" => "super",
        "inför" => "insert",

        "uprp" => "iter", // upprep
        "till_uprp" => "into_iter",
        "kartlägg" => "map",
        "vik" => "fold",
        "dränera" => "drain",
        "samla" => "collect",
        "hitta" => "find",
        "ta" => "take", 
        "produkt" => "product",

        "jmf" => "cmp",
        "Sortering" => "Ordering",
        "Större" => "Greater",
        "Mindre" => "Less",
        "Likvärdiga" => "Equal",
        "hämta" => "get",
        "tillåt" => "allow",
        "snablarns" => "panic",
        "modul" => "mod",
        "ändringsbar" => "mut",
        "ny" => "new",
        "där" => "where",
        "för" => "for",
        "hämta_eller_inför_med" => "get_or_insert_with",
        "ingångspunkt" => "main",
        "offentliggör" => "pub",
        "Inget?" => None?,
        "återvänd" => "return",
        "tillämpa" => "impl",
        "hänvisning" => "ref",
        "överensstämmer" => "match",
        "om" => "if",
        "annars" => "else",
        "själv" => "self",
        "låt" => "let",
        "statisk" => "static",
        "struktur" => "struct",
        "anta" => "expect",
        "medans" => "while",
        "använd" => "use",
        "till" => "into",
        "uppräkning" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rost(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
