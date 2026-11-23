/* ffmpeg's libavutil (n7.1, built by its own configure and make, -O3) against the Rust ports,
 * called from one C program. The ports are linked from one LTO staticlib built with the `shadow`
 * feature, so they export only `everything_rust_*` names beside libavutil's. Each row runs the
 * same calls on both sides until 200 ms pass, five times; the median is printed. */
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "libavutil/adler32.h"
#include "libavutil/avstring.h"
#include "libavutil/base64.h"
#include "libavutil/crc.h"
#include "libavutil/mathematics.h"
#include "libavutil/md5.h"
#include "libavutil/murmur3.h"
#include "libavutil/parseutils.h"
#include "libavutil/rational.h"
#include "libavutil/sha.h"

uint32_t everything_rust_av_adler32_update(uint32_t, const uint8_t *, size_t);
int everything_rust_av_base64_decode(uint8_t *, const char *, int);
char *everything_rust_av_base64_encode(char *, int, const uint8_t *, int);
uint32_t everything_rust_oneshot_crc(int, uint32_t, const uint8_t *, size_t);
int everything_rust_oneshot_md5(uint8_t *, int, const uint8_t *, size_t, int);
int everything_rust_oneshot_sha(uint8_t *, int, int, const uint8_t *, size_t, int);
int everything_rust_oneshot_murmur3(uint8_t *, int, uint64_t, const uint8_t *, size_t, int);
int64_t everything_rust_av_rescale_rnd(int64_t, int64_t, int64_t, int);
int64_t everything_rust_av_gcd(int64_t, int64_t);
double everything_rust_av_bessel_i0(double);
int everything_rust_av_reduce(int *, int *, int64_t, int64_t, int64_t);
int everything_rust_av_strcasecmp(const char *, const char *);
char *everything_rust_av_stristr(const char *, const char *);
int everything_rust_av_match_name(const char *, const char *);
int everything_rust_av_strstart(const char *, const char *, const char **);
int everything_rust_av_stristart(const char *, const char *, const char **);
AVRational everything_rust_av_mul_q(AVRational, AVRational);
AVRational everything_rust_av_add_q(AVRational, AVRational);
AVRational everything_rust_av_d2q(double, int);
int everything_rust_av_nearer_q(AVRational, AVRational, AVRational);
uint32_t everything_rust_av_q2intfloat(AVRational);
int64_t everything_rust_av_rescale_q(int64_t, AVRational, AVRational);
size_t everything_rust_av_strlcpy(char *, const char *, size_t);
int everything_rust_av_parse_video_size(int *, int *, const char *);
int everything_rust_av_find_info_tag(char *, int, const char *, const char *);

static volatile uint64_t sink;
static double now_ns(void)
{
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return t.tv_sec * 1e9 + t.tv_nsec;
}

typedef void (*pass_fn)(void);

/* Nanoseconds per pass: warm up, then repeat until 200 ms pass; the median of five runs. */
static double timed(pass_fn pass)
{
    double runs[5];
    pass();
    for (int r = 0; r < 5; r++) {
        long n = 0;
        double start = now_ns(), elapsed;
        do {
            pass();
            n++;
            elapsed = now_ns() - start;
        } while (elapsed < 200e6);
        runs[r] = elapsed / n;
    }
    for (int i = 0; i < 5; i++)
        for (int j = i + 1; j < 5; j++)
            if (runs[j] < runs[i]) {
                double t = runs[i];
                runs[i] = runs[j];
                runs[j] = t;
            }
    return runs[2];
}

static void row(const char *name, const char *unit, double per_pass_units, pass_fn c, pass_fn rust)
{
    double tc = timed(c), tr = timed(rust);
    if (!strcmp(unit, "MB/s"))
        printf("%-34s %10.0f %10.0f MB/s  %5.2fx\n", name, per_pass_units / tc * 1e3,
               per_pass_units / tr * 1e3, tc / tr);
    else
        printf("%-34s %10.1f %10.1f ns    %5.2fx\n", name, tc / per_pass_units,
               tr / per_pass_units, tc / tr);
    fflush(stdout);
}

#define MIB (1 << 20)
static uint8_t *data;
static size_t size;
static char *text;
static uint8_t out[2 * MIB];
static struct AVMD5 *md5;
static struct AVSHA *sha;
static struct AVMurMur3 *mm3;

static void c_adler(void) { sink += av_adler32_update(1, data, size); }
static void r_adler(void) { sink += everything_rust_av_adler32_update(1, data, size); }
static void c_crc(void) { sink += av_crc(av_crc_get_table(AV_CRC_32_IEEE_LE), 0, data, size); }
static void r_crc(void) { sink += everything_rust_oneshot_crc(AV_CRC_32_IEEE_LE, 0, data, size); }
static void c_md5(void) { av_md5_init(md5); av_md5_update(md5, data, size); av_md5_final(md5, out); sink += out[0]; }
static void r_md5(void) { everything_rust_oneshot_md5(out, 16, data, size, 0); sink += out[0]; }
static void c_sha1(void) { av_sha_init(sha, 160); av_sha_update(sha, data, size); av_sha_final(sha, out); sink += out[0]; }
static void r_sha1(void) { everything_rust_oneshot_sha(out, 32, 160, data, size, 0); sink += out[0]; }
static void c_sha256(void) { av_sha_init(sha, 256); av_sha_update(sha, data, size); av_sha_final(sha, out); sink += out[0]; }
static void r_sha256(void) { everything_rust_oneshot_sha(out, 32, 256, data, size, 0); sink += out[0]; }
static void c_mm3(void) { av_murmur3_init_seeded(mm3, 7); av_murmur3_update(mm3, data, size); av_murmur3_final(mm3, out); sink += out[0]; }
static void r_mm3(void) { everything_rust_oneshot_murmur3(out, 16, 7, data, size, 0); sink += out[0]; }
static void c_b64e(void) { sink += (uintptr_t)av_base64_encode((char *)out, sizeof(out), data, size); }
static void r_b64e(void) { sink += (uintptr_t)everything_rust_av_base64_encode((char *)out, sizeof(out), data, size); }
static void c_b64d(void) { sink += av_base64_decode(out, text, sizeof(out)); }
static void r_b64d(void) { sink += everything_rust_av_base64_decode(out, text, sizeof(out)); }

#define N_ARGS 4096
static int64_t ia[N_ARGS], ib[N_ARGS], ic[N_ARGS];
static double fx[N_ARGS];
static void c_rescale(void) { for (int i = 0; i < N_ARGS; i++) sink += av_rescale_rnd(ia[i], ib[i], ic[i], i % 6 == 4 ? 5 : i % 6); }
static void r_rescale(void) { for (int i = 0; i < N_ARGS; i++) sink += everything_rust_av_rescale_rnd(ia[i], ib[i], ic[i], i % 6 == 4 ? 5 : i % 6); }
static void c_gcd(void) { for (int i = 0; i < N_ARGS; i++) sink += av_gcd(ia[i], ib[i]); }
static void r_gcd(void) { for (int i = 0; i < N_ARGS; i++) sink += everything_rust_av_gcd(ia[i], ib[i]); }
static void c_reduce(void) { int n, d; for (int i = 0; i < N_ARGS; i++) { sink += av_reduce(&n, &d, ia[i], ib[i] | 1, 1 << 16) + n; } }
static void r_reduce(void) { int n, d; for (int i = 0; i < N_ARGS; i++) { sink += everything_rust_av_reduce(&n, &d, ia[i], ib[i] | 1, 1 << 16) + n; } }
static AVRational qa[N_ARGS], qb[N_ARGS], qc[N_ARGS];
static void c_mulq(void) { for (int i = 0; i < N_ARGS; i++) { AVRational r = av_mul_q(qa[i], qb[i]); sink += r.num + r.den; } }
static void r_mulq(void) { for (int i = 0; i < N_ARGS; i++) { AVRational r = everything_rust_av_mul_q(qa[i], qb[i]); sink += r.num + r.den; } }
static void c_addq(void) { for (int i = 0; i < N_ARGS; i++) { AVRational r = av_add_q(qa[i], qb[i]); sink += r.num + r.den; } }
static void r_addq(void) { for (int i = 0; i < N_ARGS; i++) { AVRational r = everything_rust_av_add_q(qa[i], qb[i]); sink += r.num + r.den; } }
static void c_d2q(void) { for (int i = 0; i < N_ARGS; i++) { AVRational r = av_d2q(fx[i], i & 1 ? 1 << 16 : INT32_MAX); sink += r.num + r.den; } }
static void r_d2q(void) { for (int i = 0; i < N_ARGS; i++) { AVRational r = everything_rust_av_d2q(fx[i], i & 1 ? 1 << 16 : INT32_MAX); sink += r.num + r.den; } }
static void c_nearer(void) { for (int i = 0; i < N_ARGS; i++) sink += av_nearer_q(qa[i], qb[i], qc[i]); }
static void r_nearer(void) { for (int i = 0; i < N_ARGS; i++) sink += everything_rust_av_nearer_q(qa[i], qb[i], qc[i]); }
static void c_q2if(void) { for (int i = 0; i < N_ARGS; i++) sink += av_q2intfloat(qa[i]); }
static void r_q2if(void) { for (int i = 0; i < N_ARGS; i++) sink += everything_rust_av_q2intfloat(qa[i]); }
static void c_rescaleq(void) { for (int i = 0; i < N_ARGS; i++) sink += av_rescale_q(ia[i], qa[i], qb[i]); }
static void r_rescaleq(void) { for (int i = 0; i < N_ARGS; i++) sink += everything_rust_av_rescale_q(ia[i], qa[i], qb[i]); }
static void c_bessel(void) { double s = 0; for (int i = 0; i < N_ARGS; i++) s += av_bessel_i0(fx[i]); sink += (uint64_t)s; }
static void r_bessel(void) { double s = 0; for (int i = 0; i < N_ARGS; i++) s += everything_rust_av_bessel_i0(fx[i]); sink += (uint64_t)s; }

static const char *const words[] = {
    "mp4", "MOV", "matroska", "webm", "h264", "HEVC", "aac", "Opus", "flac", "pcm_s16le",
    "Education consists mainly in what we have unlearned.", "hello world", "x", "",
};
#define N_WORDS (int)(sizeof(words) / sizeof(words[0]))
static const char *const names = "mov,mp4,m4a,3gp,3g2,mj2,matroska,webm,mpegts,flv,ogg";
static void c_casecmp(void) { for (int i = 0; i < N_WORDS; i++) for (int j = 0; j < N_WORDS; j++) sink += av_strcasecmp(words[i], words[j]); }
static void r_casecmp(void) { for (int i = 0; i < N_WORDS; i++) for (int j = 0; j < N_WORDS; j++) sink += everything_rust_av_strcasecmp(words[i], words[j]); }
static void c_stristr(void) { for (int i = 0; i < N_WORDS; i++) for (int j = 0; j < N_WORDS; j++) sink += (uintptr_t)av_stristr(words[i], words[j]); }
static void r_stristr(void) { for (int i = 0; i < N_WORDS; i++) for (int j = 0; j < N_WORDS; j++) sink += (uintptr_t)everything_rust_av_stristr(words[i], words[j]); }
static void c_match(void) { for (int i = 0; i < N_WORDS; i++) sink += av_match_name(words[i], names); }
static void r_match(void) { for (int i = 0; i < N_WORDS; i++) sink += everything_rust_av_match_name(words[i], names); }
static const char *const prefixes[] = { "mp4", "m", "", "Education", "hello", "MOV", "pcm_" };
#define N_PREFIXES (int)(sizeof(prefixes) / sizeof(prefixes[0]))
static void c_strstart(void) { const char *p; for (int i = 0; i < N_WORDS; i++) for (int j = 0; j < N_PREFIXES; j++) sink += av_strstart(words[i], prefixes[j], &p); }
static void r_strstart(void) { const char *p; for (int i = 0; i < N_WORDS; i++) for (int j = 0; j < N_PREFIXES; j++) sink += everything_rust_av_strstart(words[i], prefixes[j], &p); }
static void c_stristart(void) { const char *p; for (int i = 0; i < N_WORDS; i++) for (int j = 0; j < N_PREFIXES; j++) sink += av_stristart(words[i], prefixes[j], &p); }
static void r_stristart(void) { const char *p; for (int i = 0; i < N_WORDS; i++) for (int j = 0; j < N_PREFIXES; j++) sink += everything_rust_av_stristart(words[i], prefixes[j], &p); }
static void c_strlcpy(void) { for (int i = 0; i < N_WORDS; i++) sink += av_strlcpy((char *)out, words[i], 32); }
static void r_strlcpy(void) { for (int i = 0; i < N_WORDS; i++) sink += everything_rust_av_strlcpy((char *)out, words[i], 32); }

static const char *const sizes[] = { "hd1080", "1920x1080", "uhd4320", "640x480", "ntsc", "123x45foo", "4k", "352x288" };
#define N_SIZES (int)(sizeof(sizes) / sizeof(sizes[0]))
static void c_vsize(void) { int w, h; for (int i = 0; i < N_SIZES; i++) sink += av_parse_video_size(&w, &h, sizes[i]) + w; }
static void r_vsize(void) { int w, h; for (int i = 0; i < N_SIZES; i++) sink += everything_rust_av_parse_video_size(&w, &h, sizes[i]) + w; }
static const char *const info = "?tag1=val1&tag2=val2&tag3=val3&tag41=value 41&tag42=random1";
static const char *const tags[] = { "tag1", "tag3", "tag42", "nope" };
static void c_info(void) { char b[64]; for (int i = 0; i < 4; i++) sink += av_find_info_tag(b, sizeof(b), tags[i], info); }
static void r_info(void) { char b[64]; for (int i = 0; i < 4; i++) sink += everything_rust_av_find_info_tag(b, sizeof(b), tags[i], info); }

int main(void)
{
    static const size_t lengths[] = { 64, 4096, MIB };
    static const char *const labels[] = { "64 B", "4 KiB", "1 MiB" };
    uint64_t seed = 0x9E3779B97F4A7C15ull;
    data = malloc(MIB);
    text = malloc(2 * MIB);
    for (size_t i = 0; i < MIB; i++) {
        seed = seed * 6364136223846793005ull + 1442695040888963407ull;
        data[i] = seed >> 56;
    }
    for (int i = 0; i < N_ARGS; i++) {
        seed = seed * 6364136223846793005ull + 1442695040888963407ull;
        ia[i] = (int64_t)(seed >> (1 + (i % 40)));
        ib[i] = (int64_t)((seed * 31) >> (2 + (i % 50))) & INT64_MAX;
        ic[i] = ((int64_t)((seed * 17) >> (3 + (i % 45))) & INT64_MAX) | 1;
        fx[i] = (double)(seed >> 11) / (double)(1ull << 53) * 40.0;
        qa[i] = (AVRational){ (int)(seed >> (33 + i % 24)), (int)((seed >> 7) % 90000) + 1 };
        qb[i] = (AVRational){ (int)((seed * 13) >> (40 + i % 20)), (int)((seed >> 3) % 48000) + 1 };
        qc[i] = (AVRational){ (int)((seed * 7) >> (40 + i % 20)), (int)((seed >> 5) % 30000) + 1 };
    }
    md5 = av_md5_alloc();
    sha = av_sha_alloc();
    mm3 = av_murmur3_alloc();

    printf("%-34s %10s %10s       %s\n", "", "C", "Rust", "C time / Rust time");
    for (int l = 0; l < 3; l++) {
        char name[64];
        size = lengths[l];
        av_base64_encode(text, 2 * MIB, data, size);
#define ROW(label, c, r) snprintf(name, sizeof(name), "%s %s", label, labels[l]); row(name, "MB/s", (double)size, c, r)
        ROW("av_adler32_update", c_adler, r_adler);
        ROW("av_crc (CRC-32 LE)", c_crc, r_crc);
        ROW("md5", c_md5, r_md5);
        ROW("sha1", c_sha1, r_sha1);
        ROW("sha256", c_sha256, r_sha256);
        ROW("murmur3", c_mm3, r_mm3);
        ROW("av_base64_encode", c_b64e, r_b64e);
        ROW("av_base64_decode", c_b64d, r_b64d);
    }
    row("av_rescale_rnd (per call)", "ns", N_ARGS, c_rescale, r_rescale);
    row("av_gcd (per call)", "ns", N_ARGS, c_gcd, r_gcd);
    row("av_reduce (per call)", "ns", N_ARGS, c_reduce, r_reduce);
    row("av_rescale_q (per call)", "ns", N_ARGS, c_rescaleq, r_rescaleq);
    row("av_bessel_i0 (per call)", "ns", N_ARGS, c_bessel, r_bessel);
    row("av_mul_q (per call)", "ns", N_ARGS, c_mulq, r_mulq);
    row("av_add_q (per call)", "ns", N_ARGS, c_addq, r_addq);
    row("av_d2q (per call)", "ns", N_ARGS, c_d2q, r_d2q);
    row("av_nearer_q (per call)", "ns", N_ARGS, c_nearer, r_nearer);
    row("av_q2intfloat (per call)", "ns", N_ARGS, c_q2if, r_q2if);
    row("av_strcasecmp (per call)", "ns", N_WORDS * N_WORDS, c_casecmp, r_casecmp);
    row("av_stristr (per call)", "ns", N_WORDS * N_WORDS, c_stristr, r_stristr);
    row("av_match_name (per call)", "ns", N_WORDS, c_match, r_match);
    row("av_strstart (per call)", "ns", N_WORDS * N_PREFIXES, c_strstart, r_strstart);
    row("av_stristart (per call)", "ns", N_WORDS * N_PREFIXES, c_stristart, r_stristart);
    row("av_strlcpy (per call)", "ns", N_WORDS, c_strlcpy, r_strlcpy);
    row("av_parse_video_size (per call)", "ns", N_SIZES, c_vsize, r_vsize);
    row("av_find_info_tag (per call)", "ns", 4, c_info, r_info);
    return 0;
}
