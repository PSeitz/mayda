// Copyright 2016 Jeremy Mason
//
// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied, modified,
// or distributed except according to those terms.

//! Compiler plugins to generate basic encoding and decoding functions. The
//! functions generated by the `encode!` and `decode!` syntax extensions follow
//! the convention `encode_T_a_b`. The functions generated by the
//! `encode_simd!` and `decode_simd!` syntax extensions follow the convention
//! `encode_simd_T_a`.  The functions generated by the `encode_zz!` and
//! `decode_zz!` syntax extentions follow the convention `encode_zz_T` and
//! `encode_zz_T_shift`. `T` is one of the unsigned integer types, `a` is the
//! number of bits per integer, and `b` is the number of integers encoded.
//!
//! `encode!` and `decode!` take as arguments the unsigned integer type width
//! in bits and a step for the number of integers (a divisor of 32).  Functions
//! are generated for numbers of bits in the interval `1...max_bits`, and for
//! numbers of integers in multiples of the step up to 32. `encode_simd!` and
//! `decode_simd!` take as arguments the unsigned integer type width in bits
//! and a path to the relevant simd module. Functions are generated for numbers
//! of bits in the interval `1...max_bits`, and for exactly 128 integers.
//! `encode_zz!` and `decode_zz!` take as arguments the unsigned integer type
//! width in bits and a path to the relevant simd module. Functions are
//! generated for the type width and for an arbitrary number of integers.
//!
//! Pointers to the functions generated by `encode!` and `decode!` are
//! available in `ENCODE_T` and `DECODE_T`, respectively, with the pointer for
//! `encode_T_a_b` at `ENCODE_T[a - 1][b / c - 1]` where `c` is the step.
//! Pointers to the functions generated by `encode_simd!` and `decode_simd!`
//! are availabe in `ENCODE_SIMD_T` and `DECODE_SIMD_T`, respectively, with the
//! pointer for `encode_simd_T_a` at `ENCODE_SIMD_T[a - 1]`. The functions
//! generated by `encode_zz!` and `decode_zz!` are public. All arrays are
//! public and constant.
//!
//! # Safety
//!
//! The functions generated by this crate use wildly unsafe pointer operations.
//! You must verify that enough memory is already allocated after the pointers
//! that the offsets are valid. They are not intended to be used outside the
//! `pfor` crate.
//! 
//! # Examples
//!
//! The syntax extensions defined in this crate can be invoked as
//!
//! ```
//! encode!(u32, 32, 8);
//! decode!(u32, 32, 8);
//! ```
//!
//! This is replaced by 128 functions that encode u32 integers and 128
//! functions that decode u32 integers. For example, the functions that encode
//! and decode the 24 least significant bits of 8 u32 integers are
//!
//! ```
//! unsafe fn encode_u32_24_8(i_ptr: *const u32, s_ptr: *mut u32) {
//!     let mut i_ptr = i_ptr;
//!     let mut s_ptr = s_ptr;
//!     let mut out = *i_ptr as u32;
//!     i_ptr = i_ptr.offset(1);
//!     out |= (*i_ptr as u32) << 24usize;
//!     *s_ptr = out;
//!     s_ptr = s_ptr.offset(1);
//!     out = (*i_ptr >> 8usize) as u32;
//!     i_ptr = i_ptr.offset(1);
//!     out |= (*i_ptr as u32) << 16usize;
//!     *s_ptr = out;
//!     s_ptr = s_ptr.offset(1);
//!     out = (*i_ptr >> 16usize) as u32;
//!     i_ptr = i_ptr.offset(1);
//!     out |= (*i_ptr as u32) << 8usize;
//!     i_ptr = i_ptr.offset(1);
//!     *s_ptr = out;
//!     s_ptr = s_ptr.offset(1);
//!     out = *i_ptr as u32;
//!     i_ptr = i_ptr.offset(1);
//!     out |= (*i_ptr as u32) << 24usize;
//!     *s_ptr = out;
//!     s_ptr = s_ptr.offset(1);
//!     out = (*i_ptr >> 8usize) as u32;
//!     i_ptr = i_ptr.offset(1);
//!     out |= (*i_ptr as u32) << 16usize;
//!     *s_ptr = out;
//!     s_ptr = s_ptr.offset(1);
//!     out = (*i_ptr >> 16usize) as u32;
//!     i_ptr = i_ptr.offset(1);
//!     out |= (*i_ptr as u32) << 8usize;
//!     *s_ptr = out;
//! }
//!
//! unsafe fn decode_u32_24_8(s_ptr: *const u32, o_ptr: *mut u32) {
//!     let mut s_ptr = s_ptr;
//!     let mut o_ptr = o_ptr;
//!     let mask: u32 = !0 >> 8usize;
//!     let mut out;
//!     out = *s_ptr as u32;
//!     *o_ptr = out & mask;
//!     o_ptr = o_ptr.offset(1);
//!     out = (*s_ptr >> 24usize) as u32;
//!     s_ptr = s_ptr.offset(1);
//!     out |= (*s_ptr as u32) << 8usize;
//!     *o_ptr = out & mask;
//!     o_ptr = o_ptr.offset(1);
//!     out = (*s_ptr >> 16usize) as u32;
//!     s_ptr = s_ptr.offset(1);
//!     out |= (*s_ptr as u32) << 16usize;
//!     *o_ptr = out & mask;
//!     o_ptr = o_ptr.offset(1);
//!     out = (*s_ptr >> 8usize) as u32;
//!     *o_ptr = out & mask;
//!     o_ptr = o_ptr.offset(1);
//!     s_ptr = s_ptr.offset(1);
//!     out = *s_ptr as u32;
//!     *o_ptr = out & mask;
//!     o_ptr = o_ptr.offset(1);
//!     out = (*s_ptr >> 24usize) as u32;
//!     s_ptr = s_ptr.offset(1);
//!     out |= (*s_ptr as u32) << 8usize;
//!     *o_ptr = out & mask;
//!     o_ptr = o_ptr.offset(1);
//!     out = (*s_ptr >> 16usize) as u32;
//!     s_ptr = s_ptr.offset(1);
//!     out |= (*s_ptr as u32) << 16usize;
//!     *o_ptr = out & mask;
//!     o_ptr = o_ptr.offset(1);
//!     out = (*s_ptr >> 8usize) as u32;
//!     *o_ptr = out & mask;
//! }
//! ```

#![feature(plugin_registrar)]
#![feature(rustc_private)]
#![feature(quote)]
#![feature(inclusive_range_syntax)]

// Unused import in quote_tokens! macro
#![allow(unused_imports)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{DummyResult, ExtCtxt, MacResult, MacEager};
use syntax::fold::Folder;
use syntax::parse::{self, token};
use syntax::print::pprust;
use syntax::util::small_vector;

/// Registers the encode and decode syntax expansions.
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
  reg.register_macro("encode", encode_expand);
  reg.register_macro("decode", decode_expand);
  reg.register_macro("encode_simd", encode_simd_expand);
  reg.register_macro("decode_simd", decode_simd_expand);
  reg.register_macro("encode_zz", encode_zz_expand);
  reg.register_macro("decode_zz", decode_zz_expand);
}

/// Generates ENCODE_T containing function pointers, with the pointer for
/// encode_T_a_b at ENCODE_T[a - 1][b / c - 1].
fn encode_expand(cx: &mut ExtCtxt,
                 sp: codemap::Span,
                 tts: &[ast::TokenTree]) -> Box<MacResult + 'static> {
  // Arguments to the macro invocation
  let (width, step) = {
    match parse(cx, sp, tts) {
      Some(x) => x,
      None => return DummyResult::expr(sp)
    }
  };
  let ut = token::str_to_ident(&*format!("u{}", width));
  assert_eq!(32 % step, 0);
  let lengths: Vec<usize> = (1...(32 / step)).map(|a| a * step).collect();

  // idents: tokens used to define the ENCODE_T
  // items: definitions of the functions
  let mut idents = vec![token::OpenDelim(token::Bracket)];
  let mut items = Vec::new();
  for wd in 1...width {
    idents.push(token::OpenDelim(token::Bracket));
    for ln in lengths.iter() {
      // Name for the function interned
      let name = format!("encode_{}_{}_{}", ut, wd, ln);
      let ident = token::str_to_ident(&*name);
      idents.push(token::Ident(ident));
      idents.push(token::Comma);

      // Function definition constructed here
      let mut i_bits: usize;
      let mut s_bits: usize = 32;
      let mut tokens = quote_tokens!(cx,
        let mut i_ptr = i_ptr;
        let mut s_ptr = s_ptr;
        let mut out =
      );

      // For every integer to be encoded...
      for a in 0..*ln {
        i_bits = wd;

        // Encode in the available space
        let lsft = 32 - s_bits;
        if lsft == 0 {
          tokens = quote_tokens!(cx, $tokens
            *i_ptr as u32;
          );
        } else {
          tokens = quote_tokens!(cx, $tokens
            (*i_ptr as u32) << $lsft;
          );
        }

        // While the available space is not enough...
        while s_bits < i_bits {
          i_bits -= s_bits;
          let rsft = wd - i_bits;
          tokens = quote_tokens!(cx, $tokens
            *s_ptr = out;
            s_ptr = s_ptr.offset(1);
            out = (*i_ptr >> $rsft) as u32;
          );
          s_bits = 32;
        }
        s_bits -= i_bits;


        // Prepare for following iteration
        if a < ln - 1 {
          tokens = quote_tokens!(cx, $tokens
            i_ptr = i_ptr.offset(1);
          );
          if s_bits == 0 {
            tokens = quote_tokens!(cx, $tokens
              *s_ptr = out;
              s_ptr = s_ptr.offset(1);
              out =
            );
            s_bits = 32;
          } else {
            tokens = quote_tokens!(cx, $tokens
              out |=
            );
          }
        } else {
          tokens = quote_tokens!(cx, $tokens
            *s_ptr = out;
          );
        }
      }

      // Function definition pushed to items
      items.push(
        quote_item!(cx,
          unsafe fn $ident(i_ptr: *const $ut, s_ptr: *mut u32) {
            $tokens
          }
        ).unwrap()
      );
    }
    idents.push(token::CloseDelim(token::Bracket));
    idents.push(token::Comma);
  }
  idents.push(token::CloseDelim(token::Bracket));

  // idents converted from tokens to TokenTree
  let ttree: Vec<ast::TokenTree> = idents
    .into_iter()
    .map(|token| ast::TokenTree::Token(codemap::DUMMY_SP, token))
    .collect();

  // ENCODE_T definition pushed to items
  let name = format!("encode_{}", ut).to_uppercase();
  let ident = token::str_to_ident(&*name);
  let tmp = lengths.len();
  items.push(
    quote_item!(cx,
      pub const $ident: [[unsafe fn(*const $ut, *mut u32); $tmp]; $width] = $ttree;
    ).unwrap()
  );
  
  // DEBUGGING
  // for item in &items { println!("{}", pprust::item_to_string(item)); }

  MacEager::items(small_vector::SmallVector::many(items))
}

/// Generates DECODE_T containing function pointers, with the pointer for
/// decode_T_a_b at DECODE_T[a - 1][b / c - 1].
fn decode_expand(cx: &mut ExtCtxt,
                 sp: codemap::Span,
                 tts: &[ast::TokenTree]) -> Box<MacResult + 'static> {
  // Arguments to the macro invocation
  let (width, step) = {
    match parse(cx, sp, tts) {
      Some(x) => x,
      None => return DummyResult::expr(sp)
    }
  };
  let ut = token::str_to_ident(&*format!("u{}", width));
  assert_eq!(32 % step, 0);
  let lengths: Vec<usize> = (1...(32 / step)).map(|a| a * step).collect();

  // idents: tokens used to define the const DECODE_T
  // items: definitions of the functions
  let mut idents = vec![token::OpenDelim(token::Bracket)];
  let mut items = Vec::new();
  for wd in 1...width {
    idents.push(token::OpenDelim(token::Bracket));
    for ln in lengths.iter() {
      // Name for the function interned
      let name = format!("decode_{}_{}_{}", ut, wd, ln);
      let ident = token::str_to_ident(&*name);
      idents.push(token::Ident(ident));
      idents.push(token::Comma);

      // Function definition constructed here
      let mut s_bits: usize = 32;
      let mut o_bits: usize;
      let mut tokens = {
        // Handles unused mut warning
        if wd * *ln > 32 {
          quote_tokens!(cx,
            let mut s_ptr = s_ptr;
            let mut o_ptr = o_ptr;
          )
        } else {
          quote_tokens!(cx,
            let mut o_ptr = o_ptr;
          )
        }
      };
      // Handles unused variable warnings
      let mask_sft = width - wd;
      if mask_sft > 0 && wd != 32 {
        tokens = quote_tokens!(cx, $tokens
          let mask: $ut = !0 >> $mask_sft;
        );
      }
      tokens = quote_tokens!(cx, $tokens
        let mut out;
      );

      // For every integer to be decoded...
      for a in 0..*ln {
        o_bits = wd;
        tokens = quote_tokens!(cx, $tokens
          out =
        );

        // Decode anything in the available space
        let rsft = 32 - s_bits;
        tokens = {
          if rsft == 0 {
            quote_tokens!(cx, $tokens
              *s_ptr as $ut;
            )
          } else {
            quote_tokens!(cx, $tokens
              (*s_ptr >> $rsft) as $ut;
            )
          }
        };
        
        // While the available space is not enough...
        while o_bits > s_bits {
          o_bits -= s_bits;
          let lsft = wd - o_bits;
          tokens = quote_tokens!(cx, $tokens
            s_ptr = s_ptr.offset(1);
            out |= (*s_ptr as $ut) << $lsft;
          );
          s_bits = 32;
        }
        s_bits -= o_bits;

        // Move decoded value to o_ptr
        tokens = {
          if mask_sft > 0 && wd != 32 {
            quote_tokens!(cx, $tokens
              *o_ptr = out & mask;
            )
          } else {
            quote_tokens!(cx, $tokens
              *o_ptr = out;
            )
          }
        };

        // Prepare for the following iteration
        if a < ln - 1 {
          tokens = quote_tokens!(cx, $tokens
            o_ptr = o_ptr.offset(1);
          );
          if s_bits == 0 {
            tokens = quote_tokens!(cx, $tokens
              s_ptr = s_ptr.offset(1); 
            );
            s_bits = 32;
          }
        }
      }

      // Function definition pushed to items
      items.push(
        quote_item!(cx,
          unsafe fn $ident(s_ptr: *const u32, o_ptr: *mut $ut) {
            $tokens
          }
        ).unwrap()
      );
    }
    idents.push(token::CloseDelim(token::Bracket));
    idents.push(token::Comma);
  }
  idents.push(token::CloseDelim(token::Bracket));

  // idents converted from tokens to TokenTree
  let ttree: Vec<ast::TokenTree> = idents
    .into_iter()
    .map(|token| ast::TokenTree::Token(codemap::DUMMY_SP, token))
    .collect();

  // DECODE_T definition pushed to items
  let name = format!("decode_{}", ut).to_uppercase();
  let ident = token::str_to_ident(&*name);
  let tmp = lengths.len();
  items.push(
    quote_item!(cx,
      pub const $ident: [[unsafe fn(*const u32, *mut $ut); $tmp]; $width] = $ttree;
    ).unwrap()
  );
  
  // DEBUGGING
  // for item in &items { println!("{}", pprust::item_to_string(item)); }

  MacEager::items(small_vector::SmallVector::many(items))
}

/// Generates ENCODE_SIMD_T containing function pointers, with the pointer for
/// encode_simd_T_a at ENCODE_SIMD_T[a - 1].
fn encode_simd_expand(cx: &mut ExtCtxt,
                      sp: codemap::Span,
                      tts: &[ast::TokenTree]) -> Box<MacResult + 'static> {
  // Arguments to the macro invocation
  let (width, simd) = {
    match parse_simd(cx, sp, tts) {
      Some(x) => x,
      None => return DummyResult::expr(sp)
    }
  };
  let ut = token::str_to_ident(&*format!("u{}", width));
  let lanes = 128 / width;

  // Construct full path to simd
  let mut simd = simd;
  simd.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident(&*format!("u{}x{}", width, lanes)),
      parameters: ast::PathParameters::none()
    }
  );

  // Construct code to read into register
  let mut load = simd.clone();
  load.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident("load"),
      parameters: ast::PathParameters::none()
    }
  );
  let load = quote_tokens!(cx,
    let rhs = $load(i_slice, i_ind);
  );

  // idents: tokens used to define the ENCODE_SIMD_T
  // items: definitions of the functions
  let mut idents = vec![token::OpenDelim(token::Bracket)];
  let mut items = Vec::new();
  for wd in 1...width {
    // Name for the function interned
    let name = format!("encode_simd_{}_{}", ut, wd);
    let ident = token::str_to_ident(&*name);
    idents.push(token::Ident(ident));
    idents.push(token::Comma);

    // Function definition constructed here
    let s_len = wd * lanes;
    let mut i_bits: usize;
    let mut s_bits: usize = width;
    let mut tokens = quote_tokens!(cx,
      let i_slice = std::slice::from_raw_parts(i_ptr, 128);
      let mut s_slice = std::slice::from_raw_parts_mut(s_ptr as *mut $ut, $s_len);
      let mut i_ind = 0;
    );
    // Handles unused mut warning
    tokens = {
      if wd == 1 {
        quote_tokens!(cx, $tokens
          let s_ind = 0;
        )
      } else {
        quote_tokens!(cx, $tokens
          let mut s_ind = 0;
        )
      }
    };
    tokens = quote_tokens!(cx, $tokens
      $load
      let mut lhs =
    );

    // For every integer to be encoded...
    for a in 0..width {
      i_bits = wd;

      // Encode in the available space
      let lsft = width - s_bits;
      if lsft == 0 {
        tokens = quote_tokens!(cx, $tokens
          rhs;
        );
      } else {
        tokens = quote_tokens!(cx, $tokens
          rhs << $lsft;
        );
      }

      // If the available space is not enough
      if s_bits < i_bits {
        i_bits -= s_bits;
        tokens = quote_tokens!(cx, $tokens
          lhs.store(s_slice, s_ind);
          s_ind += $lanes;
          lhs = rhs >> $s_bits;
        );
        s_bits = width;
      }
      s_bits -= i_bits;

      // Prepare for the following iteration
      if a < width - 1 {
        tokens = quote_tokens!(cx, $tokens
          i_ind += $lanes;
          $load
        );
        if s_bits == 0 {
          tokens = quote_tokens!(cx, $tokens
            lhs.store(s_slice, s_ind);
            s_ind += $lanes;
            lhs =
          );
          s_bits = width;
        } else {
          tokens = quote_tokens!(cx, $tokens
            lhs = lhs |
          );
        }
      } else {
        tokens = quote_tokens!(cx, $tokens
          lhs.store(s_slice, s_ind);
        );
      }
    }

    // Function definition pushed to items
    items.push(
      quote_item!(cx,
        unsafe fn $ident(i_ptr: *const $ut, s_ptr: *mut u32) {
          $tokens
        }
      ).unwrap()
    );
  }
  idents.push(token::CloseDelim(token::Bracket));

  // idents converted from tokens to TokenTree
  let ttree: Vec<ast::TokenTree> = idents
    .into_iter()
    .map(|token| ast::TokenTree::Token(codemap::DUMMY_SP, token))
    .collect();

  // ENCODE_SIMD_T definition pushed to items
  let name = format!("encode_simd_{}", ut).to_uppercase();
  let ident = token::str_to_ident(&*name);
  items.push(
    quote_item!(cx,
      pub const $ident: [unsafe fn(*const $ut, *mut u32); $width] = $ttree;
    ).unwrap()
  );

  // DEBUGGING
  // for item in &items { println!("{}", pprust::item_to_string(item)); }

  MacEager::items(small_vector::SmallVector::many(items))
}

/// Generates DECODE_SIMD_T containing function pointers, with the pointer for
/// decode_simd_T_a at DECODE_SIMD_T[a - 1].
fn decode_simd_expand(cx: &mut ExtCtxt,
                      sp: codemap::Span,
                      tts: &[ast::TokenTree]) -> Box<MacResult + 'static> {
  // Arguments to the macro invocation
  let (width, simd) = {
    match parse_simd(cx, sp, tts) {
      Some(x) => x,
      None => return DummyResult::expr(sp)
    }
  };
  let ut = token::str_to_ident(&*format!("u{}", width));
  let lanes = 128 / width;

  // Construct full path to simd
  let mut simd = simd;
  simd.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident(&*format!("u{}x{}", width, lanes)),
      parameters: ast::PathParameters::none()
    }
  );

  // Construct path for splat
  let mut splat = simd.clone();
  splat.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident("splat"),
      parameters: ast::PathParameters::none()
    }
  );

  // Construct code to read into register
  let mut load = simd.clone();
  load.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident("load"),
      parameters: ast::PathParameters::none()
    }
  );
  let load = quote_tokens!(cx,
    let rhs = $load(s_slice, s_ind);
  );

  // idents: tokens used to define the const DECODE_SIMD_T
  // items: definitions of the functions
  let mut idents = vec![token::OpenDelim(token::Bracket)];
  let mut items = Vec::new();
  for wd in 1...width {
    // Name for the function interned
    let name = format!("decode_simd_{}_{}", ut, wd);
    let ident = token::str_to_ident(&*name);
    idents.push(token::Ident(ident));
    idents.push(token::Comma);

    // Function definition constructed here
    let s_len = wd * lanes;
    let mut s_bits: usize = width;
    let mut o_bits: usize;
    let mut tokens = quote_tokens!(cx,
      let s_slice = std::slice::from_raw_parts(s_ptr as *const $ut, $s_len);
      let mut o_slice = std::slice::from_raw_parts_mut(o_ptr, 128);
      let mut o_ind = 0;
    );
    // Handles unused mut warning
    tokens = {
      if wd == 1 {
        quote_tokens!(cx, $tokens
          let s_ind = 0;
        )
      } else {
        quote_tokens!(cx, $tokens
          let mut s_ind = 0;
        )
      }
    };
    // Handes unused variable warning
    let mask_sft = width - wd;
    if mask_sft > 0 {
      tokens = quote_tokens!(cx, $tokens
        let mask = $splat(!0) >> $mask_sft;
      );
    }
    tokens = quote_tokens!(cx, $tokens
      $load
      let mut lhs;
    );

    // For every integer to be decoded...
    for a in 0..width {
      o_bits = wd;
      tokens = quote_tokens!(cx, $tokens
        lhs =
      );

      // Decode anything in the available space
      let rsft = width - s_bits;
      tokens = {
        if rsft == 0 {
          quote_tokens!(cx, $tokens
            rhs;
          )
        } else {
          quote_tokens!(cx, $tokens
            rhs >> $rsft;
          )
        }
      };

      // If the available space is not enough...
      if o_bits > s_bits {
        o_bits -= s_bits;
        tokens = quote_tokens!(cx, $tokens
          s_ind += $lanes;
          $load
          lhs = lhs | rhs << $s_bits;
        );
        s_bits = width;
      }
      s_bits -= o_bits;

      // Move decoded value to o_ptr
      if mask_sft > 0 {
        tokens = quote_tokens!(cx, $tokens
          lhs = lhs & mask;
        );
      }
      tokens = quote_tokens!(cx, $tokens
        lhs.store(o_slice, o_ind);
      );

      // Prepare for the following iteration
      if a < width - 1 {
        tokens = quote_tokens!(cx, $tokens
          o_ind += $lanes;
        );
        if s_bits == 0 {
          tokens = quote_tokens!(cx, $tokens
            s_ind += $lanes;
            $load
          );
          s_bits = width;
        }
      }
    }

    // Function definition pushed to items
    items.push(
      quote_item!(cx,
        unsafe fn $ident(s_ptr: *const u32, o_ptr: *mut $ut) {
          $tokens
        }
      ).unwrap()
    );
  }
  idents.push(token::CloseDelim(token::Bracket));

  // idents converted from tokens to TokenTree
  let ttree: Vec<ast::TokenTree> = idents
    .into_iter()
    .map(|token| ast::TokenTree::Token(codemap::DUMMY_SP, token))
    .collect();

  // DECODE_SIMD_T definition pushed to items
  let name = format!("decode_simd_{}", ut).to_uppercase();
  let ident = token::str_to_ident(&*name);
  items.push(
    quote_item!(cx,
      pub const $ident: [unsafe fn(*const u32, *mut $ut); $width] = $ttree;
    ).unwrap()
  );
  
  // DEBUGGING
  // for item in &items { println!("{}", pprust::item_to_string(item)); }

  MacEager::items(small_vector::SmallVector::many(items))
}

/// Generates encode_zz_T and encode_zz_shift_T functions.
fn encode_zz_expand(cx: &mut ExtCtxt,
                    sp: codemap::Span,
                    tts: &[ast::TokenTree]) -> Box<MacResult + 'static> {
  // Arguments to the macro invocation
  let (width, simd) = {
    match parse_simd(cx, sp, tts) {
      Some(x) => x,
      None => return DummyResult::expr(sp)
    }
  };
  let ut = token::str_to_ident(&*format!("u{}", width));
  let it = token::str_to_ident(&*format!("i{}", width));
  let lanes = 128 / width;
  let xsft = width - 1;

  // Construct full path to simd
  let mut simd = simd;
  simd.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident(&*format!("i{}x{}", width, lanes)),
      parameters: ast::PathParameters::none()
    }
  );

  // Construct path for splat
  let mut splat = simd.clone();
  splat.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident("splat"),
      parameters: ast::PathParameters::none()
    }
  );

  // Construct code to read into register
  let mut load = simd.clone();
  load.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident("load"),
      parameters: ast::PathParameters::none()
    }
  );

  // items: definitions of the functions
  let mut items = Vec::new();

  // Name for the function interned
  let name = format!("encode_zz_{}", ut);
  let ident = token::str_to_ident(&*name);

  // Function definition constructed here
  items.push(
    quote_item!(cx,
      pub unsafe fn $ident(i_ptr: *mut $ut, length: usize) {
        let i_slice = std::slice::from_raw_parts_mut(i_ptr as *mut $it, length);
        let mut i_ind = 0;

        for _ in 0..(length / $lanes) {
          let rhs = $load(i_slice, i_ind);
          let lhs = (rhs << 1usize) ^ (rhs >> $xsft);
          lhs.store(i_slice, i_ind);
          i_ind += $lanes;
        }

        for x in i_slice[i_ind..].iter_mut() {
          let rhs = *x;
          *x = (rhs << 1usize) ^ (rhs >> $xsft);
        }
      }
    ).unwrap()
  );

  // Name for the function interned
  let name = format!("encode_zz_shift_{}", ut);
  let ident = token::str_to_ident(&*name);

  // Function definition constructed here
  items.push(
    quote_item!(cx,
      pub unsafe fn $ident(i_ptr: *mut $ut, length: usize, shift: $ut) {
        let i_slice = std::slice::from_raw_parts_mut(i_ptr as *mut $it, length);
        let mut i_ind = 0;

        let shift = shift as $it;
        let simd_shift = $splat(shift);

        for _ in 0..(length / $lanes) {
          let rhs = $load(i_slice, i_ind) - simd_shift;
          let lhs = (rhs << 1usize) ^ (rhs >> $xsft);
          lhs.store(i_slice, i_ind);
          i_ind += $lanes;
        }

        for x in i_slice[i_ind..].iter_mut() {
          let rhs = (*x).wrapping_sub(shift);
          *x = (rhs << 1usize) ^ (rhs >> $xsft);
        }
      }
    ).unwrap()
  );

  // DEBUGGING
  // for item in &items { println!("{}", pprust::item_to_string(item)); }

  MacEager::items(small_vector::SmallVector::many(items))
}

/// Generates decode_zz_T and decode_zz_shift_T functions.
fn decode_zz_expand(cx: &mut ExtCtxt,
                    sp: codemap::Span,
                    tts: &[ast::TokenTree]) -> Box<MacResult + 'static> {
  // Arguments to the macro invocation
  let (width, simd) = {
    match parse_simd(cx, sp, tts) {
      Some(x) => x,
      None => return DummyResult::expr(sp)
    }
  };
  let ut = token::str_to_ident(&*format!("u{}", width));
  let lanes = 128 / width;

  // Construct full path to simd
  let mut simd = simd;
  simd.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident(&*format!("u{}x{}", width, lanes)),
      parameters: ast::PathParameters::none()
    }
  );

  // Construct path for splat
  let mut splat = simd.clone();
  splat.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident("splat"),
      parameters: ast::PathParameters::none()
    }
  );

  // Construct code to read into register
  let mut load = simd.clone();
  load.segments.push(
    ast::PathSegment {
      identifier: token::str_to_ident("load"),
      parameters: ast::PathParameters::none()
    }
  );

  // items: definitions of the functions
  let mut items = Vec::new();

  // Name for the function interned
  let name = format!("decode_zz_{}", ut);
  let ident = token::str_to_ident(&*name);

  // Function definition constructed here
  items.push(
    quote_item!(cx,
      pub unsafe fn $ident(o_ptr: *mut $ut, length: usize) {
        let mut o_slice = std::slice::from_raw_parts_mut(o_ptr, length);
        let mut o_ind = 0;

        let ones = $splat(1);

        for _ in 0..(length / $lanes) {
          let rhs = $load(o_slice, o_ind);
          let lhs = (rhs >> 1usize) ^ (!(rhs & ones) + ones);
          lhs.store(o_slice, o_ind);
          o_ind += $lanes;
        }

        for x in o_slice[o_ind..].iter_mut() {
          let rhs = *x;
          *x = (rhs >> 1) ^ (!(rhs & 1)).wrapping_add(1);
        }
      }
    ).unwrap()
  );

  // Name for the function interned
  let name = format!("decode_zz_shift_{}", ut);
  let ident = token::str_to_ident(&*name);

  // Function definition constructed here
  items.push(
    quote_item!(cx,
      pub unsafe fn $ident(o_ptr: *mut $ut, length: usize, shift: $ut) {
        let mut o_slice = std::slice::from_raw_parts_mut(o_ptr, length);
        let mut o_ind = 0;

        let simd_shift = $splat(shift);
        let ones = $splat(1);

        for _ in 0..(length / $lanes) {
          let rhs = $load(o_slice, o_ind);
          let lhs = ((rhs >> 1usize) ^ (!(rhs & ones) + ones)) + simd_shift;
          lhs.store(o_slice, o_ind);
          o_ind += $lanes;
        }

        for x in o_slice[o_ind..].iter_mut() {
          let rhs = *x;
          *x = ((rhs >> 1) ^ (!(rhs & 1)).wrapping_add(1)).wrapping_add(shift);
        }
      }
    ).unwrap()
  );

  // DEBUGGING
  // for item in &items { println!("{}", pprust::item_to_string(item)); }

  MacEager::items(small_vector::SmallVector::many(items))
}

/// Parse the two arguments to the encode and decode syntax extensions.
fn parse(cx: &mut ExtCtxt,
         sp: codemap::Span,
         tts: &[ast::TokenTree]) -> Option<(usize, usize)> {
  let mut parser = cx.new_parser_from_tts(tts);

  let entry = cx.expander().fold_expr(parser.parse_expr().unwrap());
  let width = {
    match entry.node {
      ast::ExprKind::Lit(ref lit) => {
        match lit.node {
          ast::LitKind::Int(n, _) => n,
          _ => {
            cx.span_err(entry.span, &format!(
                "expected integer literal but got '{}'",
                pprust::lit_to_string(&**lit)));
            return None
          }
        }
      }
      _ => {
        cx.span_err(entry.span, &format!(
            "expected integer literal but got '{}'",
            pprust::expr_to_string(&*entry)));
        return None }
    }
  };
  parser.eat(&token::Comma);

  let entry = cx.expander().fold_expr(parser.parse_expr().unwrap());
  let step = {
    match entry.node {
      ast::ExprKind::Lit(ref lit) => {
        match lit.node {
          ast::LitKind::Int(n, _) => n,
          _ => {
            cx.span_err(entry.span, &format!(
                "expected integer literal but got '{}'",
                pprust::lit_to_string(&**lit)));
            return None
          }
        }
      }
      _ => {
        cx.span_err(entry.span, &format!(
            "expected integer literal but got '{}'",
            pprust::expr_to_string(&*entry)));
        return None }
    }
  };
  parser.eat(&token::Comma);

  if parser.token != token::Eof {
    cx.span_err(sp, "expected exactly two arguments");
    return None
  }

  Some((width as usize, step as usize))
}

/// Parse the two arguments to the encode_simd, decode_simd, encode_zz and
/// decode_zz syntax extensions.
fn parse_simd(cx: &mut ExtCtxt,
              sp: codemap::Span,
              tts: &[ast::TokenTree]) -> Option<(usize, ast::Path)> {
  let mut parser = cx.new_parser_from_tts(tts);

  let entry = cx.expander().fold_expr(parser.parse_expr().unwrap());
  let width = {
    match entry.node {
      ast::ExprKind::Lit(ref lit) => {
        match lit.node {
          ast::LitKind::Int(n, _) => n,
          _ => {
            cx.span_err(entry.span, &format!(
                "expected integer literal but got '{}'",
                pprust::lit_to_string(&**lit)));
            return None
          }
        }
      }
      _ => { cx.span_err(entry.span, &format!(
            "expected integer literal but got '{}'",
            pprust::expr_to_string(&*entry)));
        return None }
    }
  };
  parser.eat(&token::Comma);

  let entry = cx.expander().fold_expr(parser.parse_expr().unwrap());
  let simd = {
    match entry.node {
      ast::ExprKind::Path(_, ref p) => p.clone(),
      _ => {
        cx.span_err(entry.span, &format!(
          "expected path but got '{}'",
          pprust::expr_to_string(&*entry)));
        return None
      }
    }
  };

  parser.eat(&token::Comma);
  if parser.token != token::Eof {
    cx.span_err(sp, "expected exactly two arguments");
    return None
  }

  Some((width as usize, simd))
}
