/* Chap to C compiler runtime prelude.
 * This file is embedded into the binary and prepended to every generated C file. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdarg.h>
#include <ctype.h>
#include <time.h>
#include <errno.h>
#include <sys/time.h>

typedef enum { T_INT, T_FLOAT, T_BOOL, T_STR, T_LIST, T_MAP } Type;

typedef struct CV CV;
typedef struct {
    CV* items;
    long long len, cap;
} CL;

typedef struct {
    char** keys;
    CV* vals;
    long long len, cap;
} CM;

struct CV {
    Type t;
    union {
        long long i;
        double f;
        int b;
        char* s;
        CL* l;
        CM* m;
    };
};

__attribute__((unused)) static CV cv_int(long long x) { CV v = {.t = T_INT, .i = x}; return v; }
__attribute__((unused)) static CV cv_flt(double x) { CV v = {.t = T_FLOAT, .f = x}; return v; }
__attribute__((unused)) static CV cv_bool(int x) { CV v = {.t = T_BOOL, .b = x}; return v; }
__attribute__((unused)) static CV cv_str_take(char* p) { CV v = {.t = T_STR, .s = p}; return v; }

__attribute__((unused)) static CV cv_str(const char* x) {
    char* p = malloc(strlen(x) + 1);
    strcpy(p, x);
    return cv_str_take(p);
}

/* chap runtime errors stop execution */
__attribute__((noreturn, unused)) static void chap_error(const char* msg) {
    fprintf(stderr, "runtime error: %s\n", msg);
    exit(1);
}


/* ---- string builder used for rendering containers ---- */

typedef struct {
    char* p;
    size_t len, cap;
} SB;

__attribute__((unused)) static void sb_putsn(SB* sb, const char* s, size_t n) {
    if (sb->len + n + 1 > sb->cap) {
        while (sb->len + n + 1 > sb->cap) sb->cap *= 2;
        sb->p = realloc(sb->p, sb->cap);
    }
    memcpy(sb->p + sb->len, s, n);
    sb->len += n;
    sb->p[sb->len] = '\0';
}

__attribute__((unused)) static void sb_puts(SB* sb, const char* s) { sb_putsn(sb, s, strlen(s)); }

/* rust never prints floats with exponent notation and prints the shortest
 * roundtrip representation, this mimics that closely enough */
__attribute__((unused)) static char* cv_fmt_flt(double x) {
    if (isnan(x)) return strdup("NaN");
    if (isinf(x)) return strdup(x < 0 ? "-inf" : "inf");
    char buf[640];
    int p;
    for (p = 1; p <= 17; p++) {
        sprintf(buf, "%.*g", p, x);
        if (strtod(buf, NULL) == x) break;
    }
    if (!strchr(buf, 'e') && !strchr(buf, 'E')) return strdup(buf);
    /* %g switched to exponent notation but rust does not do that */
    for (p = 1; p <= 400; p++) {
        sprintf(buf, "%.*f", p, x);
        if (strtod(buf, NULL) == x) break;
    }
    /* strip trailing zeros: 5.00 -> 5 */
    char* dot = strchr(buf, '.');
    if (dot) {
        char* end = buf + strlen(buf);
        while (end > dot && end[-1] == '0') end--;
        if (end > dot && end[-1] == '.') end--;
        *end = '\0';
    }
    return strdup(buf);
}

/* renders a value into sb, lists like "[1 hello]" and maps like
 * {"a": 1 "b": x} with sorted keys (same format as chap Display) */
static void cv_repr_into(SB* sb, CV v) {
    char tmp[64];
    switch (v.t) {
        case T_INT:
            sprintf(tmp, "%lld", v.i);
            sb_puts(sb, tmp);
            break;
        case T_FLOAT: {
            char* s = cv_fmt_flt(v.f);
            sb_puts(sb, s);
            free(s);
            break;
        }
        case T_BOOL:
            sb_puts(sb, v.b ? "true" : "false");
            break;
        case T_STR:
            sb_puts(sb, v.s);
            break;
        case T_LIST: {
            sb_puts(sb, "[");
            for (long long k = 0; k < v.l->len; k++) {
                if (k) sb_puts(sb, " ");
                cv_repr_into(sb, v.l->items[k]);
            }
            sb_puts(sb, "]");
            break;
        }
        case T_MAP: {
            /* sort key indexes like chap does before printing */
            const char** keys = malloc(sizeof(char*) * (size_t)(v.m->len > 0 ? v.m->len : 1));
            for (long long k = 0; k < v.m->len; k++) keys[k] = v.m->keys[k];
            for (long long a = 1; a < v.m->len; a++)
                for (long long b = a; b > 0 && strcmp(keys[b - 1], keys[b]) > 0; b--) {
                    const char* t = keys[b - 1];
                    keys[b - 1] = keys[b];
                    keys[b] = t;
                }
            sb_puts(sb, "{");
            for (long long k = 0; k < v.m->len; k++) {
                if (k) sb_puts(sb, " ");
                CV* val = NULL;
                for (long long j = 0; j < v.m->len; j++)
                    if (strcmp(v.m->keys[j], keys[k]) == 0) val = &v.m->vals[j];
                sb_puts(sb, "\"");
                sb_puts(sb, keys[k]);
                sb_puts(sb, "\": ");
                if (val) cv_repr_into(sb, *val);
            }
            free(keys);
            sb_puts(sb, "}");
            break;
        }
        default:
            break;
    }
}

/* chap prints floats like rust does */
__attribute__((unused)) static char* cv_to_string(CV v) {
    SB sb = {NULL, 0, 64};
    sb.p = malloc(64);
    sb.p[0] = '\0';
    cv_repr_into(&sb, v);
    return sb.p;
}


__attribute__((unused)) static CV cv_torepr(CV v) { return cv_str_take(cv_to_string(v)); }

__attribute__((unused)) static CV cv_typeof(CV v) {
    switch (v.t) {
        case T_INT: return cv_str("int");
        case T_FLOAT: return cv_str("float");
        case T_BOOL: return cv_str("boolean");
        case T_STR: return cv_str("string");
        case T_LIST: return cv_str("list");
        case T_MAP: return cv_str("map");
        default: return cv_str("unknown");
    }
}

__attribute__((unused)) static CV cv_tofloat(CV v) {
    if (v.t != T_STR) {
        char msg[128];
        sprintf(msg, "can not convert %s to float", cv_to_string(v));
        chap_error(msg);
    }
    char* end;
    double d = strtod(v.s, &end);
    if (end == v.s || *end != '\0') {
        char msg[160];
        sprintf(msg, "can not parse %s to float", v.s);
        chap_error(msg);
    }
    return cv_flt(d);
}

/* ---- lists and maps (memory leaks on purpose, like chap says: your OS will
 * free memory after the process is done!) ---- */

__attribute__((unused)) static CL* cv_list_new(void) {
    CL* l = malloc(sizeof(CL));
    l->len = 0;
    l->cap = 4;
    l->items = malloc(sizeof(CV) * (size_t)l->cap);
    return l;
}

__attribute__((unused)) static CM* cv_map_new(void) {
    CM* m = malloc(sizeof(CM));
    m->len = 0;
    m->cap = 4;
    m->keys = malloc(sizeof(char*) * (size_t)m->cap);
    m->vals = malloc(sizeof(CV) * (size_t)m->cap);
    return m;
}

/* linear key search, fine for chap sized programs */
__attribute__((unused)) static CV* cv_map_ref(CM* m, const char* key) {
    for (long long k = 0; k < m->len; k++)
        if (strcmp(m->keys[k], key) == 0) return &m->vals[k];
    return NULL;
}

/* deep copy, chap clones values on assignment */
__attribute__((unused)) static CV cv_copy(CV v) {
    switch (v.t) {
        case T_STR: {
            char* p = strdup(v.s);
            CV out = {.t = T_STR};
            out.s = p;
            return out;
        }
        case T_LIST: {
            CL* n = malloc(sizeof(CL));
            n->len = v.l->len;
            n->cap = v.l->len > 4 ? v.l->len : 4;
            n->items = malloc(sizeof(CV) * (size_t)n->cap);
            for (long long k = 0; k < n->len; k++) n->items[k] = cv_copy(v.l->items[k]);
            CV out = {.t = T_LIST};
            out.l = n;
            return out;
        }
        case T_MAP: {
            CM* n = malloc(sizeof(CM));
            n->len = v.m->len;
            n->cap = v.m->len > 4 ? v.m->len : 4;
            n->keys = malloc(sizeof(char*) * (size_t)n->cap);
            n->vals = malloc(sizeof(CV) * (size_t)n->cap);
            for (long long k = 0; k < n->len; k++) {
                n->keys[k] = strdup(v.m->keys[k]);
                n->vals[k] = cv_copy(v.m->vals[k]);
            }
            CV out = {.t = T_MAP};
            out.m = n;
            return out;
        }
        default:
            return v;
    }
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
        case T_LIST:
            if (a.l->len != b.l->len) return 0;
            for (long long k = 0; k < a.l->len; k++)
                if (!cv_eq(a.l->items[k], b.l->items[k])) return 0;
            return 1;
        case T_MAP:
            if (a.m->len != b.m->len) return 0;
            for (long long k = 0; k < a.m->len; k++) {
                CV* other = cv_map_ref(b.m, a.m->keys[k]);
                if (!other || !cv_eq(a.m->vals[k], *other)) return 0;
            }
            return 1;
        default: return 0;
    }
}

__attribute__((unused)) static CV cv_mod(CV a, CV b) {
    if (a.t == T_FLOAT || b.t == T_FLOAT) return cv_flt(fmod(cv_num(a), cv_num(b)));
    return cv_int(a.i % b.i);
}

/* chap power only works with int, int */
__attribute__((unused)) static CV cv_pow(CV a, CV b) {
    if (a.t != T_INT || b.t != T_INT) {
        chap_error("power function works only with numbers int and float");
    }
    long long result = 1;
    for (long long k = 0; k < b.i; k++) result *= a.i;
    return cv_int(result);
}

/* converts input to string and repeats it n times */
__attribute__((unused)) static CV cv_repeat(CV v, CV count) {
    if (count.t != T_INT) chap_error("repeat function second param should be int");
    char* s = cv_to_string(v);
    long long n = count.i < 0 ? 0 : count.i;
    size_t sl = strlen(s);
    char* out = malloc(sl * (size_t)n + 1);
    char* p = out;
    for (long long k = 0; k < n; k++) {
        memcpy(p, s, sl);
        p += sl;
    }
    *p = '\0';
    free(s);
    return cv_str_take(out);
}

__attribute__((unused)) static CV cv_length(CV v) {
    if (v.t == T_STR) return cv_int((long long)strlen(v.s));
    if (v.t == T_LIST) return cv_int(v.l->len);
    chap_error("length function input param should be string or list");
}

/* converts both inputs to string and checks substring */
__attribute__((unused)) static CV cv_contains(CV a, CV b) {
    char* s1 = cv_to_string(a);
    char* s2 = cv_to_string(b);
    int r = strstr(s1, s2) != NULL;
    free(s1);
    free(s2);
    return cv_bool(r);
}

/* "hello", 1, 3 -> slice -> "el" (rust panics on bad range, we clamp instead) */
__attribute__((unused)) static CV cv_slice(CV v, CV from, CV to) {
    if (v.t != T_STR || from.t != T_INT || to.t != T_INT)
        chap_error("slice function needs string, int, int params");
    long long len = (long long)strlen(v.s);
    long long f = from.i < 0 ? 0 : from.i;
    long long t = to.i > len ? len : to.i;
    if (f > t) f = t;
    char* out = malloc((size_t)(t - f) + 1);
    memcpy(out, v.s + f, (size_t)(t - f));
    out[t - f] = '\0';
    return cv_str_take(out);
}

/* chap char_at is 1-based */
__attribute__((unused)) static CV cv_char_at(CV v, CV index) {
    if (v.t != T_STR || index.t != T_INT)
        chap_error("char_at function requires a string as first parameter and an integer as second parameter");
    long long len = (long long)strlen(v.s);
    if (index.i < 1)
        chap_error("Index is invalid. Index must be 1 or greater (1-based indexing)");
    if (index.i > len) chap_error("Index is out of bounds for string");
    char* out = malloc(2);
    out[0] = v.s[index.i - 1];
    out[1] = '\0';
    return cv_str_take(out);
}

/* ascii only, rust does full unicode */
__attribute__((unused)) static CV cv_to_upper(CV v) {
    if (v.t != T_STR) chap_error("toupper function input param should be string");
    char* out = strdup(v.s);
    for (char* p = out; *p; p++) *p = (char)toupper((unsigned char)*p);
    return cv_str_take(out);
}

__attribute__((unused)) static CV cv_to_lower(CV v) {
    if (v.t != T_STR) chap_error("tolower function input param should be string");
    char* out = strdup(v.s);
    for (char* p = out; *p; p++) *p = (char)tolower((unsigned char)*p);
    return cv_str_take(out);
}

__attribute__((unused)) static CV cv_trim(CV v) {
    if (v.t != T_STR) chap_error("trim function input param should be string");
    const char* start = v.s;
    while (*start && isspace((unsigned char)*start)) start++;
    const char* end = v.s + strlen(v.s);
    while (end > start && isspace((unsigned char)end[-1])) end--;
    size_t len = (size_t)(end - start);
    char* out = malloc(len + 1);
    memcpy(out, start, len);
    out[len] = '\0';
    return cv_str_take(out);
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

/* folds cv_add over params starting from 0 like chap add_many does */
__attribute__((unused)) static CV cv_add_many(int n, ...) {
    va_list ap;
    va_start(ap, n);
    CV sum = cv_int(0);
    for (int k = 0; k < n; k++) sum = cv_add(sum, va_arg(ap, CV));
    va_end(ap);
    return sum;
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

/* unix time in seconds as float */
__attribute__((unused)) static CV cv_now(void) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return cv_flt((double)tv.tv_sec + (double)tv.tv_usec / 1000000.0);
}

__attribute__((unused)) static void chap_sleep(double seconds) {
    if (seconds <= 0) return;
    struct timespec ts;
    ts.tv_sec = (time_t)seconds;
    ts.tv_nsec = (long)((seconds - (double)ts.tv_sec) * 1e9);
    while (nanosleep(&ts, &ts) != 0 && errno == EINTR) {}
}

static void chap_wait_check(CV v, const char* name) {
    if (v.t != T_INT) {
        char msg[128];
        sprintf(msg, "function %s needs int param as input", name);
        chap_error(msg);
    }
}

#define CHAP_WAIT(fn_name, factor)                                          \
    __attribute__((unused)) static void fn_name(CV v) {                     \
        chap_wait_check(v, #fn_name);                                       \
        chap_sleep((double)v.i * (factor));                                 \
    }

CHAP_WAIT(chap_wait_millis, 1e-3)
/* note: chap multiplies hours by 60*24 which is a day, we copy that behavior */
CHAP_WAIT(chap_wait_seconds, 1)
CHAP_WAIT(chap_wait_minutes, 60)
CHAP_WAIT(chap_wait_hours, 60 * 24)

/* seeded once on first use */
__attribute__((unused)) static void chap_seed(void) {
    static int seeded = 0;
    if (!seeded) {
        srand((unsigned)time(NULL));
        seeded = 1;
    }
}

__attribute__((unused)) static CV cv_random_bool(void) {
    chap_seed();
    return cv_bool(rand() % 2);
}

__attribute__((unused)) static CV cv_random_number(CV lo, CV hi) {
    chap_seed();
    if (lo.t == T_INT && hi.t == T_INT) {
        if (hi.i <= lo.i) return cv_int(lo.i); /* rust panics on empty range */
        return cv_int(lo.i + rand() % (int)(hi.i - lo.i));
    }
    if (lo.t == T_FLOAT && hi.t == T_FLOAT) {
        double r = (double)rand() / ((double)RAND_MAX + 1.0);
        return cv_flt(hi.f <= lo.f ? lo.f : lo.f + (hi.f - lo.f) * r);
    }
    chap_error("random_number supports int,int or float,float in input");
}

__attribute__((unused)) static CV cv_random_string(CV alphabet, CV len) {
    if (alphabet.t != T_STR || len.t != T_INT)
        chap_error("random_string function needs string, int as param (first one is alphabet and second one is length of generated string) ");
    chap_seed();
    size_t alen = strlen(alphabet.s);
    long long n = len.i < 0 ? 0 : len.i;
    if (alen == 0) chap_error("random_string alphabet must not be empty");
    char* out = malloc((size_t)n + 1);
    for (long long k = 0; k < n; k++) out[k] = alphabet.s[rand() % alen];
    out[n] = '\0';
    return cv_str_take(out);
}

__attribute__((unused)) static CV cv_random_choice(int n, ...) {
    chap_seed();
    if (n < 2) chap_error("random_choice needs many input params");
    int pick = rand() % n;
    va_list ap;
    va_start(ap, n);
    CV result = cv_int(0);
    for (int k = 0; k < n; k++) {
        CV v = va_arg(ap, CV);
        if (k == pick) result = v;
    }
    va_end(ap);
    return result;
}

/* ---- collection functions ---- */

/* inserts a copy of v, like chap does */
__attribute__((unused)) static void cv_list_push(CL* l, CV v) {
    if (l->len == l->cap) {
        l->cap *= 2;
        l->items = realloc(l->items, sizeof(CV) * (size_t)l->cap);
    }
    l->items[l->len++] = cv_copy(v);
}

__attribute__((unused)) static void cv_map_set(CM* m, const char* key, CV v) {
    CV* existing = cv_map_ref(m, key);
    if (existing) {
        *existing = cv_copy(v);
        return;
    }
    if (m->len == m->cap) {
        m->cap *= 2;
        m->keys = realloc(m->keys, sizeof(char*) * (size_t)m->cap);
        m->vals = realloc(m->vals, sizeof(CV) * (size_t)m->cap);
    }
    m->keys[m->len] = strdup(key);
    m->vals[m->len++] = cv_copy(v);
}

__attribute__((unused)) static CV cv_list_lit(int n, ...) {
    CL* l = cv_list_new();
    va_list ap;
    va_start(ap, n);
    for (int k = 0; k < n; k++) cv_list_push(l, va_arg(ap, CV));
    va_end(ap);
    CV out = {.t = T_LIST};
    out.l = l;
    return out;
}

/* args alternate: key (char*), value (CV) */
__attribute__((unused)) static CV cv_map_lit(int n, ...) {
    CM* m = cv_map_new();
    va_list ap;
    va_start(ap, n);
    for (int k = 0; k < n; k++) {
        char* key = va_arg(ap, char*);
        CV v = va_arg(ap, CV);
        cv_map_set(m, key, v);
    }
    va_end(ap);
    CV out = {.t = T_MAP};
    out.m = m;
    return out;
}

__attribute__((noreturn, unused)) static void cv_wrong_get_form(void) {
    chap_error("correct form of 'get' function: <list | map>, <index | key> -> get -> $item");
}

/* list index is 1-based in chap */
__attribute__((unused)) static CV cv_get(CV c, CV key) {
    if (c.t == T_LIST) {
        if (key.t != T_INT) cv_wrong_get_form();
        if (key.i == 0) chap_error("list index starts from 1");
        if (key.i < 0) chap_error("negative index");
        if (key.i > c.l->len) chap_error("index out of bound");
        return cv_copy(c.l->items[key.i - 1]);
    }
    if (c.t == T_MAP) {
        if (key.t != T_STR) cv_wrong_get_form();
        CV* r = cv_map_ref(c.m, key.s);
        if (!r) chap_error("key not found");
        return cv_copy(*r);
    }
    cv_wrong_get_form();
}

__attribute__((unused)) static CV cv_has(CV c, CV item) {
    if (c.t == T_LIST) {
        for (long long k = 0; k < c.l->len; k++)
            if (cv_eq(c.l->items[k], item)) return cv_bool(1);
        return cv_bool(0);
    }
    if (c.t == T_MAP) {
        if (item.t != T_STR) chap_error("second param should be a string");
        return cv_bool(cv_map_ref(c.m, item.s) != NULL);
    }
    chap_error("has first param should be a list or map");
}

__attribute__((unused)) static CV cv_index_of(CV c, CV item) {
    if (c.t != T_LIST)
        chap_error("correct form of index_of function: <list>, <item> -> index_of");
    for (long long k = 0; k < c.l->len; k++)
        if (cv_eq(c.l->items[k], item)) return cv_int(k + 1);
    return cv_int(-1);
}

__attribute__((unused)) static CV cv_pop(CV* c) {
    if (c->t != T_LIST) chap_error("pop function first param should be a list");
    CL* l = c->l;
    if (l->len == 0) chap_error("list is empty");
    return l->items[--l->len];
}

__attribute__((unused)) static CV cv_last(CV c) {
    if (c.t != T_LIST) chap_error("last function first param should be a list");
    if (c.l->len == 0) chap_error("list is empty");
    return cv_copy(c.l->items[c.l->len - 1]);
}

__attribute__((unused)) static CV cv_remove_at(CV* c, CV index) {
    if (c->t != T_LIST || index.t != T_INT)
        chap_error("correct form of remove_at function: <list>, <index> -> remove_at -> $var");
    CL* l = c->l;
    if (index.i == 0) chap_error("list index starts from 1");
    if (index.i < 0) chap_error("remove_at function negative index");
    if (index.i > l->len) chap_error("index out of bound");
    CV removed = l->items[index.i - 1];
    memmove(l->items + index.i - 1, l->items + index.i,
            sizeof(CV) * (size_t)(l->len - index.i));
    l->len--;
    return removed;
}

/* works on both lists and maps like chap insert does */
__attribute__((unused)) static void cv_insert(CV* target, CV v) {
    if (target->t == T_LIST) {
        cv_list_push(target->l, v);
        return;
    }
    if (target->t == T_MAP) {
        if (v.t != T_MAP)
            chap_error("second param should be a map with one key-value pair");
        for (long long k = 0; k < v.m->len; k++)
            cv_map_set(target->m, v.m->keys[k], v.m->vals[k]);
        return;
    }
    chap_error("insert first param should be a list or map");
}

/* removes first equal item from list / removes key from map, ignores misses */
__attribute__((unused)) static void cv_remove_item(CV* target, CV item) {
    if (target->t == T_LIST) {
        for (long long k = 0; k < target->l->len; k++) {
            if (cv_eq(target->l->items[k], item)) {
                memmove(target->l->items + k, target->l->items + k + 1,
                        sizeof(CV) * (size_t)(target->l->len - k - 1));
                target->l->len--;
                return;
            }
        }
        return;
    }
    if (target->t == T_MAP) {
        if (item.t != T_STR) chap_error("second param should be a string");
        for (long long k = 0; k < target->m->len; k++) {
            if (strcmp(target->m->keys[k], item.s) == 0) {
                memmove(target->m->keys + k, target->m->keys + k + 1,
                        sizeof(char*) * (size_t)(target->m->len - k - 1));
                memmove(target->m->vals + k, target->m->vals + k + 1,
                        sizeof(CV) * (size_t)(target->m->len - k - 1));
                target->m->len--;
                return;
            }
        }
        return;
    }
    chap_error("correct form of remove_item function: <list | map>, <item | key> -> remove_item");
}

