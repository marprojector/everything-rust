/* One-shot entry point over libavutil's MurmurHash3 context API: seeded init, update in
 * `chunk`-byte pieces (one piece when `chunk` <= 0), final. Part of everything-rust's ffmpeg case
 * study; LGPL-2.1-or-later like the libavutil code it calls (see ../LICENSE). */
#include <stdint.h>
#include <stddef.h>

#include "libavutil/mem.h"
#include "libavutil/murmur3.h"

int oneshot_murmur3(uint8_t *dst, int dst_size, uint64_t seed, const uint8_t *src, size_t len,
                    int chunk)
{
    struct AVMurMur3 *ctx;
    size_t done = 0;
    if (dst_size < 16)
        return -1;
    ctx = av_murmur3_alloc();
    if (!ctx)
        return -2;
    av_murmur3_init_seeded(ctx, seed);
    do {
        size_t n = chunk > 0 && (size_t)chunk < len - done ? (size_t)chunk : len - done;
        av_murmur3_update(ctx, src + done, n);
        done += n;
    } while (done < len);
    av_murmur3_final(ctx, dst);
    av_free(ctx);
    return 16;
}
