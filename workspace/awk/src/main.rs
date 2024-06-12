#[path = "../bindings/bindings.rs"]
mod bindings;
use bindings::*;
use libc::fileno;
use libc::free;
use libc::memchr;
use libc::rand;
use libc::regex_t;
use libc::setlocale;
use libc::srand;
use libc::strchr;
use libc::strlen;
use libc::strstr;
use libc::system;
use libc::time;
use libc::EXIT_SUCCESS;
use libc::FILE;
use libc::LC_NUMERIC;
use libc::RAND_MAX;
use libc::{regexec, regmatch_t};
use std::env;
use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Add;
use std::os::raw::{c_char, c_int};
use std::os::unix::io::{AsRawFd, RawFd};
use std::process;
use std::ptr;
use std::ptr::write_volatile;
use std::ptr::{null, null_mut};
use std::str;
use std::sync::atomic::{compiler_fence, fence, Ordering};

fn b_d_offset_rs(b: *mut u8, offset: isize) -> u8 {
    unsafe { *b.offset(offset) }
}

fn strchr_rs(s: *const u8, c: u8) -> *const u8 {
    unsafe { strchr(s as *const i8, c as i32) as *const u8 }
}

fn memchr_rs(s: *const c_void, c: i32, n: usize) -> *const u8 {
    unsafe { memchr(s, c, n) as *const u8 }
}

fn offset_from_rs(s: *const u8, b: *mut u8) -> i32 {
    unsafe { s.offset_from(b).try_into().unwrap() }
}

fn offset_rs_cvoid(p: *mut u8, offset: isize) -> *mut c_void {
    unsafe { offset_rs(p, offset) as *mut c_void }
}

fn getRsmPtr(rsm: *mut rstream) -> rstream {
    unsafe { *rsm }
}

fn file_no_rs(rsm: *mut rstream) -> RawFd {
    unsafe { fileno(getRsmPtr(rsm).F as *mut FILE) }
}

fn assign_b_rs(b: *mut u8, p: i32) {
    unsafe {
        *b.offset(p as isize) = b'\0'; // Changed from '\0' to 0
    }
}

fn qrealloc_rs(p: *mut i8, size: i32, new_size: &mut i32) -> *mut u8 {
    unsafe { qrealloc(p as *mut i8, size, new_size) as *mut u8 }
}

fn offset_rs(p: *mut u8, offset: isize) -> *mut u8 {
    unsafe { p.offset(offset) }
}

fn b_offset_rs_const_i8(b: *mut u8, offset: isize) -> *const i8 {
    unsafe { b.offset(offset) as *const i8 }
}

fn assign_b_rs_v2(b: *mut u8, p: i32, c: u8) {
    unsafe {
        *b.offset(p as isize) = c;
    }
}

fn set_v_type_user(v: *mut var) {
    unsafe {
        (*v).type_ |= VF_USER;
    }
}

fn set_rsm_buffer(rsm: *mut rstream, m: *mut u8) -> *mut rstream {
    unsafe {
        (*rsm).buffer = m as *mut i8;
        rsm
    }
}

fn set_rsm_adv(rsm: *mut rstream, a: i32) -> *mut rstream {
    unsafe {
        (*rsm).adv = a;
        rsm
    }
}

fn set_rsm_pos(rsm: *mut rstream, p: i32) -> *mut rstream {
    unsafe {
        (*rsm).pos = p;
        rsm
    }
}

fn set_rsm_size(rsm: *mut rstream, size: i32) -> *mut rstream {
    unsafe {
        (*rsm).size = size;
        rsm
    }
}

fn set_rsm_rs(rsm: *mut rstream, m: *mut u8, a: i32, p: i32, size: i32, eo: i32) -> *mut rstream {
    set_rsm_buffer(rsm, m);
    set_rsm_adv(rsm, a + eo);
    set_rsm_pos(rsm, p - eo);
    set_rsm_size(rsm, size);
    rsm
}

fn safe_read_rs(fd: RawFd, buf: *mut c_void, count: usize) -> isize {
    unsafe { safe_read(fd, buf, count) as isize }
}

fn safe_get_errno() -> i32 {
    unsafe { *libc::__errno_location() }
}

fn strstr_rs(haystack: *const i8, needle: *const u8) -> *const u8 {
    unsafe { strstr(haystack, needle as *const i8) as *const u8 }
}

fn regexec_rs(
    re: *const regex_t,
    b: *const u8,
    nmatch: usize,
    pmatch: *mut regmatch_t,
    eflags: i32,
) -> i32 {
    unsafe {
        regexec(
            re,
            b as *const i8,
            nmatch,
            pmatch as *mut regmatch_t,
            eflags,
        )
    }
}

fn safe_ptr_copy(dest: *mut u8, src: *mut u8, len: usize) {
    unsafe {
        ptr::copy(src, dest, len);
    }
}

fn get_rsplitter_ire(globs2: &mut globals2) -> *mut regex_t {
    unsafe { globs2.rsplitter.n.r.ire as *mut regex_t }
}

fn get_rsplitter_re(globs2: &mut globals2) -> *mut regex_t {
    unsafe { globs2.rsplitter.n.l.re as *mut regex_t }
}

fn set_var_type_number(v: *mut var) {
    unsafe {
        (*v).type_ |= VF_NUMBER;
    }
}

fn set_var_number(v: *mut var, val: f64) {
    unsafe {
        (*v).number = val;
    }
}

fn set_var_string_rs(v: *mut var, val: *const c_char) {
    unsafe {
        (*v).string = val as *mut c_char;
    }
}

fn setvar_p_rs(v: *mut var, val: *const c_char) -> *mut var {
    clrvar_rs(v);
    set_var_string_rs(v, val);
    handle_special_wrapper(v);
    return v;
}

fn xstrdup_rs(s: *const c_char) -> *mut c_char {
    unsafe { xstrdup(s) }
}

fn getGlobals() -> &'static mut globals {
    unsafe { &mut *ptr_to_globals.offset(-1) }
}

fn getGlobals2Ptr() -> &'static mut globals2 {
    unsafe { &mut *(ptr_to_globals as *mut globals2) }
}

fn create_c_string(s: &str) -> *mut c_char {
    let c_string = CString::new(s).expect("CString::new failed");
    c_string.into_raw() // Returns a *mut c_char and leaks the CString to prevent it from being freed.
}

fn hash_init_wrapper() -> *mut xhash_s {
    unsafe { hash_init() }
}

fn set_v_type_fstr(v: *mut var) {
    unsafe {
        (*v).type_ |= VF_FSTR;
    }
}

fn setvar_u_rs(v: *mut var, val: *const c_char) {
    clrvar_rs(v);
    set_v_type_fstr(v);
    set_var_string_rs(v, val);
}

fn iamarray_rs(v: *mut var) -> *mut xhash_s {
    unsafe { iamarray(v) }
}

fn xmalloc_rs_maxvarfmt() -> *mut c_void {
    unsafe { xmalloc((MAXVARFMT + 1).try_into().unwrap()) }
}

fn set_applet_name_rs() {
    unsafe {
        applet_name = create_c_string("target/debug/awk_rs");
    }
}

fn init_g_rs() {
    unsafe {
        init_g();
    }
}

fn initialization() {
    init_g_rs();
    set_applet_name_rs();
    let globs = getGlobals();
    globs.g_buf = xmalloc_rs_maxvarfmt() as *mut i8;
}

fn enable_locale_supp() {
    let c_locale = CString::new("C").unwrap();
    unsafe {
        setlocale(LC_NUMERIC, c_locale.as_ptr());
    }
}

fn getVvaluesPtr() -> *mut *mut c_char {
    unsafe { vValues.as_ptr() as *mut *mut c_char }
}

fn getVNamesPtr() -> *mut *mut c_char {
    unsafe { vNames.as_ptr() as *mut *mut c_char }
}

fn hash_find_wrapper_var_s(hash: *mut xhash_s, key: *const c_char) -> *mut var_s {
    unsafe { hash_find(hash, key) as *mut var_s }
}

fn get_next_vname(vnames: *mut *mut c_char) -> *mut c_char {
    unsafe {
        let next_vname = nextword(vnames);
        if next_vname.is_null() {
            // Handle the case where `nextword` returns a null pointer
            return std::ptr::null_mut();
        }

        match CStr::from_ptr(next_vname).to_str() {
            Ok(string) => {
                if let Some(first_char) = string.chars().next() {
                    if first_char == '*' {
                        // Skip the '*' character by moving the pointer ahead by one character.
                        return next_vname.add(1);
                    }
                }
            }
            Err(_) => {
                // Handle the case where `CStr::from_ptr().to_str()` fails due to invalid UTF-8 data
                eprintln!("Failed to convert C string to Rust string due to invalid UTF-8 format.");
                return std::ptr::null_mut();
            }
        }

        next_vname
    }
}

fn get_vnames_mut_first_as_char(vnames_mut: *mut *mut c_char) -> Option<char> {
    unsafe { CStr::from_ptr(*vnames_mut).to_str().unwrap().chars().next() }
}

fn set_v_type_special(v: *mut var) {
    unsafe {
        (*v).type_ |= VF_SPECIAL;
    }
}

fn check_special_char_and_update_v_type(v: *mut var, vnames_mut: *mut *mut c_char) {
    if let Some(s) = get_vnames_mut_first_as_char(vnames_mut) {
        if s == '*' {
            set_v_type_special(v);
        }
    }
}

fn check_and_update_variable(v: &mut var, vvalues_mut: *mut *mut c_char) {
    if validVvalues_mut(vvalues_mut) {
        let next_vvalue = nextword_rs(vvalues_mut);
        setvar_s_rs(v, next_vvalue);
    } else {
        setvar_i_rs(v, 0.0);
    }
}

fn evaluate_wrapper(node: *mut node, tv: *mut var_s) -> *mut var_s {
    unsafe { evaluate(node, tv) }
}

fn next_input_file_wrapper() -> *mut rstream {
    unsafe { next_input_file() }
}

fn parse_program_rs(c_str: *mut c_char) {
    unsafe {
        parse_program(c_str);
    }
}

fn llist_pop_rs(list: &mut *mut llist_t) -> *mut c_char {
    unsafe { llist_pop(list) as *mut c_char }
}

fn xfopen_stdin_rs(progname: *const c_char) -> *mut bindings::FILE {
    unsafe { xfopen_stdin(progname as *const i8) }
}

fn hash_find_wrapper_rstream(hash: *mut xhash_s, key: *const c_char) -> *mut rstream {
    unsafe { hash_find(hash, key) as *mut rstream }
}

fn set_stdin_stdout_stderr(
    stdin_file: *mut rstream,
    stdout_file: *mut rstream,
    stderr_file: *mut rstream,
) {
    unsafe {
        (*stdin_file).F = stdin;
        (*stdout_file).F = stdout;
        (*stderr_file).F = stderr;
    }
}

fn handle_special_wrapper(v: *mut var) {
    unsafe {
        handle_special(v);
    }
}

fn hash_find_wrapper_var(hash: *mut xhash_s, key: *const c_char) -> *mut var {
    unsafe { hash_find(hash, key) as *mut var }
}

fn init_tv() -> var_s {
    unsafe { std::mem::zeroed() }
}

fn fread_rs(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut bindings::FILE) -> usize {
    unsafe {
        fread(
            ptr,
            size.try_into().unwrap(),
            nmemb.try_into().unwrap(),
            stream,
        )
        .try_into()
        .unwrap()
    }
}

fn add_buffer_ptr(buffer: &mut Vec<u8>, size: usize) -> *mut u8 {
    unsafe { buffer.as_mut_ptr().add(size) }
}

fn fclose_rs(stream: *mut bindings::FILE) {
    unsafe {
        fclose(stream);
    }
}

fn as_ptr_rs(buffer: &Vec<u8>) -> *mut i8 {
    unsafe {
        let s = CStr::from_bytes_with_nul_unchecked(&buffer);
        buffer.as_ptr() as *mut i8
    }
}

fn getopt32_rs(
    argv: *mut *mut i8,
    opt_F: *mut *mut c_char,
    list_v: *mut *mut llist_t,
    list_f: *mut *mut llist_t,
) -> u32 {
    unsafe {
        getopt32(
            argv,
            OPTSTR_AWK.as_ptr() as *const i8,
            opt_F,
            list_v,
            list_f,
            std::ptr::null_mut() as *mut c_void,
        )
    }
}

fn getopt32_rsv2(
    argv: *mut *mut i8,
    opt_F: *mut *mut c_char,
    list_v: *mut *mut llist_t,
    list_f: *mut *mut llist_t,
    list_e: *mut *mut llist_t,
) -> u32 {
    unsafe {
        getopt32(
            argv,
            OPTSTR_AWK.as_ptr() as *const i8,
            opt_F,
            list_v,
            list_f,
            list_e,
        )
    }
}

fn getoptind_rs() -> i32 {
    unsafe { getoptind() }
}

fn add_argv(argv: *mut *mut i8, i: usize) -> *mut *mut i8 {
    unsafe { argv.add(i as usize) }
}

fn bb_simple_error_msg_rs(msg: *const c_char) {
    unsafe {
        bb_simple_error_msg(msg);
    }
}

fn deference_rs(ptr: *mut *mut i8) -> *mut i8 {
    unsafe { *ptr }
}

fn setari_u_rs_default(v: *mut var) {
    unsafe {
        setari_u(v, 0, CString::new("target/debug/awk_rs").unwrap().as_ptr());
    }
}

fn setari_u_rs(v: *mut var, i: i32, val: *mut *mut i8) {
    unsafe {
        setari_u(v, i, *val);
    }
}

fn check_argv_null_rs(argv: *mut *mut i8) -> bool {
    unsafe { *argv == std::ptr::null_mut() }
}
fn incvar_rs(v: *mut var) {
    unsafe {
        incvar(v);
    }
}

fn syntax_error_rs() {
    unsafe {
        syntax_error(
            CString::new(CStr::from_ptr(strerror(ERRNO.try_into().unwrap())).to_bytes())
                .unwrap()
                .as_ptr(),
        );
    }
}

fn awk_exit_rs(status: i32) {
    unsafe {
        awk_exit(status);
    }
}

fn is_assignment_rs(s: *const c_char) -> i32 {
    unsafe { is_assignment(s) }
}

fn bb_show_usage_rs() {
    unsafe {
        bb_show_usage();
    }
}

fn unescape_string_in_place_rs(c_str: *mut c_char) {
    unsafe {
        unescape_string_in_place(c_str);
    }
}

fn validVvalues_mut(vvalues_mut: *mut *mut c_char) -> bool {
    unsafe {
        !vvalues_mut.is_null() && *vvalues_mut as u8 != 255 // '\377' is 255 in decimal
    }
}

fn nextword_rs(vvalues_mut: *mut *mut c_char) -> *mut c_char {
    unsafe { nextword(vvalues_mut) }
}

fn check_op_r_n_null(op: *mut node) -> bool {
    unsafe { (*op).r.n.is_null() }
}

fn hash_find_rs(hash: *mut xhash_s, key: *const c_char) -> *mut var_s {
    unsafe { hash_find(hash, key) as *mut var_s }
}

fn hash_remove_rs(hash: *mut xhash_s, key: *const c_char) {
    unsafe { hash_remove(hash, key) }
}

fn get_var_s_rs(v: *mut var) -> *const c_char {
    unsafe { getvar_s(v) }
}

fn clear_array_rs(v: *mut xhash_s) {
    unsafe { clear_array(v) }
}

fn nvalloc_rs(size: i32) -> *mut var {
    unsafe { nvalloc(size) }
}

fn get_op_info_rs(op: *mut node) -> u32 {
    unsafe { (*op).info.try_into().unwrap() }
}

fn get_opt_lineno_rs(op: *mut node) -> i32 {
    unsafe { (*op).lineno.try_into().unwrap() }
}

fn get_op_l_n_rs(op: *mut node) -> *mut node {
    unsafe { (*op).l.n }
}

fn get_info_and(op: *mut node, info: u32) -> u32 {
    unsafe { (*op).info & info }
}

fn get_op_l_v_rs(op: *mut node) -> *mut var {
    unsafe { (*op).l.v }
}

fn get_op_l_aidx_rs(op: *mut node) -> i32 {
    unsafe { (*op).l.aidx.try_into().unwrap() }
}

fn syntax_error_rs_v2(msg: *const c_char) {
    unsafe {
        syntax_error(CStr::from_ptr(msg).as_ptr());
    }
}

fn get_op_r_n_rs(op: *mut node) -> *mut node {
    unsafe { (*op).r.n }
}

fn offset_rs_v2(p: *mut var_s, offset: isize) -> *mut var_s {
    unsafe { p.offset(offset) }
}

fn get_var_i_rs(v: *mut var) -> f64 {
    unsafe { getvar_i(v) }
}

fn get_fnargs_rs(idx: usize) -> *mut var {
    unsafe { &mut *((getGlobals2Ptr()).evaluate__fnargs).offset(idx as isize) as *mut var }
}

fn set_op_info_or(op: *mut node, info: u32) -> u32 {
    unsafe {
        (*op).info |= info;
        (*op).info
    }
}

fn set_op_info_and(op: *mut node, info: u32) -> u32 {
    unsafe {
        (*op).info &= info;
        (*op).info
    }
}

fn ptest_rs_l_n(op: *mut node) -> i32 {
    unsafe { ptest((*op).l.n) }
}

fn ptest_rs_r_n(op: *mut node) -> i32 {
    unsafe { ptest((*op).r.n) }
}

fn ptest_rs_op(op: *mut node) -> i32 {
    unsafe { ptest(op) }
}

fn hashwalk_init_rs(Lv: *mut var, Rv: *mut xhash_s) {
    unsafe {
        hashwalk_init(Lv, Rv);
    }
}

fn hashwalk_next_rs(Lv: *mut var) -> i32 {
    unsafe { hashwalk_next(Lv) }
}

fn copyvar_rs(Lv: *mut var, Rv: *mut var) -> *mut var {
    unsafe { copyvar(Lv, Rv) }
}

fn clrvar_rs(v: *mut var) {
    unsafe {
        clrvar(v);
    }
}

fn hash_search_rs(Lv: *mut xhash_s, Rs: *const c_char) -> *mut var_s {
    unsafe { hash_search(Lv, Rs) as *mut var_s }
}

fn as_regex_rs(op: *mut node, re: *mut bindings::regex_t) -> *mut bindings::regex_t {
    unsafe { as_regex(op, re) as *mut bindings::regex_t }
}

fn get_lv_ptr_rs(Lv: *mut var) -> var_s {
    unsafe { *Lv }
}

fn get_lv_x_array_nel(Lv: *mut var) -> u32 {
    unsafe { (*(*Lv).x.array).nel }
}

fn fflush_all_rs() {
    unsafe {
        fflush_all();
    }
}

fn system_rs(s: *const c_char) -> i32 {
    unsafe { system(s) }
}

fn fputs_rs(s: *const c_char, stream: *mut bindings::FILE) {
    unsafe {
        fputs(s, stream);
    }
}

fn nextarg_rs(mut op1: *mut *mut node) -> *mut node {
    unsafe { nextarg(op1) }
}

fn fmt_num_rs(buf: *mut c_char, size: i32, fmt: *const c_char, num: f64, is_float: i32) {
    unsafe {
        fmt_num(buf, size, fmt, num, is_float);
    }
}

fn bb_simple_perror_msg_and_die_rs(msg: *const c_char) {
    unsafe {
        bb_simple_perror_msg_and_die(msg);
    }
}

fn xfopen_rs(path: *const c_char, mode: *const c_char) -> *mut bindings::FILE {
    unsafe { xfopen(path, mode) }
}

fn free_rs(ptr: *mut c_void) {
    unsafe {
        free(ptr);
    }
}

fn awk_printf_rs(op1: *mut node) -> *mut c_char {
    unsafe { awk_printf(op1) }
}

fn get_op_l_progname(op: *mut node) -> *mut c_char {
    unsafe { (*op).l.new_progname }
}

fn xasprintf_rs(
    format: *const c_char,
    arg1: *const c_char,
    arg2: *const c_char,
    arg3: *const c_char,
) -> *mut c_char {
    unsafe { xasprintf(format, arg1, arg2, arg3) }
}

fn get_op_a_n_rs(op: *mut node) -> *mut node {
    unsafe { (*op).a.n }
}

fn get_op_r_n_info(op: *mut node) -> u32 {
    unsafe { (*(*op).r.n).info }
}

fn get_op_r_n_l_n(op: *mut node) -> *mut node {
    unsafe { (*(*op).r.n).l.n }
}
fn get_op_r_n_r_n(op: *mut node) -> *mut node {
    unsafe { (*(*op).r.n).r.n }
}

fn check_op_r_f_body_first_null(op: *mut node) -> bool {
    unsafe { (*(*op).r.f).body.first.is_null() }
}

fn split_f0_rs() {
    unsafe {
        split_f0();
    }
}

fn get_op_r_f_body_first(op: *mut node) -> *mut node {
    unsafe { (*(*op).r.f).body.first }
}

fn get_op_r_f_nargs(op: *mut node) -> i32 {
    unsafe { (*(*op).r.f).nargs.try_into().unwrap() }
}

fn offset_v_rs(p: *mut var, offset: isize) -> *mut var {
    unsafe { p.offset(offset) }
}

fn offset_v_from_rs(p: *mut var, offset: *mut var) -> isize {
    unsafe { p.offset_from(offset) }
}

fn nvfree_rs(v: *mut var) {
    unsafe {
        nvfree(v);
    }
}

fn istrue_rs(v: *mut var) -> i32 {
    unsafe { istrue(v) }
}

fn strcasecmp_rs(s1: *const c_char, s2: *const c_char) -> i32 {
    unsafe { strcasecmp(s1, s2) }
}

fn strcmp_rs(s1: *const c_char, s2: *const c_char) -> i32 {
    unsafe { strcmp(s1, s2) }
}

fn get_op_rn(op: *mut node) -> *mut node {
    unsafe { (*op).r.n }
}

fn is_numeric_rs(v: *mut var) -> i32 {
    unsafe { is_numeric(v) }
}

fn fclose_rs_stream(stream: *mut bindings::FILE) -> i32 {
    unsafe { fclose(stream) }
}

fn pclose_rs_stream(stream: *mut bindings::FILE) -> i32 {
    unsafe { pclose(stream) }
}

fn fsrealloc_rs(n: i32) {
    unsafe {
        fsrealloc(n);
    }
}

fn get_v_ptr_rs(v: *mut var) -> var_s {
    unsafe { *v }
}

fn wrapping_add_rs(i: usize) -> *mut var {
    let globs = getGlobals();
    globs.Fields.wrapping_add(i)
}

fn get_error_msg_enum_rs(s: &str) -> *const c_char {
    unsafe {
        match s {
            "EMSG_NO_MATH" => EMSG_NO_MATH.as_ptr(),
            "EMSG_DIV_BY_ZERO" => EMSG_DIV_BY_ZERO.as_ptr(),
            "EMSG_NEGATIVE_FIELD" => EMSG_NEGATIVE_FIELD.as_ptr(),
            "EMSG_UNDEF_FUNC" => EMSG_UNDEF_FUNC.as_ptr(),
            "EMSG_POSSIBLE_ERROR" => EMSG_POSSIBLE_ERROR.as_ptr(),
            "EMSG_NOT_ARRAY" => EMSG_NOT_ARRAY.as_ptr(),
            _ => std::ptr::null(),
        }
    }
}

fn srand_rs(seed: u32) {
    unsafe {
        srand(seed);
    }
}

fn get_null_mut_i64() -> *mut i64 {
    unsafe { std::ptr::null_mut() }
}

fn time_rs(t: *mut i64) -> i64 {
    unsafe { time(t) }
}

fn rand_rs() -> i32 {
    unsafe { rand() }
}

fn regexec_rs_i8(
    re: *mut libc::regex_t,
    s: *const i8,
    nmatch: usize,
    pmatch: *mut libc::regmatch_t,
    eflags: i32,
) -> i32 {
    unsafe { libc::regexec(re, s, nmatch, pmatch, eflags) }
}

fn regfree_rs(re: *mut bindings::regex_t) {
    unsafe {
        regfree(re);
    }
}

fn exec_builtin_rs(op: *mut node, res: *mut var) -> *mut var {
    unsafe { exec_builtin(op, res) }
}

fn fopen_for_read_rs(path: *const c_char) -> *mut bindings::FILE {
    unsafe { fopen_for_read(path) }
}

fn popen_rs(command: *const c_char, mode: *const c_char) -> *mut bindings::FILE {
    unsafe { popen(command, mode) }
}

fn get_stdout() -> *mut bindings::FILE {
    unsafe { stdout }
}

fn fflush_rs(stream: *mut bindings::FILE) {
    unsafe {
        fflush(stream);
    }
}

fn strlen_rs(s: *const c_char) -> usize {
    unsafe { strlen(s) }
}

fn fflush_rs_stdout() {
    unsafe {
        fflush(stdout);
    }
}

fn fflush_rs_stream(stream: *mut bindings::FILE) {
    unsafe {
        fflush(stream);
    }
}

extern "C" {
    fn xasprintf(
        format: *const c_char,
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: *const c_char,
    ) -> *mut c_char;
}

pub fn evaluate_rs(mut op: *mut node, mut res: *mut var) -> *mut var {
    if op.is_null() {
        return setvar_s_rs(res, null());
    }

    let globs2 = getGlobals2Ptr();
    let globs = getGlobals();

    let v1 = nvalloc_rs(2);

    while !op.is_null() {
        let opinfo = get_op_info_rs(op);
        let opn = opinfo & OPNMASK;
        globs.g_lineno = get_opt_lineno_rs(op);
        let mut op1 = get_op_l_n_rs(op);

        if (opinfo & OPCLSMASK) == OC_DELETE {
            let info = set_op_info_and(op1, OPCLSMASK);
            let mut v: *mut var = null_mut();

            if info == OC_VAR {
                v = get_op_l_v_rs(op1);
            } else if info == OC_FNARG {
                let idx = get_op_l_aidx_rs(op1);
                v = get_fnargs_rs(idx as usize);
            } else {
                syntax_error_rs_v2(get_error_msg_enum_rs("EMSG_NOT_ARRAY"));
            }

            if !check_op_r_n_null(op1) {
                let s = get_var_s_rs(evaluate_rs(get_op_r_n_rs(op1), v1));
                hash_remove_rs(iamarray_rs(v), s);
            } else {
                clear_array_rs(iamarray_rs(v));
            }

            if ((opinfo & OPCLSMASK) <= SHIFT_TIL_THIS) {
                op = get_op_a_n_rs(op);
            }
            if ((opinfo & OPCLSMASK) >= RECUR_FROM_THIS) {
                break;
            }
            if (globs.nextrec == 1) {
                break;
            }

            continue;
        }

        let mut Lv = null_mut();
        let mut Rv = null_mut();
        let mut Ls = null();
        let mut Rs = null();
        let mut L_d = 0.0;
        let mut R_d = 0.0;

        if opinfo & OF_RES1 != 0 {
            Lv = evaluate_rs(op1, v1);
        }

        // Evaluate right-hand side
        if opinfo & OF_RES2 != 0 {
            let operand = get_op_r_n_rs(op);
            Rv = evaluate_rs(operand, offset_v_rs(v1, 1));
        }

        if opinfo & OF_STR1 != 0 {
            Ls = get_var_s_rs(Lv);
        }

        if opinfo & OF_STR2 != 0 {
            Rs = get_var_s_rs(Rv);
        }

        if opinfo & OF_NUM1 != 0 {
            L_d = get_var_i_rs(Lv);
        }

        match opinfo & OPCLSMASK {
            OC_TEST => {
                if get_info_and(op, OPCLSMASK) == OC_COMMA {
                    if (opinfo & OF_CHECKED) == 1 || ptest_rs_l_n(op1) == 1 {
                        set_op_info_or(op1, OF_CHECKED);
                        if ptest_rs_r_n(op) == 1 {
                            set_op_info_and(op, !OF_CHECKED);
                        }
                        op = get_op_a_n_rs(op);
                    } else {
                        op = get_op_r_n_rs(op);
                    }
                } else {
                    op = if ptest_rs_op(op1) == 1 {
                        get_op_a_n_rs(op)
                    } else {
                        get_op_r_n_rs(op)
                    };
                }
            }
            OC_EXEC => {}
            OC_BR => {
                op = if istrue_rs(Lv) == 1 {
                    get_op_a_n_rs(op)
                } else {
                    get_op_r_n_rs(op)
                };
            }
            OC_WALKINIT => {
                let tmp = iamarray_rs(Rv);
                hashwalk_init_rs(Lv, tmp);
            }
            OC_WALKNEXT => {
                op = if hashwalk_next_rs(Lv) == 1 {
                    get_op_a_n_rs(op)
                } else {
                    get_op_r_n_rs(op)
                };
            }
            OC_PRINT | OC_PRINTF => {
                let mut F = get_stdout();
                if !check_op_r_n_null(op) {
                    let rsm = hash_find_rs(globs.fdhash, Rs) as *mut rstream;
                    let mut rsm_ptr = getRsmPtr(rsm);
                    if rsm_ptr.F.is_null() {
                        if opn == ('|' as i32).try_into().unwrap() {
                            rsm_ptr.F = popen_rs(Rs, b"w\0".as_ptr() as *const c_char);
                            if rsm_ptr.F.is_null() {
                                bb_simple_perror_msg_and_die_rs(
                                    b"popen\0".as_ptr() as *const c_char
                                );
                            }
                            rsm_ptr.is_pipe = 1;
                        } else {
                            rsm_ptr.F = xfopen_rs(
                                Rs,
                                if opn == ('w' as i32).try_into().unwrap() {
                                    b"w\0"
                                } else {
                                    b"a\0"
                                }
                                .as_ptr() as *const c_char,
                            );
                        }
                    }
                    F = rsm_ptr.F;
                }
                if (opinfo & OPCLSMASK) == OC_PRINT {
                    if op1.is_null() {
                        fputs_rs(get_var_s_rs(globs2.intvar[F0 as usize]), F);
                    } else {
                        while !op1.is_null() {
                            let v = evaluate_rs(nextarg_rs(&mut op1), v1);
                            let vptr = get_v_ptr_rs(v);
                            if vptr.type_ & VF_NUMBER != 0 {
                                fmt_num_rs(
                                    globs.g_buf,
                                    MAXVARFMT.try_into().unwrap(),
                                    get_var_s_rs(globs2.intvar[OFMT as usize]),
                                    get_var_i_rs(v),
                                    1,
                                );
                                fputs_rs(globs.g_buf, F);
                            } else {
                                fputs_rs(get_var_s_rs(v), F);
                            }
                            if !op1.is_null() {
                                fputs_rs(get_var_s_rs(globs2.intvar[OFS as usize]), F);
                            }
                        }
                    }
                    fputs_rs(get_var_s_rs(globs2.intvar[ORS as usize]), F);
                } else {
                    let s = awk_printf_rs(op1);
                    fputs_rs(s, F);
                    free_rs(s as *mut c_void);
                }
                fflush_rs(F);
            }
            OC_NEWSOURCE => {
                globs.g_progname = get_op_l_progname(op);
            }
            OC_RETURN => {
                copyvar_rs(res, Lv);
            }
            OC_NEXTFILE | OC_NEXT | OC_DONE => {
                clrvar_rs(res);
                if (opinfo & OPCLSMASK == OC_NEXTFILE) {
                    globs.nextfile = 1;
                }
                if (opinfo & OPCLSMASK == OC_NEXT) {
                    globs.nextrec = 1;
                }
            }
            OC_EXIT => {
                awk_exit_rs(L_d as i32);
            }
            OC_VAR => {
                Lv = get_op_l_v_rs(op);
                if Lv == globs2.intvar[NF as usize] {
                    split_f0_rs();
                }

                res = if !check_op_r_n_null(op) {
                    hash_find_rs(iamarray_rs(Lv), Rs) as *mut var_s
                } else {
                    Lv
                };
            }
            OC_FNARG => {
                let idx = get_op_l_aidx_rs(op);
                Lv = get_fnargs_rs(idx as usize);
                res = if !check_op_r_n_null(op) {
                    hash_find_rs(iamarray_rs(Lv), Rs) as *mut var_s
                } else {
                    Lv
                };
            }
            OC_IN => {
                if hash_search_rs(iamarray_rs(Lv), Rs) != std::ptr::null_mut() {
                    setvar_i_rs(res, 1.0);
                } else {
                    setvar_i_rs(res, 0.0);
                }
            }
            OC_REGEXP => {
                op1 = op;
                Ls = get_var_s_rs(globs2.intvar[F0 as usize]);
                let re_bindings =
                    as_regex_rs(op1, &mut globs2.evaluate__sreg) as *mut bindings::regex_t;

                // Cast the pointer from bindings::regex_t to libc::regex_t
                let re_libc = re_bindings as *mut libc::regex_t;

                // Call regexec with the libc regex_t
                let i = regexec_rs_i8(re_libc, Ls, 0, null_mut(), 0);

                // Compare pointers within the bindings::regex_t type space
                if re_bindings == &mut globs2.evaluate__sreg as *mut bindings::regex_t {
                    regfree_rs(re_bindings); // Use the appropriate type for regfree if it's also from bindings
                }

                setvar_i_rs(res, ((i == 0) as i32 ^ (opn == '!' as u32) as i32).into());
            }
            OC_MATCH => {
                op1 = get_op_rn(op);

                let re_bindings =
                    as_regex_rs(op1, &mut globs2.evaluate__sreg) as *mut bindings::regex_t;
                let re_libc = re_bindings as *mut libc::regex_t;
                let i = regexec_rs_i8(re_libc, Ls, 0, null_mut(), 0);
                if re_bindings == &mut globs2.evaluate__sreg as *mut bindings::regex_t {
                    regfree_rs(re_bindings); // Use the appropriate type for regfree if it's also from bindings
                }
                setvar_i_rs(res, ((i == 0) as i32 ^ (opn == '!' as u32) as i32).into());
            }
            OC_MOVE => {
                res = copyvar_rs(Lv, Rv);
            }
            OC_TERNARY => {
                if get_op_r_n_info(op) & OPCLSMASK != OC_COLON {
                    syntax_error_rs_v2(get_error_msg_enum_rs("EMSG_POSSIBLE_ERROR"));
                }
                res = evaluate_rs(
                    if istrue_rs(Lv) == 1 {
                        get_op_r_n_l_n(op)
                    } else {
                        get_op_r_n_r_n(op)
                    },
                    res,
                );
            }
            OC_FUNC => {
                if get_op_r_n_info(op) == 0 && !check_op_r_f_body_first_null(op) {
                    syntax_error_rs_v2(get_error_msg_enum_rs("EMSG_UNDEF_FUNC"));
                }

                let vbeg = nvalloc_rs((get_op_r_f_nargs(op) + 1).try_into().unwrap());
                let mut v = vbeg;
                let mut v_ptr = get_v_ptr_rs(v);
                while !op1.is_null() {
                    let arg = evaluate_rs(nextarg_rs(&mut op1), v1);
                    copyvar_rs(v, arg);
                    v_ptr.type_ |= VF_CHILD;
                    v_ptr.x.parent = arg;
                    v = offset_v_rs(v, 1);
                    if offset_v_from_rs(v, vbeg) >= get_op_r_f_nargs(op) as isize {
                        break;
                    }
                }
                let sv_fnargs = globs2.evaluate__fnargs;
                globs2.evaluate__fnargs = vbeg;
                let sv_progname = globs.g_progname;
                res = evaluate_rs(get_op_r_f_body_first(op), res);
                globs.g_progname = sv_progname;
                nvfree_rs(globs2.evaluate__fnargs);
                globs2.evaluate__fnargs = sv_fnargs;
            }
            OC_GETLINE | OC_PGETLINE => {
                let mut rsm = if !op1.is_null() {
                    hash_find_rs(globs.fdhash, Ls) as *mut rstream
                } else {
                    if globs.iF.is_null() {
                        globs.iF = next_input_file_wrapper();
                    }
                    globs.iF
                };

                let mut rsm_ptr = getRsmPtr(rsm);
                if rsm.is_null() || rsm_ptr.F.is_null() {
                    if rsm_ptr.F.is_null() {
                        if opinfo & OPCLSMASK == OC_PGETLINE {
                            rsm_ptr.F = popen_rs(Ls, CString::new("r\0").unwrap().as_ptr());
                            rsm_ptr.is_pipe = 1;
                        } else {
                            rsm_ptr.F = fopen_for_read_rs(Ls);
                        }
                    }
                }
                if rsm_ptr.F.is_null() {
                    setvar_i_rs(globs2.intvar[ERRNO as usize], FS.into());
                    setvar_i_rs(res, -1.0);
                } else {
                    let Rv = if check_op_r_n_null(op) {
                        globs2.intvar[F0 as usize]
                    } else {
                        Rv
                    };
                    let i = awk_getline_rs(rsm, Rv);
                    if i > 0 && op1.is_null() {
                        incvar_rs(globs2.intvar[FNR as usize]);
                        incvar_rs(globs2.intvar[NR as usize]);
                    }
                    setvar_i_rs(res, i as f64);
                }
            }
            OC_FBLTIN => {
                let mut R_d = R_d;
                match opn {
                    F_in => R_d = L_d as f64,
                    F_rn => R_d = (rand_rs() as f64) / (RAND_MAX as f64),
                    F_co => {
                        if ENABLE_FEATURE_AWK_LIBM == 1 {
                            R_d = L_d.cos();
                        }
                    }
                    F_ex => {
                        if ENABLE_FEATURE_AWK_LIBM == 1 {
                            R_d = L_d.exp();
                        }
                    }
                    F_lg => {
                        if ENABLE_FEATURE_AWK_LIBM == 1 {
                            R_d = L_d.ln();
                        }
                    }
                    F_si => {
                        if ENABLE_FEATURE_AWK_LIBM == 1 {
                            R_d = L_d.sin();
                        }
                    }
                    F_sq => {
                        if ENABLE_FEATURE_AWK_LIBM == 1 {
                            R_d = L_d.sqrt();
                        } else {
                            syntax_error_rs_v2(get_error_msg_enum_rs("EMSG_NO_MATH"));
                        }
                    }
                    F_sr => {
                        R_d = globs2.evaluate__seed as f64;
                        globs2.evaluate__seed = if !op1.is_null() {
                            L_d as u32
                        } else {
                            time_rs(get_null_mut_i64()) as u32
                        };
                        srand_rs(globs2.evaluate__seed);
                    }
                    F_ti => {
                        R_d = time_rs(get_null_mut_i64()) as f64;
                    }
                    F_le => {
                        if op1.is_null() {
                            Ls = get_var_s_rs(globs2.intvar[F0 as usize]);
                            R_d = strlen_rs(Ls) as f64;
                        } else if get_lv_ptr_rs(Lv).type_ & VF_ARRAY != 0 {
                            R_d = get_lv_x_array_nel(Lv) as f64;
                        } else {
                            R_d = strlen_rs(Ls) as f64;
                        }
                    }
                    F_sy => {
                        fflush_all_rs();
                        R_d = if ENABLE_FEATURE_ALLOW_EXEC == 1 && !Ls.is_null() {
                            system_rs(Ls) as f64 / 256.0
                        } else {
                            0.0
                        };
                    }
                    F_ff => {
                        if op1.is_null() {
                            fflush_rs_stdout();
                        } else if !Ls.is_null() {
                            let rsm = hash_find_rs(globs.fdhash, Ls) as *mut rstream;
                            let rsm_ptr = getRsmPtr(rsm);
                            fflush_rs_stream(rsm_ptr.F);
                        } else {
                            fflush_all_rs();
                        }
                    }
                    F_cl => {
                        let rsm = hash_search_rs(globs.fdhash, Ls) as *mut rstream;
                        let mut err = 0;
                        let rsm_ptr = getRsmPtr(rsm);
                        if !rsm.is_null() {
                            if !rsm_ptr.F.is_null() {
                                err = if rsm_ptr.is_pipe == 1 {
                                    pclose_rs_stream(rsm_ptr.F)
                                } else {
                                    fclose_rs_stream(rsm_ptr.F)
                                };
                                free_rs(rsm_ptr.buffer as *mut c_void);
                                hash_remove_rs(globs.fdhash, Ls);
                            }
                        }
                        if err != 0 {
                            setvar_i_rs(globs2.intvar[ERRNO as usize], ERRNO as f64);
                        }
                        R_d = err as f64;
                    }
                    _ => {}
                }
                setvar_i_rs(res, R_d);
            }
            OC_BUILTIN => {
                res = exec_builtin_rs(op, res);
            }
            OC_SPRINTF => {
                setvar_p_rs(res, awk_printf_rs(op1));
            }
            OC_UNARY => {
                let mut Ld = get_var_i_rs(Rv);
                let mut R_d = get_var_i_rs(Rv);
                match char::from_u32(opn).unwrap() {
                    'P' => {
                        R_d += 1.0;
                        Ld = R_d;
                    }
                    'p' => R_d += 1.0,
                    'M' => {
                        R_d -= 1.0;
                        Ld = R_d;
                    }
                    'm' => R_d -= 1.0,
                    '!' => Ld = if istrue_rs(Rv) == 1 { 0.0 } else { 1.0 },
                    '-' => Ld = -R_d,
                    _ => {}
                }
                setvar_i_rs(Rv, R_d);
                setvar_i_rs(res, Ld);
            }
            OC_FIELD => {
                let i = get_var_i_rs(Rv) as i32;
                if i < 0 {
                    syntax_error_rs_v2(get_error_msg_enum_rs("EMSG_NEGATIVE_FIELD"));
                } else if i == 0 {
                    res = globs2.intvar[F0 as usize];
                } else {
                    split_f0_rs();
                    if i as usize > globs.nfields.try_into().unwrap() {
                        fsrealloc_rs((i as usize).try_into().unwrap());
                    }
                    res = wrapping_add_rs((i - 1) as usize);
                }
            }
            OC_CONCAT | OC_COMMA => {
                let sep = if opinfo & OPCLSMASK == OC_COMMA {
                    get_var_s_rs(globs2.intvar[SUBSEP as usize])
                } else {
                    b"\0".as_ptr() as *const c_char
                };
                let format_str = CString::new("%s%s%s").unwrap();
                let concatenated = xasprintf_rs(format_str.as_ptr(), Ls, sep, Rs);
                setvar_p_rs(res, concatenated);
            }
            OC_LAND => {
                let oprn = get_op_rn(op);
                setvar_i_rs(
                    res,
                    if istrue_rs(Lv) == 1 {
                        (ptest_rs_r_n(oprn) as i32).into()
                    } else {
                        0.0
                    },
                );
            }
            OC_LOR => {
                let oprn = get_op_rn(op);
                setvar_i_rs(
                    res,
                    if istrue_rs(Lv) == 1 {
                        1.0
                    } else {
                        (ptest_rs_r_n(oprn) as i32).into()
                    },
                );
            }
            OC_BINARY | OC_REPLACE => {
                let R_d = get_var_i_rs(Rv);
                let mut L_d = get_var_i_rs(Lv);
                match opn as u8 as char {
                    '+' => L_d += R_d,
                    '-' => L_d -= R_d,
                    '*' => L_d *= R_d,
                    '/' => {
                        if R_d == 0.0 {
                            syntax_error_rs_v2(get_error_msg_enum_rs("EMSG_DIV_BY_ZERO"));
                        } else {
                            L_d /= R_d;
                        }
                    }
                    '&' => {
                        if ENABLE_FEATURE_AWK_LIBM == 1 {
                            L_d = L_d.powf(R_d);
                        } else {
                            syntax_error_rs_v2(get_error_msg_enum_rs("EMSG_NO_MATH"));
                        }
                    }
                    '%' => {
                        if R_d == 0.0 {
                            syntax_error_rs_v2(get_error_msg_enum_rs("EMSG_DIV_BY_ZERO"));
                        } else {
                            L_d -= (L_d / R_d).floor() * R_d;
                        }
                    }
                    _ => {}
                }
                setvar_i_rs(res, L_d);
            }
            OC_COMPARE => {
                let mut i = 0;
                let Ld = if is_numeric_rs(Lv) == 1 && is_numeric_rs(Rv) == 1 {
                    get_var_i_rs(Lv) - get_var_i_rs(Rv)
                } else {
                    let l = get_var_s_rs(Lv);
                    let r = get_var_s_rs(Rv);
                    if globs.icase == 1 {
                        strcasecmp_rs(l, r) as f64
                    } else {
                        strcmp_rs(l, r) as f64
                    }
                };
                match opn & 0xfe {
                    0 => i = (Ld > 0.0) as i32,
                    2 => i = (Ld >= 0.0) as i32,
                    4 => i = (Ld == 0.0) as i32,
                    _ => {}
                }
                setvar_i_rs(res, ((i == 0) as i32 ^ (opn & 1) as i32).into());
            }
            _ => {
                syntax_error_rs_v2(get_error_msg_enum_rs("EMSG_POSSIBLE_ERROR"));
            }
        }

        if ((opinfo & OPCLSMASK) <= SHIFT_TIL_THIS) {
            op = get_op_a_n_rs(op);
        }
        if ((opinfo & OPCLSMASK) >= RECUR_FROM_THIS) {
            break;
        }
        if (globs.nextrec == 1) {
            break;
        }
    }

    nvfree_rs(v1);
    res
}

fn awk_getline_rs(rsm: *mut rstream, v: *mut var) -> i32 {
    let mut b: *mut u8; // Changed from *mut char to *mut u8
    let mut pmatch: [regmatch_t; 2] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 2];
    let mut size: i32;
    let mut a: i32;
    let mut p: i32;
    let mut pp: i32 = 0;
    let fd: RawFd;
    let mut so: i32;
    let mut eo: i32;
    let mut r: i32;
    let mut rp: i32 = 0;
    let mut c: u8; // Changed from char to u8
    let mut m: *mut u8; // Changed from *mut char to *mut u8
    let mut s: *const u8; // Changed from *const char to *const u8

    let globs2 = getGlobals2Ptr();

    let rsm_ptr = getRsmPtr(rsm);

    fd = file_no_rs(rsm);
    m = rsm_ptr.buffer as *mut u8;
    a = rsm_ptr.adv;
    p = rsm_ptr.pos;
    size = rsm_ptr.size;
    c = globs2.rsplitter.n.info as u8; // Assuming info is a byte value

    if m.is_null() {
        m = qrealloc_rs(m as *mut i8, 256, &mut size);
    }

    loop {
        b = offset_rs(m, a as isize);
        eo = p;
        so = eo;
        r = 1;

        if p > 0 {
            if (globs2.rsplitter.n.info & OPCLSMASK) == OC_REGEXP {
                if regexec_rs(
                    (if globs2.rsplitter.n.info & 1 != 0 {
                        get_rsplitter_ire(globs2)
                    } else {
                        get_rsplitter_re(globs2)
                    }) as *const regex_t,
                    b as *const _,
                    1,
                    pmatch.as_mut_ptr(),
                    0,
                ) == 0
                {
                    so = pmatch[0].rm_so.try_into().unwrap();
                    eo = pmatch[0].rm_eo.try_into().unwrap();
                    if b_d_offset_rs(b, eo as isize) != 0 {
                        // Changed from '\0' to 0
                        break;
                    }
                }
            } else if c != 0 {
                // Changed from '\0' to 0

                s = strchr_rs(offset_rs(b, pp as isize), c);

                // s = strchr(b.offset(pp as isize) as *const i8, c as i32) as *const u8;
                if s.is_null() {
                    s = memchr_rs(
                        offset_rs(b, pp as isize) as *const c_void,
                        0,
                        (p - pp) as usize,
                    );
                }
                if !s.is_null() {
                    so = offset_from_rs(s, b);
                    eo = so + 1;
                    break;
                }
            } else {
                while b_d_offset_rs(b, eo as isize) == b'\n' {
                    // Changed from '\0' to 0
                    rp += 1;
                }

                s = strstr_rs(
                    b_offset_rs_const_i8(b, rp as isize),
                    "\n\n".as_ptr() as *const _,
                );

                // s = strstr(b_offset_rs_const_i8(b, rp as isize), "\n\n".as_ptr() as *const _) as *const u8;
                if !s.is_null() {
                    so = offset_from_rs(s, b);
                    eo = so;
                    while b_d_offset_rs(b, eo as isize) == b'\n' {
                        // Changed from '\0' to 0
                        eo += 1;
                    }

                    if b_d_offset_rs(b, eo as isize) == 0 {
                        // Changed from '\0' to 0
                        break;
                    }
                }
            }
        }

        if a > 0 {
            safe_ptr_copy(m, b, (p - a) as usize);
            b = m;
            a = 0;
        }

        m = qrealloc_rs(
            m as *mut i8,
            ((a + p + 128) as usize).try_into().unwrap(),
            &mut size,
        );
        b = offset_rs(m, a as isize);
        pp = p;
        p += safe_read_rs(fd, offset_rs_cvoid(b, p as isize), (size - p - 1) as usize) as i32;

        if (pp - p) == 0 {
            break;
        }

        if p < pp {
            p = 0;
            r = 0;
            // Assuming setvar_i_rs sets the error number, update appropriately
            setvar_i_rs(globs2.intvar[ERRNO as usize], safe_get_errno().into());
            break;
        }

        assign_b_rs(b, p);
    }

    if p == 0 {
        r -= 1;
    } else {
        c = b_d_offset_rs(b, so as isize);

        assign_b_rs(b, so);
        setvar_s_rs(v, b_offset_rs_const_i8(b, rp as isize));
        set_v_type_user(v);
        assign_b_rs(b, so);
        c = b_d_offset_rs(b, eo as isize);
        assign_b_rs(b, eo);
        setvar_s_rs(
            globs2.intvar[RT as usize],
            b_offset_rs_const_i8(b, so as isize),
        );
        assign_b_rs_v2(b, eo, c);
    }
    set_rsm_rs(rsm, m, a, p, size, eo);
    r
}

fn setvar_s_rs(v: *mut var, val: *const c_char) -> *mut var {
    let param = if val != std::ptr::null() {
        xstrdup_rs(val)
    } else {
        std::ptr::null()
    };
    return setvar_p_rs(v, param);
}

fn setvar_i_rs(v: *mut var, val: f64) {
    clrvar_rs(v);
    set_var_type_number(v);
    set_var_number(v, val);
    handle_special_wrapper(v);
}

fn awk_main_rs(argc: i32, mut argv: *mut *mut i8) -> i32 {
    initialization();

    let globals = getGlobals();
    let globals2 = getGlobals2Ptr();
    globals.ahash = hash_init_wrapper();
    globals.vhash = hash_init_wrapper();
    globals.fdhash = hash_init_wrapper();
    globals.fnhash = hash_init_wrapper();

    if ENABLE_LOCALE_SUPPORT == 1 {
        enable_locale_supp();
    }

    let mut opt: u32 = 0;
    let mut opt_F: *mut c_char;
    let mut list_v: *mut llist_t = null_mut();
    let mut list_f: *mut llist_t = null_mut();
    #[cfg(feature = "ENABLE_FEATURE_AWK_GNU_EXTENSIONS")]
    let mut list_e: *mut llist_t = null_mut();
    let mut vnames = getVNamesPtr();
    let mut vvalues = getVvaluesPtr();

    let mut i = 0;
    let mut j = 0;
    let mut tv: var_s = init_tv();
    let mut vnames_temp = vnames as *mut c_char;
    let mut vnames_mut = &mut vnames_temp as *mut *mut c_char;
    let mut vvalues_mut: *mut _ = vvalues as *mut _;
    let mut i = 0;
    while !vnames_mut.is_null() {
        if i >= globals2.intvar.len() {
            break;
        }
        let mut next_vname = get_next_vname(vnames_mut);

        let mut v = hash_find_wrapper_var_s(globals.vhash, next_vname);
        globals2.intvar[i] = v;

        if validVvalues_mut(&mut vvalues_mut) {
            let next_vvalue = nextword_rs(&mut vvalues_mut);
            setvar_s_rs(v, next_vvalue);
        } else {
            setvar_i_rs(v, 0.0);
        }

        check_special_char_and_update_v_type(v, vnames_mut);
        i += 1;
    }

    handle_special_wrapper(globals2.intvar[FS as usize] as *mut var);

    handle_special_wrapper(globals2.intvar[RS as usize] as *mut var);

    let stdin_file =
        hash_find_wrapper_rstream(globals.fdhash, CString::new("/dev/stdin").unwrap().as_ptr());
    let stdout_file = hash_find_wrapper_rstream(
        globals.fdhash,
        CString::new("/dev/stdout").unwrap().as_ptr(),
    );
    let stderr_file = hash_find_wrapper_rstream(
        globals.fdhash,
        CString::new("/dev/stderr").unwrap().as_ptr(),
    );

    set_stdin_stdout_stderr(stdin_file, stdout_file, stderr_file);

    for (key, value) in env::vars() {
        let key = CString::new(key).unwrap();
        let value = CString::new(value).unwrap();
        let xhash: *mut xhash_s = iamarray_rs(globals2.intvar[ENVIRON as usize]);
        let var_ptr = hash_find_wrapper_var(xhash, key.as_ptr()) as *mut var;
        setvar_u_rs(var_ptr, value.as_ptr());
    }

    let mut opt_F: *mut c_char = ptr::null_mut();

    #[cfg(feature = "ENABLE_FEATURE_AWK_GNU_EXTENSIONS")]
    let opt = getopt32_rsv2(argv, &mut opt_F, &mut list_v, &mut list_f, &mut list_e);
    #[cfg(not(feature = "ENABLE_FEATURE_AWK_GNU_EXTENSIONS"))]
    let opt = getopt32_rs(argv, &mut opt_F, &mut list_v, &mut list_f);
    let mut optind = getoptind_rs();
    argv = add_argv(argv, optind as usize);
    if opt & OPT_W != 0 {
        bb_simple_error_msg_rs(
            CString::new("warning: option -W is ignored")
                .unwrap()
                .as_ptr(),
        );
    }

    if opt & OPT_F != 0 {
        unescape_string_in_place_rs(opt_F);
        let fs_var = globals2.intvar[FS as usize] as *mut var;
        setvar_s_rs(fs_var, opt_F);
    }

    while !list_v.is_null() {
        if (is_assignment_rs(llist_pop_rs(&mut list_v) as *const i8) == 0) {
            bb_show_usage_rs();
        }
    }

    while !list_f.is_null() {
        let mut buffer = Vec::new();
        let g_progname = llist_pop_rs(&mut list_f);

        let from_file = xfopen_stdin_rs(g_progname as *const i8);

        loop {
            let current_len = buffer.len();
            buffer.resize(current_len + 4096, 0);
            let read_len = fread_rs(
                add_buffer_ptr(&mut buffer, current_len) as *mut libc::c_void,
                1,
                4094,
                from_file,
            );
            if read_len == 0 {
                buffer.truncate(current_len);
                buffer.push(0);
                break;
            }
        }

        fclose_rs(from_file);

        let c_str = as_ptr_rs(&buffer);
        parse_program_rs(c_str);
    }

    globals.g_progname = CString::new("cmd. line").unwrap().into_raw() as *mut c_char;

    let mut argvPtr = deference_rs(argv);
    #[cfg(feature = "ENABLE_FEATURE_AWK_GNU_EXTENSIONS")]
    while !list_e.is_null() {
        parse_program_rs(llist_pop_rs(list_e));
    }

    if (opt & (OPT_f | OPT_e)) == 0 {
        if argvPtr.is_null() {
            bb_show_usage_rs();
        }

        parse_program_rs(argvPtr);
        argv = add_argv(argv, 1);
    }

    setari_u_rs_default(globals2.intvar[ARGV as usize]);

    let mut i = 0;
    while !check_argv_null_rs(argv) {
        setari_u_rs(globals2.intvar[ARGV as usize], i + 1, argv);
        argv = add_argv(argv, 1);
        i += 1;
    }
    setvar_i_rs(globals2.intvar[ARGC as usize], (i + 1) as f64);
    evaluate_rs(globals.beginseq.first, &mut tv);
    // evaluate_wrapper(globals.beginseq.first, &mut tv);

    if globals.mainseq.first.is_null() && globals.endseq.first.is_null() {
        process::exit(EXIT_SUCCESS as i32);
    }

    if globals.iF == std::ptr::null_mut() {
        globals.iF = next_input_file_wrapper();
    }

    while !(globals.iF == std::ptr::null_mut()) {
        globals.nextfile = 0;
        setvar_i_rs(globals2.intvar[FNR as usize], 0.0);

        let mut i = awk_getline_rs(globals.iF, globals2.intvar[F0 as usize]);

        while (i > 0) {
            globals.nextrec = 0;
            incvar_rs(globals2.intvar[NR as usize]);
            incvar_rs(globals2.intvar[FNR as usize]);

            let firs = globals.mainseq.first as *mut node;

            let res = evaluate_rs(firs, &mut tv);
            // let res = evaluate_wrapper(firs, &mut tv);
            if globals.nextfile == 1 {
                break;
            }
            i = awk_getline_rs(globals.iF, globals2.intvar[F0 as usize]);
        }

        if i < 0 {
            syntax_error_rs();
        }
        globals.iF = next_input_file_wrapper();
    }
    awk_exit_rs(EXIT_SUCCESS);
    argc
}

fn main() {
    let argc = std::env::args().len() as i32;
    let argv: Vec<CString> = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    awk_main_rs(
        argc,
        argv.iter()
            .map(|arg| arg.as_ptr() as *mut i8)
            .collect::<Vec<*mut i8>>()
            .as_mut_ptr(),
    );
}
