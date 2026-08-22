/* Chap to C compiler runtime prelude.
 * This file is embedded into the binary and prepended to every generated C file. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdarg.h>
#include <ctype.h>

typedef enum { T_INT, T_FLOAT, T_BOOL, T_STR } Type;

typedef struct {
    Type t;
    union {
        long long i;
        double f;
        int b;
        char* s;
    };
} CV;

__attribute__((unused)) static CV cv_int(long long x) { CV v = {.t = T_INT, .i = x}; return v; }
__attribute__((unused)) static CV cv_flt(double x) { CV v = {.t = T_FLOAT, .f = x}; return v; }
__attribute__((unused)) static CV cv_bool(int x) { CV v = {.t = T_BOOL, .b = x}; return v; }
__attribute__((unused)) static CV cv_str_take(char* p) { CV v = {.t = T_STR, .s = p}; return v; }

__attribute__((unused)) static CV cv_str(const char* x) {
    char* p = malloc(strlen(x) + 1);
    strcpy(p, x);
    return cv_str_take(p);
}

/* chap prints floats like rust does, %g is close enough for simple values */
__attribute__((unused)) static char* cv_to_string(CV v) {
    char buf[64];
    switch (v.t) {
        case T_INT: sprintf(buf, "%lld", v.i); break;
        case T_FLOAT: sprintf(buf, "%g", v.f); break;
        case T_BOOL: strcpy(buf, v.b ? "true" : "false"); break;
        case T_STR: strcpy(buf, v.s); break;
        default: strcpy(buf, ""); break;
    }
    return strdup(buf);
}

__attribute__((unused)) static double cv_num(CV v) {
    return v.t == T_INT ? (double)v.i : v.f;
}

__attribute__((unused)) static int cv_eq(CV a, CV b) {
    if (a.t != b.t) return 0;
    switch (a.t) {
        case T_INT: return a.i == b.i;
        case T_FLOAT: return a.f == b.f;
        case T_BOOL: return a.b == b.b;
        case T_STR: return strcmp(a.s, b.s) == 0;
        default: return 0;
    }
}

__attribute__((unused)) static CV cv_mod(CV a, CV b) {
    if (a.t == T_FLOAT || b.t == T_FLOAT) return cv_flt(fmod(cv_num(a), cv_num(b)));
    return cv_int(a.i % b.i);
}

/* int op int stays int, anything with float becomes float (like chap) */
__attribute__((unused)) static CV cv_add(CV a, CV b) {
    if (a.t == T_INT && b.t == T_INT) return cv_int(a.i + b.i);
    return cv_flt(cv_num(a) + cv_num(b));
}

__attribute__((unused)) static CV cv_minus(CV a, CV b) {
    if (a.t == T_INT && b.t == T_INT) return cv_int(a.i - b.i);
    return cv_flt(cv_num(a) - cv_num(b));
}

__attribute__((unused)) static CV cv_multiply(CV a, CV b) {
    if (a.t == T_INT && b.t == T_INT) return cv_int(a.i * b.i);
    return cv_flt(cv_num(a) * cv_num(b));
}

/* chap divide always returns float, even for int / int */
__attribute__((unused)) static CV cv_divide(CV a, CV b) {
    return cv_flt(cv_num(a) / cv_num(b));
}

__attribute__((unused)) static CV cv_toint(CV v) {
    if (v.t == T_INT) return v;
    if (v.t == T_FLOAT) return cv_int((long long)v.f);
    return cv_int(strtoll(v.s, NULL, 10));
}

/* joins n args with sep */
__attribute__((unused)) static char* cv_join(int n, va_list ap, const char* sep) {
    size_t cap = 64, len = 0;
    char* out = malloc(cap);
    out[0] = '\0';
    for (int k = 0; k < n; k++) {
        CV v = va_arg(ap, CV);
        char* s = cv_to_string(v);
        size_t sl = strlen(s), sepl = k ? strlen(sep) : 0;
        size_t need = len + sl + sepl + 1;
        while (cap < need) { cap *= 2; out = realloc(out, cap); }
        if (k) strcat(out, sep);
        strcat(out, s);
        free(s);
        len += sl + sepl;
    }
    return out;
}

__attribute__((unused)) static void chap_print(int n, ...) {
    va_list ap;
    va_start(ap, n);
    char* joined = cv_join(n, ap, ", ");
    va_end(ap);
    printf("%s\n", joined);
    free(joined);
}

__attribute__((unused)) static CV cv_concat(int n, ...) {
    va_list ap;
    va_start(ap, n);
    char* joined = cv_join(n, ap, "");
    va_end(ap);
    return cv_str_take(joined);
}

__attribute__((unused)) static CV cv_and(int n, ...) {
    va_list ap;
    va_start(ap, n);
    int result = 1;
    for (int k = 0; k < n; k++) result = result && va_arg(ap, CV).b;
    va_end(ap);
    return cv_bool(result);
}

__attribute__((unused)) static CV cv_or(int n, ...) {
    va_list ap;
    va_start(ap, n);
    int result = 0;
    for (int k = 0; k < n; k++) result = result || va_arg(ap, CV).b;
    va_end(ap);
    return cv_bool(result);
}

__attribute__((unused)) static CV cv_xor(int n, ...) {
    va_list ap;
    va_start(ap, n);
    int result = 0;
    for (int k = 0; k < n; k++) result ^= va_arg(ap, CV).b;
    va_end(ap);
    return cv_bool(result);
}

/* reads one line from stdin and trims it like the chap interpreter does */
__attribute__((unused)) static CV cv_input(void) {
    char buf[4096];
    if (!fgets(buf, sizeof buf, stdin)) buf[0] = '\0';
    buf[strcspn(buf, "\n")] = '\0';
    char* start = buf;
    while (*start && isspace((unsigned char)*start)) start++;
    char* end = start + strlen(start);
    while (end > start && isspace((unsigned char)end[-1])) end--;
    *end = '\0';
    return cv_str(start);
}
