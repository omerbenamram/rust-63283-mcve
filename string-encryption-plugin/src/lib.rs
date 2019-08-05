#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]

// Built-in crates must be imported using extern crate.
extern crate syntax;
extern crate syntax_pos;

extern crate rustc_plugin;

use rand::Rng;
use rustc_plugin::Registry;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;
use syntax::ast::{Expr, Ident, LitKind};
use syntax::ext::base::get_single_str_from_tts;
use syntax::ext::base::{DummyResult, ExtCtxt, MacEager, MacResult};
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;
use syntax::tokenstream::TokenTree;
use syntax_pos::FileName;
use syntax_pos::Span;

fn encrypt_str(cx: &mut ExtCtxt<'_>, sp: Span, tts: &[TokenTree]) -> Box<dyn MacResult + 'static> {
    let text = match get_single_str_from_tts(cx, sp, tts, "e!") {
        Some(f) => f,
        None => return DummyResult::expr(sp),
    };

    create_enc_expr(cx, sp, &text)
}

fn create_enc_expr(cx: &mut ExtCtxt<'_>, sp: Span, text: &str) -> Box<dyn MacResult + 'static> {
    let (cipher_text, key) = encrypt_str_with_rand_key(text);
    let cipher_text_expr = get_expr_from_bytes(cx, sp, cipher_text);
    let key_expr = get_expr_from_bytes(cx, sp, key);

    // Generate a call to the decryption function in string-decryption
    MacEager::expr(cx.expr_call(
        sp,
        cx.expr_path(cx.path_global(
            sp,
            vec![Ident::from_str("string_decryption"), Ident::from_str("d")],
        )),
        vec![cx.expr_tuple(sp, vec![cipher_text_expr, key_expr])],
    ))
}

fn encrypt_str_with_rand_key(text: &str) -> (Vec<u8>, Vec<u8>) {
    let text = String::from(text);
    let mut text = text.into_bytes();

    let mut random = rand::thread_rng();
    let mut key = vec![0xff; text.len()];
    random.fill_bytes(&mut key);

    // Encrypt the text using XOR
    for i in 0..text.len() {
        text[i] ^= key[i];
    }

    (text, key)
}

fn get_expr_from_bytes(cx: &mut ExtCtxt<'_>, sp: Span, bytes: Vec<u8>) -> P<Expr> {
    cx.expr_lit(sp, LitKind::ByteStr(Rc::new(bytes)))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry<'_>) {
    reg.register_macro("e", encrypt_str);
}
