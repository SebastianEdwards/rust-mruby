use std::{cast, str, vec, io};
use std::libc::types::common::c95::c_void;

use std::c_str::CString;

pub type mruby_Type = u32;
pub static MRB_TT_FALSE:     u32 = 0;
pub static MRB_TT_FREE:      u32 = 1;
pub static MRB_TT_TRUE:      u32 = 2;
pub static MRB_TT_FIXNUM:    u32 = 3;
pub static MRB_TT_SYMBOL:    u32 = 4;
pub static MRB_TT_UNDEF:     u32 = 5;
pub static MRB_TT_FLOAT:     u32 = 6;
pub static MRB_TT_CPTR:      u32 = 7;
pub static MRB_TT_OBJECT:    u32 = 8;
pub static MRB_TT_CLASS:     u32 = 9;
pub static MRB_TT_MODULE:    u32 = 10;
pub static MRB_TT_ICLASS:    u32 = 11;
pub static MRB_TT_SCLASS:    u32 = 12;
pub static MRB_TT_PROC:      u32 = 13;
pub static MRB_TT_ARRAY:     u32 = 14;
pub static MRB_TT_HASH:      u32 = 15;
pub static MRB_TT_STRING:    u32 = 16;
pub static MRB_TT_RANGE:     u32 = 17;
pub static MRB_TT_EXCEPTION: u32 = 18;
pub static MRB_TT_FILE:      u32 = 19;
pub static MRB_TT_ENV:       u32 = 20;
pub static MRB_TT_DATA:      u32 = 21;
pub static MRB_TT_FIBER:     u32 = 22;
pub static MRB_TT_MAXDEFINE: u32 = 23;

type MrbBool = u8;
type MrbSym = i16;
type MrbInt = u32;

pub struct MrbString {
	tt: u32,
	color: u32,
	flags: u32,
	c: *mut RClass,
	gcnext: *mut RBasic,
	len: u32,
	capa: u32,
	ptr: *u8,
}

pub struct RHash {
	tt: mruby_Type,
	color: u32,
	flags: u32,
	c: *mut RClass,
	gcnext: *mut RBasic,
	iv: *mut IvTbl,
	ht: *mut KhHt,
}

pub struct RClass {
	tt: mruby_Type,
	color: u32,
	flags: u32,
	c: *mut RClass,
	gcnext: *mut RBasic,
	iv: *mut IvTbl,
	mt: *mut KhHt,
	_super: *mut RClass,
}

pub type MrbProc = c_void;
pub type MrbState = c_void;

type MrbLexState = c_void;
type KhHt = c_void;
type MrbPool = c_void;
type IvTbl = c_void;
type RBasic = c_void;

struct MrbParserMessage {
	lineno: i32,
	column: i32,
	message: *mut i8,
}

pub struct mruby_Value {
	value: [u8, ..8],
	tt: mruby_Type,
}

struct MrbParserState {
	mrb: *c_void,
	pool: *c_void,
	cells: *c_void,
	s: *i8,
	send: *i8,
	f: *c_void,
	cxt: *mut MrbContext,
	filename: *i8,
	lineno: i32,
	column: i32,
	lstate: MrbLexState,
	lex_strterm: *c_void,
	cond_stack: u32,
	cmdarg_stack: u32,
	paren_nest: i32,
	lpar_beg: i32,
	in_def: i32,
	in_single: i32,
	cmd_start: i32,
	locals: *c_void,
	pb: *c_void,
	buf: [i8, ..1024u],
	bidx: i32,
	all_heredocs: *c_void,
	heredocs_from_nextline: *c_void,
	parsing_heredoc: *c_void,
	lex_strterm_before_heredoc: *c_void,
	heredoc_end_now: MrbBool,
	ylval: *c_void,
	nerr: u64,
	nwarn: u64,
	tree: *c_void,
	capture_errors: i32,
	error_buffer: [MrbParserMessage, ..10u],
	warn_buffer: [MrbParserMessage, ..10u],
	filename_table: *mut MrbSym,
	filename_table_length: u64,
	current_filename_index: i32,
	jmp: int,
}

struct MrbContext {
	syms: *MrbSym,
	slen: i32,
	filename: *mut i8,
	lineno: i16,
	partial_hook: extern "C" fn(arg1: *mut MrbParserState) -> i32,
	partial_data: *c_void,
	target_class: *RClass,
	capture_errors: MrbBool,
	dump_result: MrbBool,
	no_exec: MrbBool,
}

impl mruby_Value {
	pub fn to_f64(&self) -> Option<f64> {
		match self.tt {
			MRB_TT_FLOAT => unsafe { Some(self.unchecked_to_f64()) },
			_ => None
		}
	}

	pub unsafe fn unchecked_to_f64(&self) -> f64 {
		io::BufReader::new(self.value).read_le_f64()
	}

	pub fn to_i32(&self) -> Option<i32> {
		match self.tt {
			MRB_TT_FIXNUM => unsafe { Some(self.unchecked_to_i32()) },
			_ => None
		}
	}

	pub unsafe fn unchecked_to_i32(&self) -> i32 {
		io::BufReader::new(self.value).read_le_i32()
	}

	pub fn to_bool(&self) -> Option<bool> {
		match self.tt {
			MRB_TT_FALSE => Some(false),
			MRB_TT_TRUE => Some(true),
			_ => None
		}
	}

	pub fn to_str(&self) -> Option<~str> {
		match self.tt {
			MRB_TT_STRING => unsafe { Some(self.unchecked_to_str()) },
			_ => None
		}
	}

	pub unsafe fn unchecked_to_str(&self) -> ~str {
		let string : *MrbString =   cast::transmute(self.value);
		let mruby_string : MrbString = *string;
		let strvec = vec::from_buf(mruby_string.ptr, mruby_string.len as uint);

		str::from_utf8(strvec).unwrap().to_str()
	}
}

#[link(name = "mruby")]
extern "C" {
	pub fn mrb_open() -> *MrbState;
	pub fn mrb_close(mrb: *MrbState) -> *c_void;

	pub fn mrbc_context_new(mrb: *MrbState) -> *MrbContext;
	pub fn mrbc_context_free(mrb: *MrbState, context: *MrbContext) -> *MrbContext;
	pub fn mrbc_filename(mrb: *MrbState, c: *MrbContext, s: *i8) -> CString;

	pub fn mrb_load_string(mrb: *MrbState, string: *i8) -> mruby_Value;
	pub fn mrb_load_string_cxt(mrb: *MrbState, string: *i8, context: *MrbContext) -> mruby_Value;

	pub fn mrb_parse_string(mrb: *MrbState, string: *i8, context: *MrbContext) -> *MrbParserState;

	pub fn mrb_generate_code(mrb: *MrbState, parser_state: *MrbParserState) -> *MrbProc;

	pub fn mrb_top_self(mrb: *MrbState) -> mruby_Value;

	pub fn mrb_run(mrb: *MrbState, ruby_proc: *MrbProc, value: mruby_Value) -> mruby_Value;
}
