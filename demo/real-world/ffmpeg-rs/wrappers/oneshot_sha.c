/* One-shot entry point over libavutil's SHA-1/SHA-2 (224, 256) context API: init with `bits`,
 * update in `chunk`-byte pieces (one piece when `chunk` <= 0), final. Returns the digest length
 * or av_sha_init's error. Part of everything-rust's ffmpeg
 * case study; LGPL-2.1-or-later like the libavutil code it calls (see ../LICENSE). */
#include <stdint.h>
#include <stddef.h>

#include "libavutil/mem.h"
#include "libavutil/sha.h"

int oneshot_sha(uint8_t *dst, int dst_size, int bits, const uint8_t *src, size_t len, int chunk)
{
    struct AVSHA *ctx;
    size_t done = 0;
    int ret;
    if (dst_size < 32)
        return -1;
    ctx = av_sha_alloc();
    if (!ctx)
        return -2;
    ret = av_sha_init(ctx, bits);
    if (ret < 0) {
        av_free(ctx);
        return ret;
    }
    do {
        size_t n = chunk > 0 && (size_t)chunk < len - done ? (size_t)chunk : len - done;
        av_sha_update(ctx, src + done, n);
        done += n;
    } while (done < len);
    av_sha_final(ctx, dst);
    av_free(ctx);
    return bits / 8;
}
